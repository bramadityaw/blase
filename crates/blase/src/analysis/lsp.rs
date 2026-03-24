use async_lsp::lsp_types::Location;
use camino::Utf8Path;
use line_index::LineCol;

use crate::{
    analysis::{Analysis, goto_definition, signature_help},
    server::ServerSnapshot,
};

impl Analysis {
    pub fn signature_help(
        &self,
        snap: &ServerSnapshot,
        doc_path: &Utf8Path,
        line_col: LineCol,
    ) -> Option<signature_help::SignatureHelp> {
        let config = &snap.config.read().expect("poison");
        let result =
            self.with_db(|db| signature_help::signature_help(db, config, doc_path, line_col));
        match result {
            Ok(result) => result,
            Err(_) => None,
        }
    }

    pub fn goto_def(
        &self,
        snap: &ServerSnapshot,
        doc_path: &Utf8Path,
        line_col: LineCol,
    ) -> Vec<Location> {
        let config = &snap.config.read().expect("poison");
        self.with_db(|db| {
            goto_definition::goto_definition(db, doc_path, config, line_col).unwrap_or_default()
        })
        .unwrap()
    }
}
