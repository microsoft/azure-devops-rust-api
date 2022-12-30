use azure_devops_rust_api::Credential;
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use azure_identity::{AutoRefreshingTokenCredential, DefaultAzureCredential};
use std::sync::Arc;

pub fn get_credential() -> Credential {
    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli
    match std::env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using auto-refereshing DefaultAzureCredential");
            // Authenticate using one of:
            // - `EnvironmentCredential`
            // - `ManagedIdentityCredential`
            // - `AzureCliCredential`
            let default_azure_credential = Arc::new(DefaultAzureCredential::default());
            // Use the `AutoRefreshingTokenCredential` wrapper to cache the credentials,
            // refreshing when required.
            let auto_refreshing_credential =
                Arc::new(AutoRefreshingTokenCredential::new(default_azure_credential));
            Credential::from_token_credential(auto_refreshing_credential)
        }
    }
}
