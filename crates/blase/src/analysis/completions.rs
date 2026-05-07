use std::collections::HashSet;

use ast::NodeExt;
use either::Either;
use itertools::Itertools;
use line_index::{TextRange, TextSize};
use smol_str::{SmolStr, ToSmolStr};
use type_sitter::{Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FilePosition, ParsedDocument, RootDatabase, SourceDatabase,
        def::{self, Component, ComponentAttr, DefDatabase, Directive, Layout, Name},
        text_edit::TextEdit,
    },
};

#[cfg(test)]
mod tests;

mod item;

pub use item::{CompletionItem, CompletionItemKind};

pub fn completions(
    db: &RootDatabase,
    config: &Config,
    position: FilePosition,
    trigger_char: Option<char>,
) -> Option<Vec<CompletionItem>> {
    let document = db.parsed_document(&position.path)?;
    let (ctx, analysis) = &CompletionContext::new(db, position, &document, trigger_char, config)?;
    let mut items: Vec<CompletionItem> = Vec::new();
    if let Some('{') = trigger_char {
        return complete_echo(ctx, analysis);
    }

    {
        let acc = &mut items;

        match analysis {
            ContextAnalysis::Directive(_directive) => {
                directive_completion(acc, ctx, analysis);
                //TODO: collect data on what kind of completions are helpful in a directive parameter.
            }
            ContextAnalysis::Tag { kind } => attribute_completion(&mut items, ctx, kind),
            ContextAnalysis::Document { name } => {
                directive_completion(acc, ctx, analysis);
                let (start, name) = name.as_ref()?;
                let start_offset = TextSize::from(*start);
                component_completion(acc, ctx, start_offset, name, config, analysis);
                layout_completion(acc, ctx, start_offset, name, config, analysis);
            }
        }
    }
    Some(items)
}

fn layout_completion(
    items: &mut Vec<CompletionItem>,
    ctx: &CompletionContext,
    start_offset: TextSize,
    name: &Name,
    config: &Config,
    analysis: &ContextAnalysis,
) {
    let name_range = TextRange::at(start_offset, TextSize::of(name.as_str()));
    let db = ctx.db;
    let all_docs = db.all_documents();
    let layouts = all_docs.into_iter().filter_map(|doc| {
        let layout = Layout::from_document(db, doc, config)?;
        let lname = layout.name(db);
        if lname.starts_with(name) {
            Some(layout)
        } else {
            None
        }
    });
    let completions =
        layouts.map(|layout| layout_completion_item(ctx, db, name_range, &layout, analysis));
    items.extend(completions);
}

fn layout_completion_item(
    ctx: &CompletionContext,
    db: &dyn DefDatabase,
    name_range: TextRange,
    layout: &Layout,
    analysis: &ContextAnalysis,
) -> CompletionItem {
    let name = layout.name(db);
    let tag_name = &name.tag_name();

    let mut builder = TextEdit::builder();
    builder.delete(name_range);
    let mut buf = format!("<{}>\n", dbg!(tag_name));
    buf.push_str("    $0\n");
    macros::format_to!(buf, "</{}>", tag_name);
    builder.insert(name_range.start(), buf);
    let edit = builder.finish();

    CompletionItem {
        label: tag_name.to_owned(),
        kind: CompletionItemKind::Snippet,
        edit,
        source_range: ctx.source_range(analysis),
        lookup: name.label().to_smolstr(),
        relevance: CompletionRelevance::default(),
    }
}

fn component_completion(
    items: &mut Vec<CompletionItem>,
    ctx: &CompletionContext,
    start_offset: TextSize,
    name: &Name,
    config: &Config,
    analysis: &ContextAnalysis,
) {
    let name_range = TextRange::at(start_offset, TextSize::of(name.as_str()));
    let db = ctx.db;
    let all_docs = db.all_documents();
    let components = all_docs.into_iter().filter_map(|doc| {
        let component = Component::for_document(db, doc, config)?;
        let cname = component.name(db);
        if cname.inner().starts_with(name) {
            Some(component)
        } else {
            None
        }
    });
    let completions = components
        .map(|component| component_completion_item(ctx, db, name_range, &component, analysis));
    items.extend(completions);
}

