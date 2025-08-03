// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// get_push.rs
// Pushing a commit to a repo.
use anyhow::{Context, Result};
use azure_devops_rust_api::git;
use azure_devops_rust_api::git::models::{
    change::ChangeType, item_content::ContentType, Change, GitChange, GitCommitRef, GitPush,
    GitRefUpdate, ItemContent,
};
use serde_json::json;
use std::{env, vec};

mod utils;

// Get the latest commit on specified branch of the given repository
async fn get_latest_commit_id(
    client: &git::Client,
    organization: &str,
    repository_name: &str,
    project: &str,
    branch: &str,
) -> Result<String> {
    let latest_commit = client
        .commits_client()
        .get_commits(organization, repository_name, project)
        .search_criteria_item_version_version(branch)
        .search_criteria_top(1)
        .await?
        .value;

    let latest_commit_id = latest_commit
        .first()
        .context("No commits found")?
        .commit_id
        .as_ref()
        .context("Missing commit_id")?
        .to_string();

    Ok(latest_commit_id)
}

// Create a new `GitChange` with the specified content
fn new_git_change_with_content(
    change_type: ChangeType,
    filename: &str,
    file_content: impl Into<String>,
) -> GitChange {
    let file_content = file_content.into();
    GitChange {
        change: Change {
            change_type,
            item: Some(json!({
                "path": filename
            })),
            new_content: Some(ItemContent {
                content: file_content,
                content_type: ContentType::RawText,
            }),
            source_server_item: None,
            url: None,
        },
        change_id: None,
        new_content_template: None,
        original_path: None,
    }
}

// Create a new `GitRefUpdate` with the specified base commit id and target branch
fn new_git_ref_update(base_commit_id: String, target_branch: &str) -> GitRefUpdate {
    GitRefUpdate {
        name: Some(format!("refs/heads/{target_branch}")),
        old_object_id: Some(base_commit_id),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_push <repository-name> <source-branch> <target-branch> <filename> <file-content>");
    let source_branch = env::args()
        .nth(2)
        .expect("Usage: git_push <repository-name> <source-branch> <target-branch> <filename> <file-content>");
    let target_branch = env::args()
        .nth(3)
        .expect("Usage: git_push <repository-name> <source-branch> <target-branch> <filename> <file-content>");
    let filename = env::args()
        .nth(4)
        .expect("Usage: git_push <repository-name> <source-branch> <target-branch> <filename> <file-content>");
    let file_content = env::args()
        .nth(5)
        .expect("Usage: git_push <repository-name> <source-branch> <target-branch> <filename> <file-content>");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get the latest commit id on the specified source branch from the specified repository
    let base_commit_id = get_latest_commit_id(
        &git_client,
        &organization,
        &repository_name,
        &project,
        &source_branch,
    )
    .await?;
    println!("base_commit_id: {base_commit_id}");

    // Define the base commit id and the target branch
    let ref_update = new_git_ref_update(base_commit_id, &target_branch);

    // Define the change to be pushed.
    // In this case, we are replacing the content of the specified file with the provided content.
    let change = new_git_change_with_content(ChangeType::Edit, &filename, &file_content);

    // Define the commits to be pushed.
    // In this case, we are adding a single commit with the specified change.
    let commits = vec![GitCommitRef {
        comment: Some(String::from("This is a commit via the Azure DevOps API...")),
        changes: vec![change],
        ..Default::default()
    }];

    // Define the push request.
    // In this case, we are pushing the commit to a new branch.
    let push_request = GitPush {
        commits,
        ref_updates: vec![ref_update],
        ..Default::default()
    };

    // Execute the push request.
    let push_response = git_client
        .pushes_client()
        .create(&organization, push_request, &repository_name, &project)
        .await?;
    println!("Pushed commit.\npush_response: {push_response:#?}");

    Ok(())
}
