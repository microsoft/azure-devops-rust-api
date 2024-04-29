// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_items_list.rs
// Git items (files and folders) list example.
use anyhow::Result;
use azure_devops_rust_api::git;
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
        .expect("Usage: git_items_list <repository-name>");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get all items (files and folders) in the specified repository
    let items = git_client
        .items_client()
        .list(organization, repository_name, project)
        .recursion_level("Full")
        .await?
        .value;

    for item in items.iter() {
        if let Some(path) = &item.item_model.path {
            println!("{path}");
        }
    }
    println!("{} items found", items.len());

    Ok(())
}
