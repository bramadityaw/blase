use std::ops::ControlFlow;

use async_lsp::{
    ErrorCode, ResponseError,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
        InitializeParams, InitializeResult, InitializedParams, MarkupContent, MarkupKind,
        ServerInfo, TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams,
    },
};
use futures::future::BoxFuture;

use crate::{
    document_data::DocumentData,
    lsp,
    server::{ServerSnapshot, ServerState},
};

macro_rules! box_future {
    ($val:expr) => {
        Box::pin(async move { $val })
    };
}

pub fn handle_goto_def(
    snap: ServerSnapshot,
    params: GotoDefinitionParams,
) -> BoxFuture<'static, Result<Option<GotoDefinitionResponse>, ResponseError>> {
    let _i = tracing::info_span!("handle_goto_def").entered();
    let TextDocumentPositionParams {
        text_document,
        position,
    } = params.text_document_position_params;
    let url = &text_document.uri;
    let locations = snap.analysis.goto_def(&snap, url, position);
    tracing::debug!(?locations);
    let response = match locations.len() {
        0 => None,
        1 => Some(GotoDefinitionResponse::Scalar(locations[0].clone())),
        2.. => Some(GotoDefinitionResponse::Array(locations)),
    };
    box_future!(Ok(response))
}

pub fn handle_hover(
    snap: ServerSnapshot,
    params: HoverParams,
) -> BoxFuture<'static, Result<Option<Hover>, ResponseError>> {
    let _i = tracing::info_span!("handle_hover").entered();
    let TextDocumentPositionParams {
        text_document,
        position,
    } = params.text_document_position_params;
    let url = &text_document.uri;
    tracing::info!(url=%url.path(), ?position);
    let line_col = lsp::from::line_col(position);
    let Some(text_size) = snap.analysis.line_index(url).offset(line_col) else {
        tracing::info!(url=%url.path(), "No offset found");
        return Box::pin(async move { Ok(None) });
    };
    let document = snap.analysis.parsed_document(url);
    let result = if let Some(node) = document.get_node_at(text_size) {
        let kind = node.kind();
        tracing::info!(kind);
        let value = format!(
            "{kind}: {}",
            &snap.analysis.file_contents(url)[node.byte_range()]
        );
        let contents = HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        });
        let range = lsp::into::range(node.start_position(), node.end_position());
        let hover = Hover {
            contents,
            range: Some(range),
        };
        Some(hover)
    } else {
        tracing::info!(?text_size, "no node found");
        None
    };
    tracing::debug!(?result);
    box_future!(Ok(result))
}

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
            params.content_changes,
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
                workspace_folder = ?server.config.read().expect("poison").workspace_folder,
                "using current directory",
            );
        }
        Some(folders) => {
            if folders.len() > 1 {
                let err = ResponseError::new(
                    ErrorCode::REQUEST_FAILED,
                    "Multiple workspaces not yet supported",
                );
                return box_future!(Err(err));
            }
            let mut config = server.config.write().expect("poison");
            config.capabilities = params.capabilities;
            let workspace_folder = folders[0].clone();
            tracing::debug!(url = workspace_folder.uri.path());
            config.workspace_folder = dbg!(workspace_folder.uri)
                .to_file_path()
                .expect("file:// urls should always be valid file paths");
        }
    }

    let config = server.config.read().expect("poison");

    let result = InitializeResult {
        capabilities: crate::server::server_capabilities(&config),
        server_info: Some(server_info),
    };

    Box::pin(async move { Ok(result) })
}
