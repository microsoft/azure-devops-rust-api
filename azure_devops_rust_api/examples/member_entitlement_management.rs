// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// member_entitlement_management.rs
// Member entitlement management example
use anyhow::Result;
use azure_devops_rust_api::member_entitlement_management;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");

    // Create member_entitlement_management client
    let mem_client = member_entitlement_management::ClientBuilder::new(credential).build();

    // Get entitlement summary
    println!("Entitlement summary:");
    let entitlement_summary = mem_client
        .user_entitlement_summary_client()
        .get(organization)
        .select("AccessLevels,Licenses,Projects,Groups")
        .await?;
    println!("{entitlement_summary:#?}");

    Ok(())
}
