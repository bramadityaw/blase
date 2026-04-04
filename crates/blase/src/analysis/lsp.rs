use async_lsp::lsp_types::Location;

use crate::{
    analysis::{Analysis, Cancellable, goto_definition, hover, signature_help},
    db::FilePosition,
    server::ServerSnapshot,
};

impl Analysis {
    #[tracing::instrument(skip(self, snap, position), level = "debug")]
    pub fn hover(
        &self,
        snap: &ServerSnapshot,
        position: FilePosition,
    ) -> Cancellable<Option<hover::HoverResult>> {
        let config = &snap.config.read().expect("poison");
        self.with_db(|db| hover::hover(db, config, position))
    }

    #[tracing::instrument(skip(self, snap, position), level = "debug")]
    pub fn signature_help(
        &self,
        snap: &ServerSnapshot,
        position: FilePosition,
    ) -> Cancellable<Option<signature_help::SignatureHelp>> {
        let config = &snap.config.read().expect("poison");
        self.with_db(|db| signature_help::signature_help(db, config, position))
    }

    #[tracing::instrument(skip(self, snap, position), level = "debug")]
    pub fn goto_def(
        &self,
        snap: &ServerSnapshot,
        position: FilePosition,
    ) -> Cancellable<Vec<Location>> {
        let config = &snap.config.read().expect("poison");
        self.with_db(|db| {
            goto_definition::goto_definition(db, config, position).unwrap_or_default()
        })
    }
}
