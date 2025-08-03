// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_policy_config_list.rs
// Git policy configuration list example.
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
    let repo_name = env::args()
        .nth(1)
        .expect("Usage: git_items_list <repository-name>");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get the specified repo by name, as we need to obtain the repo ID.
    let repo = git_client
        .repositories_client()
        .get_repository(&organization, &repo_name, &project)
        .await?;

    // Get all policies configured on the specified repository.
    // Many git repository APIs take either a repository ID or a repository name,
    // but this particular API only accepts a repository ID.
    let policies = git_client
        .policy_configurations_client()
        .get(organization, project)
        .repository_id(repo.id)
        .await?
        .value;

    println!("Policies:\n{policies:#?}");

    Ok(())
}
