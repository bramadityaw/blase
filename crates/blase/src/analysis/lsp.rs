use async_lsp::lsp_types::Location;
use camino::Utf8Path;
use line_index::LineCol;

use crate::{
    analysis::{Analysis, Cancellable, goto_definition, hover, signature_help},
    server::ServerSnapshot,
};

impl Analysis {
    #[tracing::instrument(skip(self, snap, line_col), level = "debug")]
    pub fn hover(
        &self,
        snap: &ServerSnapshot,
        doc_path: &Utf8Path,
        line_col: LineCol,
    ) -> Cancellable<Option<hover::HoverResult>> {
        let config = &snap.config.read().expect("poison");
        self.with_db(|db| hover::hover(db, config, doc_path, line_col))
    }

    #[tracing::instrument(skip(self, snap, line_col), level = "debug")]
    pub fn signature_help(
        &self,
        snap: &ServerSnapshot,
        doc_path: &Utf8Path,
        line_col: LineCol,
    ) -> Cancellable<Option<signature_help::SignatureHelp>> {
        let config = &snap.config.read().expect("poison");
        self.with_db(|db| signature_help::signature_help(db, config, doc_path, line_col))
    }

    #[tracing::instrument(skip(self, snap, line_col), level = "debug")]
    pub fn goto_def(
        &self,
        snap: &ServerSnapshot,
        doc_path: &Utf8Path,
        line_col: LineCol,
    ) -> Cancellable<Vec<Location>> {
        let config = &snap.config.read().expect("poison");
        self.with_db(|db| {
            goto_definition::goto_definition(db, doc_path, config, line_col).unwrap_or_default()
        })
    }
}
