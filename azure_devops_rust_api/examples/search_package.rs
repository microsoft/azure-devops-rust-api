// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// search_package.rs
// Search package example.
use anyhow::Result;
use azure_devops_rust_api::search;
use azure_devops_rust_api::search::models::{
    EntitySearchRequest, EntitySearchRequestBase, PackageSearchRequest,
};
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let search_package_name =
        env::var("PKG_NAME").expect("Must define a package name <PKG_NAME> to be searched");

    // Create a search client
    println!("Create search client");
    let search_client = search::ClientBuilder::new(credential).build();

    let entity_search_request_base = EntitySearchRequestBase {
        filters: None,
        search_text: Some(search_package_name.to_string()),
    };

    let entity_search_request = EntitySearchRequest {
        entity_search_request_base,
        top: Some(10), // Must specify `top`, otherwise search fails
        ..Default::default()
    };

    let package_search_request = PackageSearchRequest {
        entity_search_request,
    };

    // Do the search
    println!("Search...");
    let search_results = search_client
        .package_search_results_client()
        .fetch_package_search_results(organization, package_search_request)
        .await?
        .results;

    println!("Found {} results", search_results.len());
    if let Some(result) = search_results.first() {
        println!("Example search result:\n{result:#?}");
    }

    Ok(())
}
