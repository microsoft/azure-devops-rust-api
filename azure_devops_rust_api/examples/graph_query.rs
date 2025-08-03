// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// graph_query.rs
// Graph example.
use anyhow::Result;
use azure_devops_rust_api::graph;
use azure_devops_rust_api::graph::models::GraphSubjectQuery;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

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
        .await?
        .value;

    println!("Found {} subjects", subjects.len());
    if let Some(subject) = subjects.first() {
        println!("subject: {subject:#?}");
    }

    Ok(())
}
