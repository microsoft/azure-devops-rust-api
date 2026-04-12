// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// core_org_projects.rs
// Projects from Orgs example.
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

    // Max number of projects to be fetched, default max is 100
    let top_projects: i32 = 500;

    // Create core client
    let core_client = core::ClientBuilder::new(credential).build();

    let org_projects = core_client
        .projects_client()
        .list(&organization)
        .top(top_projects)
        .await?;

    println!("{org_projects:#?}");

    Ok(())
}
