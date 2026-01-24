use std::{collections::HashMap, sync::Arc};

use line_index::WideEncoding;
use tokio::sync::{Mutex, RwLock};
use tower_lsp::{
    Client, LanguageServer, jsonrpc,
    lsp_types::{
        self, ClientCapabilities, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
        InitializeParams, InitializeResult, MessageType, PositionEncodingKind, ServerCapabilities,
        TextDocumentSyncKind, Url,
    },
};
use tree_sitter::Parser;

use crate::{document_data::DocumentData, handler, line_index::PositionEncoding};

pub struct Server {
    pub caps: RwLock<ClientCapabilities>,
    pub client: Client,
    pub documents: Arc<RwLock<HashMap<Url, DocumentData>>>,
    pub parser: Arc<Mutex<Parser>>,
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

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        handler::handle_initialize(self, params).await
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        handler::handle_did_open(self, params).await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        handler::handle_did_change(self, params).await
    }
}

impl Server {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            parser: Arc::new(Mutex::new(init_blade_parser())),
            caps: RwLock::new(ClientCapabilities::default()),
        }
    }

    pub async fn info_client(&self, message: &str) {
        tracing::info!("{}", message);
        self.client.log_message(MessageType::ERROR, message).await
    }

    pub async fn negotiated_encoding(&self) -> PositionEncoding {
        let caps = self.caps.read().await;
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
