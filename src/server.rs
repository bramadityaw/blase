use std::{
    sync::{Arc, RwLock},
    thread,
};

use async_lsp::{
    ClientSocket, LanguageClient,
    client_monitor::ClientProcessMonitorLayer,
    concurrency::ConcurrencyLayer,
    lsp_types::{
        self, ClientCapabilities, NumberOrString, PositionEncodingKind, ProgressParams,
        ProgressParamsValue, ServerCapabilities, TextDocumentSyncKind, Url, WorkspaceFolder,
    },
    panic::CatchUnwindLayer,
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
    document_data::DocumentData,
    handler,
    line_index::PositionEncoding,
};

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
        router.request::<lsp_types::request::Initialize, _>(handler::handle_initialize);

        // Notifications
        router
            .notification::<lsp_types::notification::Initialized>(handler::handle_initialized)
            .notification::<lsp_types::notification::DidChangeTextDocument>(
                handler::handle_did_change,
            )
            .notification::<lsp_types::notification::DidOpenTextDocument>(handler::handle_did_open)
            .notification::<lsp_types::notification::DidSaveTextDocument>(handler::handle_did_save);

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client))
            .service(router)
    });
    server.run_buffered(input, output)
}

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: Some(PositionEncodingKind::UTF8),
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        diagnostic_provider: None,
        definition_provider: None,
        completion_provider: None,
        document_formatting_provider: None,
        signature_help_provider: None,
        hover_provider: None,
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

pub struct Config {
    pub capabilities: ClientCapabilities,
    pub workspace_folder: WorkspaceFolder,
}

pub struct ServerState {
    pub config: Arc<RwLock<Config>>,
    pub client: ClientSocket,
    pub documents: Arc<DashMap<Url, DocumentData>>,
    pub analysis_host: AnalysisHost,
}

pub struct ServerSnapshot {
    pub documents: Arc<DashMap<Url, DocumentData>>,
    pub analysis: Analysis,
}

impl ServerState {
    pub fn new(client: ClientSocket) -> Self {
        let current_dir = std::env::current_dir().expect("cannot access current directory");
        let uri = Url::from_file_path(current_dir.clone()).unwrap();
        let config = Config {
            capabilities: ClientCapabilities::default(),
            workspace_folder: WorkspaceFolder {
                uri,
                name: current_dir.display().to_string(),
            },
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
                    tracing::error!(error=%e, "failed to report parse progress");
                }
            }
        });

        tx
    }

    pub fn snapshot(&self) -> ServerSnapshot {
        let documents = Arc::clone(&self.documents);
        let analysis = self.analysis_host.analysis();
        ServerSnapshot {
            documents,
            analysis,
        }
    }

    pub fn negotiated_encoding(&self) -> PositionEncoding {
        let config = self.config.read().expect("poison");
        let client_encodings = match &config.capabilities.general {
            Some(general) => general.position_encodings.as_deref().unwrap_or_default(),
            None => &[],
        };
        for enc in client_encodings {
            if enc == &PositionEncodingKind::UTF8 {
                return PositionEncoding::Utf8;
            } else if enc == &PositionEncodingKind::UTF32 {
                return PositionEncoding::Wide(WideEncoding::Utf32);
            }
            // NB: intentionally prefer just about anything else to utf-16.
        }

        PositionEncoding::Wide(WideEncoding::Utf16)
    }
}

impl ServerSnapshot {
    pub fn get_document(
        &self,
        uri: &Url,
    ) -> Option<dashmap::mapref::one::Ref<'_, Url, DocumentData>> {
        self.documents.get(uri)
    }
}
