// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// release.rs
// Release example
use anyhow::Result;
use azure_devops_rust_api::release;
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

    // Create release client
    let release_client = release::ClientBuilder::new(credential).build();
    let folder_path = "\\".to_string();

    //  Get list of approvals for release
    println!("Approvals for release are:");
    let release_approvals = release_client
        .approvals_client()
        .list(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("{:#?}", release_approvals);

    //  Get list of folders for release
    println!("Folders for the project are:");
    let project_folders = release_client
        .folders_client()
        .list(&organization, &project, &folder_path)
        .into_future()
        .await?
        .value;
    println!("{:#?}", project_folders);

    //  Get list of deployments for release
    println!("Deployments for project are:");
    let project_deployments = release_client
        .deployments_client()
        .list(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("{:#?}", project_deployments);

    //  Get a list of releases for project
    println!("List of releases for project are:");
    let project_releases = release_client
        .releases_client()
        .list(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("{:#?}", project_releases);

    Ok(())
}
