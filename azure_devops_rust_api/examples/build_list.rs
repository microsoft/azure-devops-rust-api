// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// build_list.rs
// Repository list example.
use anyhow::Result;
use azure_devops_rust_api::build;
use std::env;
use time::{ext::NumericalDuration, OffsetDateTime};

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a build client
    println!("Create build client");
    let build_client = build::ClientBuilder::new(credential).build();

    // Query all the builds in the past hour
    let end_time = OffsetDateTime::now_utc();
    let start_time = end_time - 1.hours();

    // Get all builds in the specified organization/project in the past hour
    println!("Get list");
    let builds = build_client
        .builds_client()
        .list(organization, project)
        .min_time(start_time)
        .max_time(end_time)
        .await?
        .value;

    println!("Found {} builds", builds.len());
    if let Some(build) = builds.first() {
        println!("Example build struct: {build:#?}");
    }

    Ok(())
}