fn component_completion_item(
    ctx: &CompletionContext,
    db: &dyn DefDatabase,
    name_range: TextRange,
    component: &Component,
    analysis: &ContextAnalysis,
) -> CompletionItem {
    let signature = &component.signature(db);
    let attributes = &signature.attrs;
    let component_name = &signature.name;
    let label = component_name.tag_name();

    let mut builder = TextEdit::builder();
    builder.delete(name_range);

    let mut buf = String::new();
    macros::format_to!(buf, "<x-{}", component_name);

    if let Some(attributes) = attributes {
        let len = attributes.len();
        let mut i = 1;
        for ComponentAttr {
            name,
            default_value: _,
        } in attributes.iter()
        {
            macros::format_to!(buf, " {}=\"${}\"", name, i);
            i += 1;
            if i == len {
                i = 0;
            }
        }
    }

    buf.push_str("/>");

    builder.insert(name_range.start(), buf);

    let edit = builder.finish();
    CompletionItem {
        label,
        kind: CompletionItemKind::Snippet,
        edit,
        source_range: ctx.source_range(analysis),
        lookup: component_name.inner().to_smolstr(),
        relevance: CompletionRelevance::default(),
    }
}

fn attribute_completion(items: &mut Vec<CompletionItem>, ctx: &CompletionContext, tag: &Tag<'_>) {
    if ctx.node.is::<ast::blade::EndTag>() {
        return;
    }

    let analysis = &ContextAnalysis::Tag { kind: *tag };
    let db = ctx.db;
    match tag {
        Tag::Component(component) => {
            let Some(attrs) = component.attrs(db).clone() else {
                return;
            };
            let cmps = attrs
                .into_iter()
                .map(|attr| component_attribute_completion(ctx, attr, analysis));
            items.extend(cmps);
        }
        Tag::Layout(_) => return,
        Tag::Html(tag) => {
            cov_mark::hit!(attr_completion);
            let mut directives = Directive::globally_available();
            directives.extend([Directive::Class, Directive::Style, Directive::Disabled]);

            let tag_name = match tag {
                Either::Left(tag) => tag.tag_name(),
                Either::Right(tag) => tag.tag_name(),
            };
            db.parsed_document(&ctx.position.path).map(|doc| {
                match doc.text_for_node(db, tag_name) {
                    Some("input") => directives.extend([Directive::ReadOnly, Directive::Required]),
                    Some("option") => directives.extend([Directive::Selected]),
                    _ => (),
                }
            });

            let cmps = directives
                .into_iter()
                .map(|directive| directive_to_completion(ctx, analysis, directive, false));
            items.extend(cmps);
        }
    }
}

fn component_attribute_completion(
    ctx: &CompletionContext,
    attr: &ComponentAttr,
    analysis: &ContextAnalysis,
) -> CompletionItem {
    let node = ctx.node;
    tracing::debug!(?node);
    let start_offset = ctx.position.offset;
    let attr_name = attr.name.as_str();
    let mut insert_text = String::new();
    let mut builder = TextEdit::builder();
    'delete: {
        if node.is::<ast::blade::AttributeName>() {
            let Ok((start, end)) =
                (|| -> Result<(TextSize, TextSize), std::num::TryFromIntError> {
                    let start = TextSize::try_from(node.start_byte())?;
                    let end = TextSize::try_from(node.end_byte())?;
                    Ok((start, end))
                })()
            else {
                tracing::error!(range = ?node.byte_range(), "TextSize overflow");
                break 'delete;
            };
            let range = TextRange::new(start, end);
            builder.delete(range);
        }
    }
    macros::format_to!(insert_text, "{}=\"$0\"", attr_name);
    builder.insert(start_offset, insert_text);

    let edit = builder.finish();
    CompletionItem {
        label: attr_name.to_owned(),
        kind: CompletionItemKind::Snippet,
        edit,
        source_range: ctx.source_range(analysis),
        lookup: SmolStr::new(attr_name),
        relevance: CompletionRelevance::default(),
    }
}

