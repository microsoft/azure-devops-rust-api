// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// service_endpoint.rs
// Service Endpoint (aka "Service Connection") example.
use anyhow::Result;
use azure_devops_rust_api::service_endpoint;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a service_endpoint client
    let service_endpoint_client = service_endpoint::ClientBuilder::new(credential).build();

    // Use the client to list all service endpoints (aka "service connections")
    let service_endpoints = service_endpoint_client
        .endpoints_client()
        .get_service_endpoints(&organization, &project)
        .await?
        .value;
    println!("Total service_endpoints: {}", service_endpoints.len());

    // Display the returned service endpoints
    for endpoint in service_endpoints.iter() {
        println!(
            "{:38} {:40} {:?}",
            endpoint.id,
            endpoint.name,
            endpoint.description.as_deref().unwrap_or("")
        );
    }

    // Display an example service endpoint struct
    if let Some(endpoint) = service_endpoints.first() {
        println!("\nExample service_endpoint struct:");
        println!("{endpoint:#?}");
    }

    Ok(())
}
