// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// graph_query.rs
// Graph example.
use anyhow::Result;
use azure_devops_rust_api::graph;
use azure_devops_rust_api::graph::models::GraphSubjectQuery;
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
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential {}))
        }
    };

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let name = env::args().nth(1).expect("Usage: graph_query <name>");

    // Create a "graph" client
    let graph_client = graph::ClientBuilder::new(credential).build();

    // Create a query for a user with the specified name
    let query = GraphSubjectQuery {
        query: Some(name.to_string()),
        scope_descriptor: None,
        subject_kind: vec!["User".to_string()],
    };

    // Query the specified user
    let subjects = graph_client
        .subject_query_client()
        .query(&organization, query)
        .into_future()
        .await?
        .value;

    println!("Found {} subjects", subjects.len());
    if let Some(subject) = subjects.iter().next() {
        println!("subject: {:#?}", subject);
    }

    Ok(())
}
