// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// release_logs.rs
// Release logs example.
// The log data is saved as a zip file - use `unzip` to extract
use anyhow::{anyhow, Result};
use azure_core::http::StatusCode;
use azure_devops_rust_api::release;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let release_id: i32 = env::args()
        .nth(1)
        .expect("Usage: release-logs <release-id> <output-file>")
        .parse::<i32>()
        .expect("release-id parameter must be an integer");
    let output_file: String = env::args()
        .nth(2)
        .expect("Usage: release-logs <release-id> <output-file>");

    // Create release client
    let release_client = release::ClientBuilder::new(credential).build();

    // Get release logs
    println!("\nDownloading release logs for release {release_id}");
    let (status, _headers, body) = release_client
        .releases_client()
        .get_logs(organization, project, release_id)
        .send()
        .await?
        .into_raw_response()
        .deconstruct();

    if status != StatusCode::Ok {
        println!("Request failed. status:{status}");
        return Err(anyhow!("Request failed"));
    }

    // Write the data as a zipfile
    println!("Writing data to zipfile: {output_file}");
    let data = body.collect().await?;
    let mut file = File::create(&output_file)?;
    file.write_all(&data)?;
    println!("Logs saved");

    println!("Use 'unzip {output_file}' to extract the logs");

    Ok(())
}
