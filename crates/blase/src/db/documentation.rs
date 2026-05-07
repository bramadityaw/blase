use type_sitter::{HasChildren, Node};

use crate::db::def::{Component, DefDatabase, Document, DocumentId, Layout};

/// Holds documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Documentation(String);

impl Documentation {
    pub fn new(s: String) -> Self {
        Documentation(s)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Documentation> for String {
    fn from(value: Documentation) -> Self {
        value.0
    }
}

pub trait HasDocs: Document + Copy {
    fn docs(&self, db: &dyn DefDatabase) -> Option<Documentation> {
        let id = self.id();
        let document = &id.document(db);
        let doc = document
            .root_node()
            .downcast::<ast::blade::Document>()
            .ok()?;
        let child = doc.children(&mut doc.walk()).next()?.ok()?;
        use ast::blade::anon_unions::Anon122367149080002252186915888317997925741::Comment;
        let comment = match child {
            Comment(comment) => document.text_for_node(db, comment)?,
            _ => return None,
        };
        extract_docs(comment)
    }
}

impl HasDocs for Layout {}

impl HasDocs for Component {}

fn extract_docs(comment: &str) -> Option<Documentation> {
    let text = comment.strip_prefix("{{")?.strip_suffix("}}")?;
    let documentation = text
        .split_inclusive('\n')
        .map(|s| {
            let text = s.strip_prefix("---").or(s.strip_prefix("--")).unwrap_or(s);
            let trimmed = text_procs::trim_indent(text);
            if trimmed.starts_with("\n") {
                trimmed + "<br/>"
            } else {
                trimmed
            }
        })
        .collect::<String>();
    Some(Documentation::new(documentation))
}
