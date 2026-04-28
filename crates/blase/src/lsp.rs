//! Plumbing between application code and the Language Server Protocol

use async_lsp::{
    LanguageClient,
    lsp_types::{Diagnostic, PublishDiagnosticsParams, Url},
};

use crate::{handler::Event, server::ServerState};

impl ServerState {
    pub fn publish_diagnostics(
        &mut self,
        uri: Url,
        diagnostics: Vec<Diagnostic>,
        version: Option<i32>,
    ) {
        if let Err(e) = self.client.publish_diagnostics(PublishDiagnosticsParams {
            uri,
            diagnostics,
            version,
        }) {
            tracing::error!("Failed to publish diagnostics: {e}");
        };
    }

    pub fn emit(&mut self, event: Event) -> async_lsp::Result<()> {
        self.client.emit(event)
    }
}

pub mod from_proto;
pub mod into_proto;
