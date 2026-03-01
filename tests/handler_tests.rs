use std::ops::ControlFlow;

use async_lsp::ClientSocket;
use async_lsp::lsp_types::{
    ClientCapabilities, DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams,
    InitializedParams, Position, TextDocumentContentChangeEvent, TextDocumentItem, Url,
    VersionedTextDocumentIdentifier, WorkspaceFolder,
};

use blase::document_data::DocumentData;
use blase::handler;
use blase::server::ServerState;

fn create_test_server() -> ServerState {
    let client = ClientSocket::new_closed();
    ServerState::new(client)
}

fn create_test_url(path: &str) -> Url {
    #[cfg(unix)]
    let path = &format!("file://{}", path);
    #[cfg(not(unix))]
    let path = &format!("file://C:/{}", path);
    Url::parse(path).unwrap()
}

#[test]
fn test_handle_initialize_with_workspace_folder() {
    let mut server = create_test_server();
    let url = create_test_url("/test/workspace");
    let params = InitializeParams {
        workspace_folders: Some(vec![WorkspaceFolder {
            uri: url.clone(),
            name: "test".to_string(),
        }]),
        capabilities: ClientCapabilities::default(),
        ..Default::default()
    };

    let result = futures::executor::block_on(handler::handle_initialize(&mut server, params));

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.server_info.is_some());
    let server_info = result.server_info.unwrap();
    assert_eq!(server_info.name, "blase");
    assert!(server_info.version.is_some());
}

#[test]
fn test_handle_initialize_multiple_workspaces_returns_error() {
    let mut server = create_test_server();
    let params = InitializeParams {
        workspace_folders: Some(vec![
            WorkspaceFolder {
                uri: create_test_url("/test/workspace1"),
                name: "workspace1".to_string(),
            },
            WorkspaceFolder {
                uri: create_test_url("/test/workspace2"),
                name: "workspace2".to_string(),
            },
        ]),
        capabilities: ClientCapabilities::default(),
        ..Default::default()
    };

    let result = futures::executor::block_on(handler::handle_initialize(&mut server, params));

    assert!(result.is_err());
}

#[test]
fn test_handle_did_open_inserts_document() {
    let mut server = create_test_server();
    let url = create_test_url("/test/file.blade.php");
    let params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: url.clone(),
            language_id: "blade".to_string(),
            version: 1,
            text: "Hello World".to_string(),
        },
    };

    let result = handler::handle_did_open(&mut server, params);

    assert!(matches!(result, ControlFlow::Continue(())));
    assert!(server.documents.contains_key(&url));
    let doc = server.documents.get(&url).unwrap();
    assert_eq!(doc.contents, "Hello World");
}

#[test]
fn test_handle_did_change_updates_document() {
    let mut server = create_test_server();
    let url = create_test_url("/test/file.blade.php");

    server.documents.insert(
        url.clone(),
        DocumentData {
            contents: "Hello World".to_string(),
        },
    );

    let params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: url.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Some(async_lsp::lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 5,
                },
                end: Position {
                    line: 0,
                    character: 5,
                },
            }),
            range_length: None,
            text: " Rust".to_string(),
        }],
    };

    let result = handler::handle_did_change(&mut server, params);

    assert!(matches!(result, ControlFlow::Continue(())));
    let doc = server.documents.get(&url).unwrap();
    assert_eq!(doc.contents, "Hello Rust World");
}

#[test]
fn test_handle_initialized_returns_continue() {
    let mut server = create_test_server();
    let params = InitializedParams {};

    let result = handler::handle_initialized(&mut server, params);

    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_handle_initialize_does_not_load_files_into_analysis() {
    let mut server = create_test_server();
    let url = create_test_url("/test/workspace");
    let params = InitializeParams {
        workspace_folders: Some(vec![WorkspaceFolder {
            uri: url.clone(),
            name: "test".to_string(),
        }]),
        capabilities: ClientCapabilities::default(),
        ..Default::default()
    };

    let _result = futures::executor::block_on(handler::handle_initialize(&mut server, params));

    let analysis = server.snapshot().analysis;
    let db = analysis.raw_database();

    let files_count = db.files_count();
    assert_eq!(
        files_count, 0,
        "handle_initialize should not load files into analysis database"
    );
}

#[test]
fn test_analysis_host_can_load_files() {
    let mut server = create_test_server();
    let url = create_test_url("/test/resources/views/index.blade.php");
    let contents = "<div>Hello World</div>";

    server.analysis_host.set_source_file(url.clone(), contents);

    let analysis = server.snapshot().analysis;
    let db = analysis.raw_database();

    let files_count = db.files_count();
    assert_eq!(
        files_count, 1,
        "analysis host should have one file after set_source_file"
    );
}
