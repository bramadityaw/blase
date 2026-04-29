use std::ops::Not;

use camino::Utf8Path;

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FileRange, ParsedDocument, RootDatabase,
        def::{ComponentName, LayoutName, ViewName},
    },
};

pub enum SymbolKind {
    View,
    Component,
    Layout,
}

pub struct SymbolInformation {
    pub name: String,
    pub range: FileRange,
    pub kind: SymbolKind,
}

pub fn workspace_symbols(
    db: &RootDatabase,
    query: String,
    config: &Config,
) -> Option<Vec<SymbolInformation>> {
    let symbols = db
        .all_documents()
        .into_iter()
        .filter_map(|doc| filter_document(db, doc, &query, &config.workspace_folder))
        .collect::<Vec<_>>();

    symbols.is_empty().not().then_some(symbols)
}

fn filter_document(
    db: &dyn DocumentDatabase,
    doc: &ParsedDocument,
    query: &str,
    ws_path: &Utf8Path,
) -> Option<SymbolInformation> {
    if let Some(name) = ComponentName::from_document(db, doc, ws_path) {
        let tag_name = name.tag_name();
        if tag_name.contains(query) {
            let info = SymbolInformation {
                name: tag_name,
                range: FileRange {
                    path: doc.source.path(db).to_path_buf(),
                    range: Default::default(),
                },
                kind: SymbolKind::Component,
            };
            return Some(info);
        }
    }
    if let Some(name) = LayoutName::from_document(db, doc, ws_path) {
        let tag_name = name.tag_name();
        if tag_name.contains(query) {
            let info = SymbolInformation {
                name: tag_name,
                range: FileRange {
                    path: doc.source.path(db).to_path_buf(),
                    range: Default::default(),
                },
                kind: SymbolKind::Layout,
            };
            return Some(info);
        }
    }

    if let Some(name) = ViewName::from_document(db, doc, ws_path) {
        let tag_name = name.as_str();
        if tag_name.contains(query) {
            let info = SymbolInformation {
                name: tag_name.to_owned(),
                range: FileRange {
                    path: doc.source.path(db).to_path_buf(),
                    range: Default::default(),
                },
                kind: SymbolKind::Layout,
            };
            return Some(info);
        }
    }

    None
}
