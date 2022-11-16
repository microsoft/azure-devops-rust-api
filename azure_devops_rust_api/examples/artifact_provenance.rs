// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// artifact_provenance.rs
// Artifact provenance example, demonstrating how to obtain json
// information about the package's origin; such as identity of publisher
// and, if available, package code repository information.
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

    if let Some(feed) = feeds.iter().next() {
        if let Some(feed_id) = &feed.feed_core.id {
            let packages = artifacts_client
                .artifact_details_client()
                .get_packages(&organization, feed_id, &project)
                .into_future()
                .await?
                .value;

            if let Some(package) = packages.iter().next() {
                let name = package.name.as_deref().unwrap_or("");
                let id = package.id.as_deref().unwrap_or("");
                let version_id = package
                    .versions
                    .first()
                    .expect("No package version information available")
                    .id
                    .as_deref()
                    .unwrap_or("");
                println!("{:30}{:40}{:40}", name, id, version_id);

                let provenance = artifacts_client
                    .artifact_details_client()
                    .get_package_version_provenance(
                        &organization,
                        feed_id,
                        id,
                        version_id,
                        &project,
                    )
                    .into_future()
                    .await?
                    .provenance;

                println!("{:#?}", provenance);
            }
        }
    }

    Ok(())
}
