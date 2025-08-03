// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// work.rs
// Work module examples:
// - Get team settings
// - Get iterations
// - Get current iteration
// - Get current iteration work items

use anyhow::{Context, Result};
use azure_devops_rust_api::work;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let team = env::var("ADO_TEAM").expect("Must define ADO_TEAM");

    // Create a work client
    let work_client = work::ClientBuilder::new(credential).build();

    // Get team settings
    let team_settings = work_client
        .teamsettings_client()
        .get(&organization, &project, &team)
        .await?;
    println!("Team settings:\n{team_settings:#?}");

    // Get all iterations
    let iterations = work_client
        .iterations_client()
        .list(&organization, &project, &team)
        .await?;
    println!("\nIterations:\n{iterations:#?}");

    // Get current iteration
    let current_iteration = work_client
        .iterations_client()
        .list(&organization, &project, &team)
        .timeframe("current")
        .await?;
    println!("\nCurrent iteration:\n{current_iteration:#?}");

    // Get current iteration id
    let current_iteration_id = current_iteration
        .value
        .first()
        .context("No current iteration")?
        .id
        .as_ref()
        .context("Current iteration has no id")?;
    println!("\ncurrent_iteration_id: {current_iteration_id}");

    // Get current iteration work items
    let current_iteration_workitems = work_client
        .iterations_client()
        .get_iteration_work_items(&organization, &project, current_iteration_id, &team)
        .await?;
    println!("\nCurrent iteration workitems:\n{current_iteration_workitems:#?}");

    println!(
        "\nCurrent iteration work_item_relations length: {}",
        current_iteration_workitems.work_item_relations.len()
    );

    Ok(())
}