fn directive_completion(
    items: &mut Vec<CompletionItem>,
    ctx: &CompletionContext,
    analysis: &ContextAnalysis,
) {
    if ctx.trigger_char != Some('@') {
        return;
    }
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
        return;
    }

    let mut directives = Directive::globally_available();
    let mut switched = false;

    if let ContextAnalysis::Directive(Directive::Switch) = analysis {
        directives.extend([Directive::Break, Directive::Case, Directive::Default]);
        switched = true;
    }

    let ancestors = ctx.node.ancestors();
    for ancestor in ancestors {
        if ancestor.is::<ast::blade::Conditional>() {
            directives.extend([Directive::ElseIf, Directive::Else]);
        }
        if ancestor.is::<ast::blade::Loops>() {
            directives.extend([Directive::Break, Directive::Continue]);
        }
        if !switched && ancestor.is::<ast::blade::Switch>() {
            directives.extend([Directive::Break, Directive::Case, Directive::Default]);
            switched = true;
        }
    }

    let cmps = directives
        .into_iter()
        .map(|directive| directive_to_completion(ctx, analysis, directive, switched));
    items.extend(cmps);
}

fn directive_to_completion(
    ctx: &CompletionContext,
    analysis: &ContextAnalysis,
    directive: Directive,
    switched: bool,
) -> item::CompletionItem {
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
            Directive::IncludeWhen | Directive::IncludeUnless => "($1, '$2')",
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
        source_range: ctx.source_range(analysis),
        relevance,
    }
}

fn complete_echo(
    ctx: &CompletionContext,
    analysis: &ContextAnalysis,
) -> Option<Vec<CompletionItem>> {
    let node = ctx.node;

    for ancestor in node.ancestors() {
        if ancestor.is::<ast::blade::PhpStatement>() {
            let mut cursor = ancestor.walk();
            let mut children = ancestor.untyped_children(&mut cursor);
            let br = children.next();
            if children.count() != 0
                && let Some(br) = br
                && !br.is::<ast::blade::symbols::LBraceLBrace>()
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
                    source_range: ctx.source_range(analysis),
                    relevance: CompletionRelevance::default(),
                }
            };
            return vec![item].into();
        };

        if ancestor.is_error() {
            let mut cursor = ancestor.walk();
            let mut child_tokens = ancestor.untyped_children(&mut cursor);
            let first = child_tokens.next().unwrap_or(ancestor);
            // Don't complete if inside {{  }}
            if ast::node_is!(
                first,
                ast::blade::symbols::LBraceLBrace | ast::blade::symbols::LBraceNotNot
            ) {
                let stray_tokens = child_tokens.collect::<Vec<_>>();
                if let &[.., snd_last, last] = stray_tokens.as_slice() {
                    if ast::node_is!(
                        last,
                        ast::blade::symbols::RBraceRBrace | ast::blade::symbols::NotNotRBrace
                    ) || (last.is::<ast::blade::symbols::RBrace>()
                        && ast::node_is!(
                            snd_last,
                            ast::blade::symbols::RBrace | ast::blade::symbols::Not
                        ))
                    {
                        return None;
                    }
                }
            }
        }
    }

    cov_mark::hit!(echo_complete);

    let lbrace = TextSize::of('{');
    let delete_range = {
        let offset = ctx.position.offset;
        let double_lbrace = TextSize::of("{{");
        let range = TextRange::at(
            offset.checked_sub(double_lbrace).unwrap_or(offset),
            double_lbrace,
        );
        let contents = ctx.db.contents(&ctx.position.path).unwrap();
        let no_braces = contents[range].replace('{', "");
        match no_braces.len() {
            1 => {
                if no_braces == "!" {
                    Some(range)
                } else {
                    Some(TextRange::at(
                        offset.checked_sub(lbrace).unwrap_or(offset),
                        lbrace,
                    ))
                }
            }
            0 => Some(range),
            _ => unreachable!(),
        }
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
                source_range: ctx.source_range(analysis),
                relevance: CompletionRelevance::default(),
            }
        })
        .collect();
    Some(cmps)
}

#[derive(Clone)]
pub struct CompletionContext<'a> {
    db: &'a RootDatabase,
    position: FilePosition,
    trigger_char: Option<char>,
    node: type_sitter::UntypedNode<'a>,
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

