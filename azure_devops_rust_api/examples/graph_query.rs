// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// graph_query.rs
// Graph example.
use anyhow::Result;
use azure_core::ClientOptions;
use azure_devops_rust_api::graph;
use azure_devops_rust_api::graph::models::GraphSubjectQuery;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
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

    // Get ADO server configuration via environment variables
    // Service endpoint is different from most ADO APIs - it has to have "vssps" prefix, e.g. https://vssps.dev.azure.com/
    let service_endpoint = "https://vssps.dev.azure.com";
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let name = env::args().nth(1).expect("Usage: graph_query <name>");

    // Create a `graph` client
    let client = graph::Client::new(
        service_endpoint,
        credential,
        vec![],
        ClientOptions::default(),
    )
    .subject_query_client();

    // Create a query for a `User` with the specified name
    let query = GraphSubjectQuery {
        query: Some(name.to_string()),
        scope_descriptor: None,
        subject_kind: vec!["User".to_string()],
    };

    // Use the client to query the specified user
    let subjects = client
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
