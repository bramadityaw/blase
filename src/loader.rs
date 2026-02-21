use std::{path::Path, sync::mpsc};

use async_lsp::lsp_types::{
    ProgressParamsValue, Url, WorkDoneProgress, WorkDoneProgressBegin, WorkDoneProgressEnd,
    WorkDoneProgressReport,
};
use crossbeam_channel::{Sender, unbounded};
use walkdir::{DirEntry, WalkDir};

use crate::server::ServerState;

fn walk_files(path: &Path) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file() && e.path().is_absolute())
}

impl ServerState {
    pub fn load_workspace(&mut self, progress_sender: Sender<ProgressParamsValue>) {
        let _p = tracing::info_span!("load_workspace").entered();
        let workspace = self
            .config
            .read()
            .expect("poison")
            .workspace_folder
            .uri
            .to_file_path()
            .unwrap();

        tracing::info!("loading workspace at: {:?}", workspace.as_path());

        let mut entries = walk_files(&workspace.join("resources/views")).collect::<Vec<_>>();
        entries.extend(walk_files(&workspace.join("app/Views")));

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
                    let path = entry.path();
                    if let Ok(contents) = std::fs::read_to_string(path)
                        && let Ok(url) = Url::from_file_path(path)
                    {
                        _ = tx.send((url, contents));
                        let percentage = ((i + 1) as f64 / total_entries as f64 * 100.0) as u32;

                        _ = progress_sender.send(ProgressParamsValue::WorkDone(
                            WorkDoneProgress::Report(WorkDoneProgressReport {
                                cancellable: None,
                                message: Some(format!(
                                    "{} {} of {}",
                                    path.to_str().unwrap(),
                                    i + 1,
                                    total_entries
                                )),
                                percentage: Some(percentage),
                            }),
                        ));
                    }
                }

                _ = progress_sender.send(ProgressParamsValue::WorkDone(WorkDoneProgress::End(
                    WorkDoneProgressEnd { message: None },
                )));
            }
        });
        while let Ok((url, contents)) = rx.try_recv() {
            self.analysis_host.set_source_file(url, &contents);
        }
    }
}
