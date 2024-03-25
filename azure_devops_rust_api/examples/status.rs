// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// status.rs
// Service status example.
use anyhow::Result;
use azure_devops_rust_api::status;
use azure_devops_rust_api::status::models::geography_with_health::Health;
use azure_devops_rust_api::Credential;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // The status API is unauthenticated, so don't provide usual credentials
    let credential = Credential::unauthenticated();

    // Create a status client
    println!("Create status client");
    let status_client = status::ClientBuilder::new(credential).build();

    // Get service status
    println!("Get service status");
    let status = status_client.health_client().get().await?;

    println!(
        "{:?}: {}",
        status.status.status.health, status.status.status.message,
    );

    for service in status.services.iter() {
        println!("{}:", service.id);
        let (healthy, unhealthy): (Vec<_>, Vec<_>) = service
            .geographies
            .iter()
            .partition(|health| health.health == Health::Healthy);
        let healthy: Vec<&str> = healthy
            .iter()
            .map(|health| health.geography.id.as_ref())
            .collect();
        let unhealthy: Vec<&str> = unhealthy
            .iter()
            .map(|health| health.geography.id.as_ref())
            .collect();
        println!("  healthy:   {}", healthy.join(" "));
        println!("  unhealthy: {}", unhealthy.join(" "));
    }

    Ok(())
}
