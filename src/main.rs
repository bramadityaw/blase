use blase::Server;
use tokio;
use tower_lsp::{Client, LspService, Server as LspServer};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init_server(client: Client) -> Server {
    Server::new(client)
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(init_server);
    LspServer::new(stdin, stdout, socket).serve(service).await;
}
