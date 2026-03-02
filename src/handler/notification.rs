use std::ops::ControlFlow;

use async_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, InitializedParams,
};
use camino::Utf8PathBuf;

use crate::{document_data::DocumentData, server::ServerState};

pub fn handle_did_save(
    server: &mut ServerState,
    DidSaveTextDocumentParams { text_document, .. }: DidSaveTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_did_save").entered();
    let path = Utf8PathBuf::from_path_buf(text_document.uri.to_file_path().unwrap()).unwrap();
    let analysis = server.snapshot().analysis;
    let contents = analysis.file_contents(&path);
    let parsed_document = analysis.parsed_document(&path);
    let diagnostics = parsed_document.syntax_errors(&contents);
    server.publish_diagnostics(text_document.uri, diagnostics, None);

    ControlFlow::Continue(())
}

pub fn handle_did_close(
    server: &mut ServerState,
    DidCloseTextDocumentParams { text_document }: DidCloseTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_did_close").entered();

    let path = Utf8PathBuf::from_path_buf(text_document.uri.to_file_path().unwrap()).unwrap();
    if server.documents.remove(&path).is_none() {
        tracing::error!(url = path.as_str(), "orphan DidCloseTextDocument");
    }
    ControlFlow::Continue(())
}

pub fn handle_did_change(
    server: &mut ServerState,
    DidChangeTextDocumentParams {
        text_document,
        content_changes,
    }: DidChangeTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_did_save").entered();
    let path = Utf8PathBuf::from_path_buf(text_document.uri.to_file_path().unwrap()).unwrap();
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

    let analysis = server.snapshot().analysis;
    let contents = analysis.file_contents(&path);
    let parsed_document = analysis.parsed_document(&path);
    let diagnostics = parsed_document.syntax_errors(&contents);
    server.publish_diagnostics(text_document.uri, diagnostics, None);

    ControlFlow::Continue(())
}

pub fn handle_did_open(
    server: &mut ServerState,
    DidOpenTextDocumentParams { text_document }: DidOpenTextDocumentParams,
) -> ControlFlow<async_lsp::Result<()>> {
    let _p = tracing::info_span!("handle_did_open").entered();
    let path = Utf8PathBuf::from_path_buf(text_document.uri.to_file_path().unwrap()).unwrap();
    let document = DocumentData {
        contents: text_document.text,
    };
    tracing::info!(
        "opened {} with content length {}",
        &path,
        document.contents.len()
    );
    server.documents.insert(path, document);
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
