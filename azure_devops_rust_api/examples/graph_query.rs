// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// graph.rs
// Graph example.
use azure_devops_rust_api::graph;
use azure_devops_rust_api::graph::models::GraphSubjectQuery;
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let credential = Arc::new(azure_identity::token_credentials::AzureCliCredential {});

    // Service endpoint is different from most ADO APIs - it has to have "vssps" prefix, e.g. https://vssps.dev.azure.com/
    let service_endpoint = "https://vssps.dev.azure.com";
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let name = env::args().nth(1).expect("Usage: graph_query <name>");

    let client =
        graph::operations::Client::new(service_endpoint, credential, vec![]).subject_query();

    let query = GraphSubjectQuery {
        query: Some(name.to_string()),
        scope_descriptor: None,
        subject_kind: vec!["User".to_string()],
    };

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
