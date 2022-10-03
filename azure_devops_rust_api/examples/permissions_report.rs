// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// permissions_report.rs
// Permissions report example
use anyhow::Result;
use azure_devops_rust_api::permissions_report;
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

    // Create permissions_report client
    let permissions_report_client = permissions_report::ClientBuilder::new(credential).build();

    // Get Permissions reports
    println!("Permissions reports:");
    let permissions_reports = permissions_report_client
        .permissions_report_client()
        .list(&organization)
        .into_future()
        .await?
        .value;
    println!("{:#?}", permissions_reports);

    Ok(())
}
