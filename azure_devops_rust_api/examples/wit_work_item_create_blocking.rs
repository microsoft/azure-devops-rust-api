// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wit_work_item_create_blocking.rs
// Work Item creation example, using the blocking (non-async) API.
use anyhow::Result;
use azure_devops_rust_api::wit;
use azure_devops_rust_api::wit::models::{json_patch_operation::Op, JsonPatchOperation};
use serde_json::json;
use std::env;

mod utils;

fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a blocking wit client
    let wit_client = wit::blocking::ClientBuilder::new(credential).build();

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
    // of a work item (see the comments at the end of wit_work_item_create.rs for some examples)
    let body = vec![title];

    // Create a work item
    let work_item = wit_client
        .work_items_client()
        .create(organization, body, project, work_item_type)
        .send()?
        .into_body()?;

    println!("{work_item:#?}");

    Ok(())
}
