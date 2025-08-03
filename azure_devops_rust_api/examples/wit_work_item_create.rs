// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wit_work_item_create.rs
// Work Item creation example.
use anyhow::Result;
use azure_devops_rust_api::wit;
use azure_devops_rust_api::wit::models::{json_patch_operation::Op, JsonPatchOperation};
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

    // Create a wit client
    let wit_client = wit::ClientBuilder::new(credential).build();

    // Assign the type of work item to create
    let work_item_type = "User Story";

    // Define the title of the work item to be created
    let title = JsonPatchOperation {
        from: None,
        op: Some(Op::Add),
        path: Some("/fields/System.Title".to_owned()),
        value: Some(json!("Example User Story title")),
    };

    // Each operation lives in a vector, additional elements can be added to fill in other fields
    // of a work item, see the comments at the end of this file for some examples
    let body = vec![title];

    // Create a work item
    let work_item = wit_client
        .work_items_client()
        .create(organization, body, project, work_item_type)
        .await?;

    println!("{work_item:#?}");

    Ok(())
}

// When creating a work item you can also assign an iteration

// let iteration = JsonPatchOperation {
// 	from: None,
// 	op: Some(Op::Add),
// 	path: Some("/fields/System.IterationPath".to_owned()),
// 	value: Some(json!("my-iteration".to_owned()))
// };
//
// e.g create a work item with a title and in an iteration
//
// let body = vec![title, iteration];

// When creating a work item you can also assign a parent

// let parent = JsonPatchOperation {
// 	from: None,
// 	op: Some(Op::Add),
// 	path: Some("/relations/-".to_owned()),
// 	value: Some(json!({
// 		"rel": "System.LinkTypes.Hierarchy-Reverse",
// 		"url": format!("https://dev.azure.com/{}/{}/_apis/wit/workItems/{}", organisation, project, parent_id)
// 	}))
// }
//
// e.g create a work item with a title, in an iteration and assign a parent item
//
// let body = vec![title, iteration, parent];
