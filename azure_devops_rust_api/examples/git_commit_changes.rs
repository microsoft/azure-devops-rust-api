// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// get_commits.rs
// Getting all the commits from a PR example.
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
        .expect("Usage: git_commit_changes <repository-name> <commit_id>");
    let commit_id = env::args()
        .nth(2)
        .expect("Usage: git_commit_changes <repository-name> <commit_id>");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    let commit_changes = git_client
        .commits_client()
        .get_changes(&organization, &commit_id, &repository_name, &project)
        .await?;
    println!("Commit changes:\n{commit_changes:#?}");

    Ok(())
}
