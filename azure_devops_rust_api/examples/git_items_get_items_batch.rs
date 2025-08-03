// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_items_get_items_batch.rs
// Git get items (files and folders) batch example.
use anyhow::Result;
use azure_devops_rust_api::git;
use azure_devops_rust_api::git::models::{GitItemDescriptor, GitItemRequestData};
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_items_get <repository-name> <file_path>");
    let file_path = env::args()
        .nth(2)
        .expect("Usage: git_items_get <repository-name> <file_path>");

    let git_item_request_data = GitItemRequestData {
        include_content_metadata: Some(true),
        include_links: Some(true),
        item_descriptors: vec![GitItemDescriptor {
            path: Some(file_path.clone()),
            recursion_level: None,
            version: None,
            version_options: None,
            version_type: None,
        }],
        latest_processed_change: Some(true),
    };

    // Create a git client
    let git_client = git::ClientBuilder::new(credential.clone()).build();

    let items_batch = git_client
        .items_client()
        .get_items_batch(
            &organization,
            git_item_request_data,
            &repository_name,
            &project,
        )
        .await?;

    println!("\n{file_path} metadata items batch:\n{items_batch:#?}");

    Ok(())
}
