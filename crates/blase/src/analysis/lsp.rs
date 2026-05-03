use camino::Utf8Path;

use crate::{
    analysis::{
        Analysis, Cancellable, completions, diagnostics, goto_definition, hover, references,
        signature_help, workspace_symbols,
    },
    config::Config,
    db::{FilePosition, FileRange},
};

impl Analysis {
    #[tracing::instrument(skip(self, config))]
    pub fn completion(
        &self,
        config: &Config,
        position: FilePosition,
        trigger_char: Option<char>,
    ) -> Cancellable<Option<Vec<completions::CompletionItem>>> {
        self.with_db(|db| completions::completions(db, config, position, trigger_char))
    }

    #[tracing::instrument(skip(self, config))]
    pub fn hover(
        &self,
        config: &Config,
        position: FilePosition,
    ) -> Cancellable<Option<hover::HoverResult>> {
        self.with_db(|db| hover::hover(db, config, position))
    }

    #[tracing::instrument(skip(self, config))]
    pub fn signature_help(
        &self,
        config: &Config,
        position: FilePosition,
    ) -> Cancellable<Option<signature_help::SignatureHelp>> {
        self.with_db(|db| signature_help::signature_help(db, config, position))
    }

    #[tracing::instrument(skip(self, config))]
    pub fn goto_def(&self, config: &Config, position: FilePosition) -> Cancellable<Vec<FileRange>> {
        self.with_db(|db| {
            goto_definition::goto_definition(db, config, position).unwrap_or_default()
        })
    }

    #[tracing::instrument(skip(self, config))]
    pub fn semantic_diagnostics(
        &self,
        config: &Config,
        path: &Utf8Path,
    ) -> Cancellable<Vec<diagnostics::Diagnostic>> {
        self.with_db(|db| diagnostics::semantic_diagnostics(db, config, path))
    }

    #[tracing::instrument(skip(self, config))]
    pub fn full_diagnostics(
        &self,
        config: &Config,
        path: &Utf8Path,
    ) -> Cancellable<Vec<diagnostics::Diagnostic>> {
        self.with_db(|db| diagnostics::full_diagnostics(db, config, path))
    }

    #[tracing::instrument(skip(self, config))]
    pub fn references(
        &self,
        config: &Config,
        position: FilePosition,
    ) -> Cancellable<Option<Vec<references::ReferenceSearchResult>>> {
        self.with_db(|db| references::references(db, config, position))
    }

    #[tracing::instrument(skip(self, config))]
    pub fn workspace_symbols(
        &self,
        config: &Config,
        query: String,
    ) -> Cancellable<Option<Vec<workspace_symbols::SymbolInformation>>> {
        self.with_db(|db| workspace_symbols::workspace_symbols(db, query, config))
    }
}
