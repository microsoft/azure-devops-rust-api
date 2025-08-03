// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// build_get.rs
// Build get example.
use anyhow::Result;
use azure_devops_rust_api::build;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let build_id: i32 = env::args()
        .nth(1)
        .expect("Usage: build_get <build-id>")
        .parse()?;

    // Create a build client
    println!("Create build client");
    let build_client = build::ClientBuilder::new(credential).build();

    // Get specified build
    println!("Get build {build_id}");
    let build = build_client
        .builds_client()
        .get(organization, project, build_id)
        .await?;

    println!("Build:\n{build:#?}");

    Ok(())
}
