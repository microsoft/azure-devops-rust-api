// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Azure DevOps telemetry support.
use async_trait::async_trait;
use azure_core::{Context, Policy, PolicyResult, Request, Response};
use std::sync::Arc;
use tracing::{error, info};

/// Basic request logger policy.
///
/// This policy logs the request and response at `info` level using the `tracing` crate.
/// If the request fails, it logs the error at `error` level.
///
/// To use the policy, add it to the client builder:
/// ```rust
///     let git_client = git::ClientBuilder::new(credential)
///            .per_call_policies(vec![telemetry::request_logger_policy()])
///            .build();
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RequestLogger {}

impl RequestLogger {
    fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl azure_core::Policy for RequestLogger {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        info!(
            method = %request.method(),
            url = %request.url(),
            headers = ?request.headers(),
            body = ?request.body(),
            "Request"
        );
        let now = time::Instant::now();
        // Call the next policy in the chain, and await the response
        let rsp = next[0].send(ctx, request, &next[1..]).await;

        // Log the response
        let elapsed_time = now.elapsed().as_seconds_f32();
        match rsp {
            Ok(rsp) => {
                // To get the body content, we need to consume the response stream.
                // We then have to subsequently reconstruct the response stream
                // to pass back to the caller.
                let (status_code, headers, response_body) = rsp.deconstruct();
                let response_body = response_body.collect().await;
                info!(
                    status_code = %status_code,
                    headers = ?headers,
                    response_body = ?response_body,
                    elapsed_time = %elapsed_time,
                    "Response"
                );
                let response_stream =
                    Box::pin(futures::stream::once(futures::future::ready(response_body)));
                Ok(Response::new(status_code, headers, response_stream))
            }
            Err(err) => {
                error!(
                    err = ?err,
                    "Request failed"
                );
                Err(err)
            }
        }
    }
}

/// Create a new instance of the `RequestLogger` policy.
pub fn request_logger_policy() -> Arc<dyn Policy> {
    Arc::new(RequestLogger::new())
}
