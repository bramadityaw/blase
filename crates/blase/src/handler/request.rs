//! This module is responsible for implementing handlers for Language Server
//! Protocol. This module specifically handles requests.

use async_lsp::{
    ErrorCode, ResponseError,
    lsp_types::{
        GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
        InitializeParams, InitializeResult, MarkupContent, MarkupKind, ServerInfo, SignatureHelp,
        SignatureHelpParams,
    },
};
use camino::Utf8PathBuf;

use crate::{
    lsp,
    server::{ServerState, ServerStateSnapshot},
};

pub fn handle_signature_help(
    snap: ServerStateSnapshot,
    params: SignatureHelpParams,
) -> Result<Option<SignatureHelp>, ResponseError> {
    let _i = tracing::info_span!("handle_signature_help").entered();
    let config = snap.config.read().expect("poison");
    let position = match lsp::into_proto::cancellable(lsp::from_proto::file_position(
        &snap,
        params.text_document_position_params,
    ))? {
        Some(position) => position,
        None => return Ok(None),
    };
    let help = lsp::into_proto::cancellable(snap.analysis.signature_help(&config, position))?;
    let help = help
        .map(|help| lsp::into_proto::signature_help(help, config.signature_help_label_offsets()));
    Ok(help)
}

pub fn handle_goto_def(
    snap: ServerStateSnapshot,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>, ResponseError> {
    let _i = tracing::info_span!("handle_goto_def").entered();
    let position = match lsp::into_proto::cancellable(lsp::from_proto::file_position(
        &snap,
        params.text_document_position_params,
    ))? {
        Some(position) => position,
        None => return Ok(None),
    };
    let config = snap.config.read().expect("poison");
    let locations = lsp::into_proto::cancellable(snap.analysis.goto_def(&config, position))?;
    tracing::debug!(?locations);
    let response = match locations.len() {
        0 => None,
        1 => Some(GotoDefinitionResponse::Scalar(locations[0].clone())),
        2.. => Some(GotoDefinitionResponse::Array(locations)),
    };
    Ok(response)
}

pub fn handle_hover(
    snap: ServerStateSnapshot,
    params: HoverParams,
) -> Result<Option<Hover>, ResponseError> {
    let _i = tracing::info_span!("handle_hover").entered();
    let position = match lsp::into_proto::cancellable(lsp::from_proto::file_position(
        &snap,
        params.text_document_position_params,
    ))? {
        Some(position) => position,
        None => return Ok(None),
    };
    let config = snap.config.read().expect("poison");
    let (hover, line_index) = lsp::into_proto::cancellable((|| {
        let line_index = snap.file_line_index(&position.path)?;
        let hover = snap.analysis.hover(&config, position)?;
        Ok((hover, line_index))
    })())?;
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

pub fn handle_initialize(
    server: &mut ServerState,
    params: InitializeParams,
) -> futures::future::BoxFuture<'static, Result<InitializeResult, ResponseError>> {
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
                return Box::pin(async move { Err(err) });
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
