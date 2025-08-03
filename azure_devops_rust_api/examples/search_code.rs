// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// search_code.rs
// Search code example.
use anyhow::Result;
use azure_devops_rust_api::search;
use azure_devops_rust_api::search::models::{
    CodeSearchRequest, EntitySearchRequest, EntitySearchRequestBase,
};
use serde_json::json;
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
        .expect("Usage: search_code <repository-name> <branch> <search-text>");

    let branch_name = env::args()
        .nth(2)
        .expect("Usage: search_code <repository-name> <branch> <search-text>");

    let search_text = env::args()
        .nth(3)
        .expect("Usage: search_code <repository-name> <branch> <search-text>");

    // Create a search client
    println!("Create search client");
    let search_client = search::ClientBuilder::new(credential).build();

    // Create a search request to search within a specific repository and branch.
    // You could make the same query without a filter which would search the entire project.
    //
    // Unfortunately the filters field format is not currently documented.
    //
    // There is an example request in the REST API documentation which shows a selection of fields
    // that can be used in the filter field:
    //   https://learn.microsoft.com/en-us/rest/api/azure/devops/search/code-search-results/fetch-code-search-results?view=azure-devops-rest-7.1&tabs=HTTP
    //
    // In testing I found that if you specify a filter, it must include the `Project` field.
    let entity_search_request_base = EntitySearchRequestBase {
        filters: Some(json!({
            "Project": [
                project
            ],
            "Repository": [
                repository_name
            ],
            "Branch": [
                branch_name
            ]
        })),
        search_text: Some(search_text),
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
        println!("Example search result:\n{result:#?}");
    }

    Ok(())
}
