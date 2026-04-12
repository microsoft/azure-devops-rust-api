// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// build_source_provider_list.rs
// Source provider list example.
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

    // Create a build client
    println!("Create build client");
    let build_client = build::ClientBuilder::new(credential).build();

    // Get all source providers in the specified organization/project
    println!("Get source providers list");
    let source_providers = build_client
        .source_providers_client()
        .list(organization, project)
        .await?
        .value;

    println!("Found {} source_providers", source_providers.len());
    println!("\nSource Providers:\n {source_providers:#?}");

    Ok(())
}
