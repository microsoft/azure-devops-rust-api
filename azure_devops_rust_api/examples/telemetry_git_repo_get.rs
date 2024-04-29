// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// telemetry_git_repo_get.rs
// Demonstrates how to use a custom policy in the client pipeline.
// This example uses the telemetry request logger policy to trace request and response details.
// This can be helpful for debugging and understanding the behavior of the client.
//
// See:
// - https://docs.rs/azure_core/latest/azure_core/struct.Pipeline.html
// - https://docs.rs/azure_core/latest/azure_core/trait.Policy.html

use anyhow::Result;
use azure_devops_rust_api::{git, telemetry};
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repo_name = env::args()
        .nth(1)
        .expect("Usage: telemetry_git_repo_get <repository-name>");

    // Create a git client with the telemetry request logger policy
    let git_client = git::ClientBuilder::new(credential)
        .per_call_policies(vec![telemetry::request_logger_policy()])
        .build();

    // Get specified repo
    let _repo = git_client
        .repositories_client()
        .get_repository(&organization, &repo_name, &project)
        .await?;
    println!("Finished");

    Ok(())
}
