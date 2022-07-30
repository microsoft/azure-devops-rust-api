// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_list.rs
// Repository list example.
use azure_devops_rust_api::git;
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Get authentication credentials via the az cli
    let credential = Arc::new(azure_identity::AzureCliCredential {});

    // Get ADO server configuration via environment variables
    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a `git` client
    let client = git::operations::Client::new(service_endpoint, credential, vec![]);

    // Use the client to list all repositories in the specified organization/project
    let repos = client
        .repositories()
        .list(organization, project)
        .into_future()
        .await?
        .value;

    for repo in repos.iter() {
        println!("{}", repo.name);
    }
    println!("{} repos found", repos.len());

    Ok(())
}
