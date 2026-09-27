// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_list_reqwest_blocking.rs
// Repository list example, using the blocking (non-async) API with reqwest.
use anyhow::Result;
use azure_devops_rust_api::blocking::ReqwestBlockingClient;
use azure_devops_rust_api::{git, Credential};
use std::env;

fn main() -> Result<()> {
    // Blocking requests only support PAT authentication
    let token = env::var("ADO_TOKEN").expect("Must define ADO_TOKEN");
    let credential = Credential::from_pat(token);

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a blocking git client, which sends requests using the supplied HTTP client
    let git_client =
        git::blocking::ClientBuilder::new(credential, ReqwestBlockingClient::new()).build();

    // Get all repositories in the specified organization/project
    let repos = git_client
        .repositories_client()
        .list(organization, project)
        .send()?
        .into_body()?
        .value;

    for repo in repos.iter() {
        println!("{}", repo.name);
    }
    println!("{} repos found", repos.len());

    Ok(())
}
