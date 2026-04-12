// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// release_get_release.rs
// Get a specific release example
use anyhow::Result;
use azure_devops_rust_api::release;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

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
        .await?;
    println!("{release:#?}");

    // Get manual interventions on a release
    println!("\nManual interventions:");
    let manual_interventions = release_client
        .manual_interventions_client()
        .list(&organization, &project, release_id)
        .await?
        .value;
    println!("{manual_interventions:#?}");

    Ok(())
}
