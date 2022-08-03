// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_get.rs
// Repository get example.
use azure_devops_rust_api::git;
use azure_devops_rust_api::Credential;
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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
    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repo_name = env::args()
        .nth(1)
        .expect("Usage: git_repo_get <repository-name>");

    // Create a "git" client
    let client = git::operations::Client::new(service_endpoint, credential, vec![]);

    // Use the client to get the specified repo
    let repo = client
        .repositories()
        .get_repository(&organization, &repo_name, &project)
        .into_future()
        .await?;
    println!("{:#?}", repo);

    // Use the client to get all pull requests on the specified repo
    let prs = client
        .pull_requests()
        .get_pull_requests(&organization, &repo.id, &project)
        .into_future()
        .await?
        .value;

    println!("Found {} pull requests", prs.len());
    if let Some(pr) = prs.iter().next() {
        println!("Example PR struct:");
        println!("{:#?}", pr);
    }

    Ok(())
}
