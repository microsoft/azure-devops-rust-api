// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents a session token used to access Azure DevOps resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionToken {
    #[serde(rename = "accessId", default, skip_serializing_if = "Option::is_none")]
    pub access_id: Option<String>,
    #[doc = "This is populated when user requests a compact token. The alternate token value is self describing token."]
    #[serde(
        rename = "alternateToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub alternate_token: Option<String>,
    #[serde(
        rename = "authorizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        rename = "hostAuthorizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_authorization_id: Option<String>,
    #[serde(rename = "isPublic", default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(
        rename = "publicData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(
        rename = "targetAccounts",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_accounts: Vec<String>,
    #[doc = "This is computed and not returned in Get queries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_from: Option<time::OffsetDateTime>,
    #[serde(
        rename = "validTo",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_to: Option<time::OffsetDateTime>,
}
impl SessionToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionTokenResult {
    #[serde(rename = "hasError", default, skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[doc = "Represents a session token used to access Azure DevOps resources"]
    #[serde(
        rename = "sessionToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_token: Option<SessionToken>,
    #[serde(
        rename = "sessionTokenError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_token_error: Option<session_token_result::SessionTokenError>,
}
impl SessionTokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod session_token_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionTokenError {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "displayNameRequired")]
        DisplayNameRequired,
        #[serde(rename = "invalidDisplayName")]
        InvalidDisplayName,
        #[serde(rename = "invalidValidTo")]
        InvalidValidTo,
        #[serde(rename = "invalidScope")]
        InvalidScope,
        #[serde(rename = "userIdRequired")]
        UserIdRequired,
        #[serde(rename = "invalidUserId")]
        InvalidUserId,
        #[serde(rename = "invalidUserType")]
        InvalidUserType,
        #[serde(rename = "accessDenied")]
        AccessDenied,
        #[serde(rename = "failedToIssueAccessToken")]
        FailedToIssueAccessToken,
        #[serde(rename = "invalidClient")]
        InvalidClient,
        #[serde(rename = "invalidClientType")]
        InvalidClientType,
        #[serde(rename = "invalidClientId")]
        InvalidClientId,
        #[serde(rename = "invalidTargetAccounts")]
        InvalidTargetAccounts,
        #[serde(rename = "hostAuthorizationNotFound")]
        HostAuthorizationNotFound,
        #[serde(rename = "authorizationNotFound")]
        AuthorizationNotFound,
        #[serde(rename = "failedToUpdateAccessToken")]
        FailedToUpdateAccessToken,
        #[serde(rename = "sourceNotSupported")]
        SourceNotSupported,
        #[serde(rename = "invalidSourceIP")]
        InvalidSourceIp,
        #[serde(rename = "invalidSource")]
        InvalidSource,
        #[serde(rename = "duplicateHash")]
        DuplicateHash,
        #[serde(rename = "sshPolicyDisabled")]
        SshPolicyDisabled,
        #[serde(rename = "invalidToken")]
        InvalidToken,
        #[serde(rename = "tokenNotFound")]
        TokenNotFound,
        #[serde(rename = "invalidAuthorizationId")]
        InvalidAuthorizationId,
        #[serde(rename = "failedToReadTenantPolicy")]
        FailedToReadTenantPolicy,
        #[serde(rename = "globalPatPolicyViolation")]
        GlobalPatPolicyViolation,
        #[serde(rename = "fullScopePatPolicyViolation")]
        FullScopePatPolicyViolation,
        #[serde(rename = "patLifespanPolicyViolation")]
        PatLifespanPolicyViolation,
        #[serde(rename = "invalidTokenType")]
        InvalidTokenType,
        #[serde(rename = "invalidAudience")]
        InvalidAudience,
        #[serde(rename = "invalidSubject")]
        InvalidSubject,
        #[serde(rename = "deploymentHostNotSupported")]
        DeploymentHostNotSupported,
    }
}
#[doc = "A paginated list of session tokens. Session tokens correspond to OAuth credentials such as personal access tokens (PATs) and other OAuth authorizations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenAdminPagedSessionTokens {
    #[doc = "The continuation token that can be used to retrieve the next page of session tokens, or <code>null</code> if there is no next page."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "The list of all session tokens in the current page."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SessionToken>,
}
impl TokenAdminPagedSessionTokens {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request to revoke a particular delegated authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenAdminRevocation {
    #[doc = "The authorization ID of the OAuth authorization to revoke."]
    #[serde(
        rename = "authorizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_id: Option<String>,
}
impl TokenAdminRevocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule which is applied to disable any incoming delegated authorization which matches the given properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenAdminRevocationRule {
    #[doc = "A datetime cutoff. Tokens created before this time will be rejected. This is an optional parameter. If omitted, defaults to the time at which the rule was created."]
    #[serde(
        rename = "createdBefore",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_before: Option<time::OffsetDateTime>,
    #[doc = "A string containing a space-delimited list of OAuth scopes. A token matching any one of the scopes will be rejected. For a list of all OAuth scopes supported by Azure DevOps, see:<https://docs>.microsoft.com/en-us/azure/devops/integrate/get-started/authentication/oauth?view=azure-devops#scopes This is a mandatory parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
}
impl TokenAdminRevocationRule {
    pub fn new() -> Self {
        Self::default()
    }
}
