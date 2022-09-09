// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_pr_files_changed.rs
// Getting all the file changed in a PR example.
use anyhow::Result;
use azure_devops_rust_api::git;
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli.
    let credential = match env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("INFO:Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("INFO:Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential {}))
        }
    };

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_pr_commits <repository-name> <pull_request_id>");
    let pull_request_id: i32 = env::args()
        .nth(2)
        .expect("Usage: git_pr_commits <repository-name> <pull_request_id>")
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
        .into_future()
        .await?.value;

    // Get each commit in the PR
    for commit in pr_commits.iter() {
        let pr_commit_id = &commit.commit_id;
        print!("Commit_Id  :{:?}\n", pr_commit_id);
        
        // Get the commit changes in a commit
        let pr_commits_changes = git_client
        .commits_client()
        .get_changes(&organization, pr_commit_id, &repository_name, &project)
        .into_future()
        .await?
        .changes;

        // Get file changes in the commit
        for change in pr_commits_changes.iter() {
            let file_change     = &change.change;
            let git_object_type = &file_change.item["gitObjectType"].as_str().unwrap();
            let change_type     = &file_change.change_type;
            let file_name       = &file_change.item["path"].as_str().unwrap();

            // Checking only the file name not folder,  file is blob type object and tree is folder type git object
            if git_object_type == &"blob" {
                println!(
                    "Change_Type: {:?}, File_Name: {:?}",
                    &change_type,
                    &file_name[1..]);
                
            }
        }
        println!("----------------------------------------------------------");
    }

    Ok(())
}
