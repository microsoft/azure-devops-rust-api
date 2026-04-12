// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_pr_work_items.rs
// Example: Getting work items(s) associated with a PR
use anyhow::Result;
use azure_devops_rust_api::git;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_pr_work_items <repository-name> <pull_request_id>");
    let pull_request_id: i32 = env::args()
        .nth(2)
        .expect("Usage: git_pr_work_items <repository-name> <pull_request_id>")
        .parse()
        .unwrap();
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a "git" client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get work item(s) associated with PR
    let pr_work_items = git_client
        .pull_request_work_items_client()
        .list(&organization, &repository_name, pull_request_id, &project)
        .await?;

    println!("PR work items:");
    println!("{pr_work_items:#?}");

    Ok(())
}
