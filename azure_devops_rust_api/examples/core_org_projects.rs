// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// core_org_projects.rs
// Projects from Orgs example.
use anyhow::Result;
use azure_devops_rust_api::core;
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

    // Max number of projects to be fetched, default max is 100
    let top_projects: i32 = 500;

    // Create core client
    let core_client = core::ClientBuilder::new(credential).build();

    let org_projects = core_client
        .projects_client()
        .list(&organization)
        .top(top_projects)
        .into_future()
        .await?;

    println!("{:#?}", org_projects);

    Ok(())
}
