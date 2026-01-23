use tower_lsp::{
    jsonrpc,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
        ServerInfo, TextDocumentItem,
    },
};

use crate::{Server, document_data::DocumentData};

pub async fn handle_did_change(server: &Server, params: DidChangeTextDocumentParams) {
    let uri = params.text_document.uri;
    let mut documents = server.documents.write().await;
    if let Some(document) = documents.get_mut(&uri) {
        let new_contents = crate::util::apply_document_changes(
            server.negotiated_encoding().await,
            std::str::from_utf8(&document.data).unwrap(),
            params.content_changes,
        )
        .into_bytes();
        let mut parser = server.parser.lock().await;
        // Ideally, we also pass in the old tree.
        let tree = parser
            .parse(new_contents.as_slice(), None)
            .expect("Language has been set at Server construction");
        *document = DocumentData {
            version: params.text_document.version,
            data: new_contents,
            tree,
        };
    }
}

pub async fn handle_did_open(server: &Server, params: DidOpenTextDocumentParams) {
    let DidOpenTextDocumentParams {
        text_document:
            TextDocumentItem {
                uri,
                language_id: _,
                version,
                text,
            },
    } = params;
    let mut documents = server.documents.write().await;
    let mut parser = server.parser.lock().await;
    let tree = parser
        .parse(&text, None)
        .expect("Language has been set at Server construction");
    let document = DocumentData {
        data: text.into_bytes(),
        tree,
        version,
    };
    documents.insert(uri, document);
}

pub async fn handle_initialize(
    server: &Server,
    params: InitializeParams,
) -> jsonrpc::Result<InitializeResult> {
    let server_info = ServerInfo {
        name: "blase".into(),
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
    };

    {
        let mut caps = server.caps.write().await;
        *caps = params.capabilities;
    }

    let result = InitializeResult {
        capabilities: crate::server::server_capabilities(),
        server_info: Some(server_info),
        offset_encoding: None,
    };

    Ok(result)
}
