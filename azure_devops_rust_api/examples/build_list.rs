// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// build_list.rs
// Repository list example.
use azure_devops_rust_api::build;
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let credential = Arc::new(azure_identity::token_credentials::AzureCliCredential {});

    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let client = build::operations::Client::new(service_endpoint, credential, vec![]);

    let builds = client
        .builds()
        .list(organization, project)
        .into_future()
        .await
        .unwrap()
        .value;

    println!("Found {} builds", builds.len());
    if let Some(build) = builds.iter().next() {
        println!("build: {:#?}", build);
    }

    Ok(())
}
