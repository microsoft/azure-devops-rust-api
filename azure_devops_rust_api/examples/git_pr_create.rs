// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_pr_create.rs
// Create Pull Request example.
use anyhow::Result;
use azure_devops_rust_api::git;
use git::models::{GitPullRequestCreateOptions, WebApiCreateTagRequestData};
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    const USAGE: &str =
        "Usage: git_pr_create <repository-name> <src_branch> <target_branch> <title> <description>";

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let repo_name = env::args().nth(1).expect(USAGE);
    let src_branch: String = env::args().nth(2).expect(USAGE);
    let target_branch: String = env::args().nth(3).expect(USAGE);
    let title: String = env::args().nth(4).expect(USAGE);
    let description: String = env::args().nth(5).expect(USAGE);

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Create GitPullRequestCreateOptions with all the mandatory parameters
    println!("Create PR to merge {src_branch} => {target_branch}");
    let mut pr_create_options = GitPullRequestCreateOptions::new(
        // Need to specify full git refs path
        format!("refs/heads/{src_branch}"),
        format!("refs/heads/{target_branch}"),
        title,
    );

    // Set any additional optional parameters
    pr_create_options.description = Some(description);
    // Label creation is unfortunately currently not very ergonomic...
    pr_create_options.labels = vec![
        WebApiCreateTagRequestData::new("example_label1".to_string()),
        WebApiCreateTagRequestData::new("example_label2".to_string()),
    ];

    // Define the new PR
    let pr = git_client
        .pull_requests_client()
        .create(organization, repo_name, project, pr_create_options)
        .await?;
    println!("Created PR:\n{pr:#?}");

    Ok(())
}
