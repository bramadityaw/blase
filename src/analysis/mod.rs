use std::sync::Arc;

use async_lsp::lsp_types;

use crate::db::{BladeDocument, RootDatabase, SourceFile, parse_document};

#[derive(Default)]
pub struct AnalysisHost {
    db: RootDatabase,
}

impl AnalysisHost {
    pub fn set_source_file(&mut self, url: lsp_types::Url, contents: &str) {
        self.db.set_source_file(url, contents);
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
    pub fn source_file(&self, url: &lsp_types::Url) -> SourceFile {
        self.db.source_file(url)
    }

    pub fn parsed_document(&self, url: &lsp_types::Url) -> BladeDocument {
        self.with_db(|db| {
            let source = self.source_file(url);
            parse_document(db, source)
        })
    }

    pub fn file_contents(&self, url: &lsp_types::Url) -> Arc<str> {
        self.with_db(|db| {
            let source = self.source_file(url);
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
