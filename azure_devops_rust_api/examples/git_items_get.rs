// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// git_items_get.rs
// Git items (files and folders) get example.
use anyhow::Context as AnyhowContext;
use anyhow::Result;
use azure_devops_rust_api::git;
use futures::StreamExt;
use std::env;
use std::io::Write;

mod utils;
use utils::AcceptZipPolicy;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repository_name = env::args()
        .nth(1)
        .expect("Usage: git_items_get <repository-name> <file_path>");
    let file_path = env::args()
        .nth(2)
        .expect("Usage: git_items_get <repository-name> <file_path>");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential.clone()).build();

    // To get the file metadata, it appears that you need to specify format as "json"
    let item = git_client
        .items_client()
        .get(&organization, &repository_name, &file_path, &project)
        .format("json")
        .await?;

    println!("\n{file_path} metadata:\n{item:#?}");

    // If no format is specified, the file contents are returned
    let rsp = git_client
        .items_client()
        .get(&organization, &repository_name, &file_path, &project)
        .send()
        .await?
        .into_raw_response();

    let file_data = rsp.into_body().collect_string().await?;
    println!("\n{file_path} contents:\n{file_data}");

    // Download the entire repo as a zip archive.

    // It is possible to download files as a zip archive by setting the
    // `accept` header to `application/zip`. The API doesn't directly
    // support this, but we can work around this by creating a new client
    // with a policy to set the `accept` header.
    let git_zip_client = git::ClientBuilder::new(credential)
        .per_call_policies(vec![AcceptZipPolicy::new_policy()])
        .build();

    // If the accept header specifies "application/zip", the files are returned in a zip archive.
    // If the `file_path` parameter is empty, the entire repository is returned.
    let rsp = git_zip_client
        .items_client()
        .get(&organization, &repository_name, "", &project)
        .download(true)
        .send()
        .await?
        .into_raw_response();

    let mut body = rsp.into_body();
    let mut file = std::fs::File::create("full_repo.zip")?;
    while let Some(chunk_result) = body.next().await {
        let chunk = chunk_result.context("Error reading chunk")?;
        println!("Writing chunk");
        file.write_all(&chunk)?;
    }
    println!("Done writing full_repo.zip");

    Ok(())
}
