// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_pr_work_items.rs
// Example: Getting work items(s) associated with a PR
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
        .into_future()
        .await?;
    println!("PR work items:");
    println!("{:#?}", pr_work_items);

    Ok(())
}
