use std::ops::ControlFlow;

use async_lsp::{
    LanguageClient, ResponseError,
    lsp_types::{
        self, Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, InitializeResult,
        PublishDiagnosticsParams, ServerInfo, TextDocumentIdentifier, TextDocumentItem,
    },
};
use futures::future::BoxFuture;
use tree_sitter::{Query, QueryCursor, StreamingIterator};

use crate::{document_data::DocumentData, server::ServerState};

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

pub fn handle_did_save(
    server: &mut ServerState,
    params: DidSaveTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let DidSaveTextDocumentParams {
        text_document: TextDocumentIdentifier { uri },
        text,
    } = params;
    let snap = server.snapshot();
    if let Some(doc) = snap.get_document(&uri) {
        let diagnostics = syntax_errors(&doc);
        if !diagnostics.is_empty() {
            server.publish_diagnostics(uri, diagnostics, None);
        }
    };

    ControlFlow::Continue(())
}

pub fn handle_did_change(
    server: &mut ServerState,
    params: DidChangeTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let uri = params.text_document.uri;
    {
        let mut documents = server.documents.write().expect("poison");
        if let Some(document) = documents.get_mut(&uri) {
            let new_contents = crate::util::apply_document_changes(
                server.negotiated_encoding(),
                std::str::from_utf8(&document.data).unwrap(),
                params.content_changes,
            )
            .into_bytes();
            let mut parser = server.parser.lock().expect("poison");
            let tree = parser
                .parse(new_contents.as_slice(), None)
                .expect("Language has been set at Server construction");
            *document = DocumentData {
                version: params.text_document.version,
                data: new_contents,
                tree,
            };
        }
    }
    if let Some(doc) = server.snapshot().get_document(&uri) {
        let diagnostics = syntax_errors(&doc);
        if !diagnostics.is_empty() {
            server.publish_diagnostics(uri, diagnostics, None);
        }
    };
    ControlFlow::Continue(())
}

pub fn handle_did_open(
    server: &mut ServerState,
    params: DidOpenTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let DidOpenTextDocumentParams {
        text_document:
            TextDocumentItem {
                uri,
                language_id: _,
                version,
                text,
            },
    } = params;
    let mut documents = server.documents.write().expect("poison");
    let mut parser = server.parser.lock().expect("poison");
    let tree = parser
        .parse(&text, None)
        .expect("Language has been set at Server construction");
    let document = DocumentData {
        data: text.into_bytes(),
        tree,
        version,
    };
    documents.insert(uri, document);
    ControlFlow::Continue(())
}

pub fn handle_initialize(
    server: &mut ServerState,
    params: InitializeParams,
) -> BoxFuture<'static, Result<InitializeResult, ResponseError>> {
    let server_info = ServerInfo {
        name: "blase".into(),
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
    };

    {
        let mut caps = server.caps.write().expect("poison");
        *caps = params.capabilities;
    }

    let result = InitializeResult {
        capabilities: crate::server::server_capabilities(),
        server_info: Some(server_info),
    };

    Box::pin(async move { Ok(result) })
}
