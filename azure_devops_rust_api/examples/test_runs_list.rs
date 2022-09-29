// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// test_runs_list.rs
// Getting test runs example
use anyhow::Result;
use azure_devops_rust_api::test;
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

    // Max number of runs for the project to be fetched
    let top_test_runs: i32 = 200;

    // Create test_run client
    let test_run_client = test::ClientBuilder::new(credential).build();

    // Get all runs for project
    println!("The test runs for project are:");
    let test_runs = test_run_client
        .runs_client()
        .list(&organization, &project)
        .top(top_test_runs)
        .into_future()
        .await?
        .value;
    println!("{:#?}", test_runs);

    Ok(())
}
