use tower_lsp::{
    jsonrpc,
    lsp_types::{
        self, Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, InitializeResult,
        ServerInfo, TextDocumentIdentifier, TextDocumentItem,
    },
};
use tree_sitter::{Query, QueryCursor, StreamingIterator};

use crate::{Server, document_data::DocumentData};

fn syntax_errors(doc: &DocumentData) -> Vec<Diagnostic> {
    const ERROR_QUERY: &'static str = "(ERROR) @error";
    Query::new(&doc.tree.language(), ERROR_QUERY)
        .map(|query| {
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&query, doc.tree.root_node(), doc.data.as_slice());

            let mut diags = Vec::new();
            while let Some(m) = matches.next() {
                for capture in m.captures.iter() {
                    let node = capture.node;
                    diags.push(Diagnostic {
                        range: lsp_types::Range {
                            start: lsp_types::Position {
                                line: node.start_position().row as u32,
                                character: node.start_position().column as u32,
                            },
                            end: lsp_types::Position {
                                line: node.end_position().row as u32,
                                character: node.end_position().column as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: None,
                        message: "Syntax error!".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
            }
            diags
        })
        .unwrap_or_else(|e| {
            tracing::error!("Error querying for syntax errors: {}", e);
            Vec::new()
        })
}

pub async fn handle_did_save(server: &Server, params: DidSaveTextDocumentParams) {
    let DidSaveTextDocumentParams {
        text_document: TextDocumentIdentifier { uri },
        ..
    } = params;
    let snap = server.snapshot();
    if let Some(doc) = snap.get_document(&uri).await {
        let diagnostics = syntax_errors(&doc);
        if !diagnostics.is_empty() {
            server
                .client
                .publish_diagnostics(uri, diagnostics, Some(doc.version))
                .await
        }
    };
}

pub async fn handle_did_change(server: &Server, params: DidChangeTextDocumentParams) {
    let uri = params.text_document.uri;
    let mut documents = server.documents.write().await;
    if let Some(document) = documents.get_mut(&uri) {
        let new_contents = crate::util::apply_document_changes(
            server.negotiated_encoding().await,
            std::str::from_utf8(&document.data).unwrap(),
            params.content_changes,
        )
        .into_bytes();
        let mut parser = server.parser.lock().await;
        // Ideally, we also pass in the old tree.
        let tree = parser
            .parse(new_contents.as_slice(), None)
            .expect("Language has been set at Server construction");
        *document = DocumentData {
            version: params.text_document.version,
            data: new_contents,
            tree,
        };
        let diagnostics = syntax_errors(&document);
        if !diagnostics.is_empty() {
            server
                .client
                .publish_diagnostics(uri, diagnostics, Some(params.text_document.version))
                .await
        }
    }
}

pub async fn handle_did_open(server: &Server, params: DidOpenTextDocumentParams) {
    let DidOpenTextDocumentParams {
        text_document:
            TextDocumentItem {
                uri,
                language_id: _,
                version,
                text,
            },
    } = params;
    let mut documents = server.documents.write().await;
    let mut parser = server.parser.lock().await;
    let tree = parser
        .parse(&text, None)
        .expect("Language has been set at Server construction");
    let document = DocumentData {
        data: text.into_bytes(),
        tree,
        version,
    };
    documents.insert(uri, document);
}

pub async fn handle_initialize(
    server: &Server,
    params: InitializeParams,
) -> jsonrpc::Result<InitializeResult> {
    let server_info = ServerInfo {
        name: "blase".into(),
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
    };

    {
        let mut caps = server.caps.write().await;
        *caps = params.capabilities;
    }

    let result = InitializeResult {
        capabilities: crate::server::server_capabilities(),
        server_info: Some(server_info),
        offset_encoding: None,
    };

    Ok(result)
}
