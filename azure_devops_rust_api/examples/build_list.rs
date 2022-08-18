// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// build_list.rs
// Repository list example.
use anyhow::Result;
use azure_core::ClientOptions;
use azure_devops_rust_api::build;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli.
    let credential = match env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential {}))
        }
    };

    // Get ADO server configuration via environment variables
    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a "build" client
    println!("Create client");
    let client = build::Client::new(
        service_endpoint,
        credential,
        vec![],
        ClientOptions::default(),
    );

    println!("Get list");
    // Use the client to list all builds in the specified organization/project
    let builds = client
        .builds_client()
        .list(organization, project)
        .top(1)
        .into_future()
        .await?
        .value;

    println!("Found {} builds", builds.len());
    if let Some(build) = builds.iter().next() {
        println!("Example build struct: {:#?}", build);
    }

    Ok(())
}
