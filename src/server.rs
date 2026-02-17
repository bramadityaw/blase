use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use async_lsp::{
    ClientSocket,
    client_monitor::ClientProcessMonitorLayer,
    concurrency::ConcurrencyLayer,
    lsp_types::{
        self, ClientCapabilities, PositionEncodingKind, ServerCapabilities, TextDocumentSyncKind,
        Url,
    },
    panic::CatchUnwindLayer,
    router::Router,
    server::LifecycleLayer,
    tracing::TracingLayer,
};
use futures::{AsyncRead, AsyncWrite};
use line_index::WideEncoding;
use tower::ServiceBuilder;
use tree_sitter::Parser;

use crate::{document_data::DocumentData, handler, line_index::PositionEncoding};

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

fn init_blade_parser() -> Parser {
    let mut parser = Parser::new();
    let language: tree_sitter::Language = tree_sitter_blade::LANGUAGE.into();
    if let Err(err) = parser.set_language(&language) {
        match err {
            tree_sitter::LanguageError::Version(ver) => panic!(
                "
Mismatched versions:
   tree_sitter: {}
   tree_sitter_blade: {}
",
                ver,
                language.abi_version()
            ),
        };
    }
    parser
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

pub struct ServerState {
    pub caps: RwLock<ClientCapabilities>,
    pub client: ClientSocket,
    pub documents: Arc<RwLock<HashMap<Url, DocumentData>>>,
    pub parser: Arc<Mutex<Parser>>,
}

pub struct ServerSnapshot {
    pub documents: Arc<RwLock<HashMap<Url, DocumentData>>>,
}

impl ServerState {
    pub fn new(client: ClientSocket) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            parser: Arc::new(Mutex::new(init_blade_parser())),
            caps: RwLock::new(ClientCapabilities::default()),
        }
    }

    pub fn snapshot(&self) -> ServerSnapshot {
        let documents = Arc::clone(&self.documents);
        ServerSnapshot { documents }
    }

    pub fn negotiated_encoding(&self) -> PositionEncoding {
        let caps = self.caps.read().expect("poison");
        let client_encodings = match &caps.general {
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
    pub fn get_document(&self, uri: &Url) -> Option<DocumentData> {
        let documents = self.documents.read().expect("poison");
        documents.get(uri).cloned()
    }
}
