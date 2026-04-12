// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_diff_files_between_base_and_target_branch.rs
// Getting files modified in the branch
use anyhow::Result;
use azure_devops_rust_api::git;
use std::collections::HashSet;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Getting repo, base and target branch name from command line
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_diff_files_between_base_and_target_branch <repository-name> <base-branch-name> <target-branch-name>");

    let base_branch_name = env::args()
        .nth(2)
        .expect("Usage: git_diff_files_between_base_and_target_branch <repository-name> <base-branch-name> <target-branch-name>");

    let target_branch_name = env::args()
        .nth(3)
        .expect("Usage: git_diff_files_between_base_and_target_branch <repository-name> <base-branch-name> <target-branch-name>");

    // Set the max number of commits to get, default is 100
    let top_commits: i32 = 500;

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get diff client between branches
    let diffs = git_client
        .diffs_client()
        .get(organization, repository_name, project)
        .top(top_commits)
        .base_version(base_branch_name)
        .target_version(target_branch_name)
        .await?
        .changes;

    // Record unique filenames which are changed in the PR
    let mut files_diff_between_branches = HashSet::<String>::new();

    // Get files name which are present in the target branch
    for diff in diffs.iter() {
        if let Some(item) = &diff.change.item {
            let git_object_type = item["gitObjectType"].as_str().unwrap();
            if git_object_type == "blob" {
                let file_name = item["path"].as_str().unwrap();
                files_diff_between_branches.insert(file_name.to_string());
            }
        }
    }

    // Unique files changed in the PR
    println!("These files are modified in pr(target) branch:");
    for file_name in files_diff_between_branches.iter() {
        println!("{file_name}")
    }

    Ok(())
}
