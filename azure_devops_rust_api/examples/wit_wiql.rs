// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wit_wiql.rs
// Work Item WIQL query example.
//
// Work Item Query Language (WIQL) is a SQL-like syntax for querying work items.
// For more info see: https://docs.microsoft.com/en-us/azure/devops/boards/queries/wiql-syntax?view=azure-devops
use anyhow::Result;
use azure_devops_rust_api::wit;
use azure_devops_rust_api::wit::models::WorkItemBatchGetRequest;
use std::env;

mod utils;

// From: https://learn.microsoft.com/en-us/azure/devops/boards/queries/wiql-syntax
//
//   The WIQL syntax is used to execute the "Query By Wiql REST API".
//   Currently, there is no way to call the API to return the detailed work item
//   information from a WIQL query directly. No matter which fields you include
//   in the SELECT statement, the API only returns the work item IDs. To get the
//   full information, you need to perform two steps:
//   (1) get the ID of the work items from a WIQL, and
//   (2) get the work items via "Get a list of work items by ID and for specific fields."

// Returns a WIQL query string to get all open bugs in the given project and area path.
fn open_bugs_query(project: &str, area_path: &str) -> String {
    format!(
        r#"
        SELECT
        [System.Id]
        FROM workitems
        WHERE
        [System.TeamProject] = '{project}'
        AND [System.AreaPath] = '{area_path}'
        AND [System.WorkItemType] = 'Bug'
        AND [System.State] <> 'Resolved'
        AND [System.State] <> 'Closed'
        "#
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential
    let credential = utils::get_credential();

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let team = env::var("ADO_TEAM").expect("Must define ADO_TEAM");
    let area_path = env::var("ADO_AREA_PATH").expect("Must define ADO_AREA_PATH");

    // Create a wit client
    let wit_client = wit::ClientBuilder::new(credential).build();

    // Create a WIQL query.
    let wiql_query = wit::models::Wiql {
        query: Some(open_bugs_query(&project, &area_path)),
    };

    // Execute the query.
    // Note that WIQL queries only return work item IDs.
    // To get the full work item, you must use either `get_work_item()`
    // or `get_work_items_batch()`.
    let query_result = wit_client
        .wiql_client()
        .query_by_wiql(&organization, wiql_query, &project, &team)
        .await?;
    println!("WIQL query result:\n{query_result:#?}");

    // Extract work item IDs from the query result into a vector.
    let mut work_item_ids: Vec<i32> = query_result
        .work_items
        .iter()
        .filter_map(|wi| wi.id)
        .collect();
    println!("Work item count: {}", work_item_ids.len());

    // Note: `work_item_batch_get_request` returns a maximum of 200 work items.
    // If you ask for more than 200 work items, the request hangs!
    const MAX_WORK_ITEMS_PER_BATCH_REQUEST: usize = 200;
    if work_item_ids.len() > MAX_WORK_ITEMS_PER_BATCH_REQUEST {
        println!("Truncating work item IDs to {MAX_WORK_ITEMS_PER_BATCH_REQUEST}.");
        work_item_ids.truncate(MAX_WORK_ITEMS_PER_BATCH_REQUEST);
    }

    // Create a batch request to get the work item details.
    let work_item_batch_get_request = WorkItemBatchGetRequest {
        ids: work_item_ids,
        ..Default::default()
    };

    // Execute the batch request.
    let work_items = wit_client
        .work_items_client()
        .get_work_items_batch(&organization, work_item_batch_get_request, &project)
        .await?
        .value;
    println!("\nWork items:\n{work_items:#?}");
    println!("Work item count: {}", work_items.len());

    for work_item in &work_items {
        let work_item_id = work_item.id;

        let state = work_item
            .fields
            .get("System.State")
            .and_then(|value| value.as_str())
            .unwrap_or("");

        let title = work_item
            .fields
            .get("System.Title")
            .and_then(|value| value.as_str())
            .unwrap_or("<no title>");

        println!("[{work_item_id}] {state:10} {title}");
    }

    Ok(())
}
