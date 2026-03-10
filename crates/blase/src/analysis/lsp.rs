use async_lsp::lsp_types::{Location, SignatureInformation};
use camino::Utf8Path;
use line_index::LineCol;

use crate::{
    analysis::{Analysis, goto_definition},
    db::SourceDatabase,
    server::ServerSnapshot,
};

impl Analysis {
    pub fn signature_help(
        &self,
        snap: &ServerSnapshot,
        path: &Utf8Path,
        line_col: LineCol,
    ) -> Option<(SignatureInformation, Option<usize>)> {
        let document = self.parsed_document(path).ok()??;
        let line_index = self.with_db(|db| db.line_index(path)).ok()??;
        let offset = line_index.offset(line_col)?;
        let node = document.get_node_at(offset)?;
        todo!()
    }

    pub fn goto_def(
        &self,
        snap: &ServerSnapshot,
        doc_path: &Utf8Path,
        line_col: LineCol,
    ) -> Vec<Location> {
        let work_path = &snap.workspace_folder();
        self.with_db(|db| {
            goto_definition::goto_definition(db, doc_path, work_path, line_col).unwrap_or_default()
        })
        .unwrap()
    }
}
