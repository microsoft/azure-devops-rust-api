// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// wiki_pages_create_or_update.rs
// Wiki page creation/update example.
use anyhow::Result;
use azure_devops_rust_api::wiki::{self, pages};
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    // Get wiki params based on arguments
    let wiki_id = env::args()
        .nth(1)
        .expect("Usage: Wiki ID to create/update a page under <wiki-id>");
    let wiki_page_path = env::args()
        .nth(2)
        .expect("Usage: Wiki page path to create/update <wiki-page-path>");
    let wiki_content = env::args()
        .nth(3)
        .expect("Usage: Wiki content to be inserted into the page <wiki-content>");

    // Create a wiki pages client
    let wiki_pages_client = wiki::ClientBuilder::new(credential).build().pages_client();

    // To update an existing wiki page the page version, called an eTag, must be supplied in an `If-Match` header. NB: the RequestBuilder will insert this header for you when you call it with the `eTag`.
    // This function call returns `Some(String)` containing the eTag if the pages exists, otherwise
    // it will return `None` indicating that the page needs to be created.
    let op_etag: Option<String> = get_wiki_page_etag(
        &wiki_pages_client,
        &wiki_page_path,
        &organization,
        &project,
        &wiki_id,
    )
    .await;

    // The content to be displayed on the page
    let wiki_body = wiki::models::WikiPageCreateOrUpdateParameters {
        content: Some(wiki_content),
    };

    // Based on whether the page exists either update or create
    match op_etag {
        Some(etag) => {
            println!("Updating wiki page...");
            match wiki_pages_client
                .create_or_update(
                    organization,
                    wiki_body,
                    project,
                    wiki_id,
                    wiki_page_path,
                    etag,
                )
                .await
            {
                Ok(p) => println!("Page updated: {:?}", p.remote_url),
                Err(e) => panic!("Failed to update wiki page: {e}"),
            }
        }
        None => {
            println!("Creating wiki page...");
            match wiki_pages_client
                .create_or_update(
                    organization,
                    wiki_body,
                    project,
                    wiki_id,
                    wiki_page_path,
                    "a123", // fake version value, unused in creation operation
                )
                .await
            {
                Ok(p) => println!("Page created: {:?}", p.remote_url),
                Err(e) => panic!("Failed to create wiki page: {e}"),
            }
        }
    }

    Ok(())
}

/// Using a [pages::Client] attempt to retrieve the version (`eTag`) of a wiki page at `page_path`.
/// The `eTag` is required when updating an existing wiki page.
///
/// If the page does not exist [None] is returned and in such cases the page needs to be created
async fn get_wiki_page_etag(
    client: &pages::Client,
    page_path: &String,
    organisation: &String,
    project: &String,
    wiki_id: &String,
) -> Option<String> {
    match client
        .get_page(organisation, project, wiki_id)
        .path(page_path)
        .send()
        .await
    {
        Ok(r) => {
            let etag = r.headers().e_tag().unwrap().to_owned();
            println!("Etag for page {page_path}, {etag}");
            Some(etag)
        }
        Err(e) => {
            // If the response is a 404 then we need to create the page
            if e.http_status()
                .expect("Failed to retrieve HTTP status")
                .canonical_reason()
                == "Not Found"
            {
                println!("Wiki page does not exist");
                None
            } else {
                panic!("Failed to retrieve etag")
            }
        }
    }
}
