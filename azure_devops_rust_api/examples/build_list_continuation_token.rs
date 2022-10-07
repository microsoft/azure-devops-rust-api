// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// build_list_continuation_token.rs
// Example demonstrating how to make large queries using continuation tokens.
use anyhow::{anyhow, Context, Result};
use azure_core::StatusCode;
use azure_devops_rust_api::build;
use azure_devops_rust_api::build::models::{Build, BuildList};
use azure_devops_rust_api::Credential;
use serde_json;
use std::env;
use std::sync::Arc;
use time::format_description::well_known::Rfc3339;

const NUM_BUILD_BATCHES: usize = 5;

async fn get_builds(
    build_client: &build::Client,
    organization: &str,
    project: &str,
    continuation_token: &Option<String>,
) -> Result<(Vec<Build>, Option<String>)> {
    let mut list_builder = build_client.builds_client().list(organization, project);

    if let Some(continuation_token) = continuation_token {
        println!(
            "Query builds with continuation_token: {}",
            continuation_token
        );
        list_builder = list_builder.continuation_token(continuation_token)
    } else {
        println!("Query builds with no continuation_token");
    }

    let (status, headers, body) = list_builder.send().await?.into_raw_response().deconstruct();

    if status != StatusCode::Ok {
        println!("Request failed");
        return Err(anyhow!("Request failed"));
    }

    let new_continuation_token = headers.get_optional_string(
        &azure_core::headers::HeaderName::from_static("x-ms-continuationtoken"),
    );

    let body_data = body.collect_string().await?;
    let build_list: BuildList = serde_json::from_str(&body_data)
        .with_context(|| format!("Failed to parse BuildList: {}", &body_data))?;

    println!("Received {} builds", build_list.count.unwrap_or(0));

    Ok((build_list.value, new_continuation_token))
}

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

    // Create a build client
    let build_client = build::ClientBuilder::new(credential).build();

    let mut continuation_token = None;

    // Query several batches of builds. Each batch has 1000 builds (by default)
    println!("Num build batches: {}", NUM_BUILD_BATCHES);
    for batch in 0..NUM_BUILD_BATCHES {
        let (builds, new_continuation_token) =
            get_builds(&build_client, &organization, &project, &continuation_token).await?;

        if let Some(build) = builds.iter().next() {
            println!(
                "First build of batch {} start time: {}\n",
                batch,
                build.start_time.unwrap().format(&Rfc3339)?
            );
        }
        continuation_token = new_continuation_token;

        if continuation_token == None {
            println!("continuation_token is None - exiting");
        }
    }

    Ok(())
}
