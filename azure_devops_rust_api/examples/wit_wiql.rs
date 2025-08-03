// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wit_wiql.rs
// Work Item WIQL query example.
//
// Work Item Query Language (WIQL) is a SQL-like syntax for querying work items.
// For more info see: https://docs.microsoft.com/en-us/azure/devops/boards/queries/wiql-syntax?view=azure-devops
use anyhow::Result;
use azure_devops_rust_api::wit;
use azure_devops_rust_api::wit::models::{Wiql, WorkItem, WorkItemBatchGetRequest, WorkItemList};
use futures::TryStreamExt;
use std::env;
use std::future::IntoFuture;

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

// Returns a WIQL query string to get all active workitems in the given project and area path.
fn active_workitems_query(project: &str, area_path: &str) -> String {
    format!(
        r#"
        SELECT
        [System.Id]
        FROM workitems
        WHERE
        [System.TeamProject] = '{project}'
        AND [System.AreaPath] = '{area_path}'
        AND [System.State] = 'Active'
        "#
    )
}

// Returns the ids of the work items matching the given WIQL query.
async fn query_workitem_ids(
    wit_client: &wit::Client,
    organization: &str,
    project: &str,
    team: &str,
    query: &str,
) -> Result<Vec<i32>> {
    // Create a WIQL query.
    let wiql_query = Wiql {
        query: Some(query.to_string()),
    };

    // Execute the query.
    // Note that WIQL queries only return work item IDs.
    // To get the full work item, you must use either `get_work_item()`
    // or `get_work_items_batch()`.
    let query_result = wit_client
        .wiql_client()
        .query_by_wiql(organization, wiql_query, project, team)
        .await?;
    println!("WIQL query result:\n{query_result:#?}");

    // Extract work item IDs from the query result into a vector.
    let workitem_ids = query_result
        .work_items
        .iter()
        .filter_map(|wi| wi.id)
        .collect();

    Ok(workitem_ids)
}

// Returns the work items matching the given work item IDs.
async fn query_workitems(
    wit_client: &wit::Client,
    organization: &str,
    project: &str,
    workitem_ids: &[i32],
) -> Result<Vec<WorkItem>> {
    // The maximum number of work items that can be requested in a single batch is 200.
    // https://learn.microsoft.com/en-us/rest/api/azure/devops/wit/work-items/get-work-items-batch?view=azure-devops-rest-7.1&tabs=HTTP
    const MAX_WORK_ITEMS_PER_BATCH_REQUEST: usize = 200;

    // Split the work item IDs into batches of `MAX_WORK_ITEMS_PER_BATCH_REQUEST`.
    let workitem_batch_get_requests: Vec<WorkItemBatchGetRequest> = workitem_ids
        .chunks(MAX_WORK_ITEMS_PER_BATCH_REQUEST)
        .map(|chunk| WorkItemBatchGetRequest {
            ids: chunk.to_vec(),
            ..Default::default()
        })
        .collect();

    println!("workitem_batch_get_requests: {workitem_batch_get_requests:#?}");

    // Get work item batches in parallel and collect into a vector
    let workitem_batches: Vec<WorkItemList> = workitem_batch_get_requests
        .into_iter()
        .map(|work_item_batch_get_request| {
            wit_client
                .work_items_client()
                .get_work_items_batch(organization, work_item_batch_get_request, project)
                .into_future()
        })
        .collect::<futures::stream::FuturesOrdered<_>>()
        .try_collect::<Vec<_>>()
        .await?;

    // Flatten the vector of work item batches into a vector of work items
    let workitems: Vec<WorkItem> = workitem_batches
        .into_iter()
        .flat_map(|workitems_batch| workitems_batch.value)
        .collect();

    Ok(workitems)
}

// Displays the given work items.
fn display_workitems(workitems: &[WorkItem]) {
    for workitem in workitems {
        let workitem_id = workitem.id;

        let state = workitem
            .fields
            .get("System.State")
            .and_then(|value| value.as_str())
            .unwrap_or("");

        let title = workitem
            .fields
            .get("System.Title")
            .and_then(|value| value.as_str())
            .unwrap_or("<no title>");

        let workitem_type = workitem
            .fields
            .get("System.WorkItemType")
            .and_then(|value| value.as_str())
            .unwrap_or("");

        let assigned_to = workitem
            .fields
            .get("System.AssignedTo")
            .and_then(|assignee| assignee.get("displayName"))
            .and_then(|value| value.as_str())
            .unwrap_or("");

        println!("[{workitem_id:8}] {workitem_type:10} {state:10} {assigned_to:30} {title}");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let team = env::var("ADO_TEAM").expect("Must define ADO_TEAM");
    let area_path = env::var("ADO_AREA_PATH").expect("Must define ADO_AREA_PATH");
    let query = env::args()
        .nth(1)
        .expect("Usage: wit_wiql <workitems|bugs>");

    let query = match query.as_str() {
        "workitems" => active_workitems_query(&project, &area_path),
        "bugs" => open_bugs_query(&project, &area_path),
        _ => panic!("Usage: wit_wiql <workitems|bugs>"),
    };

    // Create a wit client
    let wit_client = wit::ClientBuilder::new(credential).build();

    let workitem_ids =
        query_workitem_ids(&wit_client, &organization, &project, &team, &query).await?;

    let workitems = query_workitems(&wit_client, &organization, &project, &workitem_ids).await?;

    println!("\nWork items:\n{workitems:#?}");
    println!("Work item count: {}", workitems.len());

    display_workitems(&workitems);

    Ok(())
}
