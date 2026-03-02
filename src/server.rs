use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
};

use async_lsp::{
    ClientSocket, LanguageClient,
    client_monitor::ClientProcessMonitorLayer,
    concurrency::ConcurrencyLayer,
    lsp_types::{
        self, ClientCapabilities, HoverProviderCapability, NumberOrString, PositionEncodingKind,
        ProgressParams, ProgressParamsValue, ServerCapabilities, TextDocumentSyncCapability,
        TextDocumentSyncKind, Url,
    },
    panic::CatchUnwindBuilder,
    router::Router,
    server::LifecycleLayer,
    tracing::TracingLayer,
};
use crossbeam_channel::{Sender, unbounded};
use dashmap::DashMap;
use futures::{AsyncRead, AsyncWrite};
use line_index::WideEncoding;
use tower::ServiceBuilder;

use crate::{
    analysis::{Analysis, AnalysisHost},
    config::Config,
    document_data::DocumentData,
    handler,
    line_index::PositionEncoding,
};

/// Adds calls to tracing to the default CatchUnwindLayer handler
#[tracing::instrument(level = "error")]
fn panic_handler(method: &str, payload: Box<dyn std::any::Any + Send>) -> async_lsp::ResponseError {
    let msg = match payload.downcast::<String>() {
        Ok(msg) => *msg,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(msg) => (*msg).into(),
            Err(_payload) => "unknown".into(),
        },
    };
    tracing::error!(msg);
    async_lsp::ResponseError::new(
        async_lsp::ErrorCode::INTERNAL_ERROR,
        format!("Request handler of {method} panicked: {msg}"),
    )
}

/// Runs the language server on buffered input
pub fn run_server(
    input: impl AsyncRead,
    output: impl AsyncWrite,
) -> impl Future<Output = async_lsp::Result<()>> {
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        let mut router = Router::new(ServerState::new(client.clone()));

        // Ignore any unsupported notifications;
        router.unhandled_notification(|_, notif| {
            tracing::info!(notif.method, "ignored unknown notification");
            std::ops::ControlFlow::Continue(())
        });

        // Requests
        router
            .request::<lsp_types::request::Initialize, _>(handler::request::handle_initialize)
            .request::<lsp_types::request::HoverRequest, _>(|state, params| {
                handler::request::handle_hover(state.snapshot(), params)
            })
            .request::<lsp_types::request::GotoDefinition, _>(|state, params| {
                handler::request::handle_goto_def(state.snapshot(), params)
            });

        // Notifications
        router
            .notification::<lsp_types::notification::Initialized>(
                handler::notification::handle_initialized,
            )
            .notification::<lsp_types::notification::DidChangeTextDocument>(
                handler::notification::handle_did_change,
            )
            .notification::<lsp_types::notification::DidOpenTextDocument>(
                handler::notification::handle_did_open,
            )
            .notification::<lsp_types::notification::DidCloseTextDocument>(
                handler::notification::handle_did_close,
            )
            .notification::<lsp_types::notification::DidSaveTextDocument>(
                handler::notification::handle_did_save,
            );

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindBuilder::new_with_handler(panic_handler))
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client))
            .service(router)
    });
    server.run_buffered(input, output)
}

pub fn server_capabilities(config: &Config) -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: match config.negotiated_encoding() {
            PositionEncoding::Utf8 => Some(PositionEncodingKind::UTF8),
            PositionEncoding::Wide(wide) => match wide {
                WideEncoding::Utf16 => Some(PositionEncodingKind::UTF16),
                WideEncoding::Utf32 => Some(PositionEncodingKind::UTF32),
                _ => None,
            },
        },
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        diagnostic_provider: None,
        completion_provider: None,
        document_formatting_provider: None,
        signature_help_provider: None,
        rename_provider: None,
        semantic_tokens_provider: None,

        // Methods below this line are unsupported
        selection_range_provider: None,
        type_definition_provider: None,
        implementation_provider: None,
        references_provider: None,
        document_highlight_provider: None,
        document_symbol_provider: None,
        workspace_symbol_provider: None,
        code_action_provider: None,
        code_lens_provider: None,
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        document_link_provider: None,
        color_provider: None,
        folding_range_provider: None,
        declaration_provider: None,
        execute_command_provider: None,
        workspace: None,
        call_hierarchy_provider: None,
        moniker_provider: None,
        linked_editing_range_provider: None,
        inline_value_provider: None,
        inlay_hint_provider: None,
        experimental: None,
    }
}

pub struct ServerState {
    pub config: Arc<RwLock<Config>>,
    pub client: ClientSocket,
    pub documents: Arc<DashMap<Url, DocumentData>>,
    pub analysis_host: AnalysisHost,
}

impl ServerState {
    pub fn new(client: ClientSocket) -> Self {
        let current_dir = std::env::current_dir().expect("cannot access current directory");
        let config = Config {
            capabilities: ClientCapabilities::default(),
            workspace_folder: current_dir,
        };
        Self {
            client,
            documents: Arc::new(DashMap::new()),
            config: Arc::new(RwLock::new(config)),
            analysis_host: AnalysisHost::default(),
        }
    }

    pub fn with_report_progress(&self, token: String) -> Sender<ProgressParamsValue> {
        let (tx, rx) = unbounded();
        let mut socket = self.client.clone();

        thread::spawn(move || {
            while let Ok(value) = rx.recv() {
                if let Err(e) = socket.progress(ProgressParams {
                    token: NumberOrString::String(token.clone()),
                    value,
                }) {
                    tracing::error!(error=%e, "failed to report progress");
                }
            }
        });

        tx
    }

    pub fn snapshot(&self) -> ServerSnapshot {
        let config = Arc::clone(&self.config);
        let documents = Arc::clone(&self.documents);
        let analysis = self.analysis_host.analysis();
        ServerSnapshot {
            documents,
            analysis,
            config,
        }
    }

    pub fn negotiated_encoding(&self) -> PositionEncoding {
        let config = self.config.read().expect("poison");
        config.negotiated_encoding()
    }
}

pub struct ServerSnapshot {
    pub config: Arc<RwLock<Config>>,
    pub documents: Arc<DashMap<Url, DocumentData>>,
    pub analysis: Analysis,
}

impl ServerSnapshot {
    pub fn get_document(
        &self,
        uri: &Url,
    ) -> Option<dashmap::mapref::one::Ref<'_, Url, DocumentData>> {
        self.documents.get(uri)
    }

    pub fn workspace_folder(&self) -> PathBuf {
        let config = self.config.read().expect("poison");
        config.workspace_folder.clone()
    }
}
