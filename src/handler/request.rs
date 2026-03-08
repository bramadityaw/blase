//! This module is responsible for implementing handlers for Language Server
//! Protocol. This module specifically handles requests.

use std::sync::Arc;

use async_lsp::{
    ErrorCode, ResponseError,
    lsp_types::{
        GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
        InitializeParams, InitializeResult, MarkupContent, MarkupKind, ServerInfo, SignatureHelp,
        SignatureHelpParams, TextDocumentPositionParams,
    },
};
use camino::Utf8PathBuf;
use futures::future::BoxFuture;

use crate::{
    lsp,
    server::{ServerSnapshot, ServerState},
};

macro_rules! box_future {
    ($val:expr) => {
        Box::pin(async move { $val })
    };
}

pub fn handle_signature_help(
    snap: ServerSnapshot,
    params: SignatureHelpParams,
) -> BoxFuture<'static, Result<Option<SignatureHelp>, ResponseError>> {
    let _i = tracing::info_span!("handle_signature_help").entered();
    let TextDocumentPositionParams {
        text_document,
        position,
    } = params.text_document_position_params;
    let path = lsp::from::utf8_path(&text_document.uri);
    let line_col = lsp::from::line_col(position);
    let help = snap
        .analysis
        .signature_help(&snap, &path, line_col)
        .map(|(info, active)| SignatureHelp {
            signatures: vec![info],
            active_signature: Some(0),
            active_parameter: active.map(|x| x as u32),
        });
    box_future!(Ok(help))
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
    let path = lsp::from::utf8_path(&text_document.uri);
    let line_col = lsp::from::line_col(position);
    let locations = snap.analysis.goto_def(&snap, &path, line_col);
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
    let path = &lsp::from::utf8_path(&text_document.uri);
    tracing::info!(path = path.as_str(), ?position);
    let analysis = snap.analysis;
    let line_col = lsp::from::line_col(position);
    let Some(source_file) = analysis.source_file(path) else {
        tracing::info!(path = path.as_str(), "No source file found");
        return box_future!(Ok(None));
    };
    let Some(text_size) = analysis
        .with_db(|db| source_file.line_index(db))
        .ok()
        .and_then(|idx| idx.offset(line_col))
    else {
        tracing::info!(path = path.as_str(), "No offset found");
        return box_future!(Ok(None));
    };
    let result = if let Ok(Some(document)) = analysis.parsed_document(path)
        && let Some(node) = document.get_node_at(text_size)
        && let Ok(contents) = &analysis.with_db(|db| Arc::clone(source_file.contents(db)))
    {
        let kind = node.kind();
        tracing::info!(kind);
        let value = format!("{kind}: {}", &contents[node.byte_range()]);
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
                workspace_folder = %server.config.read().expect("poison").workspace_folder,
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
            let work = dbg!(workspace_folder.uri)
                .to_file_path()
                .expect("file:// urls should always be valid file paths");
            config.workspace_folder = Utf8PathBuf::from_path_buf(work).unwrap();
        }
    }

    let config = server.config.read().expect("poison");

    let result = InitializeResult {
        capabilities: crate::server::server_capabilities(&config),
        server_info: Some(server_info),
    };

    Box::pin(async move { Ok(result) })
}
