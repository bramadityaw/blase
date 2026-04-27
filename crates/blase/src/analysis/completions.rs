use std::collections::HashSet;

use ast::NodeExt;
use either::Either;
use itertools::Itertools;
use line_index::{TextRange, TextSize};
use smol_str::SmolStr;
use type_sitter::{Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FilePosition, ParsedDocument, RootDatabase, SourceDatabase,
        def::{self, Directive, get_tag_name},
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
    let ctx = &CompletionContext::new(db, position, &document, trigger_char, config)?;
    if let Some(trigger) = trigger_char {
        tracing::debug!(?trigger);
        match trigger {
            // Echo statement
            '{' => return complete_echo(ctx),
            // Directive
            '@' => return complete_directive(ctx),
            _ => (),
        };
    }
    None
}

fn complete_directive(ctx: &CompletionContext) -> Option<Vec<CompletionItem>> {
    // Do not complete if its after two @s
    if let Some(contents) = ctx.db.contents(&ctx.position.path)
        && let Some(range) = ctx
            .position
            .offset
            .checked_sub(2.into())
            .map(|offset| TextRange::at(offset, 2.into()))
        && let Some(ch) = contents[range].chars().next()
        && ch == '@'
    {
        cov_mark::hit!(after_at_at);
        return None;
    }

    let mut directives = Directive::globally_available();
    let mut switched = false;

    let db = ctx.db;
    let doc = &db.parsed_document(&ctx.position.path).unwrap();
    match &ctx.kind {
        ContextKind::Tag { kind } => match kind {
            Tag::Component(_) | Tag::Layout(_) => return None,
            Tag::Html(tag) => {
                directives.extend([Directive::Class, Directive::Style, Directive::Disabled]);

                let tag_name = match tag {
                    Either::Left(tag) => tag.tag_name(),
                    Either::Right(tag) => tag.tag_name(),
                };
                match doc.text_for_node(db, tag_name)? {
                    "input" => directives.extend([Directive::ReadOnly, Directive::Required]),
                    "option" => directives.extend([Directive::Selected]),
                    _ => (),
                }
            }
        },
        ContextKind::Directive(Directive::Switch) => {
            directives.extend([Directive::Break, Directive::Case, Directive::Default]);
            switched = true;
        }
        ContextKind::Document { ident: _ } | ContextKind::Directive(_) => (),
    }

    let ancestors = ctx.node.ancestors();
    for ancestor in ancestors {
        if ast::node_is!(ancestor, ast::blade::Conditional) {
            directives.extend([Directive::ElseIf, Directive::Else]);
        }
        if ast::node_is!(ancestor, ast::blade::Loops) {
            directives.extend([Directive::Break, Directive::Continue]);
        }
        if !switched && ast::node_is!(ancestor, ast::blade::Switch) {
            directives.extend([Directive::Break, Directive::Case, Directive::Default]);
            switched = true;
        }
    }

    let items = directives
        .into_iter()
        .map(|directive| {
            let at_size = TextSize::of('@');
            let offset = ctx
                .position
                .offset
                .checked_sub(at_size)
                .unwrap_or(ctx.position.offset);

            let mut builder = TextEdit::builder();
            builder.delete(TextRange::at(offset, at_size));
            let mut insert = directive.label().to_string();
            if directive.has_param(switched) {
                insert += match directive {
                    Directive::Foreach | Directive::Forelse => "($1 as $2)",
                    Directive::For => "($1; $2; $3)",
                    _ => "($1)",
                };
            }
            if matches!(directive, Directive::Case) {
                insert += "\n    $0\n   @break";
            };
            if let Some(ender) = directive.ender() {
                insert += "\n    $0\n";
                insert += ender.label();
            }
            builder.insert(offset, insert);
            let edit = builder.finish();

            let relevance = CompletionRelevance {
                is_directive_end: directive.is_end(),
                is_after_switch: switched
                    && matches!(
                        directive,
                        Directive::Break | Directive::Case | Directive::Default
                    ),
            };

            CompletionItem {
                label: directive.label().to_string(),
                kind: CompletionItemKind::Snippet,
                lookup: SmolStr::new_inline(&directive.lookup()),
                edit,
                source_range: ctx.source_range(&ctx.kind),
                relevance,
            }
        })
        .collect();
    Some(items)
}

