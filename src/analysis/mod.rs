use async_lsp::lsp_types;

use crate::db::{RootDatabase, SourceFile};

#[derive(Default)]
pub struct AnalysisHost {
    db: RootDatabase,
}

impl AnalysisHost {
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

    pub fn with_db<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&RootDatabase) -> R,
    {
        fun(&self.db)
    }
}
