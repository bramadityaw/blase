use std::{collections::HashMap, sync::Arc};

use line_index::WideEncoding;
use tokio::sync::{Mutex, RwLock};
use tower_lsp::{
    Client, LanguageServer, jsonrpc,
    lsp_types::{
        self, ClientCapabilities, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
        InitializeParams, InitializeResult, MessageType, PositionEncodingKind, ServerCapabilities,
        ServerInfo, TextDocumentItem, TextDocumentSyncKind, Url,
    },
};
use tree_sitter::Parser;

use crate::{document_data::DocumentData, line_index::PositionEncoding};

pub struct Server {
    caps: RwLock<ClientCapabilities>,
    client: Client,
    documents: Arc<RwLock<HashMap<Url, DocumentData>>>,
    parser: Arc<Mutex<Parser>>,
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

/// blase aims to support the following methods:
/// - Goto Definition
fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: Some(PositionEncodingKind::UTF8),
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        definition_provider: None,

        // Methods below this line are unsupported
        selection_range_provider: None,
        hover_provider: None,
        completion_provider: None,
        signature_help_provider: None,
        type_definition_provider: None,
        implementation_provider: None,
        references_provider: None,
        document_highlight_provider: None,
        document_symbol_provider: None,
        workspace_symbol_provider: None,
        code_action_provider: None,
        code_lens_provider: None,
        document_formatting_provider: None,
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        rename_provider: None,
        document_link_provider: None,
        color_provider: None,
        folding_range_provider: None,
        declaration_provider: None,
        execute_command_provider: None,
        workspace: None,
        call_hierarchy_provider: None,
        semantic_tokens_provider: None,
        moniker_provider: None,
        linked_editing_range_provider: None,
        inline_value_provider: None,
        inlay_hint_provider: None,
        diagnostic_provider: None,
        experimental: None,
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        let server_info = ServerInfo {
            name: "blase".into(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        };

        {
            let mut caps = self.caps.write().await;
            *caps = params.capabilities;
        }

        let result = InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(server_info),
            offset_encoding: None,
        };

        Ok(result)
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let DidOpenTextDocumentParams {
            text_document:
                TextDocumentItem {
                    uri,
                    language_id: _,
                    version,
                    text,
                },
        } = params;
        let mut documents = self.documents.write().await;
        let mut parser = self.parser.lock().await;
        let tree = parser
            .parse(&text, None)
            .expect("Language has been set at Server construction");
        let document = DocumentData {
            data: text.into_bytes(),
            tree,
            version,
        };
        let msg = format!(
            "Opened {}: {}",
            uri.as_str(),
            document.tree.root_node().to_sexp()
        );
        self.info_client(&msg).await;
        documents.insert(uri, document);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut documents = self.documents.write().await;
        if let Some(document) = documents.get_mut(&uri) {
            let new_contents = crate::util::apply_document_changes(
                self.negotiated_encoding().await,
                std::str::from_utf8(&document.data).unwrap(),
                params.content_changes,
            )
            .into_bytes();
            let mut parser = self.parser.lock().await;
            // Ideally, we also pass in the old tree.
            let tree = parser
                .parse(new_contents.as_slice(), None)
                .expect("Language has been set at Server construction");
            let msg = format!("Changed {}: {}", uri.as_str(), tree.root_node().to_sexp());
            self.info_client(&msg).await;
            *document = DocumentData {
                version: params.text_document.version,
                data: new_contents,
                tree,
            };
        }
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