fn complete_echo(ctx: &CompletionContext) -> Option<Vec<CompletionItem>> {
    let mut node = ctx.node;
    if ast::node_is!(node, ast::blade::PhpStatement) {
        if let Ok(esc) = node.downcast::<ast::blade::Escaped>() {
            let esc = esc.upcast();
            let mut cursor = esc.walk();
            let mut children = esc.untyped_children(&mut cursor);
            let br = children.next();
            if children.count() != 0
                && let Some(br) = br
                && !ast::node_is!(br, ast::blade::symbols::LBraceLBrace)
            {
                return None;
            }
            let offset = ctx.position.offset;
            let cmp = "{{ $0 }}";
            let range = TextRange::at(offset.checked_sub(2.into()).unwrap_or(offset), 2.into());
            let item = {
                let mut builder = TextEdit::builder();
                builder.delete(range);
                builder.insert(ctx.position.offset, cmp.to_owned());
                let label = cmp.replace("$0", "");
                let edit = builder.finish();
                CompletionItem {
                    lookup: SmolStr::new(&label),
                    label,
                    kind: CompletionItemKind::Snippet,
                    edit,
                    source_range: ctx.source_range(&ctx.kind),
                    relevance: CompletionRelevance::default(),
                }
            };
            return vec![item].into();
        }
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

    let delete_range = if ast::node_is!(node, ast::blade::Text | ast::blade::Document)
        && let Some(trigger) = ctx.trigger_char
    {
        match trigger {
            '{' => {
                let offset = ctx.position.offset;
                let range = TextRange::at(offset.checked_sub(2.into()).unwrap_or(offset), 2.into());
                let contents = ctx.db.contents(&ctx.position.path).unwrap();
                let no_braces = contents[range].replace('{', "");
                match no_braces.len() {
                    1 => {
                        if no_braces == "!" {
                            Some(range)
                        } else {
                            Some(TextRange::at(
                                offset.checked_sub(1.into()).unwrap_or(offset),
                                1.into(),
                            ))
                        }
                    }
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
            let label = cmp.replace("$0", "");
            let edit = builder.finish();
            CompletionItem {
                lookup: SmolStr::new(&label),
                label,
                kind: CompletionItemKind::Snippet,
                edit,
                source_range: ctx.source_range(&ctx.kind),
                relevance: CompletionRelevance::default(),
            }
        })
        .collect();
    Some(cmps)
}

pub struct CompletionContext<'a> {
    db: &'a RootDatabase,
    position: FilePosition,
    trigger_char: Option<char>,
    node: type_sitter::UntypedNode<'a>,
    kind: ContextKind<'a>,
}

/// Get the nearest child of this node from the offset. Returns the node itself if it is a leaf node.
fn get_nearest_child<'a, N: Node<'a>>(node: N, offset: usize) -> UntypedNode<'a> {
    let node = node.upcast();
    let mut cursor = node.walk();
    let children = node
        .untyped_children(&mut cursor)
        .map(|child| {
            let start = child.start_byte();
            let end = child.end_byte();
            let distance = if offset > end {
                offset.checked_sub(end)
            } else {
                start.checked_sub(offset)
            };
            (distance.unwrap_or_default(), child)
        })
        .sorted_by_key(|k| k.0)
        .collect::<Vec<_>>();
    children.into_iter().map(|(_, n)| n).next().unwrap_or(node)
}

pub enum ContextKind<'a> {
    // The cursor is inside or after a directive
    Directive(Directive),
    // The cursor is inside an element; specifically the element's start tag or self-closing tag
    Tag { kind: Tag<'a> },
    // The cursor is in a Text or Document node.
    // If ident is Some, then the cursor is after an identifier and records the beginning offset of
    // that identifier and the identifier itself
    Document { ident: Option<(u32, SmolStr)> },
}

impl<'a> std::fmt::Debug for ContextKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Directive(arg0) => f.debug_tuple("Directive").field(arg0).finish(),
            Self::Tag { kind } => f.debug_struct("Tag").field("kind", kind).finish(),
            Self::Document { ident } => {
                let mut f = f.debug_struct("Document");
                let n: Option<String> = None;
                match ident {
                    Some((_, s)) => {
                        f.field("ident", &Some(s.to_string()));
                    }
                    None => {
                        f.field("ident", &n);
                    }
                }
                f.finish()
            }
        }
    }
}