#[derive(Clone, PartialEq, Eq)]
pub enum ContextAnalysis<'a> {
    // The cursor is inside or after a directive
    Directive(Directive),
    // The cursor is inside an element; specifically the element's start tag or self-closing tag
    Tag { kind: Tag<'a> },
    // The cursor is in a Text or Document node.
    // If ident is Some, then the cursor is after an identifier and records the beginning offset of
    // that identifier and the identifier itself
    Document { name: Option<(u32, Name)> },
}

impl<'a> std::fmt::Debug for ContextAnalysis<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Directive(arg0) => f.debug_tuple("Directive").field(arg0).finish(),
            Self::Tag { kind } => f.debug_struct("Tag").field("kind", kind).finish(),
            Self::Document { name: ident } => {
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[salsa::tracked]
impl<'a> Tag<'a> {
    fn determine(
        db: &dyn DefDatabase,
        node: UntypedNode<'a>,
        doc: &ParsedDocument,
        config: &Config,
    ) -> Option<Self> {
        let (tag_name, is_start) = ast::match_node!(node, {
            ast::blade::StartTag(tag) => (tag.tag_name().ok()?, true),
            ast::blade::SelfClosingTag(tag) => (tag.tag_name().ok()?, false),
            ast::blade::Element(element) => return Self::determine(db, element.tag()?, doc, config),
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

fn analyze_context<'db>(ctx: &mut CompletionContext<'db>, config: &Config) -> ContextAnalysis<'db> {
    let FilePosition { ref path, offset } = ctx.position;

    'bail: {
        let node = ctx.node;
        if node.is_error() {
            if let Some(start) = get_first_child(node) {
                ctx.node = get_nearest_child(node, offset.into());
                if is_directive_start(start) {
                    let Some(directive) = Directive::from_node(start) else {
                        break 'bail;
                    };
                    return ContextAnalysis::Directive(directive);
                }
            }
        }
    }

    let ctx: &CompletionContext = ctx;
    let db = ctx.db;

    let ancestors = ctx.node.ancestors();
    for ancestor in ancestors {
        let offset = offset.into();
        if ast::node_is!(ancestor, ast::blade::Document | ast::blade::Text) {
            let contents = &db.contents(path).unwrap();
            return ContextAnalysis::Document {
                name: extract_ident(contents, offset)
                    .map(|(start, ident)| (start, Name::new(ident))),
            };
        }

        let document = &db
            .parsed_document(path)
            .expect("file does not exist. this is a bug");
        if let Some(tag) = Tag::determine(db, ancestor, document, config) {
            return ContextAnalysis::Tag { kind: tag };
        }
    }

    ContextAnalysis::Document { name: None }
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
    ) -> Option<(Self, ContextAnalysis<'db>)> {
        let mut node = doc.get_node_at(offset)?;
        if node.is::<ast::blade::Comment>() {
            return None;
        }
        if node.is::<ast::blade::Document>() {
            let src = &doc.contents(db);
            let range = offset.into()..=offset.into();
            if let Some(c) = src.get(range).and_then(|s| s.chars().next())
                && c.is_ascii_whitespace()
            {
                node = get_nearest_child(node, offset.into());
            }
        }

        let mut ctx = CompletionContext {
            position,
            db,
            trigger_char,
            node,
        };
        let analysis = analyze_context(&mut ctx, config);
        Some((ctx, analysis))
    }

    pub fn source_range(&self, analysis: &ContextAnalysis) -> TextRange {
        let node = self.node;
        if ast::node_is!(node, ast::blade::TagName | ast::blade::AttributeName) {
            let start = node.start_byte() as u32;
            let end = node.end_byte() as u32;
            return TextRange::new(start.into(), end.into());
        }

        if let ContextAnalysis::Document { name: Some(ident) } = analysis {
            let (start, ident) = ident;
            return TextRange::at((*start).into(), (ident.as_str().len() as u32).into());
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

pub const TRIGGER_CHARS: &[char] = &[
    '@', // directives
    '{', // echo statements
    ':', // expression attributes
];
