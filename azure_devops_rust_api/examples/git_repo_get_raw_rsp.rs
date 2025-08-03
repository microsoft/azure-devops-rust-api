// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_get_raw_rsp.rs
// Repository get example, demonstrating how to obtain the raw response
// which enables inspection of the response status, headers and body.
use anyhow::Result;
use azure_devops_rust_api::git;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repo_name = env::args()
        .nth(1)
        .expect("Usage: git_repo_get_raw <repository-name>");

    // Create a "git" client
    let git_client = git::ClientBuilder::new(credential).build();

    // Invoke the operation via `send()` rather than `into_future()` to get a raw response
    let rsp = git_client
        .repositories_client()
        .get_repository(&organization, &repo_name, &project)
        .send()
        .await?
        .into_raw_response();

    // Display the raw response details
    println!("status: {:#?}", rsp.status());
    println!("headers:\n{:#?}", rsp.headers());
    println!("body:\n{:#?}", rsp.into_body().collect_string().await?);

    Ok(())
}
