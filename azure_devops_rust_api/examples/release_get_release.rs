// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// release_get_release.rs
// Get a specific release example
use anyhow::Result;
use azure_devops_rust_api::release;
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

    // Create release client
    let release_client = release::ClientBuilder::new(credential).build();

    // Get the release ID from user
    let release_id: i32 = env::args()
        .nth(1)
        .expect("Usage: release_get_specific_release <release_id>")
        .parse()
        .unwrap();

    // Query a specific release
    println!("\nRelease:");
    let release = release_client
        .releases_client()
        .get_release(&organization, &project, release_id)
        .into_future()
        .await?;
    println!("{:#?}", release);

    Ok(())
}
