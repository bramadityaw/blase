use std::{
    sync::{Arc, RwLock},
    thread,
};

use async_lsp::{
    ClientSocket, LanguageClient,
    client_monitor::ClientProcessMonitorLayer,
    concurrency::ConcurrencyLayer,
    lsp_types::{self, ClientCapabilities, ProgressParamsValue},
    panic::CatchUnwindBuilder,
    router::Router,
    server::LifecycleLayer,
    tracing::TracingLayer,
};
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam_channel::{Sender, unbounded};
use dashmap::DashMap;
use futures::{AsyncRead, AsyncWrite};
use tower::ServiceBuilder;

use crate::{
    analysis::{Analysis, AnalysisHost, Cancellable},
    config::Config,
    db::SourceDatabase,
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
        macro_rules! wrap_responder {
            ($( $path:ident )::+) => {
                |state, params| {
                    let snap = state.snapshot();
                    Box::pin(async move { $($path)::+(snap, params) })
                }
            };

            //TODO: Since some handlers take more than a second to respond,
            //we should notify the user of their request's progress
            //(PROGRESS $( $path:ident )::+) => {
            //    |server, params| {
            //        let token = stringify!(blase/$($path)::+);
            //        let progress_sender = server.with_report_progress(token.to_string());
            //        let snap = server.snapshot();
            //        Box::pin(async move { $($path)::+(snap, params, progress_sender) })
            //    }
            //};
        }

        router.event::<handler::Event>(handler::event::handle_event);

        router
            .request::<lsp_types::request::References, _>(wrap_responder!(
                handler::request::handle_references
            ))
            .request::<lsp_types::request::Completion, _>(wrap_responder!(
                handler::request::handle_completion
            ))
            .request::<lsp_types::request::SignatureHelpRequest, _>(wrap_responder!(
                handler::request::handle_signature_help
            ))
            .request::<lsp_types::request::HoverRequest, _>(wrap_responder!(
                handler::request::handle_hover
            ))
            .request::<lsp_types::request::GotoDefinition, _>(wrap_responder!(
                handler::request::handle_goto_def
            ))
            .request::<lsp_types::request::Initialize, _>(|state, params| {
                handler::request::handle_initialize(state, params)
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

pub struct ServerState {
    pub config: Arc<RwLock<Config>>,
    pub client: ClientSocket,
    pub documents: Arc<DashMap<Utf8PathBuf, DocumentData>>,
    pub analysis_host: AnalysisHost,
}

impl ServerState {
    pub fn new(client: ClientSocket) -> Self {
        let current_dir = std::env::current_dir().expect("cannot access current directory");
        let config = Config {
            capabilities: ClientCapabilities::default(),
            workspace_folder: Utf8PathBuf::from_path_buf(current_dir).unwrap(),
            client_info: None,
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
                if let Err(e) = socket.progress(lsp_types::ProgressParams {
                    token: lsp_types::NumberOrString::String(token.clone()),
                    value,
                }) {
                    tracing::error!(error=%e, "failed to report progress");
                }
            }
        });

        tx
    }

    pub fn snapshot(&self) -> ServerStateSnapshot {
        let config = Arc::clone(&self.config);
        let documents = Arc::clone(&self.documents);
        let analysis = self.analysis_host.analysis();
        ServerStateSnapshot {
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

pub struct ServerStateSnapshot {
    pub config: Arc<RwLock<Config>>,
    pub documents: Arc<DashMap<Utf8PathBuf, DocumentData>>,
    pub analysis: Analysis,
}

impl ServerStateSnapshot {
    pub fn get_document(
        &self,
        uri: &Utf8Path,
    ) -> Option<dashmap::mapref::one::Ref<'_, Utf8PathBuf, DocumentData>> {
        self.documents.get(uri)
    }

    pub fn workspace_folder(&self) -> Utf8PathBuf {
        let config = self.config.read().expect("poison");
        config.workspace_folder()
    }

    pub(crate) fn file_line_index(
        &self,
        path: &Utf8Path,
    ) -> Cancellable<Option<crate::line_index::LineIndex>> {
        let config = self.config.read().expect("poison");
        let line_index = self.analysis.with_db(|db| db.line_index(path))?;
        let endings = self
            .analysis
            .with_db(|db| db.source_file(path).map(|file| file.endings(db)))?;
        Ok(line_index.and_then(|index| {
            Some(crate::line_index::LineIndex {
                index,
                endings: endings?,
                encoding: config.negotiated_encoding(),
            })
        }))
    }
}
