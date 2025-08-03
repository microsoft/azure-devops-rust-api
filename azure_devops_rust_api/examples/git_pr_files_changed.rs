// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_pr_files_changed.rs
// Getting all the files changed in a PR example.
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
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_pr_files_changed <repository-name> <pull_request_id>");
    let pull_request_id: i32 = env::args()
        .nth(2)
        .expect("Usage: git_pr_files_changed <repository-name> <pull_request_id>")
        .parse()
        .unwrap();
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Set the max number of commits to get, default is 100
    let top_commits: i32 = 500;

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get Commits for the PR
    let pr_commits = git_client
        .pull_request_commits_client()
        .get_pull_request_commits(&organization, &repository_name, pull_request_id, &project)
        .top(top_commits)
        .await?
        .value;

    // Record unique filenames which are changed in the PR
    let mut files_changed = HashSet::<String>::new();

    // Get each commit in the PR
    println!("\nCommits:");
    for commit in pr_commits.iter() {
        let commit_id = commit.commit_id.clone().unwrap_or("".to_string());
        let comment = match &commit.comment {
            Some(comment) => comment.clone(),
            _ => "".to_string(),
        };
        println!("{commit_id} {comment}");

        // Get the commit changes in a commit
        let pr_commits_changes = git_client
            .commits_client()
            .get_changes(&organization, commit_id, &repository_name, &project)
            .await?
            .changes;

        // Get files changed in the commit
        for change in pr_commits_changes.iter() {
            if let Some(item) = &change.change.item {
                // We are only interested in files not directories.
                // files are "blob" type, directories are "folder" type.
                if let (Some("blob"), Some(filename)) =
                    (item["git_object_type"].as_str(), item["path"].as_str())
                {
                    files_changed.insert(filename.to_string());
                }
            }
        }
    }
    println!("\nChanged files:");
    // Unique files changed in the PR
    for filename in files_changed.iter() {
        println!("{filename}")
    }

    Ok(())
}
