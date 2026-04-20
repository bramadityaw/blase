/// `CompletionItem` describes a single completion entity which expands to 1 or more entries in the
/// editor pop-up.
pub struct CompletionItem {
    label: String,

    /// Range of identifier that is being completed.
    ///
    /// It should be used primarily for UI, but we also use this to convert
    /// generic TextEdit into LSP's completion edit (see conv.rs).
    ///
    /// `source_range` must contain the completion offset. `text_edit` should
    /// start with what `source_range` points to, or VSCode will filter out the
    /// completion silently.
    pub source_range: TextRange,
    /// What happens when user selects this item.
    pub text_edit: TextEdit,
}
