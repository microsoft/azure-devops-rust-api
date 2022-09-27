// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// ims_query.rs
// Identity query example.
use anyhow::Result;
use azure_devops_rust_api::ims;
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
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential::new()))
        }
    };

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
        .into_future()
        .await?
        .value;

    println!("Found {} identities", identities.len());
    for identity in &identities {
        if let Some(id) = &identity.identity_base.id {
            println!("{id} {user_email}");
        }
    }

    if let Some(identity) = identities.iter().next() {
        println!("{:#?}", identity);
    }

    Ok(())
}
