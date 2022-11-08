// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// build_list_sync.rs
// Repository list example, demonstrating how to call the async API from sync code
// For more info see: https://tokio.rs/tokio/topics/bridging
use anyhow::Result;
use azure_devops_rust_api::build;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;
use time::{ext::NumericalDuration, OffsetDateTime};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli
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

    // Create a `current_thread` tokio runtime to use when invoking async functions
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // Create a build client
    println!("Create build client");
    let build_client = build::ClientBuilder::new(credential.clone()).build();

    // Query all the builds in the past hour
    let end_time = OffsetDateTime::now_utc();
    let start_time = end_time - 1.hours();

    // Get all builds in the specified organization/project in the past hour
    let builds = rt
        .block_on(async {
            println!("Get list");
            build_client
                .builds_client()
                .list(&organization, &project)
                .min_time(start_time)
                .max_time(end_time)
                .into_future()
                .await
        })?
        .value;

    println!("Found {} builds", builds.len());
    if let Some(build) = builds.iter().next() {
        println!("Example build struct: {:#?}", build);
    }

    Ok(())
}
