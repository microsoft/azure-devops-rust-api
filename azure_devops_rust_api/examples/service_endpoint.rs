// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// service_endpoint.rs
// Service Endpoint (aka "Service Connection") example.
use anyhow::Result;
use azure_devops_rust_api::service_endpoint;
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

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a service_endpoint client
    let service_endpoint_client = service_endpoint::ClientBuilder::new(credential).build();

    // Use the client to list all service endpoints (aka "service connections")
    let service_endpoints = service_endpoint_client
        .endpoints_client()
        .get_service_endpoints(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("Total service_endpoints: {}", service_endpoints.len());

    // Display the returned service endpoints
    for endpoint in service_endpoints.iter() {
        println!(
            "{:38} {:40} {}",
            endpoint.id, endpoint.name, endpoint.description
        );
    }

    // Display an example service endpoint struct
    if let Some(endpoint) = service_endpoints.iter().next() {
        println!("\nExample service_endpoint struct:");
        println!("{:#?}", endpoint);
    }

    Ok(())
}
