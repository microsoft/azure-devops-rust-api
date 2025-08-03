// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// search_code.rs
// Search code example.
use anyhow::Result;
use azure_devops_rust_api::search;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let repository_name = env::args()
        .nth(1)
        .expect("Usage: Repo name to be searched <repository-name>");

    // Create a search client
    println!("Create search client");
    let search_client = search::ClientBuilder::new(credential).build();

    // Do the search
    println!("Search...");
    let search_results = search_client
        .repositories_client()
        .get(organization, project, repository_name)
        .await?;

    println!("{search_results:#?}");
    Ok(())
}