pub enum Tag<'a> {
    Component(def::Component),
    Layout(def::Layout),
    Html(Either<ast::blade::StartTag<'a>, ast::blade::SelfClosingTag<'a>>),
}

impl<'a> std::fmt::Debug for Tag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Component(_arg0) => f.debug_tuple("Component").finish(),
            Self::Layout(_arg0) => f.debug_tuple("Layout").finish(),
            Self::Html(_arg0) => f.debug_tuple("Html").finish(),
        }
    }
}

impl<'a> Tag<'a> {
    fn determine(
        db: &dyn DocumentDatabase,
        node: UntypedNode<'a>,
        doc: &ParsedDocument,
        config: &Config,
    ) -> Option<Self> {
        let (tag_name, is_start) = ast::match_node!(node, {
            ast::blade::StartTag(tag) => (get_tag_name(tag)?, true),
            ast::blade::SelfClosingTag(tag) => (get_tag_name(tag)?, false),
            _ => return None,
        });

        if let Some(component) = def::Component::for_tagname(db, tag_name, doc, config) {
            return Some(Self::Component(component));
        }

        if let Some(layout) = def::Layout::for_tagname(db, tag_name, doc, config) {
            return Some(Self::Layout(layout));
        }

        let tag = if is_start {
            Either::Left(node.downcast::<ast::blade::StartTag>().unwrap())
        } else {
            Either::Right(node.downcast::<ast::blade::SelfClosingTag>().unwrap())
        };

        Some(Self::Html(tag))
    }
}

fn is_directive_start(node: UntypedNode<'_>) -> bool {
    ast::node_is!(
        node,
        ast::blade::symbols::Atif
            | ast::blade::symbols::Atunless
            | ast::blade::symbols::Atisset
            | ast::blade::symbols::Atisset
            | ast::blade::symbols::Atempty
            | ast::blade::symbols::Atauth
            | ast::blade::symbols::Atguest
            | ast::blade::symbols::Atproduction
            | ast::blade::symbols::Atenv
            | ast::blade::symbols::Atsession
            | ast::blade::symbols::Atcontext
            | ast::blade::symbols::Athassection
            | ast::blade::symbols::Atsectionmissing
            | ast::blade::symbols::Atswitch
            | ast::blade::symbols::Atfor
            | ast::blade::symbols::Atforeach
            | ast::blade::symbols::Atforelse
            | ast::blade::symbols::Atwhile
    )
}

fn analyze_context_kind<'db>(
    db: &'db RootDatabase,
    node: UntypedNode<'db>,
    FilePosition { path, offset }: &FilePosition,
    config: &Config,
) -> ContextKind<'db> {
    'bail: {
        if node.is_error() {
            let mut cursor = node.walk();
            let mut children = node.untyped_children(&mut cursor);
            if let Some(start) = children.next() {
                if is_directive_start(start) {
                    let Some(directive) = Directive::from_node(start) else {
                        break 'bail;
                    };
                    return ContextKind::Directive(directive);
                }
            }
        }
    }

    let offset = (*offset).into();
    if ast::node_is!(node, ast::blade::Document | ast::blade::Text) {
        let contents = &db.contents(path).unwrap();
        return ContextKind::Document {
            ident: extract_ident(contents, offset)
                .map(|(start, ident)| (start, SmolStr::new(ident))),
        };
    }

    let mut node = node;
    if let Ok(element) = node.downcast::<ast::blade::Element>() {
        node = get_nearest_child(element, offset);
    }
    let document = &db
        .parsed_document(path)
        .expect("file does not exist. this is a bug");
    if ast::node_is!(node, ast::blade::StartTag | ast::blade::SelfClosingTag)
        && let Some(tag) = Tag::determine(db, node, document, config)
    {
        return ContextKind::Tag { kind: tag };
    }

    ContextKind::Document { ident: None }
}

