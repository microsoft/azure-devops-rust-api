// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// Tests for the blocking client, using a local mock HTTP server.
// Each blocking test is run with both the ureq and reqwest HTTP clients.
#![cfg(all(feature = "git", feature = "ureq", feature = "reqwest_blocking"))]

use azure_devops_rust_api::blocking::{HttpClient, ReqwestBlockingClient, UreqClient};
use azure_devops_rust_api::{git, Credential};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;

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

fn git_client(url: &str) -> git::Client {
    git::ClientBuilder::new(Credential::from_pat("secret"))
        .endpoint(azure_core::http::Url::parse(url).unwrap())
        .build()
}

fn blocking_git_client(url: &str, http_client: impl HttpClient + 'static) -> git::blocking::Client {
    git::blocking::ClientBuilder::new(Credential::from_pat("secret"), http_client)
        .endpoint(azure_core::http::Url::parse(url).unwrap())
        .build()
}

fn query_returns_parsed_body(http_client: impl HttpClient + 'static) {
    let (url, rx) = mock_server(200, REPO_LIST_JSON);
    let repos = blocking_git_client(&url, http_client)
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
    assert!(req
        .header("user-agent")
        .unwrap()
        .starts_with("azsdk-rust-azure_devops_rust_api/"));
}

fn post_sends_json_body(http_client: impl HttpClient + 'static) {
    let (url, rx) = mock_server(
        201,
        r#"{"id":"r3","name":"new","project":{"name":"proj","visibility":"private"},"url":"http://x/r3"}"#,
    );
    let options = git::models::GitRepositoryCreateOptions {
        name: Some("new".to_string()),
        ..Default::default()
    };
    let repo = blocking_git_client(&url, http_client)
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

fn error_status_is_error(http_client: impl HttpClient + 'static) {
    let (url, _rx) = mock_server(404, r#"{"message":"not found"}"#);
    let err = blocking_git_client(&url, http_client)
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

fn token_credential_is_rejected(http_client: impl HttpClient + 'static) {
    #[derive(Debug)]
    struct NoToken;
    #[async_trait::async_trait]
    impl azure_core::credentials::TokenCredential for NoToken {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<azure_core::credentials::TokenRequestOptions<'_>>,
        ) -> azure_core::Result<azure_core::credentials::AccessToken> {
            unreachable!()
        }
    }
    let credential = Credential::from_token_credential(std::sync::Arc::new(NoToken));
    let err = git::blocking::ClientBuilder::new(credential, http_client)
        .build()
        .repositories_client()
        .list("org", "proj")
        .send()
        .unwrap_err();
    assert!(matches!(
        err.kind(),
        azure_core::error::ErrorKind::Credential
    ));
}

/// Generates a test per HTTP client for each of the given test functions.
macro_rules! blocking_tests {
    ($($test:ident),*) => {
        mod ureq {
            $(#[test] fn $test() { super::$test(super::UreqClient::new()) })*
        }
        mod reqwest {
            $(#[test] fn $test() { super::$test(super::ReqwestBlockingClient::new()) })*
        }
    };
}

blocking_tests!(
    query_returns_parsed_body,
    post_sends_json_body,
    error_status_is_error,
    token_credential_is_rejected
);

#[test]
fn build_request_without_sending() {
    let request = git_client("https://example.com")
        .repositories_client()
        .list("org", "proj")
        .include_hidden(true)
        .build_request()
        .unwrap();
    assert_eq!(request.method(), azure_core::http::Method::Get);
    assert_eq!(
        request.url().as_str(),
        "https://example.com//org/proj/_apis/git/repositories?api-version=7.1-preview&includeHidden=true"
    );
    assert!(request
        .headers()
        .get_optional_str(&azure_core::http::headers::AUTHORIZATION)
        .is_none());
}

/// The async API is unchanged by the `build_request()` refactor.
#[tokio::test]
async fn async_query_still_works() {
    let (url, rx) = mock_server(200, REPO_LIST_JSON);
    let repos = git_client(&url)
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
