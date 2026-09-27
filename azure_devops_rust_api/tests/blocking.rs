// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// Tests for the blocking client, using a local mock HTTP server.
#![cfg(all(feature = "git", feature = "blocking"))]

use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
use azure_devops_rust_api::{git, Credential};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{mpsc, Arc};

const REPO_LIST_JSON: &str = r#"{"count":2,"value":[
    {"id":"r1","name":"repo1","project":{"name":"proj","visibility":"private"},"url":"http://x/r1"},
    {"id":"r2","name":"repo2","project":{"name":"proj","visibility":"private"},"url":"http://x/r2"}
]}"#;

/// A captured HTTP request: request line, lower-cased headers, and body.
struct Captured {
    request_line: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Captured {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.as_str())
    }
}

/// Starts a server that answers a single request with the given status and body.
/// Returns the server's base URL and a receiver for the captured request.
fn mock_server(status: u16, body: &'static str) -> (String, mpsc::Receiver<Captured>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();
        let mut headers = Vec::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let line = line.trim_end();
            if line.is_empty() {
                break;
            }
            let (name, value) = line.split_once(':').unwrap();
            headers.push((name.to_lowercase(), value.trim().to_string()));
        }
        let content_length = headers
            .iter()
            .find(|(n, _)| n == "content-length")
            .map_or(0, |(_, v)| v.parse().unwrap());
        let mut request_body = vec![0; content_length];
        reader.read_exact(&mut request_body).unwrap();
        tx.send(Captured {
            request_line: request_line.trim_end().to_string(),
            headers,
            body: String::from_utf8(request_body).unwrap(),
        })
        .unwrap();
        let mut stream = stream;
        write!(
            stream,
            "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
    });
    (url, rx)
}

fn url(url: &str) -> azure_core::http::Url {
    azure_core::http::Url::parse(url).unwrap()
}

fn blocking_git_client(server_url: &str) -> git::blocking::Client {
    git::blocking::ClientBuilder::new(Credential::from_pat("secret"))
        .endpoint(url(server_url))
        .build()
}

#[test]
fn query_returns_parsed_body() {
    let (server_url, rx) = mock_server(200, REPO_LIST_JSON);
    let repos = blocking_git_client(&server_url)
        .repositories_client()
        .list("org", "proj")
        .send()
        .unwrap()
        .into_body()
        .unwrap();
    assert_eq!(repos.count, Some(2));
    assert_eq!(repos.value[1].name, "repo2");

    let req = rx.recv().unwrap();
    assert_eq!(
        req.request_line,
        "GET //org/proj/_apis/git/repositories?api-version=7.1-preview HTTP/1.1"
    );
    // base64(":secret")
    assert_eq!(req.header("authorization"), Some("Basic OnNlY3JldA=="));
    // Set by the azure_core pipeline, which the blocking client uses.
    assert!(req
        .header("user-agent")
        .unwrap()
        .contains("azsdk-rust-devops_rust_api/"));
}

#[test]
fn post_sends_json_body() {
    let (server_url, rx) = mock_server(
        201,
        r#"{"id":"r3","name":"new","project":{"name":"proj","visibility":"private"},"url":"http://x/r3"}"#,
    );
    let options = git::models::GitRepositoryCreateOptions {
        name: Some("new".to_string()),
        ..Default::default()
    };
    let repo = blocking_git_client(&server_url)
        .repositories_client()
        .create("org", options, "proj")
        .send()
        .unwrap()
        .into_body()
        .unwrap();
    assert_eq!(repo.id, "r3");

    let req = rx.recv().unwrap();
    assert!(req
        .request_line
        .starts_with("POST //org/proj/_apis/git/repositories?"));
    assert_eq!(req.header("content-type"), Some("application/json"));
    assert_eq!(req.body, r#"{"name":"new"}"#);
}

#[test]
fn error_status_is_error() {
    let (server_url, _rx) = mock_server(404, r#"{"message":"not found"}"#);
    let err = blocking_git_client(&server_url)
        .repositories_client()
        .get_repository("org", "missing", "proj")
        .send()
        .unwrap_err();
    match err.kind() {
        azure_core::error::ErrorKind::HttpResponse { status, .. } => {
            assert_eq!(*status, azure_core::http::StatusCode::NotFound)
        }
        kind => panic!("unexpected error kind: {kind:?}"),
    }
}

#[test]
fn token_credential_is_supported() {
    #[derive(Debug)]
    struct FixedToken;
    #[async_trait::async_trait]
    impl TokenCredential for FixedToken {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            Ok(AccessToken::new(
                "oauth-token",
                time::OffsetDateTime::now_utc() + time::Duration::hours(1),
            ))
        }
    }

    let (server_url, rx) = mock_server(200, REPO_LIST_JSON);
    let credential = Credential::from_token_credential(Arc::new(FixedToken));
    git::blocking::ClientBuilder::new(credential)
        .endpoint(url(&server_url))
        .build()
        .repositories_client()
        .list("org", "proj")
        .send()
        .unwrap();

    let req = rx.recv().unwrap();
    assert_eq!(req.header("authorization"), Some("Bearer oauth-token"));
}

#[test]
fn created_from_async_client() {
    let (server_url, _rx) = mock_server(200, REPO_LIST_JSON);
    let async_client = git::ClientBuilder::new(Credential::from_pat("secret"))
        .endpoint(url(&server_url))
        .build();
    let repos = git::blocking::Client::from(async_client)
        .repositories_client()
        .list("org", "proj")
        .send()
        .unwrap()
        .into_body()
        .unwrap();
    assert_eq!(repos.value.len(), 2);
}

#[test]
fn concurrent_use_from_multiple_threads() {
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let (server_url, _rx) = mock_server(200, REPO_LIST_JSON);
                blocking_git_client(&server_url)
                    .repositories_client()
                    .list("org", "proj")
                    .send()
                    .unwrap()
                    .into_body()
                    .unwrap()
                    .value
                    .len()
            })
        })
        .collect();
    for handle in handles {
        assert_eq!(handle.join().unwrap(), 2);
    }
}

/// Using the blocking API from within an async runtime returns an error, rather than panicking.
#[tokio::test]
async fn blocking_in_async_runtime_is_error() {
    let err = blocking_git_client("http://127.0.0.1:1")
        .repositories_client()
        .list("org", "proj")
        .send()
        .unwrap_err();
    assert!(err.to_string().contains("async runtime"), "{err}");
}

/// The async API is unchanged.
#[tokio::test]
async fn async_query_still_works() {
    let (server_url, rx) = mock_server(200, REPO_LIST_JSON);
    let repos = git::ClientBuilder::new(Credential::from_pat("secret"))
        .endpoint(url(&server_url))
        .build()
        .repositories_client()
        .list("org", "proj")
        .await
        .unwrap();
    assert_eq!(repos.value.len(), 2);

    let req = rx.recv().unwrap();
    assert_eq!(
        req.request_line,
        "GET //org/proj/_apis/git/repositories?api-version=7.1-preview HTTP/1.1"
    );
    assert_eq!(req.header("authorization"), Some("Basic OnNlY3JldA=="));
}
