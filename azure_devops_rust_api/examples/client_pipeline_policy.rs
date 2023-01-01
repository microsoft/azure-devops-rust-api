// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// client_pipeline_policy.rs
// Client pipeline policy middleware example.
// See:
// - https://docs.rs/azure_core/latest/azure_core/struct.Pipeline.html
// - https://docs.rs/azure_core/latest/azure_core/trait.Policy.html

use anyhow::Result;
use azure_devops_rust_api::git;
use std::env;
use std::sync::Arc;

use async_trait::async_trait;
use azure_core::{Context, Policy, PolicyResult, Request};
use env_logger::Env;
use log::info;

mod utils;

/// Basic request logger policy
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
        info!("> Request:\n{:#?}", request);
        let now = time::Instant::now();
        // Call the next policy in the chain, and await the response
        let rsp = next[0].send(ctx, request, &next[1..]).await;
        let elapsed_time = now.elapsed();
        info!("Request took {} secs", elapsed_time.as_seconds_f32());
        info!("< Response:\n{:#?}", rsp);
        rsp
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging - set default log level to info
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Get authentication credential
    let credential = utils::get_credential();

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let request_logger_policy = Arc::new(RequestLogger::new()) as Arc<dyn Policy>;

    // Create a git client with our custom policy
    let git_client = git::ClientBuilder::new(credential)
        .per_call_policies(vec![request_logger_policy])
        .build();

    // Get all repositories in the specified organization/project
    let _repos = git_client
        .repositories_client()
        .list(organization, project)
        .into_future()
        .await?
        .value;

    Ok(())
}
