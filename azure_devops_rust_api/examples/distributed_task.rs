// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
// distributed_task.rs
// Distributed Task example
use anyhow::Result;
use azure_devops_rust_api::distributed_task;
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

    // Create distributed task client
    let distributed_task_client = distributed_task::ClientBuilder::new(credential).build();

    //  Get a list of agent pools for the org
    println!("Agents pools for the org are:");
    let distributed_task_agents_pools = distributed_task_client
        .pools_client()
        .get_agent_pools(&organization)
        .into_future()
        .await?
        .value;
    println!("{:#?}", distributed_task_agents_pools);

    //  Get a list of agent queues for the project
    println!("Agents queues for the project are:");
    let distributed_task_agent_queues = distributed_task_client
        .queues_client()
        .get_agent_queues(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("{:#?}", distributed_task_agent_queues);

    // Get all variable groups for the project
    println!("Variable groups for the project are:");
    let distributed_task_variable_groups = distributed_task_client
        .variablegroups_client()
        .get_variable_groups(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("{:#?}", distributed_task_variable_groups);

    Ok(())
}
