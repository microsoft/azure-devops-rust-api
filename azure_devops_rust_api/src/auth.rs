// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Azure DevOps authentication support.
//!
//! For more background information on Azure DevOps authentication see: [Azure DevOps authentication](https://docs.microsoft.com/en-us/azure/devops/integrate/get-started/authentication/authentication-guidance)

use azure_core::credentials::TokenCredential;
use azure_core::error::{Result, ResultExt};
use base64::{prelude::BASE64_STANDARD, Engine};
use std::sync::Arc;

/// A credential for authenticating with Azure DevOps.
///
/// Supports:
/// - [Azure DevOps Personal Access Token (PAT)](https://docs.microsoft.com/en-us/azure/devops/organizations/accounts/use-personal-access-tokens-to-authenticate).
/// - OAuth token credential obtained via the [`azure_identity`](https://crates.io/crates/azure_identity) crate.
#[derive(Clone)]
pub enum Credential {
    Unauthenticated,
    Pat(String),
    TokenCredential(Arc<dyn TokenCredential>),
}

impl Credential {
    /// Creates a new `Credential` for unauthenticated operations.
    pub fn unauthenticated() -> Self {
        Credential::Unauthenticated
    }

    /// Creates a new `Credential` using the supplied PAT token.
    pub fn from_pat(pat: impl Into<String>) -> Self {
        let pat = pat.into();
        Credential::Pat(pat)
    }

    /// Creates a new `Credential` using the supplied object that implements [`TokenCredential`](https://docs.rs/azure_core/latest/azure_core/auth/trait.TokenCredential.html).
    ///
    /// Note that the supplied object must be wrapped in an `Arc<...>`.
    pub fn from_token_credential<T>(token_credential: Arc<T>) -> Self
    where
        T: TokenCredential + 'static,
    {
        let token_credential = token_credential as Arc<dyn TokenCredential>;
        Credential::TokenCredential(token_credential)
    }

    /// Returns the HTTP authorization header value containing the credential.
    #[allow(dead_code)]
    pub(crate) async fn http_authorization_header(
        &self,
        scopes: &[&str],
    ) -> Result<Option<String>> {
        match self {
            Credential::Unauthenticated => Ok(None),
            // PAT tokens are passed using Basic authentication.
            Credential::Pat(pat) => Ok(Some(format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!(":{}", &pat))
            ))),
            // OAuth tokens are passed using Bearer authentication.
            Credential::TokenCredential(token_credential) => {
                let token_response = token_credential
                    .get_token(scopes, None)
                    .await
                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                Ok(Some(format!("Bearer {}", token_response.token.secret())))
            }
        }
    }
}
