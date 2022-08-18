// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_get.rs
// Repository get example.
use anyhow::Result;
use azure_core::ClientOptions;
use azure_devops_rust_api::git;
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
    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repo_name = env::args()
        .nth(1)
        .expect("Usage: git_repo_get <repository-name>");

    // Create a "git" client
    let client = git::Client::new(
        service_endpoint,
        credential,
        vec![],
        ClientOptions::default(),
    );

    // Use the client to get the specified repo
    let repo = client
        .repositories_client()
        .get_repository(&organization, &repo_name, &project)
        .into_future()
        .await?;
    println!("{:#?}", repo);

    // Use the client to get up to 10 pull requests on the specified repo
    let prs = client
        .pull_requests_client()
        .get_pull_requests(&organization, &repo.id, &project)
        .top(10)
        .into_future()
        .await?
        .value;

    println!("\nFound {} pull requests", prs.len());
    for pr in &prs {
        println!("{:<8}{}", pr.pull_request_id, pr.title.as_ref().unwrap());
    }

    if let Some(pr) = prs.iter().next() {
        println!("\nExample PR struct:");
        println!("{:#?}", pr);
    }

    // Use the client to get up to 10 refs on the specified repo
    let git_refs = client
        .refs_client()
        .list(&organization, &repo.id, &project)
        .top(10)
        .into_future()
        .await?
        .value;

    println!("\nGot {} refs", git_refs.len());
    for git_ref in &git_refs {
        println!("{:<50}{}", git_ref.name, git_ref.object_id);
    }

    if let Some(git_ref) = git_refs.iter().next() {
        println!("\nExample ref struct:");
        println!("{:#?}", git_ref);
    }

    Ok(())
}
