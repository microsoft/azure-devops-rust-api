// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// search_work_item.rs
// Search work item example.
use anyhow::Result;
use azure_devops_rust_api::search;
use azure_devops_rust_api::search::models::{
    EntitySearchRequest, EntitySearchRequestBase, WorkItemSearchRequest,
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
    let search_work_item_text = env::var("WORK_ITEM_SEARCH_TEXT")
        .expect("Must define a text <PKG_NAME> to be searched for work item");

    // Create a search client
    println!("Create search client");
    let search_client = search::ClientBuilder::new(credential).build();

    let entity_search_request_base = EntitySearchRequestBase {
        filters: None,
        search_text: Some(search_work_item_text.to_string()),
    };

    let entity_search_request = EntitySearchRequest {
        entity_search_request_base,
        top: Some(10), // Must specify `top`, otherwise search fails
        ..Default::default()
    };

    // define body for the request
    let work_item_search_request = WorkItemSearchRequest {
        entity_search_request,
    };

    // Do the search
    println!("Search...");
    let search_results = search_client
        .work_item_search_results_client()
        .fetch_work_item_search_results(organization, work_item_search_request, project)
        .await?
        .results;

    println!("Found {} results", search_results.len());

    if let Some(result) = search_results.first() {
        println!("Example search work item result:\n{result:#?}");
    }

    Ok(())
}
