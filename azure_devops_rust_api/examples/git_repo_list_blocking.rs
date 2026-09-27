// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_repo_list_blocking.rs
// Repository list example, using the blocking (non-async) API.
use anyhow::Result;
use azure_devops_rust_api::git;
use std::env;

mod utils;

fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a blocking git client
    let git_client = git::blocking::ClientBuilder::new(credential).build();

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
