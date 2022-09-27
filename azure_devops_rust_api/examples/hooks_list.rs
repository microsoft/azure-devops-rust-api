// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// hooks_list.rs
// Service hooks example
use anyhow::Result;
use azure_devops_rust_api::hooks;
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
    // Create service hook client
    let hook_client = hooks::ClientBuilder::new(credential).build();

    // Get all consumers
    println!("The service hook consumers are:");
    let service_hook_consumers = hook_client
        .consumers_client()
        .list(&organization)
        .into_future()
        .await?
        .value;
    println!("{:#?}", service_hook_consumers);

    // Get all publishers
    println!("The service hook publishers are:");
    let service_hook_publishers = hook_client
        .publishers_client()
        .list(&organization)
        .into_future()
        .await?
        .value;
    println!("{:#?}", service_hook_publishers);

    // Get all subscriptions
    println!("The service hook subscriptions are:");
    let service_hook_subscriptions = hook_client
        .subscriptions_client()
        .list(&organization)
        .into_future()
        .await?
        .value;
    println!("{:#?}", service_hook_subscriptions);

    Ok(())
}
