//! This module is responsible for implementing handlers for Language Server
//! Protocol. This module specifically handles requests.

use async_lsp::{
    ErrorCode, ResponseError,
    lsp_types::{
        self, CompletionParams, CompletionResponse, GotoDefinitionParams, GotoDefinitionResponse,
        Hover, HoverContents, HoverParams, InitializeParams, InitializeResult, Location,
        MarkupContent, MarkupKind, ReferenceParams, ServerInfo, SignatureHelp, SignatureHelpParams,
    },
};
use camino::Utf8PathBuf;
use itertools::Itertools;
use line_index::TextRange;

use crate::{
    config,
    db::FileRange,
    lsp,
    server::{ServerState, ServerStateSnapshot},
};

pub fn handle_references(
    snap: ServerStateSnapshot,
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>, ResponseError> {
    let _i = tracing::info_span!("handle_references").entered();
    let config = &snap.config.read().expect("poison");
    let Some(position) = lsp::into_proto::cancellable(lsp::from_proto::file_position(
        &snap,
        &params.text_document_position,
    ))?
    else {
        return Ok(None);
    };

    let Some(refs) = lsp::into_proto::cancellable(snap.analysis.references(config, position))?
    else {
        return Ok(None);
    };

    let include_declaration = params.context.include_declaration;
    let locations = refs
        .into_iter()
        .flat_map(|refs| {
            let defs = if include_declaration {
                refs.defined_files.map(|files| {
                    files.into_iter().map(|path| FileRange {
                        path,
                        range: TextRange::default(),
                    })
                })
            } else {
                None
            }
            .into_iter()
            .flatten();

            refs.references
                .into_iter()
                .flat_map(|(path, ranges)| {
                    ranges
                        .into_iter()
                        .unique_by(|range| range.start())
                        .map(move |range| {
                            let path = path.to_owned();
                            FileRange { path, range }
                        })
                })
                .chain(defs)
        })
        .unique()
        .filter_map(|frange| {
            match lsp::into_proto::cancellable(lsp::into_proto::location(&snap, frange)).ok() {
                Some(loc) => loc,
                None => None,
            }
        })
        .collect::<Vec<_>>();

    Ok(Some(locations))
}

pub fn handle_completion(
    snap: ServerStateSnapshot,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>, ResponseError> {
    let _i = tracing::info_span!("handle_completion").entered();
    let config = &snap.config.read().expect("poison");
    let trigger_char = params
        .context
        .as_ref()
        .and_then(|s| s.trigger_character.as_ref())
        .and_then(|s| s.chars().next());
    let tdpp = &params.text_document_position;

    let position = match lsp::into_proto::cancellable(lsp::from_proto::file_position(&snap, tdpp))?
    {
        Some(position) => position,
        None => return Ok(None),
    };

    let line_index = match lsp::into_proto::cancellable(snap.file_line_index(&position.path))? {
        Some(index) => index,
        None => return Ok(None),
    };

    let items = match lsp::into_proto::cancellable(snap.analysis.completion(
        config,
        position,
        trigger_char,
    ))? {
        Some(items) => items,
        None => return Ok(None),
    };

    Ok(Some(lsp::into_proto::completion_response(
        config,
        &line_index,
        tdpp,
        items,
    )))
}

pub fn handle_signature_help(
    snap: ServerStateSnapshot,
    params: SignatureHelpParams,
) -> Result<Option<SignatureHelp>, ResponseError> {
    let _i = tracing::info_span!("handle_signature_help").entered();
    let config = snap.config.read().expect("poison");
    let position = match lsp::into_proto::cancellable(lsp::from_proto::file_position(
        &snap,
        &params.text_document_position_params,
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
        &params.text_document_position_params,
    ))? {
        Some(position) => position,
        None => return Ok(None),
    };
    let config = snap.config.read().expect("poison");
    let locations = lsp::into_proto::cancellable(snap.analysis.goto_def(&config, position))?
        .into_iter()
        .filter_map(|range| {
            let path = range.path.clone();
            match lsp::into_proto::location(&snap, range) {
                Ok(Some(loc)) => Some(loc),
                Ok(None) | Err(_) => {
                    tracing::error!("Line index not found");
                    tracing::error!(?path);
                    None
                }
            }
        })
        .collect::<Vec<lsp_types::Location>>();
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
        &params.text_document_position_params,
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
            config.client_info =
                params
                    .client_info
                    .map(|async_lsp::lsp_types::ClientInfo { name, version: _ }| {
                        config::ClientInfo { name }
                    });
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
        capabilities: crate::capabilities::server_capabilities(&config),
        server_info: Some(server_info),
    };

    Box::pin(async move { Ok(result) })
}
