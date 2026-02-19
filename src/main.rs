use tracing::Level;

#[tokio::main]
async fn main() {
    let dir = std::env::temp_dir();
    eprintln!("file logging at directory: {dir:?}");

    let file_appender = tracing_appender::rolling::daily(dir, "blase.log");
    let (file_appender, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(Level::INFO)
        .with_writer(file_appender)
        .init();

    // Prefer truly asynchronous piped stdin/stdout without blocking tasks.
    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
        async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
    );
    // Fallback to spawn blocking read/write otherwise.
    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    if let Err(e) = blase::server::run_server(stdin, stdout).await {
        tracing::error!("Failed running server: {e}");
        std::process::exit(-1);
    };
}
