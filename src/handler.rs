use std::ops::ControlFlow;

use async_lsp::{
    ErrorCode, ResponseError,
    lsp_types::{
        self, Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, InitializeResult,
        InitializedParams, ServerInfo, TextDocumentIdentifier, TextDocumentItem,
    },
};
use futures::future::BoxFuture;
use tree_sitter::{Query, QueryCursor, StreamingIterator};

use crate::{
    db::{BladeDocument, parse_document},
    document_data::DocumentData,
    server::ServerState,
};

fn syntax_errors(doc: &BladeDocument, contents: &str) -> Vec<Diagnostic> {
    const ERROR_QUERY: &'static str = "(ERROR) @error";
    Query::new(&doc.tree.language(), ERROR_QUERY)
        .map(|query| {
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&query, doc.tree.root_node(), contents.as_bytes());

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
        ..
    } = params;

    let analysis = server.snapshot().analysis;
    let source = analysis.source_file(&uri);
    let contents = analysis.with_db(|db| source.contents(db).clone());
    let parsed_document = analysis.with_db(|db| parse_document(db, source));
    let diagnostics = syntax_errors(&parsed_document, &contents);
    server.publish_diagnostics(uri, diagnostics, None);

    ControlFlow::Continue(())
}

pub fn handle_did_change(
    server: &mut ServerState,
    params: DidChangeTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let url = params.text_document.uri;
    if let Some(mut document) = server.documents.get_mut(&url) {
        let new_contents = crate::util::apply_document_changes(
            server.negotiated_encoding(),
            &document.contents,
            params.content_changes,
        );
        let db = server.analysis_host.raw_database_mut();
        db.set_source_file(url.clone(), &new_contents);
        document.contents = new_contents;
    }

    let analysis = server.snapshot().analysis;
    let source = analysis.source_file(&url);
    let contents = analysis.with_db(|db| source.contents(db).clone());
    let parsed_document = analysis.with_db(|db| parse_document(db, source));
    let diagnostics = syntax_errors(&parsed_document, &contents);
    server.publish_diagnostics(url, diagnostics, None);

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
                version: _,
                text,
            },
    } = params;
    let document = DocumentData { contents: text };
    server.documents.insert(uri, document);
    ControlFlow::Continue(())
}

pub fn handle_initialized(
    server: &mut ServerState,
    _params: InitializedParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let token = "blase/load_workspace".to_string();
    let progress_sender = server.with_report_progress(token);

    server.load_workspace(progress_sender);

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

    match params.workspace_folders {
        None => {
            tracing::info!(
                "using current directory: {}",
                server
                    .config
                    .read()
                    .expect("poison")
                    .workspace_folder
                    .uri
                    .to_string()
            );
        }
        Some(folders) => {
            if folders.len() > 1 {
                let err = ResponseError::new(
                    ErrorCode::REQUEST_FAILED,
                    "Multiple workspaces not yet supported",
                );
                return Box::pin(async move { Err(err) });
            }
            let mut config = server.config.write().expect("poison");
            config.capabilities = params.capabilities;
            let workspace_folder = folders[0].clone();
            config.workspace_folder = workspace_folder;
        }
    }

    let result = InitializeResult {
        capabilities: crate::server::server_capabilities(),
        server_info: Some(server_info),
    };

    Box::pin(async move { Ok(result) })
}
