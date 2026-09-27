// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Blocking (non-`async`) API support.
//!
//! Each API module provides a `blocking::ClientBuilder`, which is supplied with an
//! [`HttpClient`] to send requests. Implementations are provided for
//! [`ureq`](https://docs.rs/ureq) (`ureq` feature) and
//! [`reqwest::blocking`](https://docs.rs/reqwest) (`reqwest_blocking` feature).
//!
//! ```no_run
//! use azure_devops_rust_api::{blocking::UreqClient, git, Credential};
//!
//! let credential = Credential::from_pat("<pat>");
//! let git_client = git::blocking::ClientBuilder::new(credential, UreqClient::new()).build();
//!
//! let repos = git_client
//!     .repositories_client()
//!     .list("<organization>", "<project>")
//!     .send()?
//!     .into_body()?
//!     .value;
//! # Ok::<(), azure_core::Error>(())
//! ```
//!
//! Limitations:
//! - Only PAT and unauthenticated credentials are supported.
//! - Requests do not go through the `azure_core` pipeline, so no retry policy is applied.
//! - The `artifacts_download` module is async-only.

use azure_core::http::{RawResponse, Request};
use std::sync::LazyLock;

/// `User-Agent` header value, in the same format as `azure_core` uses.
static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    // Like `azure_core`, omit the `azure_` crate name prefix.
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_name = crate_name.strip_prefix("azure_").unwrap_or(crate_name);
    format!(
        "azsdk-rust-{}/{} (blocking; {}; {})",
        crate_name,
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    )
});

/// A blocking HTTP client that can send an `azure_core` [`Request`].
///
/// Implementations should return a [`RawResponse`] for any HTTP status code;
/// error status codes are converted into errors by the caller.
pub trait HttpClient: Send + Sync {
    /// Sends the request and returns the fully-buffered response.
    fn execute(&self, request: &Request) -> azure_core::Result<RawResponse>;
}

/// Sends a request, converting error status codes into errors.
// Unused if no module with a blocking client is enabled.
#[allow(dead_code)]
pub(crate) fn send(
    http_client: &dyn HttpClient,
    request: &mut Request,
) -> azure_core::Result<RawResponse> {
    // Requests sent via the async pipeline get a `User-Agent` header from `azure_core`,
    // so set an equivalent header here.
    request.insert_header(azure_core::http::headers::USER_AGENT, USER_AGENT.as_str());
    let response = http_client.execute(request)?;
    // `check_success` is async, but does no I/O on a fully-buffered `RawResponse`,
    // so this completes immediately. It is used so that errors match the async API.
    futures::executor::block_on(azure_core::http::check_success(response, None))
}

#[cfg(feature = "ureq")]
pub use self::ureq_client::UreqClient;

#[cfg(feature = "reqwest_blocking")]
pub use self::reqwest_client::ReqwestBlockingClient;

/// Helpers shared by the [`HttpClient`] implementations.
#[cfg(any(feature = "ureq", feature = "reqwest_blocking"))]
mod convert {
    use azure_core::error::{Error, ErrorKind};
    use azure_core::http::{headers::Headers, Body, Request};

    pub(super) fn io_error(e: impl std::error::Error + Send + Sync + 'static) -> Error {
        Error::new(ErrorKind::Io, e)
    }

    pub(super) fn request_body(request: &Request) -> azure_core::Result<Vec<u8>> {
        match request.body() {
            Body::Bytes(bytes) => Ok(bytes.to_vec()),
            Body::SeekableStream(_) => Err(Error::with_message(
                ErrorKind::Other,
                "streaming request bodies are not supported by blocking clients",
            )),
        }
    }

    /// Converts response headers, skipping any whose value is not valid UTF-8.
    pub(super) fn response_headers<'a>(
        headers: impl Iterator<Item = (&'a str, &'a [u8])>,
    ) -> Headers {
        let mut result = Headers::new();
        for (name, value) in headers {
            if let Ok(value) = std::str::from_utf8(value) {
                result.insert(name.to_owned(), value.to_owned());
            }
        }
        result
    }
}

