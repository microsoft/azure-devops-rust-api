// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// core_project_teams.rs
// Project Teams from organization example.
use anyhow::Result;
use azure_devops_rust_api::core;
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

    // Max number of teams to be fetched, default max is 100
    let top_teams: i32 = 500;

    // Create core client
    let core_client = core::ClientBuilder::new(credential).build();

    let project_teams = core_client
        .teams_client()
        .get_teams(&organization, &project)
        .top(top_teams)
        .into_future()
        .await?
        .value;

    // Display team names
    println!("\nProject teams:");
    for team in project_teams.iter() {
        match &team.web_api_team_ref.name{
            Some(name) => println!("{}", name),
            _ => {}
        }
    }

    Ok(())
}
