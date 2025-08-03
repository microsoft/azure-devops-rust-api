// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// artifact_provenance.rs
// Artifact provenance example, demonstrating how to obtain json
// information about the package's origin; such as identity of publisher
// and, if available, package code repository information.
use anyhow::Result;
use azure_devops_rust_api::artifacts;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

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
        .await?
        .value;

    if let Some(feed) = feeds.first() {
        if let Some(feed_id) = &feed.feed_core.id {
            let packages = artifacts_client
                .artifact_details_client()
                .get_packages(&organization, feed_id, &project)
                .await?
                .value;

            if let Some(package) = packages.first() {
                let name = package.name.as_deref().unwrap_or("");
                let id = package.id.as_deref().unwrap_or("");
                let version_id = package
                    .versions
                    .first()
                    .expect("No package version information available")
                    .id
                    .as_deref()
                    .unwrap_or("");
                println!("{name:30}{id:40}{version_id:40}");

                let provenance = artifacts_client
                    .artifact_details_client()
                    .get_package_version_provenance(
                        &organization,
                        feed_id,
                        id,
                        version_id,
                        &project,
                    )
                    .await?
                    .provenance;

                println!("{provenance:#?}");
            }
        }
    }

    Ok(())
}
