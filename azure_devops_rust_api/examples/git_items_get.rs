// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_items_get.rs
// Git items (files and folders) get example.
use anyhow::Result;
use azure_devops_rust_api::git;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli.
    let credential = match env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential::new()))
        }
    };

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
        .into_future()
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
