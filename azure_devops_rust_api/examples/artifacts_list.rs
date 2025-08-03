// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// artifacts_list.rs
// Artifacts list example.
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

    println!("Found {} feeds", feeds.len());
    for feed in &feeds {
        let id = feed.feed_core.id.as_deref().unwrap_or("");
        let name = feed.feed_core.name.as_deref().unwrap_or("");
        let url = feed.url.as_deref().unwrap_or("");
        println!("{id:40}{name:30}{url}");
    }

    if let Some(feed) = feeds.first() {
        println!("\nExample feed struct:\n{feed:#?}");

        if let Some(feed_id) = &feed.feed_core.id {
            println!("\nFeed packages:");
            let packages = artifacts_client
                .artifact_details_client()
                .get_packages(&organization, feed_id, &project)
                .await?
                .value;

            for package in &packages {
                let id = package.id.as_deref().unwrap_or("");
                let name = package.name.as_deref().unwrap_or("");
                let url = package.url.as_deref().unwrap_or("");
                println!("{id:40}{name:30}{url}");
            }
        }
    }

    Ok(())
}
