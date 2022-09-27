// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// policy.rs
// Policy example.
use anyhow::Result;
use azure_devops_rust_api::policy;
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
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a policy client
    let policy_client = policy::ClientBuilder::new(credential).build();

    // Get all policy configurations in the specified organization/project
    let policy_types = policy_client
        .types_client()
        .list(&organization, &project)
        .into_future()
        .await?
        .value;

    for policy_type in policy_types.iter() {
        let type_ref = &policy_type.policy_type_ref;
        println!(
            "{} {:32} {}",
            type_ref.id, type_ref.display_name, policy_type.description
        );
    }
    println!("{} policy types found", policy_types.len());

    let work_item_linking_policy_id = policy_types
        .iter()
        .find_map(|pt| {
            if pt.policy_type_ref.display_name == "Work item linking" {
                Some(pt.policy_type_ref.id.clone())
            } else {
                None
            }
        })
        .unwrap();
    println!(
        "Work item linking policy id: {}",
        work_item_linking_policy_id
    );

    let configs = policy_client
        .configurations_client()
        .list(&organization, &project)
        .policy_type(work_item_linking_policy_id)
        .into_future()
        .await?
        .value;
    println!("{} work item policy configurations found", configs.len());

    if let Some(config) = configs.iter().next() {
        println!("Example config:\n{:#?}", config);
    }

    Ok(())
}
