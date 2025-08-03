// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// ims_query.rs
// Identity query example.
use anyhow::Result;
use azure_devops_rust_api::ims;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let user_email = env::args()
        .nth(1)
        .expect("Usage: identity_query <email_address>");

    // Create an "ims" client
    let ims_client = ims::ClientBuilder::new(credential).build();

    // Query the specified user
    let identities = ims_client
        .identities_client()
        .read_identities(organization)
        .search_filter("General")
        .filter_value(&user_email)
        .await?
        .value;

    println!("Found {} identities", identities.len());
    for identity in &identities {
        if let Some(id) = &identity.identity_base.id {
            println!("{id} {user_email}");
        }
    }

    if let Some(identity) = identities.first() {
        println!("{identity:#?}");
    }

    Ok(())
}
