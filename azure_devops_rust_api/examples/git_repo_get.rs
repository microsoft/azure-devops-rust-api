// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_get.rs
// Repository get example.
use azure_devops_rust_api::git;
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
    let repo_name = env::args()
        .nth(1)
        .expect("Usage: git_repo_get <repository-name>");

    let client = git::operations::Client::new(service_endpoint, credential, vec![]);

    let repo = client
        .repositories()
        .get_repository(&organization, &repo_name, &project)
        .into_future()
        .await?;
    println!("{:#?}", repo);

    let prs = client
        .pull_requests()
        .get_pull_requests(&organization, &repo.id, &project)
        .into_future()
        .await?
        .value;

    println!("Found {} pull requests", prs.len());
    if let Some(pr) = prs.iter().next() {
        println!("Example PR:");
        println!("{:#?}", pr);
    }

    Ok(())
}
