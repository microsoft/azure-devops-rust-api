// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// permissions_report.rs
// Permissions report example
use anyhow::Result;
use azure_devops_rust_api::permissions_report;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");

    // Create permissions_report client
    let permissions_report_client = permissions_report::ClientBuilder::new(credential).build();

    // Get Permissions reports
    println!("Permissions reports:");
    let permissions_reports = permissions_report_client
        .permissions_report_client()
        .list(&organization)
        .await?
        .value;
    println!("{permissions_reports:#?}");

    Ok(())
}
