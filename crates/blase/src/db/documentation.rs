use phpdoc_parser::{self as phpdoc, InlineTag, PhpDoc, TextSegment};
use tree_sitter::{Query, QueryCursor, StreamingIterator};
use type_sitter::{HasChildren, Node, UntypedNode};

use crate::db::def::{
    Component, ComponentKind, DefDatabase, Document, DocumentId, Layout, queries,
};

/// Holds documentation
#[derive(Debug, Clone)]
pub enum Documentation {
    Plain(String),
    PhpDoc(PhpDoc),
}

impl Documentation {
    pub fn render(&self) -> String {
        fn render_segments(buf: &mut String, segments: &[TextSegment]) {
            for segment in segments.iter() {
                match segment {
                    phpdoc_parser::TextSegment::Text(text) => buf.push_str(text),
                    phpdoc_parser::TextSegment::InlineTag(inline_tag) => {
                        let InlineTag {
                            name,
                            body,
                            span: _,
                        } = inline_tag;
                        macros::format_to!(buf, "{{@{}", name);
                        if let Some(body) = body {
                            macros::format_to!(buf, " {} ", body);
                        }
                        buf.push('}');
                    }
                }
            }
        }
        match self {
            Documentation::Plain(s) => s.to_owned(),
            Documentation::PhpDoc(PhpDoc {
                summary,
                description,
                tags,
                span: _,
            }) => {
                let mut buf = String::new();
                if let Some(summary) = summary {
                    render_segments(&mut buf, &summary.segments);
                }
                if let Some(description) = description {
                    buf.push_str("\n\n");
                    render_segments(&mut buf, &description.segments);
                }

                for phpdoc_parser::PhpDocTag {
                    name,
                    body,
                    span: _,
                } in tags
                {
                    macros::format_to!(buf, "\n\n@{}", name);
                    if let Some(body) = body {
                        buf.push(' ');
                        render_segments(&mut buf, &body.segments);
                    }
                }

                buf.trim_start().into()
            }
        }
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

impl HasDocs for Component {
    fn docs(&self, db: &dyn DefDatabase) -> Option<Documentation> {
        let id = self.id();
        let document = &id.document(db);
        let comment = match self.kind(db) {
            ComponentKind::Class => {
                let root = document.root_node();
                let root = root.raw();
                let contents = document.contents(db);
                let mut cursor = QueryCursor::new();
                let query =
                    Query::new(&root.language(), queries::COMMENTS_FOR_CLASS_COMPONENTS).ok()?;
                let mut matches = cursor.matches(&query, *root, contents.as_bytes());
                let comment = (|| {
                    while let Some(m) = matches.next() {
                        for m in m.captures {
                            let node = UntypedNode::new(m.node);
                            let Ok(comment) = node.downcast::<ast::php::Comment>() else {
                                continue;
                            };
                            return document.text_for_node(db, comment);
                        }
                    }
                    None
                })()?;
                comment
            }
            ComponentKind::Anon => {
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
                comment
            }
        };
        extract_docs(comment)
    }
}

fn extract_docs(comment: &str) -> Option<Documentation> {
    if comment.starts_with("/**") {
        let documentation = phpdoc::parse(comment);
        Some(Documentation::PhpDoc(documentation))
    } else if let Some(text) = comment
        .strip_prefix("{{")
        .and_then(|s| s.strip_suffix("}}"))
    {
        let documentation = text
            .split_inclusive('\n')
            .map(|s| {
                let text = s.strip_prefix("---").or(s.strip_prefix("--")).unwrap_or(s);
                text_procs::trim_indent(text)
            })
            .collect::<String>();
        Some(Documentation::Plain(documentation))
    } else {
        None
    }
}
