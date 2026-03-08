use std::path::Path;

use async_lsp::lsp_types::{
    ProgressParamsValue, WorkDoneProgress, WorkDoneProgressBegin, WorkDoneProgressEnd,
    WorkDoneProgressReport,
};
use camino::Utf8PathBuf;
use crossbeam_channel::{Sender, unbounded};
use walkdir::{DirEntry, WalkDir};

use crate::server::ServerState;

fn walk_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path.as_ref())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file() && e.path().is_absolute())
}

impl ServerState {
    pub fn load_workspace(&mut self, progress_sender: Sender<ProgressParamsValue>) {
        let _p = tracing::info_span!("load_workspace").entered();
        let workspace = self.config.read().expect("poison").workspace_folder.clone();

        tracing::info!("loading workspace at: {:?}", workspace.as_path());

        let mut entries = walk_files(&workspace.join("resources/views")).collect::<Vec<_>>();
        entries.extend(walk_files(&workspace.join("app/View")));

        let total_entries = entries.len();

        _ = progress_sender.send(ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(
            WorkDoneProgressBegin {
                title: "Loading files".to_string(),
                cancellable: None,
                message: Some(format!("{} of {}", 0, total_entries)),
                percentage: Some(0),
            },
        )));
        let (tx, rx) = unbounded();
        std::thread::spawn({
            move || {
                for (i, entry) in entries.into_iter().enumerate() {
                    let path = entry.path().to_owned();
                    if let Ok(contents) = std::fs::read_to_string(&path)
                        && let Ok(path) = Utf8PathBuf::from_path_buf(path)
                    {
                        let percentage = ((i + 1) as f64 / total_entries as f64 * 100.0) as u32;

                        _ = progress_sender.send(ProgressParamsValue::WorkDone(
                            WorkDoneProgress::Report(WorkDoneProgressReport {
                                cancellable: None,
                                message: Some(format!("{} {} of {}", path, i + 1, total_entries)),
                                percentage: Some(percentage),
                            }),
                        ));

                        _ = tx.send((path, contents));
                    }
                }

                _ = progress_sender.send(ProgressParamsValue::WorkDone(WorkDoneProgress::End(
                    WorkDoneProgressEnd { message: None },
                )));
            }
        });
        while let Ok((path, contents)) = rx.recv() {
            tracing::trace!(%path, len = contents.len());
            self.analysis_host.set_source_file(path, &contents);
        }
    }
}
