//! This module is responsible for implementing handlers for Language Server
//! Protocol. This module specifically handles requests.

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
    fn inner(
        snap: ServerSnapshot,
        params: SignatureHelpParams,
    ) -> Result<Option<SignatureHelp>, ResponseError> {
        let _i = tracing::info_span!("handle_signature_help").entered();
        let config = snap.config.read().expect("poison");
        let TextDocumentPositionParams {
            text_document,
            position,
        } = params.text_document_position_params;
        let path = lsp::from_proto::utf8_path(&text_document.uri);
        let line_col = lsp::from_proto::line_col(position);
        let help = snap.analysis.signature_help(&snap, &path, line_col);
        let help = lsp::into_proto::cancellable(help)?.map(|help| {
            lsp::into_proto::signature_help(help, config.signature_help_label_offsets())
        });
        Ok(help)
    }

    box_future!(inner(snap, params))
}

pub fn handle_goto_def(
    snap: ServerSnapshot,
    params: GotoDefinitionParams,
) -> BoxFuture<'static, Result<Option<GotoDefinitionResponse>, ResponseError>> {
    fn inner(
        snap: ServerSnapshot,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>, ResponseError> {
        let _i = tracing::info_span!("handle_goto_def").entered();
        let TextDocumentPositionParams {
            text_document,
            position,
        } = params.text_document_position_params;
        let path = lsp::from_proto::utf8_path(&text_document.uri);
        let line_col = lsp::from_proto::line_col(position);
        let locations =
            lsp::into_proto::cancellable(snap.analysis.goto_def(&snap, &path, line_col))?;
        tracing::debug!(?locations);
        let response = match locations.len() {
            0 => None,
            1 => Some(GotoDefinitionResponse::Scalar(locations[0].clone())),
            2.. => Some(GotoDefinitionResponse::Array(locations)),
        };
        Ok(response)
    }

    box_future!(inner(snap, params))
}

pub fn handle_hover(
    snap: ServerSnapshot,
    params: HoverParams,
) -> BoxFuture<'static, Result<Option<Hover>, ResponseError>> {
    fn inner(snap: ServerSnapshot, params: HoverParams) -> Result<Option<Hover>, ResponseError> {
        let _i = tracing::info_span!("handle_hover").entered();
        let TextDocumentPositionParams {
            text_document,
            position,
        } = params.text_document_position_params;
        let path = &lsp::from_proto::utf8_path(&text_document.uri);
        let line_col = lsp::from_proto::line_col(position);
        let hover = lsp::into_proto::cancellable(snap.analysis.hover(&snap, path, line_col))?;
        let line_index = lsp::into_proto::cancellable(snap.file_line_index(path))?;
        let hover_result = || {
            let hover = hover?;
            Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover.markup.into(),
                }),
                range: Some(lsp::into_proto::range(&line_index?, hover.range)),
            })
        };
        Ok(hover_result())
    }
    box_future!(inner(snap, params))
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
            let work = workspace_folder
                .uri
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
