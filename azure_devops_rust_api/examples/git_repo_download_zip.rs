// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_items_list.rs
// Git items (files and folders) list example.
use anyhow::Context as AnyhowContext;
use anyhow::Result;
use azure_devops_rust_api::git;
use azure_devops_rust_api::git::models::git_item::GitObjectType;
use azure_devops_rust_api::git::models::GitItem;
use futures::StreamExt;
use std::env;
use std::io::Write;

mod utils;
use utils::AcceptZipPolicy;

/// Get all items (files and folders) in the specified repository
async fn all_repo_items(
    git_client: &git::Client,
    organization: &str,
    repository_name: &str,
    project: &str,
) -> Result<Vec<GitItem>> {
    let items = git_client
        .items_client()
        .list(organization, repository_name, project)
        .recursion_level("Full")
        .await?
        .value;
    Ok(items)
}

/// Get the object ids of all blobs in the specified git items
fn repo_blob_ids(items: &[GitItem]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.git_object_type == Some(GitObjectType::Blob))
        .filter_map(|item| item.object_id.clone())
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_items_list <repository-name>");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential.clone()).build();

    // Get all items (files and folders) in the specified repository
    let items = all_repo_items(&git_client, &organization, &repository_name, &project).await?;
    let blob_ids = repo_blob_ids(&items);

    for blob_id in blob_ids.iter() {
        println!("{blob_id}");
    }
    println!("{} blobs found", blob_ids.len());

    // Download all the repo blobs as a zip archive.
    //
    // Note that this is not the same as downloading the entire repo as a zip archive.
    // The repo zip archive includes the repo metadata, whereas this only includes the blobs.
    //
    // This is useful if you want to download the blobs for a subset of the repo.
    // For example, if you want to download the blobs for a specific branch, you can
    // first get the branch name, then get the commit id for the branch, then get the
    // tree id for the commit, then get the items for the tree, then get the blob ids
    // for the items, then download the blobs.
    //
    // NOTE: The filenames in the zip archive are the blob ids.

    // The current version of the API doesn't set the `accept` header correctly for `get_blobs_zip()`.
    // Work around this by adding a custom policy to set the header.
    let git_zip_client = git::ClientBuilder::new(credential)
        .per_call_policies(vec![AcceptZipPolicy::new_policy()])
        .build();

    let mut body = git_zip_client
        .blobs_client()
        .get_blobs_zip(organization, blob_ids, repository_name, project)
        .send()
        .await?
        .into_raw_response()
        .into_body();

    let mut file = std::fs::File::create("blobs.zip")?;
    while let Some(chunk_result) = body.next().await {
        let chunk = chunk_result.context("Error reading chunk")?;
        println!("Writing chunk");
        file.write_all(&chunk)?;
    }
    println!("Done writing blobs.zip");

    Ok(())
}
