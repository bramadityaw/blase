//! This module is responsible for implementing handlers for Language Server
//! Protocol. This module specifically handles notifications.

use async_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, InitializedParams,
};
use std::ops::ControlFlow;

use crate::{document_data::DocumentData, lsp, server::ServerState};

#[tracing::instrument(skip(server))]
pub fn handle_did_save(
    server: &mut ServerState,
    DidSaveTextDocumentParams { text_document, .. }: DidSaveTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let path = lsp::from_proto::utf8_path(&text_document.uri);
    let analysis = server.snapshot().analysis;
    if let Ok(diagnostics) = analysis.parse_errors(&path) {
        server.publish_diagnostics(
            text_document.uri,
            diagnostics.into_iter().map(Into::into).collect(),
            None,
        );
    }

    if let Err(e) = server.emit(crate::handler::Event::DiagnosticUpdate(path)) {
        return ControlFlow::Break(Err(e));
    }

    ControlFlow::Continue(())
}

#[tracing::instrument(skip(server))]
pub fn handle_did_close(
    server: &mut ServerState,
    DidCloseTextDocumentParams { text_document }: DidCloseTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let path = lsp::from_proto::utf8_path(&text_document.uri);
    if server.documents.remove(&path).is_none() {
        tracing::error!(url = path.as_str(), "orphan DidCloseTextDocument");
    }
    ControlFlow::Continue(())
}

#[tracing::instrument(skip(server))]
pub fn handle_did_change(
    server: &mut ServerState,
    DidChangeTextDocumentParams {
        text_document,
        content_changes,
    }: DidChangeTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let path = lsp::from_proto::utf8_path(&text_document.uri);
    if let Some(mut document) = server.documents.get_mut(&path) {
        let new_contents = crate::util::apply_document_changes(
            server.negotiated_encoding(),
            &document.contents,
            content_changes,
        );
        server
            .analysis_host
            .set_source_file(path.clone(), &new_contents);
        document.contents = new_contents;
    }

    if let Err(e) = server.emit(crate::handler::Event::DiagnosticUpdate(path)) {
        return ControlFlow::Break(Err(e));
    }

    ControlFlow::Continue(())
}

#[tracing::instrument(skip(server))]
pub fn handle_did_open(
    server: &mut ServerState,
    DidOpenTextDocumentParams { text_document }: DidOpenTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let path = lsp::from_proto::utf8_path(&text_document.uri);
    let document = DocumentData {
        contents: text_document.text,
    };
    tracing::info!(
        "opened {} with content length {}",
        &path,
        document.contents.len()
    );
    server.documents.insert(path.clone(), document);

    if let Err(e) = server.emit(crate::handler::Event::DiagnosticUpdate(path)) {
        return ControlFlow::Break(Err(e));
    }

    ControlFlow::Continue(())
}

#[tracing::instrument(skip(server))]
pub fn handle_initialized(
    server: &mut ServerState,
    _params: InitializedParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let token = "blase/load_workspace".to_string();
    let progress_sender = server.with_report_progress(token);

    server.load_workspace(progress_sender);
    tracing::info!("finished initialized");

    ControlFlow::Continue(())
}
