use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use line_index::LineIndex;

use crate::db::{ParsedDocument, RootDatabase, SourceFile, parse_document};

#[derive(Default)]
pub struct AnalysisHost {
    db: RootDatabase,
}

mod lsp;

impl AnalysisHost {
    pub fn set_source_file(&mut self, path: Utf8PathBuf, contents: &str) {
        self.db.set_source_file(path, contents);
    }

    pub fn analysis(&self) -> Analysis {
        Analysis {
            db: self.db.clone(),
        }
    }

    pub fn raw_database(&self) -> &RootDatabase {
        &self.db
    }

    pub fn raw_database_mut(&mut self) -> &mut RootDatabase {
        &mut self.db
    }
}

pub struct Analysis {
    db: RootDatabase,
}

impl Analysis {
    pub fn source_file(&self, path: &Utf8Path) -> SourceFile {
        self.db.source_file(path)
    }

    pub fn raw_database(&self) -> &RootDatabase {
        &self.db
    }

    pub fn line_index(&self, path: &Utf8Path) -> LineIndex {
        let text = self.file_contents(path);
        LineIndex::new(&text)
    }

    pub fn parsed_document(&self, path: &Utf8Path) -> ParsedDocument {
        self.with_db(|db| {
            let source = self.source_file(path);
            parse_document(db, source)
        })
    }

    pub fn file_contents(&self, path: &Utf8Path) -> Arc<str> {
        self.with_db(|db| {
            let source = self.source_file(path);
            Arc::clone(&source.contents(db))
        })
    }

    pub fn with_db<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&RootDatabase) -> R,
    {
        fun(&self.db)
    }
}
