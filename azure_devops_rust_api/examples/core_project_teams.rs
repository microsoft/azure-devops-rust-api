// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// core_project_teams.rs
// Project Teams from organization example.
use anyhow::Result;
use azure_devops_rust_api::core;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Max number of teams to be fetched, default max is 100
    let top_teams: i32 = 500;

    // Create core client
    let core_client = core::ClientBuilder::new(credential).build();

    let project_teams = core_client
        .teams_client()
        .get_teams(&organization, &project)
        .top(top_teams)
        .await?
        .value;

    // Display team names
    println!("\nProject teams:");
    for team in project_teams.iter() {
        if let Some(name) = &team.web_api_team_ref.name {
            println!("{name}");
        }
    }

    Ok(())
}
