// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// hooks_list.rs
// Service hooks example
use anyhow::Result;
use azure_devops_rust_api::hooks;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    // Create service hook client
    let hook_client = hooks::ClientBuilder::new(credential).build();

    // Get all consumers
    println!("The service hook consumers are:");
    let service_hook_consumers = hook_client
        .consumers_client()
        .list(&organization)
        .await?
        .value;
    println!("{service_hook_consumers:#?}");

    // Get all publishers
    println!("The service hook publishers are:");
    let service_hook_publishers = hook_client
        .publishers_client()
        .list(&organization)
        .await?
        .value;
    println!("{service_hook_publishers:#?}");

    // Get all subscriptions
    println!("The service hook subscriptions are:");
    let service_hook_subscriptions = hook_client
        .subscriptions_client()
        .list(&organization)
        .await?
        .value;
    println!("{service_hook_subscriptions:#?}");

    Ok(())
}
