use crate::{
    analysis::{Analysis, Cancellable, completions, goto_definition, hover, signature_help},
    config::Config,
    db::{FilePosition, FileRange},
};

impl Analysis {
    #[tracing::instrument(skip(self, config, position), level = "debug")]
    pub fn completion(
        &self,
        config: &Config,
        position: FilePosition,
        trigger_char: Option<char>,
    ) -> Cancellable<Option<Vec<completions::CompletionItem>>> {
        self.with_db(|db| completions::completions(db, config, position, trigger_char))
    }

    #[tracing::instrument(skip(self, config, position), level = "debug")]
    pub fn hover(
        &self,
        config: &Config,
        position: FilePosition,
    ) -> Cancellable<Option<hover::HoverResult>> {
        self.with_db(|db| hover::hover(db, config, position))
    }

    #[tracing::instrument(skip(self, config, position), level = "debug")]
    pub fn signature_help(
        &self,
        config: &Config,
        position: FilePosition,
    ) -> Cancellable<Option<signature_help::SignatureHelp>> {
        self.with_db(|db| signature_help::signature_help(db, config, position))
    }

    #[tracing::instrument(skip(self, config, position), level = "debug")]
    pub fn goto_def(&self, config: &Config, position: FilePosition) -> Cancellable<Vec<FileRange>> {
        self.with_db(|db| {
            goto_definition::goto_definition(db, config, position).unwrap_or_default()
        })
    }
}
