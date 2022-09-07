// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// get_commits.rs
// Getting all the commits from a PR example.
use anyhow::Result;
use azure_devops_rust_api::git;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli.
    let credential = match env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("INFO:Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("INFO:Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential {}))
        }
    };

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_pr_commits <repository-name> <commit_id>");
    let commit_id = env::args()
        .nth(2)
        .expect("Usage: git_pr_commits <repository-name> <commit_id>");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    let commit_changes = git_client
        .commits_client()
        .get_changes(&organization, &commit_id, &repository_name, &project)
        .into_future()
        .await?;
    println!("commit_changes:\n{:#?}", commit_changes);

    Ok(())
}
