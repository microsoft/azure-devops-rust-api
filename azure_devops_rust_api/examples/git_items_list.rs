// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_items_list.rs
// Git items (files and folders) list example.
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
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential::new()))
        }
    };

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_items_list <repository-name>");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get all items (files and folders) in the specified repository
    let items = git_client
        .items_client()
        .list(organization, repository_name, project)
        .recursion_level("Full")
        .into_future()
        .await?
        .value;

    for item in items.iter() {
        if let Some(path) = &item.item_model.path {
            println!("{path}");
        }
    }
    println!("{} items found", items.len());

    Ok(())
}
