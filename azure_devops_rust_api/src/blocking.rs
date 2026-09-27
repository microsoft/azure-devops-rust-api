// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Blocking (non-`async`) API support.
//!
//! Each API module provides a `blocking::Client`, which wraps the async client and runs
//! each request on a hidden tokio runtime:
//!
//! ```no_run
//! use azure_devops_rust_api::{git, Credential};
//!
//! let credential = Credential::from_pat("<pat>");
//! let git_client = git::blocking::ClientBuilder::new(credential).build();
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
//! Because requests are sent by the async client, all credential types, retry options
//! and pipeline policies are supported.
//!
//! The blocking API must not be used from within an async runtime (it returns an error
//! if it is). Use the async API instead.
//!
//! The `artifacts_download` module is async-only.

use azure_core::error::{Error, ErrorKind};
use std::future::Future;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

/// Marker type for request builders created by a blocking client, whose `send()` blocks.
#[derive(Clone, Copy, Debug, Default)]
pub struct Blocking;

/// The runtime used to run requests for all blocking clients.
///
/// A single worker thread drives I/O (e.g. connection keep-alive) between requests.
static RUNTIME: LazyLock<std::io::Result<Runtime>> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("azure_devops_rust_api-blocking")
        .enable_all()
        .build()
});

/// Runs `future` to completion on the blocking runtime.
// Unused if no API module is enabled.
#[allow(dead_code)]
pub(crate) fn block_on<T>(
    future: impl Future<Output = azure_core::Result<T>>,
) -> azure_core::Result<T> {
    if tokio::runtime::Handle::try_current().is_ok() {
        return Err(Error::with_message(
            ErrorKind::Other,
            "the blocking API cannot be used from within an async runtime; use the async API instead",
        ));
    }
    let runtime = RUNTIME.as_ref().map_err(|e| {
        Error::with_message(
            ErrorKind::Io,
            format!("failed to create blocking runtime: {e}"),
        )
    })?;
    runtime.block_on(future)
}
