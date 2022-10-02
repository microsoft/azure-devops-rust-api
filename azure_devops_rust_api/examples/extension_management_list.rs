// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// extension_management_list.rs
// extension_management_list example.
use anyhow::Result;
use azure_devops_rust_api::extension_management;
use azure_devops_rust_api::extension_management::models::InstalledExtension;
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

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");

    // Create a extension_management_client
    let extension_management_client = extension_management::ClientBuilder::new(credential).build();

    // Get all the installed extensions
    let installed_extensions = extension_management_client
        .installed_extensions_client()
        .list(organization)
        .into_future()
        .await?
        .value;

    println!("Installed extensions:");
    for extension in installed_extensions.iter() {
        match extension {
            InstalledExtension {
                extension_name: Some(name),
                publisher_name: Some(publisher),
                version: Some(version),
                ..
            } => {
                println!("{:65}{:24}{:40}", name, version, publisher);
            }
            _ => {}
        }
    }

    if let Some(extension) = installed_extensions.iter().next() {
        println!("\nExample extension:\n{:#?}", extension);
    }
    Ok(())
}
