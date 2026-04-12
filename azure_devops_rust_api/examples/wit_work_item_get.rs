// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wit_work_item_get.rs
// Work Item query example.
use anyhow::{anyhow, Context, Result};
use azure_devops_rust_api::wit;
use azure_devops_rust_api::wit::models::WorkItemRelation;
use std::env;

mod utils;

// For info on work item link types see:
// https://learn.microsoft.com/en-us/azure/devops/boards/queries/link-type-reference?view=azure-devops
const CHILD_RELATION_TYPE: &str = "System.LinkTypes.Hierarchy-Forward";
const PARENT_RELATION_TYPE: &str = "System.LinkTypes.Hierarchy-Reverse";
const RELATED_RELATION_TYPE: &str = "System.LinkTypes.Related";

// Extract work item id from url.
// Work item url is of the form: https://dev.azure.com/.../<id>/_apis/wit/workItems/<work-item-id>
fn work_item_id_from_url(url: &str) -> Result<i32> {
    url.rsplit('/')
        .next()
        .ok_or_else(|| anyhow!("Failed to extract last segment of URL: {url}"))?
        .parse::<i32>()
        .with_context(|| format!("Failed to parse work item id from url: {url}"))
}

// Extract work item type from work item.
fn work_item_type(work_item: &wit::models::WorkItem) -> String {
    work_item
        .fields
        .get("System.WorkItemType")
        .and_then(|value| value.as_str())
        .unwrap_or("<unknown>")
        .to_string()
}

// Return work item ids of related work items with the specified relation type.
fn work_item_relations(work_item: &wit::models::WorkItem, relation_type: &str) -> Vec<i32> {
    work_item
        .relations
        .iter()
        .filter(|relation| relation.link.rel == relation_type)
        .filter_map(|relation| work_item_id_from_url(&relation.link.url).ok())
        .collect()
}

fn relation_name(relation: &WorkItemRelation) -> String {
    relation.link.attributes["name"]
        .as_str()
        .unwrap_or("<unknown>")
        .to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let work_item_id: i32 = env::args()
        .nth(1)
        .expect("Usage: wit <work_item_id>")
        .parse()
        .expect("integer id");

    // Create a wit client
    let wit_client = wit::ClientBuilder::new(credential).build();

    // Get specified work item
    let work_item = wit_client
        .work_items_client()
        .get_work_item(&organization, work_item_id, &project)
        .expand("All")
        .await?;

    println!("Work item [{work_item_id}]:\n{work_item:#?}");

    // Show work item type
    println!("Work item type: {}", work_item_type(&work_item));

    // Show child work items
    let children = work_item_relations(&work_item, CHILD_RELATION_TYPE);
    println!(
        "\n[{work_item_id}] {} children: {:#?}",
        children.len(),
        children
    );

    // Show parent work items (expect to see only 0 or 1)
    let parent = work_item_relations(&work_item, PARENT_RELATION_TYPE);
    println!("\n[{work_item_id}] {} parent: {:#?}", parent.len(), parent);

    // Show related work items
    let related = work_item_relations(&work_item, RELATED_RELATION_TYPE);
    println!(
        "\n[{work_item_id}] {} related: {:#?}",
        related.len(),
        related
    );

    // Show all work item relations
    println!(
        "\n[{work_item_id}] All {} relations:",
        work_item.relations.len()
    );
    for relation in work_item.relations.iter() {
        println!(
            "  {:30} {:40} {}",
            relation_name(relation),
            relation.link.rel,
            relation.link.url
        );
    }

    if !children.is_empty() {
        // Use a batch get request to query specific fields for child work items
        let batch_get_request = wit::models::WorkItemBatchGetRequest {
            ids: children,
            fields: vec![
                "System.Id".to_string(),
                "System.Title".to_string(),
                "System.WorkItemType".to_string(),
                "System.State".to_string(),
            ],
            as_of: None,
            expand: None,
            error_policy: None,
        };

        let child_details = wit_client
            .work_items_client()
            .get_work_items_batch(&organization, batch_get_request, &project)
            .await?
            .value;

        println!("\nChild work item batch get with selected fields:");
        for child in child_details.iter() {
            println!("[{work_item_id}] {child:#?}", work_item_id = child.id,);
        }
    }

    Ok(())
}
