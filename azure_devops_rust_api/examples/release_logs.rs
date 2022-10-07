// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// release_logs.rs
// Release logs example.
// The log data is saved as a zip file - use `unzip` to extract
use anyhow::{anyhow, Result};
use azure_devops_rust_api::release;
use azure_devops_rust_api::Credential;
use std::env;
use std::fs::File;
use std::io::prelude::*;
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
    let release_id: i32 = env::args()
        .nth(1)
        .expect("Usage: release-logs <release-id> <output-file>")
        .parse::<i32>()
        .ok()
        .expect("Must Provide release ID");
    let output_file: String = env::args()
        .nth(2)
        .expect("Usage: release-logs <release-id> <output-file>");

    // Create release client
    let release_client = release::ClientBuilder::new(credential).build();

    // Get release logs
    println!("\nDownloading release logs for release {}", release_id);
    let (status, _headers, body) = release_client
        .releases_client()
        .get_logs(organization, project, release_id)
        .send()
        .await?
        .into_raw_response()
        .deconstruct();

    if status != azure_core::StatusCode::Ok {
        println!("Request failed. status:{}", status);
        return Err(anyhow!("Request failed"));
    }

    // Write the data as a zipfile
    println!("Writing data to zipfile: {}", output_file);
    let data = body.collect().await?;
    let mut file = File::create(&output_file)?;
    file.write_all(&data)?;
    println!("Logs saved");

    println!("Use 'unzip {}' to extract the logs", output_file);

    Ok(())
}