fn get_first_child(node: UntypedNode<'_>) -> Option<UntypedNode<'_>> {
    let mut cursor = node.walk();
    node.untyped_children(&mut cursor).next()
}

impl<'db> CompletionContext<'db> {
    pub fn new(
        db: &'db RootDatabase,
        position @ FilePosition { path: _, offset }: FilePosition,
        doc: &'db ParsedDocument,
        trigger_char: Option<char>,
        config: &Config,
    ) -> Option<Self> {
        let mut node = doc.get_node_at(offset)?;
        if ast::node_is!(node, ast::blade::Comment) {
            return None;
        }
        if ast::node_is!(node, ast::blade::Document) {
            let src = &doc.contents(db);
            let range = offset.into()..=offset.into();
            if let Some(c) = src.get(range).and_then(|s| s.chars().next())
                && c.is_ascii_whitespace()
            {
                node = get_nearest_child(node, offset.into());
            }
        }

        let ctx = CompletionContext {
            kind: analyze_context_kind(db, node, &position, config),
            position,
            db,
            trigger_char,
            node,
        };
        Some(ctx)
    }

    pub fn source_range(&self, scope: &ContextKind) -> TextRange {
        let node = self.node;
        if ast::node_is!(node, ast::blade::TagName) {
            let start = node.start_byte() as u32;
            let end = node.end_byte() as u32;
            return TextRange::new(start.into(), end.into());
        }

        if let ContextKind::Document { ident: Some(ident) } = scope {
            let (start, ident) = ident;
            return TextRange::at((*start).into(), (ident.len() as u32).into());
        }
        cov_mark::hit!(source_range_ctx_empty);
        TextRange::empty(self.position.offset)
    }
}

fn is_ident(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-' | b':' | b'.' | b'@')
}

fn extract_ident(contents: &str, offset: usize) -> Option<(u32, &str)> {
    let (start, ident) = text_procs::match_in_offset(contents, offset, is_ident)?;

    // Return None if the first character is a digit or the entire string parses as an integer/float
    let ch = ident.chars().next()?;
    if ch.is_ascii_digit() || ident.parse::<usize>().is_ok() || ident.parse::<f32>().is_ok() {
        return None;
    }
    Some((start, ident))
}

#[test]
fn test_extract_ident() {
    let text = "foo bar_baz p123 ";
    assert_eq!(extract_ident(text, 2,), Some((0, "foo"))); // inside "foo"
    assert_eq!(extract_ident(text, 3,), Some((0, "foo"))); // at space, left "foo"
    assert_eq!(extract_ident(text, 4,), Some((4, "bar_baz"))); // space before 'b'
    assert_eq!(extract_ident(text, 8,), Some((4, "bar_baz"))); // inside "bar_baz"
    assert_eq!(extract_ident(text, 11,), Some((4, "bar_baz"))); // space before "123"
    assert_eq!(extract_ident(text, 14,), Some((12, "p123"))); // inside digits
    assert_eq!(extract_ident(text, 16,), Some((12, "p123"))); // after end, left "123"
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct CompletionRelevance {
    is_directive_end: bool,
    is_after_switch: bool,
}

impl CompletionRelevance {
    const BASE_SCORE: u32 = u32::MAX / 2;
    pub fn score(self) -> u32 {
        let mut score = Self::BASE_SCORE;
        let CompletionRelevance {
            is_directive_end,
            is_after_switch,
        } = self;
        if is_directive_end {
            score -= 1;
        }
        if is_after_switch {
            score += 1;
        }
        score
    }

    /// Returns true when the score for this threshold is above
    /// some threshold such that we think it is especially likely
    /// to be relevant.
    pub fn is_relevant(&self) -> bool {
        self.score() > Self::BASE_SCORE
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompletionItemKind {
    Snippet,
}
