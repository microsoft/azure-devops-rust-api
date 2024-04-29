// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_list.rs
// Repository list example.
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
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get all repositories in the specified organization/project
    let repos = git_client
        .repositories_client()
        .list(organization, project)
        .await?
        .value;

    for repo in repos.iter() {
        println!("{}", repo.name);
    }
    println!("{} repos found", repos.len());

    Ok(())
}
