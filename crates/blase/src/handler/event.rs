use std::ops::ControlFlow;

use camino::Utf8PathBuf;

use crate::{lsp, server::ServerState};

#[derive(Debug)]
pub enum Event {
    DiagnosticUpdate(Utf8PathBuf),
}

#[test]
fn event_is_send() {
    fn is_send<T: Send>() {}
    is_send::<Event>();
}

#[tracing::instrument(skip(server))]
pub fn handle_event(
    server: &mut ServerState,
    event: Event,
) -> ControlFlow<Result<(), async_lsp::Error>> {
    let result = (|| match event {
        Event::DiagnosticUpdate(ref path) => {
            let snap = server.snapshot();
            let Some(line_index) = &snap.file_line_index(path)? else {
                return Ok(());
            };
            let analysis = snap.analysis;
            let config = &snap.config.read().expect("poison");

            let diags = analysis
                .full_diagnostics(config, path)?
                .into_iter()
                .map(|d| lsp::into_proto::diagnostic(line_index, d))
                .collect::<Vec<_>>();

                server.publish_diagnostics(lsp::into_proto::url(path), diags, None);
            Ok(())
        }
    })();

    match lsp::into_proto::cancellable(result) {
        Ok(()) => ControlFlow::Continue(()),
        Err(err) => ControlFlow::Break(Err(err.into())),
    }
}
