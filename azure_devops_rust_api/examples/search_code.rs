// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// search_code.rs
// Search code example.
use anyhow::Result;
use azure_devops_rust_api::search;
use azure_devops_rust_api::search::models::{
    CodeSearchRequest, EntitySearchRequest, EntitySearchRequestBase,
};
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a search client
    println!("Create search client");
    let search_client = search::ClientBuilder::new(credential).build();

    let entity_search_request_base = EntitySearchRequestBase {
        filters: None,
        search_text: Some("file:Cargo.toml futures".to_string()),
    };

    let entity_search_request = EntitySearchRequest {
        entity_search_request_base,
        top: Some(10), // Must specify `top`, otherwise search fails
        ..Default::default()
    };

    let code_search_request = CodeSearchRequest {
        entity_search_request,
        include_snippet: Some(true),
    };

    // Do the search
    println!("Search...");
    let search_results = search_client
        .code_search_results_client()
        .fetch_code_search_results(organization, code_search_request, project)
        .await?
        .results;

    println!("Found {} results", search_results.len());
    if let Some(result) = search_results.first() {
        println!("Example search result:\n{:#?}", result);
    }

    Ok(())
}
