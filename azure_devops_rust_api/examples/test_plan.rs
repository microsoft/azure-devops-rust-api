// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// test_plan.rs
// Getting test plan example
use anyhow::Result;
use azure_devops_rust_api::test_plan;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential
    let credential = utils::get_credential();

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create test_plan client
    let test_plan_client = test_plan::ClientBuilder::new(credential).build();

    // Get all test plans for project
    println!("The test runs for project are:");
    let test_plans = test_plan_client
        .test_plans_client()
        .list(&organization, &project)
        .await?
        .value;
    println!("{:#?}", test_plans);

    Ok(())
}
