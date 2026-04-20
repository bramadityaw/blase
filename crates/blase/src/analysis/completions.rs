use std::collections::HashSet;

use ast::NodeExt;
use line_index::TextRange;
use type_sitter::{Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FilePosition, ParsedDocument, RootDatabase, SourceDatabase,
        text_edit::TextEdit,
    },
};

#[cfg(test)]
mod tests;

pub fn completions(
    db: &RootDatabase,
    config: &Config,
    position: FilePosition,
    trigger_char: Option<char>,
) -> Option<Vec<CompletionItem>> {
    let document = db.parsed_document(&position.path)?;
    let ctx = CompletionContext::new(db, position, &document, trigger_char)?;
    if let Some(trigger) = trigger_char {
        tracing::debug!(?trigger);
        match trigger {
            // Echo statement
            '{' | '!' => complete_echo(ctx),
            // Directive
            '@' => todo!(),
            // Component/Layout custom element
            '-' => todo!(),
            _ => None,
        }
    } else {
        None
    }
}

pub struct CompletionContext<'a> {
    db: &'a RootDatabase,
    position: FilePosition,
    trigger_char: Option<char>,
    /// The token before the cursor, in the original file.
    node: type_sitter::UntypedNode<'a>,
}

impl<'db> CompletionContext<'db> {
    pub fn new(
        db: &'db RootDatabase,
        position @ FilePosition { path: _, offset }: FilePosition,
        doc: &'db ParsedDocument,
        trigger_char: Option<char>,
    ) -> Option<CompletionContext<'db>> {
        let node = doc.get_node_at(offset)?;
        if ast::node_is!(node, ast::blade::Comment) {
            return None;
        }

        let ctx = CompletionContext {
            position,
            db,
            trigger_char,
            node,
        };
        Some(ctx)
    }

    pub fn source_range(&self) -> TextRange {
        if is_any_identifier(&self.node) {
            let start = self.node.start_byte() as u32;
            let end = self.node.end_byte() as u32;

            TextRange::new(start.into(), end.into())
        } else {
            TextRange::empty(self.position.offset)
        }
    }
}

fn is_any_identifier(node: &UntypedNode<'_>) -> bool {
    ast::node_is!(node, ast::blade::TagName | ast::blade::Directive)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CompletionFieldsToResolve {
    pub resolve_label_details: bool,
    pub resolve_tags: bool,
    pub resolve_detail: bool,
    pub resolve_documentation: bool,
    pub resolve_filter_text: bool,
    pub resolve_text_edit: bool,
    pub resolve_command: bool,
}

impl CompletionFieldsToResolve {
    pub fn from_client_capabilities(client_capability_fields: &HashSet<&str>) -> Self {
        Self {
            resolve_label_details: client_capability_fields.contains("labelDetails"),
            resolve_tags: client_capability_fields.contains("tags"),
            resolve_detail: client_capability_fields.contains("detail"),
            resolve_documentation: client_capability_fields.contains("documentation"),
            resolve_filter_text: client_capability_fields.contains("filterText"),
            resolve_text_edit: client_capability_fields.contains("textEdit"),
            resolve_command: client_capability_fields.contains("command"),
        }
    }

    pub const fn empty() -> Self {
        Self {
            resolve_label_details: false,
            resolve_tags: false,
            resolve_detail: false,
            resolve_documentation: false,
            resolve_filter_text: false,
            resolve_text_edit: false,
            resolve_command: false,
        }
    }
}

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

#[derive(Debug)]
pub enum CompletionItemKind {
    Snippet,
}

fn complete_echo(ctx: CompletionContext) -> Option<Vec<CompletionItem>> {
    let mut node = ctx.node;
    if ast::node_is!(node, ast::blade::PhpStatement) {
        return None;
    };
    if node.is_error() {
        let mut cursor = ctx.node.walk();
        let mut child_tokens = ctx.node.untyped_children(&mut cursor);
        node = child_tokens.next()?;
        // Don't complete if inside {{  }}
        if ast::node_is!(
            node,
            ast::blade::symbols::LBraceLBrace | ast::blade::symbols::LBraceNotNot
        ) {
            let stray_tokens = child_tokens.collect::<Vec<_>>();
            if let &[.., snd_last, last] = stray_tokens.as_slice() {
                if ast::node_is!(
                    last,
                    ast::blade::symbols::RBraceRBrace | ast::blade::symbols::NotNotRBrace
                ) || (ast::node_is!(last, ast::blade::symbols::RBrace)
                    && ast::node_is!(
                        snd_last,
                        ast::blade::symbols::RBrace | ast::blade::symbols::Not
                    ))
                {
                    cov_mark::hit!(inside_echo_error);
                    return None;
                }
            }
        }
    };

    dbg!(ast::node_is!(node, ast::blade::symbols::DoubleQuote));
    let delete_range = if ast::node_is!(node, ast::blade::Text | ast::blade::Document)
        && let Some(trigger) = ctx.trigger_char
    {
        match trigger {
            '{' => {
                let offset = ctx.position.offset;
                let range = TextRange::at(offset.checked_sub(2.into()).unwrap_or(offset), 2.into());
                let contents = ctx.db.contents(&ctx.position.path).unwrap();
                match contents[range].replace('{', "").len() {
                    1 => Some(TextRange::at(
                        offset.checked_sub(1.into()).unwrap_or(offset),
                        1.into(),
                    )),
                    0 => Some(range),
                    _ => unreachable!(),
                }
            }
            _ => None,
        }
    } else {
        Some(TextRange::at(
            ctx.position.offset.checked_sub(1.into())?,
            1.into(),
        ))
    };

    let cmps = ["{{ $0 }}", "{!! $0 !!}"]
        .into_iter()
        .map(|cmp| {
            let mut builder = TextEdit::builder();
            if let Some(range) = delete_range {
                cov_mark::hit!(delete_range);
                builder.delete(range);
            }
            builder.insert(ctx.position.offset, cmp.to_owned());
            let cmp = cmp.replace("$0", "");
            let edit = builder.finish();
            CompletionItem {
                label: cmp,
                kind: CompletionItemKind::Snippet,
                edit,
                source_range: ctx.source_range(),
            }
        })
        .collect();
    Some(cmps)
}
