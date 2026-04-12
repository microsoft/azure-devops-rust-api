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
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let test_plan_id_value = env::var("TEST_PLAN_ID").expect("Must define PLAN_ID for the test");
    let test_plan_id: i32 = test_plan_id_value
        .parse()
        .expect("Failed to parse TEST_PLAN_ID");
    // Create test_plan client
    let test_plan_client = test_plan::ClientBuilder::new(credential).build();

    // Get all test plans for project
    println!("The test plan for project are:");
    let test_plans = test_plan_client
        .test_plans_client()
        .list(&organization, &project)
        .await?
        .value;
    println!("{test_plans:#?}");

    // Get all suites for a test plan for a project
    println!("The suites for the a plan for project are:");
    let test_suites = test_plan_client
        .test_suites_client()
        .get_test_suites_for_plan(&organization, &project, test_plan_id)
        .await?
        .value;
    println!("{test_suites:#?}");

    // Get all test plan variables for project
    println!("The test plans variables for project are:");
    let test_plan_variables = test_plan_client
        .variables_client()
        .list(&organization, &project)
        .await?
        .value;
    println!("{test_plan_variables:#?}");

    // Get all test plan configuration for project
    println!("The test plan configuration for project are:");
    let test_plan_configuration = test_plan_client
        .configurations_client()
        .list(&organization, &project)
        .await?
        .value;
    println!("{test_plan_configuration:#?}");

    Ok(())
}
