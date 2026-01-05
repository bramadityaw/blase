use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tower_lsp::{
    Client, LanguageServer,
    jsonrpc::{self, Result},
    lsp_types::{
        Diagnostic, DiagnosticSeverity, DidOpenTextDocumentParams, InitializeParams,
        InitializeResult, InitializedParams, MessageType, Position, Range, ServerCapabilities,
        ServerInfo, TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind,
    },
};
use tracing::{error, info};
use tree_sitter::{Node, Parser, Tree};

pub struct Server {
    client: Client,
    documents: Arc<RwLock<HashMap<String, Tree>>>,
    parser: Arc<RwLock<Parser>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, initialize_params: InitializeParams) -> Result<InitializeResult> {
        handle_initialize(self, &initialize_params)
    }

    async fn initialized(&self, _: InitializedParams) {
        let msg = "initialized blade language server";
        info!(msg);
        self.client.log_message(MessageType::INFO, msg).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        match handle_did_open(self, &params).await {
            Ok(_) => {}
            Err(err) => {
                let message = format!("failed opening file: {}", err);
                error!(message);
                self.client.log_message(MessageType::ERROR, message).await
            }
        }
    }
}

impl Server {
    pub fn new(client: Client) -> Self {
        let mut parser = Parser::new();
        if let Err(err) = parser.set_language(&tree_sitter_blade::LANGUAGE.into()) {
            panic!("Failed to initialize Blade parser: {}", err);
        }
        Self {
            client,
            parser: Arc::new(RwLock::new(parser)),
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn publish_diagnostics(server: &Server, document: &TextDocumentItem) -> Result<()> {
    let diagnostics = get_diagnostics(server, document)?;
    server
        .client
        .publish_diagnostics(document.uri.clone(), diagnostics, Some(document.version))
        .await;
    Ok(())
}

fn get_diagnostics(server: &Server, document: &TextDocumentItem) -> Result<Vec<Diagnostic>> {
    let mut parser = server.parser.write().map_err(|err| {
        error!("Failed to get on parser: {}", err);
        jsonrpc::Error::internal_error()
    })?;
    let uri = document.uri.to_string();
    let tree = {
        let mut documents = server.documents.write().map_err(|err| {
            error!("Failed to open document store: {}", err);
            jsonrpc::Error::internal_error()
        })?;
        let old_tree = documents.get(&uri);
        // TODO: call old_tree.edit() with appropriate InputEdit
        let tree = parser
            .parse(document.text.clone(), old_tree)
            .expect("Language is set on server creation");
        documents
            .insert(uri, tree)
            .expect("Old tree has been getted before")
    };

    Ok(diagnose_syntax(&tree, document.text.as_bytes()))
}

/// Convert a tree-sitter node to an LSP range
fn node_to_range(node: &Node) -> Range {
    let start_position = node.start_position();
    let end_position = node.end_position();

    Range {
        start: Position {
            line: start_position.row as u32,
            character: start_position.column as u32,
        },
        end: Position {
            line: end_position.row as u32,
            character: end_position.column as u32,
        },
    }
}
/// Generate a descriptive error message for a node
fn get_error_message(node: &Node, source_code: impl AsRef<[u8]>) -> String {
    let node_kind = node.kind();

    // Try to get some context about what was expected
    let mut message = match node_kind {
        "ERROR" => "Syntax error".to_string(),
        _ => format!("Unexpected syntax: {}", node_kind),
    };

    // If the node has children that might give us more info
    let child_count = node.child_count();
    if child_count == 0 {
        // Try to get the text if it's a leaf node
        let node_text = node
            .utf8_text(source_code.as_ref())
            .unwrap_or_default()
            .to_string();

        if !node_text.is_empty() && node_text.len() < 50 {
            message = format!("Unexpected token: '{}'", node_text);
        }
    }

    message
}

fn diagnose_syntax(tree: &Tree, source_code: &[u8]) -> Vec<Diagnostic> {
    let root_node = tree.root_node();
    let mut diagnostics = Vec::new();

    // Walk the tree using a stack for depth-first traversal
    let mut stack = vec![root_node];

    while let Some(node) = stack.pop() {
        // Check if this node has an error
        if node.is_error() || node.is_missing() {
            let range = node_to_range(&node);
            let message = if node.is_missing() {
                format!("Missing {}", node.kind())
            } else {
                // For error nodes, try to get more specific error info
                get_error_message(&node, source_code)
            };

            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("tree-sitter".to_string()),
                message,
                related_information: None,
                tags: None,
                data: None,
            });
        }

        // Add children to the stack for further traversal
        let mut child = node.child(0);
        let mut children = Vec::new();

        while let Some(c) = child {
            children.push(c);
            child = c.next_sibling();
        }

        // Reverse to maintain correct order (since we're using a stack)
        stack.extend(children.into_iter().rev());
    }

    diagnostics
}

async fn handle_did_open(server: &Server, params: &DidOpenTextDocumentParams) -> Result<()> {
    let text_document = params.text_document.clone();
    server
        .client
        .log_message(
            MessageType::INFO,
            format!("File opened: {}", &text_document.uri),
        )
        .await;
    publish_diagnostics(server, &text_document).await
}

fn server_capabilities() -> ServerCapabilities {
    let ServerCapabilities {
        text_document_sync: _,
        diagnostic_provider,
        hover_provider,
        completion_provider,
        signature_help_provider,
        definition_provider,
        implementation_provider,
        semantic_tokens_provider,
        document_formatting_provider,

        position_encoding,
        selection_range_provider,
        type_definition_provider,
        references_provider,
        document_highlight_provider,
        document_symbol_provider,
        workspace_symbol_provider,
        code_action_provider,
        code_lens_provider,
        document_range_formatting_provider,
        document_on_type_formatting_provider,
        rename_provider,
        document_link_provider,
        color_provider,
        folding_range_provider,
        declaration_provider,
        execute_command_provider,
        workspace,
        call_hierarchy_provider,
        moniker_provider,
        linked_editing_range_provider,
        inline_value_provider,
        inlay_hint_provider,
        experimental,
    } = ServerCapabilities::default();

    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        diagnostic_provider,          // TODO
        definition_provider,          // TODO
        hover_provider,               // TODO
        completion_provider,          // TODO
        signature_help_provider,      // TODO
        implementation_provider,      // TODO
        semantic_tokens_provider,     // TODO
        document_formatting_provider, // TODO

        position_encoding,
        selection_range_provider,
        type_definition_provider,
        references_provider,
        document_highlight_provider,
        document_symbol_provider,
        workspace_symbol_provider,
        code_action_provider,
        code_lens_provider,
        document_range_formatting_provider,
        document_on_type_formatting_provider,
        rename_provider,
        document_link_provider,
        color_provider,
        folding_range_provider,
        declaration_provider,
        execute_command_provider,
        workspace,
        call_hierarchy_provider,
        moniker_provider,
        linked_editing_range_provider,
        inline_value_provider,
        inlay_hint_provider,
        experimental,
    }
}

fn handle_initialize(server: &Server, params: &InitializeParams) -> Result<InitializeResult> {
    let server_capabilities = server_capabilities();
    let InitializeResult {
        capabilities: _,
        server_info: _,
        offset_encoding,
    } = InitializeResult::default();

    info!("sending initialize response");

    Ok(InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(ServerInfo {
            name: "blase".into(),
            version: Some(String::from(env!("CARGO_PKG_VERSION"))),
        }),
        offset_encoding,
    })
}
