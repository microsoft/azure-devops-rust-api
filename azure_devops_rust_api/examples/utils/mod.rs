// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{Context, Policy, PolicyResult, Request};
use azure_devops_rust_api::Credential;
use azure_identity::DefaultAzureCredentialBuilder;
use std::sync::Arc;

fn authenticate_with_default_credential() -> Credential {
    println!("Authenticate using auto-refreshing DefaultAzureCredential");
    // `DefaultAzureCredential` can authenticate using one of:
    // - `EnvironmentCredential`
    // - `ManagedIdentityCredential`
    // - `AzureCliCredential`
    // For examples we just want to use AzureCliCredential, so exclude the
    // other mechanisms.
    // It would be simpler to directly create `AzureCliCredential` here, but I want to
    // demonstrate use of `DefaultAzureCredentialBuilder`.
    let default_azure_credential = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_environment_credential()
            .exclude_managed_identity_credential()
            .build(),
    );

    Credential::from_token_credential(default_azure_credential)
}

#[allow(dead_code)]
pub fn get_credential() -> Credential {
    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli
    match std::env::var("ADO_TOKEN") {
        Ok(token) if !token.is_empty() => {
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        _ => authenticate_with_default_credential(),
    }
}

// Define a simple policy to always set the `accept` header to `application/zip`
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AcceptZipPolicy {}

impl AcceptZipPolicy {
    #[allow(dead_code)]
    pub fn new_policy() -> Arc<dyn Policy> {
        Arc::new(AcceptZipPolicy {})
    }
}

/// Always set the `accept` header to `application/zip`
#[async_trait]
impl azure_core::Policy for AcceptZipPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        request.insert_header("accept", "application/zip");
        next[0].send(ctx, request, &next[1..]).await
    }
}
