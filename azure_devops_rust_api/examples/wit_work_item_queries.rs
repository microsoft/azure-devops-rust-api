// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wit_work_item_queries.rs
// Work Item query list example.
use anyhow::Result;
use azure_devops_rust_api::wit;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a wit client
    let wit_client = wit::ClientBuilder::new(credential).build();

    // Get all work item queries
    let work_item_queries = wit_client
        .queries_client()
        .list(&organization, &project)
        .await?;

    println!("All work item queries:\n{work_item_queries:#?}");

    Ok(())
}
