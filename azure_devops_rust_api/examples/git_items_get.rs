// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_items_get.rs
// Git items (files and folders) get example.
use anyhow::Result;
use azure_devops_rust_api::git;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential
    let credential = utils::get_credential();

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_items_get <repository-name> <file_path>");
    let file_path = env::args()
        .nth(2)
        .expect("Usage: git_items_get <repository-name> <file_path>");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // To get the file metadata, it appears that you need to specify format as "json"
    let item = git_client
        .items_client()
        .get(&organization, &repository_name, &file_path, &project)
        .format("json")
        .await?;

    println!("\n{file_path} metadata:\n{:#?}", item);

    // If no format is specified, the file contents are returned
    let rsp = git_client
        .items_client()
        .get(&organization, &repository_name, &file_path, &project)
        .send()
        .await?
        .into_raw_response();

    let file_data = rsp.into_body().collect_string().await?;
    println!("\n{file_path} contents:\n{}", file_data);

    Ok(())
}
