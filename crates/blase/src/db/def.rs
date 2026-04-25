//! This module defines data models of semantic objects
//! in a Blade template

use std::sync::Arc;

use camino::Utf8Path;
use convert_case::ccase;
use smol_str::SmolStr;
use type_sitter::{HasChild, Node};

use crate::{
    config::Config,
    db::{DocumentDatabase, ParsedDocument, SourceFile},
    resolve_path,
    util::FileType,
};

#[salsa::db]
pub trait DefDatabase: DocumentDatabase {
    fn component_signature(&self, id: ComponentId) -> Arc<ComponentSignature>;
}

#[salsa::db]
impl DefDatabase for super::RootDatabase {
    fn component_signature(&self, id: ComponentId) -> Arc<ComponentSignature> {
        ComponentSignature::query(self, id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    str: SmolStr,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.str.fmt(f)
    }
}

impl Name {
    pub fn new(str: &str) -> Self {
        let str = if str.len() <= 23 {
            SmolStr::new_inline(str)
        } else {
            SmolStr::new(str)
        };
        Self { str }
    }

    pub fn as_str(&self) -> &str {
        &self.str
    }
}

/// A Component's signature consists of attributes and slots.
///
/// A component receives the following attributes:
/// If it is class-based, attributes are its constructor arguments
/// If it is anonymous, attributes are defined in @props directive
/// Other attributes (such as HTML attributes) are determined to be available
/// if the component uses the $attribute variable
///
/// If no slots are used, then auto-complete should supply a self-closing tag.
///
/// TODO: Determine what slots are available
#[derive(Clone, PartialEq)]
pub struct ComponentSignature {
    name: Name,
    attrs: Option<Box<[ComponentAttr]>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentAttr {
    pub(crate) name: Name,
    pub(crate) default_value: Option<SmolStr>,
}

macro_rules! walk_children {
    ($node:expr, |$child:pat_param| $block:block) => {{
        use ::type_sitter::{Node, HasChildren};
        let __node = $node;
        let mut cursor = __node.walk();
        for $child in __node.children(&mut cursor) $block
    }};
}

#[salsa::tracked]
impl ComponentAttr {
    const QUOTES: [char; 2] = ['\'', '"'];
    pub fn from_anon(db: &dyn DefDatabase, document: ParsedDocument) -> Vec<ComponentAttr> {
        assert_eq!(document.filetype, FileType::Blade);
        let root = document.root_node();
        tracing::debug!(kind = root.kind());
        root.downcast::<ast::blade::Document>()
            .ok()
            .and_then(|doc| {
                walk_children!(doc, |e| {
                    use ast::blade::anon_unions::Anon84799576569979127075025455267281272019::Props;
                    let element = e.ok()?;
                    match element {
                        Props(props) => {
                            let mut cursor = props.walk();
                            match props.child().ok()? {
                                ast::blade::anon_unions::ArrayCreationExpression_EncapsedString_String::ArrayCreationExpression(array_creation_expression) =>{
                                    let attrs = array_creation_expression
                                        .array_element_initializers(&mut cursor)
                                        .filter_map(|element| {
                                            use ast::blade::anon_unions::ArrayElementKeyValueInitializer_ArrayElementSpreadingInitializer_ArrayElementValueInitializer::*;

                                            match element.and_then(|e| e.child()).ok()? {
                                                ArrayElementKeyValueInitializer(array_element) => {
                                                    let name = document
                                                        .text_for_node(db, array_element.key().ok()?)
                                                        .map(|name| {
                                                            let name = name.trim_matches(Self::QUOTES);
                                                            Name::new(name)
                                                        })?;
                                                    let default_value = document.text_for_node(db, array_element.value().ok()?).map(SmolStr::new);
                                                    Some(ComponentAttr { name, default_value })
                                                },
                                                ArrayElementValueInitializer(array_element) => {
                                                    use ast::blade::{Expression::PrimaryExpression, PrimaryExpression::Literal};
                                                    let value = array_element.expression().ok()?;
                                                    match value {
                                                        PrimaryExpression(e) => {
                                                            match e {
                                                            Literal(literal) => {
                                                                let name = match literal {
                                                                    ast::blade::Literal::EncapsedString(encapsed_string) => document.text_for_node(db, encapsed_string),
                                                                    ast::blade::Literal::String(string) => document.text_for_node(db, string),
                                                                    _ => None,
                                                                }?
                                                                .trim_matches(Self::QUOTES);
                                                                Some(ComponentAttr { name: Name::new(name), default_value: None })
                                                            }
                                                            _ => None,
                                                        }},
                                                        _ => None,
                                                    }
                                                },
                                                _ => None,
                                            }
                                        })
                                        .collect();
                                    return Some(attrs);
                                },
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                });
                None
            })
            .unwrap_or_default()
    }

    pub fn from_class(db: &dyn DefDatabase, document: ParsedDocument) -> Vec<ComponentAttr> {
        assert_eq!(document.filetype, FileType::PHP);
        let root = document.root_node();
        root.downcast::<ast::php::Program>()
            .ok()
            .and_then(|program| {
                walk_children!(program, |child| {
                    ast::match_node!(child, {
                        ast::php::ClassDeclaration(class_decl) => {
                            let mut cursor = class_decl.walk();
                            let class_items = class_decl.body().ok()?.children(&mut cursor);
                            for item in class_items {
                                ast::match_node!(item, {
                                    ast::php::MethodDeclaration(method_decl) => {
                                        let mut cursor = method_decl.walk();
                                        let method_name = method_decl.name().ok()?;
                                        if document.text_for_node(db, method_name)? != "__construct" {
                                            continue;
                                        }
                                        let method_params = method_decl.parameters().ok()?;
                                        return Some(method_params
                                            .children(&mut cursor)
                                            .filter_map(|param| {
                                                use ast::php::anon_unions::PropertyPromotionParameter_SimpleParameter_VariadicParameter::*;
                                                match param.ok()? {
                                                    PropertyPromotionParameter(prop_parameter) => {
                                                        let name = document.text_for_node(db, prop_parameter.name().ok()?)?;
                                                        let name = name.strip_prefix('$').unwrap();
                                                        let default_value = prop_parameter.default_value().and_then(|val| {
                                                            let text = document.text_for_node(db, val)?;
                                                            Some(SmolStr::new(text))
                                                        });
                                                        Some(ComponentAttr { name: Name::new(name), default_value })
                                                    },
                                                    SimpleParameter(simple_parameter) => {
                                                        let name = document.text_for_node(db, simple_parameter.name().ok()?)?;
                                                        let name = name.strip_prefix('$').unwrap();
                                                        let default_value = simple_parameter.default_value().and_then(|val| {
                                                            let text = document.text_for_node(db, val)?;
                                                            Some(SmolStr::new(text))
                                                        });
                                                        Some(ComponentAttr { name: Name::new(name), default_value })
                                                    },
                                                    _ => None,
                                                }
                                            })
                                            .collect::<Vec<_>>());
                                    },
                                    _ => continue,
                                });
                            }
                        },
                        _ => continue,
                    })
                });
                None
            })
            .unwrap_or_default()
    }
    #[tracing::instrument(skip(db))]
    pub fn query(db: &dyn DefDatabase, id: ComponentId) -> Vec<ComponentAttr> {
        let document = id.document(db);
        match id.kind(db) {
            ComponentKind::Class => Self::from_class(db, document),
            ComponentKind::Anon => Self::from_anon(db, document),
        }
    }
}

#[salsa::tracked]
impl ComponentSignature {
    pub fn query(db: &dyn DefDatabase, id: ComponentId) -> Arc<ComponentSignature> {
        let kind = id.kind(db);
        let file = id.file(db);
        let filename = file.path(db).file_name().unwrap();
        let name = match kind {
            ComponentKind::Class => {
                Name::new(&ccase!(kebab, filename.strip_suffix(".php").unwrap()))
            }
            ComponentKind::Anon => Name::new(filename.strip_suffix(".blade.php").unwrap()),
        };
        let attrs = ComponentAttr::query(db, id);
        let attrs = match attrs.len() {
            0 => None,
            _ => Some(attrs.into_boxed_slice()),
        };
        Arc::new(Self { name, attrs })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub id: LayoutId,
}

impl DocumentId for LayoutId {
    fn file_(&self, db: &dyn DocumentDatabase) -> SourceFile {
        self.file(db)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum LayoutKind {
    Class,
    Anon,
}

#[salsa::interned(no_lifetime)]
pub struct LayoutId {
    pub file: SourceFile,
    pub kind: LayoutKind,
}

impl LayoutId {
    pub fn path<'db>(&self, db: &'db dyn DocumentDatabase) -> &'db Utf8Path {
        self.file(db).path(db)
    }
}

pub enum LayoutName {
    Default,
    Name(Name),
}

impl LayoutName {
    pub fn new(name: &str) -> Option<LayoutName> {
        let name = name.strip_prefix("x-")?.strip_suffix("-layout")?;
        let layout = if name.is_empty() {
            LayoutName::Default
        } else {
            LayoutName::Name(Name::new(name))
        };
        Some(layout)
    }

    pub fn tag_name(&self) -> String {
        match self {
            LayoutName::Default => "x-layout".to_string(),
            LayoutName::Name(name) => format!("x-{}-layout", name),
        }
    }

    pub fn class_name(&self) -> String {
        let layout_class_name =
            convert_case::ccase!(pascal, self.tag_name().strip_prefix("x-").unwrap());
        layout_class_name
    }
}

impl Layout {
    pub fn name(&self, db: &dyn DocumentDatabase) -> Name {
        let file = self.id.file(db);
        let kind = self.id.kind(db);
        let filename = file.path(db).file_name().unwrap();
        match kind {
            LayoutKind::Class => Name::new(&ccase!(kebab, filename.strip_suffix(".php").unwrap())),
            LayoutKind::Anon => Name::new(filename.strip_suffix(".blade.php").unwrap()),
        }
    }

    pub fn for_tagname(
        db: &dyn DocumentDatabase,
        tag: ast::blade::TagName<'_>,
        doc: &ParsedDocument,
        config: &Config,
    ) -> Option<Self> {
        let name = doc.text_for_node(db, tag)?;
        let (class_path, resources_path) =
            resolve_path::layout_paths(LayoutName::new(name)?, config);
        if let Some(class_doc) = db.parsed_document(&class_path) {
            let id = LayoutId::new(db, class_doc.source, LayoutKind::Class);
            Some(Self { id })
        } else if let Some(res_doc) = db.parsed_document(&resources_path) {
            let id = LayoutId::new(db, res_doc.source, LayoutKind::Anon);
            Some(Self { id })
        } else {
            None
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directive {
    // conditionals
    If, ElseIf, Else, EndIf,
    Unless, EndUnless,
    Isset, EndIsset,
    Empty, EndEmpty,
    Auth, EndAuth,
    Guest, EndGuest,
    Production, EndProduction,
    Env, EndEnv,
    Session, EndSession,
    Context, EndContext,

    HasSection, SectionMissing,

    // Switch statement
    Switch, Case, Default, EndSwitch,

    Break,

    Continue,

    // Loops
    For, EndFor,
    Foreach, EndForeach,
    Forelse, EndForelse,
    While, EndWhile,

    // Attribute directive
    Class, Style, Disabled,
    Checked, Selected, ReadOnly, Required,

    // Include
    Include, IncludeIf, IncludeWhen, IncludeUnless, IncludeFirst, IncludeIsolated,

    Each,

    Once,

    Php, EndPhp,

    Use,
}

impl Directive {
    pub fn is_inline(&self) -> bool {
        self.ender().is_none()
    }

    pub fn from_node(node: type_sitter::UntypedNode<'_>) -> Option<Self> {
        let directive = ast::match_node!(node, {
            ast::blade::symbols::Atif(_) => Directive::If,
            ast::blade::symbols::Atunless(_) => Directive::Unless,
            ast::blade::symbols::Atisset(_) => Directive::Isset,
            ast::blade::symbols::Atempty(_) => Directive::Empty,
            ast::blade::symbols::Atauth(_) => Directive::Auth,
            ast::blade::symbols::Atguest(_) => Directive::Guest,
            ast::blade::symbols::Atproduction(_) => Directive::Production,
            ast::blade::symbols::Atenv(_) => Directive::Env,
            ast::blade::symbols::Atsession(_) => Directive::Session,
            ast::blade::symbols::Atcontext(_) => Directive::Context,
            ast::blade::symbols::Athassection(_) => Directive::HasSection,
            ast::blade::symbols::Atsectionmissing(_) => Directive::SectionMissing,
            ast::blade::symbols::Atswitch(_) => Directive::Switch,
            ast::blade::symbols::Atfor(_) => Directive::For,
            ast::blade::symbols::Atforeach(_) => Directive::Foreach,
            ast::blade::symbols::Atforelse(_) => Directive::Forelse,
            ast::blade::symbols::Atwhile(_) => Directive::While,
            ast::blade::symbols::Atclass(_) => Directive::Class,
            ast::blade::symbols::Atstyle(_) => Directive::Style,
            ast::blade::symbols::Atdisabled(_) => Directive::Disabled,
            ast::blade::symbols::Atchecked(_) => Directive::Checked,
            ast::blade::symbols::Atselected(_) => Directive::Selected,
            ast::blade::symbols::Atreadonly(_) => Directive::ReadOnly,
            ast::blade::symbols::Atrequired(_) => Directive::Required,
            ast::blade::symbols::Atinclude(_) => Directive::Include,
            ast::blade::symbols::Atincludeif(_) => Directive::IncludeIf,
            ast::blade::symbols::Atincludewhen(_) => Directive::IncludeWhen,
            ast::blade::symbols::Atincludeunless(_) => Directive::IncludeUnless,
            ast::blade::symbols::Atincludefirst(_) => Directive::IncludeFirst,
            ast::blade::symbols::Atincludeisolated(_) => Directive::IncludeIsolated,
            ast::blade::symbols::Atcase(_) => Directive::Case,
            ast::blade::symbols::Atdefault(_) => Directive::Default,
            ast::blade::symbols::Atbreak(_) => Directive::Break,
            ast::blade::symbols::Atcontinue(_) => Directive::Continue,
            ast::blade::symbols::Ateach(_) => Directive::Each,
            ast::blade::symbols::Atonce(_) => Directive::Once,
            ast::blade::symbols::Atphp(_) => Directive::Php,
            ast::blade::symbols::Atuse(_) => Directive::Use,
            ast::blade::symbols::Atelseif(_) => Directive::ElseIf,
            ast::blade::symbols::Atelse(_) => Directive::Else,
            ast::blade::symbols::Atendif(_) => Directive::EndIf,
            ast::blade::symbols::Atendisset(_) => Directive::EndIsset,
            ast::blade::symbols::Atendunless(_) => Directive::EndUnless,
            ast::blade::symbols::Atendempty(_) => Directive::EndEmpty,
            ast::blade::symbols::Atendauth(_) => Directive::EndAuth,
            ast::blade::symbols::Atendguest(_) => Directive::EndGuest,
            ast::blade::symbols::Atendproduction(_) => Directive::EndProduction,
            ast::blade::symbols::Atendenv(_) => Directive::EndEnv,
            ast::blade::symbols::Atendsession(_) => Directive::EndSession,
            ast::blade::symbols::Atendcontext(_) => Directive::EndContext,
            ast::blade::symbols::Atendswitch(_) => Directive::EndSwitch,
            ast::blade::symbols::Atendfor(_) => Directive::EndFor,
            ast::blade::symbols::Atendwhile(_) => Directive::EndWhile,
            ast::blade::symbols::Atendforeach(_) => Directive::EndForeach,
            ast::blade::symbols::Atendforelse(_) => Directive::EndForelse,
            ast::blade::symbols::Atendphp(_) => Directive::EndPhp,
            _ => return None,
        });
        Some(directive)
    }

    pub fn is_end(&self) -> bool {
        self.lookup().starts_with("end")
    }

    pub fn ender(&self) -> Option<Self> {
        let me = match self {
            Directive::If | Directive::HasSection | Directive::SectionMissing => Directive::EndIf,
            Directive::Isset => Directive::EndIsset,
            Directive::Unless => Directive::EndUnless,
            Directive::Empty => Directive::EndEmpty,
            Directive::Auth => Directive::EndAuth,
            Directive::Guest => Directive::EndGuest,
            Directive::Production => Directive::EndProduction,
            Directive::Env => Directive::EndEnv,
            Directive::Session => Directive::EndSession,
            Directive::Context => Directive::EndContext,
            Directive::Switch => Directive::EndSwitch,
            Directive::For => Directive::EndFor,
            Directive::While => Directive::EndWhile,
            Directive::Foreach => Directive::EndForeach,
            Directive::Forelse => Directive::EndForelse,
            Directive::Php => Directive::EndPhp,
            _ => return None,
        };
        Some(me)
    }

    pub fn globally_available() -> Vec<Self> {
        use Directive::*;
        vec![
            If,
            EndIf,
            Unless,
            EndUnless,
            Isset,
            EndIsset,
            Empty,
            EndEmpty,
            Auth,
            EndAuth,
            Guest,
            EndGuest,
            Production,
            EndProduction,
            Env,
            EndEnv,
            Session,
            EndSession,
            Context,
            EndContext,
            HasSection,
            SectionMissing,
            Switch,
            EndSwitch,
            For,
            EndFor,
            Foreach,
            EndForeach,
            Forelse,
            EndForelse,
            While,
            EndWhile,
            Php,
            EndPhp,
        ]
    }

    // TODO: What to make of directives with optional paramaters?
    pub fn has_param(&self, switched: bool) -> bool {
        if matches!(self, Directive::Break) && switched {
            return false;
        }
        !self.is_end()
            && !matches!(
                self,
                Directive::Php
                    | Directive::Production
                    | Directive::Default
                    | Directive::Empty
                    | Directive::Else
                    | Directive::Once
            )
    }

    pub fn lookup(&self) -> String {
        let label = self.label();
        label.replace('@', "")
    }

    pub fn label(&self) -> &'static str {
        match self {
            Directive::Default => "@default",
            Directive::If => "@if",
            Directive::ElseIf => "@elseif",
            Directive::Else => "@else",
            Directive::EndIf => "@endif",
            Directive::Unless => "@unless",
            Directive::EndUnless => "@endunless",
            Directive::Isset => "@isset",
            Directive::EndIsset => "@endisset",
            Directive::Empty => "@empty",
            Directive::EndEmpty => "@endempty",
            Directive::Auth => "@auth",
            Directive::EndAuth => "@endauth",
            Directive::Guest => "@guest",
            Directive::EndGuest => "@endguest",
            Directive::Production => "@production",
            Directive::EndProduction => "@endproduction",
            Directive::Env => "@env",
            Directive::EndEnv => "@endenv",
            Directive::Session => "@session",
            Directive::EndSession => "@endsession",
            Directive::Context => "@context",
            Directive::EndContext => "@endcontext",
            Directive::HasSection => "@hassection",
            Directive::SectionMissing => "@sectionmissing",
            Directive::Switch => "@switch",
            Directive::Case => "@case",
            Directive::EndSwitch => "@endswitch",
            Directive::Break => "@break",
            Directive::Continue => "@continue",
            Directive::For => "@for",
            Directive::EndFor => "@endfor",
            Directive::Foreach => "@foreach",
            Directive::EndForeach => "@endforeach",
            Directive::Forelse => "@forelse",
            Directive::EndForelse => "@endforelse",
            Directive::While => "@while",
            Directive::EndWhile => "@endwhile",
            Directive::Class => "@class",
            Directive::Style => "@style",
            Directive::Disabled => "@disabled",
            Directive::Checked => "@checked",
            Directive::Selected => "@selected",
            Directive::ReadOnly => "@readonly",
            Directive::Required => "@required",
            Directive::Include => "@include",
            Directive::IncludeIf => "@includeif",
            Directive::IncludeWhen => "@includewhen",
            Directive::IncludeUnless => "@includeunless",
            Directive::IncludeFirst => "@includefirst",
            Directive::IncludeIsolated => "@includeisolated",
            Directive::Each => "@each",
            Directive::Once => "@once",
            Directive::Php => "@php",
            Directive::EndPhp => "@endphp",
            Directive::Use => "@use",
        }
    }
}

#[salsa::interned(no_lifetime)]
pub struct ComponentId {
    file: SourceFile,
    kind: ComponentKind,
}
impl std::fmt::Debug for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ComponentId").field(&self.0).finish()
    }
}

pub trait Document {
    type Id: DocumentId;
    fn id(&self) -> Self::Id;
}

impl Document for Component {
    type Id = ComponentId;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Document for Layout {
    type Id = LayoutId;
    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait DocumentId {
    fn file_(&self, db: &dyn DocumentDatabase) -> SourceFile;

    fn path<'db>(&self, db: &'db dyn DocumentDatabase) -> &'db Utf8Path {
        self.file_(db).path(db)
    }

    fn document(&self, db: &dyn DocumentDatabase) -> ParsedDocument {
        let file = self.file_(db);
        db.parsed_document(file.path(db)).unwrap()
    }
}

impl DocumentId for ComponentId {
    fn file_(&self, db: &dyn DocumentDatabase) -> SourceFile {
        self.file(db)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Component {
    pub(crate) id: ComponentId,
}

pub struct ComponentName(Name);

impl ComponentName {
    pub fn new(name: &str) -> Option<ComponentName> {
        if name.ends_with("-layout") {
            return None;
        }
        let name = name.strip_prefix("x-")?;
        Some(ComponentName(Name::new(name)))
    }

    pub fn path(&self) -> String {
        let ComponentName(name) = self;
        name.as_str().replace('.', std::path::MAIN_SEPARATOR_STR)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ComponentKind {
    Class,
    Anon,
}

pub fn get_tag_name<'tree, Tag: Node<'tree>>(tag: Tag) -> Option<ast::blade::TagName<'tree>> {
    let tag_name = ast::match_node!(tag, {
        ast::blade::SelfClosingTag(self_tag) => {
            self_tag.tag_name()
        },
        ast::blade::StartTag(start_tag) => {
            start_tag.tag_name()
        },
        ast::blade::EndTag(end_tag) => {
            end_tag.tag_name()
        },
        _ => return None,
    });
    tag_name.ok()
}

impl Component {
    #[inline]
    pub fn signature(&self, db: &dyn DefDatabase) -> Arc<ComponentSignature> {
        db.component_signature(self.id)
    }

    pub fn name(&self, db: &dyn DefDatabase) -> Name {
        self.signature(db).name.clone()
    }

    pub fn attrs(&self, db: &dyn DefDatabase) -> Option<Box<[ComponentAttr]>> {
        self.signature(db).attrs.clone()
    }

    pub fn for_attr(
        db: &dyn DefDatabase,
        attr: ast::blade::Attribute,
        doc: &ParsedDocument,
        config: &Config,
    ) -> Option<Self> {
        let tag = attr.parent()?;
        let tag_name = get_tag_name(tag)?;
        let component = Self::for_tagname(db, tag_name, doc, config)?;
        Some(component)
    }

    pub fn for_tagname(
        db: &dyn DocumentDatabase,
        tag: ast::blade::TagName<'_>,
        doc: &ParsedDocument,
        config: &Config,
    ) -> Option<Self> {
        let name = doc.text_for_node(db, tag)?;
        let (class_path, resources_path) =
            resolve_path::component_paths(ComponentName::new(name)?, config);
        if let Some(class_doc) = db.parsed_document(&class_path) {
            let id = ComponentId::new(db, class_doc.source, ComponentKind::Class);
            Some(Self { id })
        } else if let Some(res_doc) = db.parsed_document(&resources_path) {
            let id = ComponentId::new(db, res_doc.source, ComponentKind::Anon);
            Some(Self { id })
        } else {
            None
        }
    }
}
