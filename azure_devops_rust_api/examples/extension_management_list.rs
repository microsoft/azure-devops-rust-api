// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// extension_management_list.rs
// extension_management_list example.
use anyhow::Result;
use azure_devops_rust_api::extension_management;
use azure_devops_rust_api::extension_management::models::InstalledExtension;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential
    let credential = utils::get_credential()?;

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");

    // Create a extension_management_client
    let extension_management_client = extension_management::ClientBuilder::new(credential).build();

    // Get all the installed extensions
    let installed_extensions = extension_management_client
        .installed_extensions_client()
        .list(organization)
        .await?
        .value;

    println!("Installed extensions:");
    for extension in installed_extensions.iter() {
        if let InstalledExtension {
            extension_name: Some(name),
            publisher_name: Some(publisher),
            version: Some(version),
            ..
        } = extension
        {
            println!("{name:65}{version:24}{publisher:40}");
        }
    }

    if let Some(extension) = installed_extensions.first() {
        println!("\nExample extension:\n{extension:#?}");
    }

    Ok(())
}
