// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wit.rs
// Work Item example.
use anyhow::Result;
use azure_devops_rust_api::wit;
use azure_devops_rust_api::wit::models::WorkItemRelation;
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
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let work_item_id: i32 = env::args()
        .nth(1)
        .expect("Usage: wit <work_item_id>")
        .parse()
        .expect("integer id");

    // Create a wit client
    let wit_client = wit::ClientBuilder::new(credential).build();

    // Get work items
    let work_item = wit_client
        .work_items_client()
        .get_work_item(&organization, work_item_id, &project)
        .expand("All")
        .into_future()
        .await?;

    println!("{:#?}", work_item);

    const CHILD_LINK_TYPE: &str = "System.LinkTypes.Hierarchy-Forward";
    let children: Vec<&WorkItemRelation> = work_item
        .relations
        .iter()
        .filter(|relation| relation.link.rel == CHILD_LINK_TYPE)
        .collect();

    println!("\n{} child work items found", children.len());
    for child in children.iter() {
        println!("{}", child.link.url);
    }

    const ARTIFACT_LINK_TYPE: &str = "ArtifactLink";
    let artifacts: Vec<&WorkItemRelation> = work_item
        .relations
        .iter()
        .filter(|relation| relation.link.rel == ARTIFACT_LINK_TYPE)
        .collect();

    println!("\n{} related artifacts found", artifacts.len());
    for artifact in artifacts.iter() {
        println!("{}", artifact.link.url);
    }

    Ok(())
}
