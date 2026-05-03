use std::collections::HashSet;

use async_lsp::lsp_types::{
    CompletionOptions, CompletionOptionsCompletionItem, HoverProviderCapability, OneOf,
    PositionEncodingKind, SaveOptions, ServerCapabilities, SignatureHelpOptions,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    WorkDoneProgressOptions,
};
use line_index::WideEncoding;

use crate::{
    analysis::completions::{self, CompletionFieldsToResolve},
    config::Config,
    line_index::PositionEncoding,
};

impl Config {
    pub fn completion_resolve_support_properties(&self) -> HashSet<&str> {
        self.capabilities
            .text_document
            .as_ref()
            .and_then(|text| text.completion.as_ref())
            .and_then(|completion_caps| completion_caps.completion_item.as_ref())
            .and_then(|completion_item_caps| completion_item_caps.resolve_support.as_ref())
            .map(|resolve_support| resolve_support.properties.iter())
            .into_iter()
            .flatten()
            .map(|s| s.as_str())
            .collect()
    }

    fn completions_resolve_provider(&self) -> bool {
        let client_capabilities = self.completion_resolve_support_properties();
        let fields_to_resolve =
            CompletionFieldsToResolve::from_client_capabilities(&client_capabilities);
        fields_to_resolve != CompletionFieldsToResolve::empty()
    }

    #[allow(non_snake_case)]
    pub fn has_completion_item_resolve_additionalTextEdits(&self) -> bool {
        (|| {
            Some(
                self.capabilities
                    .text_document
                    .as_ref()?
                    .completion
                    .as_ref()?
                    .completion_item
                    .as_ref()?
                    .resolve_support
                    .as_ref()?
                    .properties
                    .iter()
                    .any(|cap_string| cap_string.as_str() == "additionalTextEdits"),
            )
        })() == Some(true)
    }

    fn completion_label_details_support(&self) -> bool {
        (|| -> _ {
            self.capabilities
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .label_details_support
        })() == Some(true)
    }

    pub fn completion_item(&self) -> Option<CompletionOptionsCompletionItem> {
        Some(CompletionOptionsCompletionItem {
            label_details_support: Some(self.completion_label_details_support()),
        })
    }

    pub fn insert_replace_support(&self) -> bool {
        (|| {
            self.capabilities
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .insert_replace_support
        })()
        .unwrap_or_default()
    }

    pub fn signature_help_label_offsets(&self) -> bool {
        (|| {
            self.capabilities
                .text_document
                .as_ref()?
                .signature_help
                .as_ref()?
                .signature_information
                .as_ref()?
                .parameter_information
                .as_ref()?
                .label_offset_support
        })()
        .unwrap_or_default()
    }

    pub fn did_save_text_document_dynamic_registration(&self) -> bool {
        let caps = (|| -> _ {
            self.capabilities
                .text_document
                .as_ref()?
                .synchronization
                .clone()
        })()
        .unwrap_or_default();
        caps.did_save == Some(true) && caps.dynamic_registration == Some(true)
    }
}

pub fn server_capabilities(config: &Config) -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: match config.negotiated_encoding() {
            PositionEncoding::Utf8 => Some(PositionEncodingKind::UTF8),
            PositionEncoding::Wide(wide) => match wide {
                WideEncoding::Utf16 => Some(PositionEncodingKind::UTF16),
                WideEncoding::Utf32 => Some(PositionEncodingKind::UTF32),
                _ => None,
            },
        },
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                will_save: None,
                will_save_wait_until: None,
                save: if config.did_save_text_document_dynamic_registration() {
                    None
                } else {
                    Some(SaveOptions::default().into())
                },
            },
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: {
                let trigger_chars = ['(', '"', '\'', '='];
                Some(trigger_chars.into_iter().map(String::from).collect())
            },
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        completion_provider: Some(CompletionOptions {
            resolve_provider: if config.client_is_neovim() {
                config
                    .has_completion_item_resolve_additionalTextEdits()
                    .then_some(true)
            } else {
                Some(config.completions_resolve_provider())
            },
            trigger_characters: {
                let trigger_chars = completions::TRIGGER_CHARS;
                Some(trigger_chars.into_iter().copied().map(String::from).collect())
            },
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
            completion_item: config.completion_item(),
        }),
        references_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),

        // Methods below this line are unsupported
        document_formatting_provider: None,
        diagnostic_provider: None,
        rename_provider: None,
        semantic_tokens_provider: None,
        selection_range_provider: None,
        type_definition_provider: None,
        implementation_provider: None,
        document_highlight_provider: None,
        document_symbol_provider: None,
        code_action_provider: None,
        code_lens_provider: None,
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        document_link_provider: None,
        color_provider: None,
        folding_range_provider: None,
        declaration_provider: None,
        execute_command_provider: None,
        workspace: None,
        call_hierarchy_provider: None,
        moniker_provider: None,
        linked_editing_range_provider: None,
        inline_value_provider: None,
        inlay_hint_provider: None,
        experimental: None,
    }
}