#[cfg(feature = "ureq")]
mod ureq_client {
    use super::convert::{io_error, request_body, response_headers};
    use super::HttpClient;
    use azure_core::http::{RawResponse, Request, StatusCode};

    /// [`HttpClient`] implementation using [`ureq`].
    #[derive(Clone)]
    pub struct UreqClient {
        agent: ureq::Agent,
    }

    impl UreqClient {
        /// Creates a new client with default configuration.
        pub fn new() -> Self {
            let agent = ureq::Agent::config_builder()
                .http_status_as_error(false)
                .build()
                .into();
            Self { agent }
        }
    }

    impl Default for UreqClient {
        fn default() -> Self {
            Self::new()
        }
    }

    impl From<ureq::Agent> for UreqClient {
        /// Wraps an existing agent. The agent must be configured with
        /// `http_status_as_error(false)`.
        fn from(agent: ureq::Agent) -> Self {
            Self { agent }
        }
    }

    impl HttpClient for UreqClient {
        fn execute(&self, request: &Request) -> azure_core::Result<RawResponse> {
            let mut builder = ureq::http::Request::builder()
                .method(request.method().as_ref())
                .uri(request.url().as_str());
            for (name, value) in request.headers().iter() {
                builder = builder.header(name.as_str(), value.as_str());
            }
            let http_request = builder.body(request_body(request)?).map_err(io_error)?;

            let mut response = self.agent.run(http_request).map_err(io_error)?;

            let status = StatusCode::from(response.status().as_u16());
            let headers = response_headers(
                response
                    .headers()
                    .iter()
                    .map(|(name, value)| (name.as_str(), value.as_bytes())),
            );
            let body = response
                .body_mut()
                .with_config()
                .limit(u64::MAX)
                .read_to_vec()
                .map_err(io_error)?;
            Ok(RawResponse::from_bytes(status, headers, body))
        }
    }
}

#[cfg(feature = "reqwest_blocking")]
mod reqwest_client {
    use super::convert::{io_error, request_body, response_headers};
    use super::HttpClient;
    use azure_core::http::{RawResponse, Request, StatusCode};

    /// [`HttpClient`] implementation using [`reqwest::blocking`].
    ///
    /// Note that `reqwest::blocking` must not be used from within an async runtime.
    #[derive(Clone)]
    pub struct ReqwestBlockingClient {
        client: reqwest::blocking::Client,
    }

    impl ReqwestBlockingClient {
        /// Creates a new client with default configuration.
        pub fn new() -> Self {
            Self {
                client: reqwest::blocking::Client::new(),
            }
        }
    }

    impl Default for ReqwestBlockingClient {
        fn default() -> Self {
            Self::new()
        }
    }

    impl From<reqwest::blocking::Client> for ReqwestBlockingClient {
        /// Wraps an existing client.
        fn from(client: reqwest::blocking::Client) -> Self {
            Self { client }
        }
    }

    impl HttpClient for ReqwestBlockingClient {
        fn execute(&self, request: &Request) -> azure_core::Result<RawResponse> {
            let method = reqwest::Method::from_bytes(request.method().as_ref().as_bytes())
                .map_err(io_error)?;
            let mut builder = self.client.request(method, request.url().as_str());
            for (name, value) in request.headers().iter() {
                builder = builder.header(name.as_str(), value.as_str());
            }

            let response = builder
                .body(request_body(request)?)
                .send()
                .map_err(io_error)?;

            let status = StatusCode::from(response.status().as_u16());
            let headers = response_headers(
                response
                    .headers()
                    .iter()
                    .map(|(name, value)| (name.as_str(), value.as_bytes())),
            );
            let body = response.bytes().map_err(io_error)?;
            Ok(RawResponse::from_bytes(status, headers, body))
        }
    }
}
