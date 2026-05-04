use line_index::TextRange;
use smol_str::SmolStr;

use crate::{analysis::completions::CompletionRelevance, db::text_edit::TextEdit};

#[derive(Clone)]
pub struct CompletionItem {
    /// Label in the completion pop up which identifies completion.
    pub label: String,
    pub kind: CompletionItemKind,
    /// The sequence of edits that happen when a user selects this item.
    pub edit: TextEdit,
    /// Range of identifier that is being completed.
    ///
    /// `source_range` must contain the completion offset. `text_edit` should
    /// start with what `source_range` points to, or VSCode will filter out the
    /// completion silently.
    pub source_range: TextRange,

    pub lookup: SmolStr,
    pub relevance: CompletionRelevance,
}

impl CompletionItem {
    pub fn lookup(&self) -> &str {
        self.lookup.as_str()
    }
}

// We use custom debug for CompletionItem to make snapshot tests more readable.
impl std::fmt::Debug for CompletionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("CompletionItem");
        s.field("label", &self.label);
        if self.edit.len() == 1 {
            let atom = self.edit.iter().next().unwrap();
            s.field("delete", &atom.delete);
            s.field("insert", &atom.insert);
        } else {
            s.field("text_edit", &self.edit);
        }
        s.field("kind", &self.kind);
        s.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompletionItemKind {
    Snippet,
}
