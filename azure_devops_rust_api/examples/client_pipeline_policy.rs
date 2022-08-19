// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// client_pipeline_policy.rs
// Client pipeline policy middleware example.
// See:
// - https://docs.rs/azure_core/latest/azure_core/struct.Pipeline.html
// - https://docs.rs/azure_core/latest/azure_core/trait.Policy.html

use anyhow::Result;
use azure_devops_rust_api::git;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

use async_trait::async_trait;
use azure_core::{Context, Policy, PolicyResult, Request};
use log::info;

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
    // Initialize logging
    env_logger::init();

    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli.
    let credential = match env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential {}))
        }
    };

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let request_logger_policy = Arc::new(RequestLogger::new()) as Arc<dyn Policy>;

    // Create a git client with our custom policy
    let git_client =
        git::ClientBuilder::new(credential).per_call_policies(vec![request_logger_policy]).build();

    // Get all repositories in the specified organization/project
    let _repos = git_client
        .repositories_client()
        .list(organization, project)
        .into_future()
        .await?
        .value;

    Ok(())
}
