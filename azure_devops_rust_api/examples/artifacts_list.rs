// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// artifacts_list.rs
// Artifacts list example.
use anyhow::Result;
use azure_devops_rust_api::artifacts;
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

    // Create an artifacts client
    println!("Create artifacts client");
    let artifacts_client = artifacts::ClientBuilder::new(credential).build();

    // Query all the artifact feeds
    let feeds = artifacts_client
        .feed_management_client()
        .get_feeds(&organization, &project)
        .into_future()
        .await?
        .value;

    println!("Found {} feeds", feeds.len());
    for feed in &feeds {
        let id = feed.feed_core.id.as_deref().unwrap_or("");
        let name = feed.feed_core.name.as_deref().unwrap_or("");
        let url = feed.url.as_deref().unwrap_or("");
        println!("{:40}{:30}{}", id, name, url);
    }

    if let Some(feed) = feeds.iter().next() {
        println!("\nExample feed struct:\n{:#?}", feed);

        if let Some(feed_id) = &feed.feed_core.id {
            println!("\nFeed packages:");
            let packages = artifacts_client
                .artifact_details_client()
                .get_packages(&organization, feed_id, &project)
                .into_future()
                .await?
                .value;

            for package in &packages {
                let id = package.id.as_deref().unwrap_or("");
                let name = package.name.as_deref().unwrap_or("");
                let url = package.url.as_deref().unwrap_or("");
                println!("{:40}{:30}{}", id, name, url);
            }
        }
    }

    Ok(())
}
