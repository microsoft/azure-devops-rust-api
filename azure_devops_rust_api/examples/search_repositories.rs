// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// search_code.rs
// Search code example.
use anyhow::Result;
use azure_devops_rust_api::search;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli
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

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let repository_name = env::args()
        .nth(1)
        .expect("Usage: Repo name to be searched <repository-name>");

    // Create a search client
    println!("Create search client");
    let search_client = search::ClientBuilder::new(credential).build();

    // Do the search
    println!("Search...");
    let search_results = search_client
        .repositories_client()
        .get(organization, project, repository_name)
        .into_future()
        .await?;

    println!("{:#?}", search_results);
    Ok(())
}
