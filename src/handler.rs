use std::ops::ControlFlow;

use async_lsp::{
    ErrorCode, ResponseError,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        HoverContents, HoverParams, InitializeParams, InitializeResult, InitializedParams,
        MarkupContent, ServerInfo, TextDocumentIdentifier, TextDocumentItem,
        TextDocumentPositionParams,
    },
};
use futures::future::BoxFuture;

use crate::{
    document_data::DocumentData,
    server::{ServerSnapshot, ServerState},
};

pub fn handle_did_save(
    server: &mut ServerState,
    params: DidSaveTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_did_save").entered();
    let DidSaveTextDocumentParams {
        text_document: TextDocumentIdentifier { uri: url },
        ..
    } = params;

    let analysis = server.snapshot().analysis;
    let contents = analysis.file_contents(&url);
    let parsed_document = analysis.parsed_document(&url);
    let diagnostics = parsed_document.syntax_errors(&contents);
    server.publish_diagnostics(url, diagnostics, None);

    ControlFlow::Continue(())
}

pub fn handle_did_change(
    server: &mut ServerState,
    params: DidChangeTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_did_save").entered();
    let url = params.text_document.uri;
    if let Some(mut document) = server.documents.get_mut(&url) {
        let new_contents = crate::util::apply_document_changes(
            server.negotiated_encoding(),
            &document.contents,
            dbg!(params.content_changes),
        );
        server
            .analysis_host
            .set_source_file(url.clone(), &new_contents);
        document.contents = new_contents;
    }

    let analysis = server.snapshot().analysis;
    let contents = analysis.file_contents(&url);
    let parsed_document = analysis.parsed_document(&url);
    let diagnostics = parsed_document.syntax_errors(&contents);
    server.publish_diagnostics(url, diagnostics, None);

    ControlFlow::Continue(())
}

pub fn handle_did_open(
    server: &mut ServerState,
    params: DidOpenTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_did_open").entered();
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
    tracing::info!(
        "opened {} with content length {}",
        &uri,
        document.contents.len()
    );
    server.documents.insert(uri, document);
    ControlFlow::Continue(())
}

pub fn handle_initialized(
    server: &mut ServerState,
    _params: InitializedParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_initialized").entered();
    let token = "blase/load_workspace".to_string();
    let progress_sender = server.with_report_progress(token);

    server.load_workspace(progress_sender);
    tracing::info!("finished initialized");

    ControlFlow::Continue(())
}

pub fn handle_initialize(
    server: &mut ServerState,
    params: InitializeParams,
) -> BoxFuture<'static, Result<InitializeResult, ResponseError>> {
    let _p = tracing::info_span!("handle_initialize").entered();
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
