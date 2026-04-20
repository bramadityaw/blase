use std::{
    panic::{AssertUnwindSafe, UnwindSafe},
    sync::Arc,
};

use camino::{Utf8Path, Utf8PathBuf};
use line_index::LineIndex;

use crate::db::{
    DocumentDatabase, ParsedDocument, RootDatabase, SourceDatabase, SourceFile, parse_document,
};

#[derive(Default)]
pub struct AnalysisHost {
    db: RootDatabase,
}

#[cfg(test)]
mod fixture;

pub mod completions;
mod goto_definition;
pub mod hover;
mod lsp;
pub mod signature_help;

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

pub type Cancellable<R> = Result<R, salsa::Cancelled>;

impl Analysis {
    pub fn contents(&self, path: &Utf8Path) -> Option<&str> {
        let file = self.db.source_file(path)?;
        Some(&file.contents(&self.db))
    }

    pub fn source_file(&self, path: &Utf8Path) -> Option<SourceFile> {
        self.db.source_file(path)
    }

    pub fn file_exists(&self, path: &Utf8Path) -> bool {
        self.source_file(path).is_some()
    }

    pub fn raw_database(&self) -> &RootDatabase {
        &self.db
    }

    pub fn parse_errors(&self, path: &Utf8Path) -> Cancellable<Vec<crate::db::ParseError>> {
        self.with_db(|db| db.parse_errors(path))
    }

    pub fn parsed_document(&self, path: &Utf8Path) -> Cancellable<Option<ParsedDocument>> {
        match self.source_file(path) {
            Some(source) => self.with_db(|db| Some(parse_document(db, source))),
            None => Ok(None),
        }
    }

    pub fn line_index(&self, path: &Utf8Path) -> Cancellable<Option<Arc<LineIndex>>> {
        self.with_db(|db| db.line_index(path))
    }

    pub fn with_db<F, R>(&self, fun: F) -> Cancellable<R>
    where
        F: FnOnce(&RootDatabase) -> R + UnwindSafe,
    {
        let db = AssertUnwindSafe(&self.db);
        salsa::Cancelled::catch(|| fun(*db))
    }
}

#[test]
fn analysis_is_send() {
    fn is_send<T: Send>() {}
    is_send::<Analysis>();
}
