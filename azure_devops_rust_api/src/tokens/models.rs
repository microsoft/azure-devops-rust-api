// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessTokenResult {
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<JsonWebToken>,
    #[serde(
        rename = "accessTokenError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token_error: Option<access_token_result::AccessTokenError>,
    #[serde(
        rename = "authorizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_id: Option<String>,
    #[serde(
        rename = "errorDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_description: Option<String>,
    #[serde(rename = "hasError", default, skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(
        rename = "isFirstPartyClient",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_first_party_client: Option<bool>,
    #[serde(
        rename = "refreshToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<RefreshTokenGrant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(
        rename = "validTo",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_to: Option<time::OffsetDateTime>,
}
impl AccessTokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_token_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTokenError {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "grantTypeRequired")]
        GrantTypeRequired,
        #[serde(rename = "authorizationGrantRequired")]
        AuthorizationGrantRequired,
        #[serde(rename = "clientSecretRequired")]
        ClientSecretRequired,
        #[serde(rename = "redirectUriRequired")]
        RedirectUriRequired,
        #[serde(rename = "invalidAuthorizationGrant")]
        InvalidAuthorizationGrant,
        #[serde(rename = "invalidAuthorizationScopes")]
        InvalidAuthorizationScopes,
        #[serde(rename = "invalidRefreshToken")]
        InvalidRefreshToken,
        #[serde(rename = "authorizationNotFound")]
        AuthorizationNotFound,
        #[serde(rename = "authorizationGrantExpired")]
        AuthorizationGrantExpired,
        #[serde(rename = "accessAlreadyIssued")]
        AccessAlreadyIssued,
        #[serde(rename = "invalidRedirectUri")]
        InvalidRedirectUri,
        #[serde(rename = "accessTokenNotFound")]
        AccessTokenNotFound,
        #[serde(rename = "invalidAccessToken")]
        InvalidAccessToken,
        #[serde(rename = "accessTokenAlreadyRefreshed")]
        AccessTokenAlreadyRefreshed,
        #[serde(rename = "invalidClientSecret")]
        InvalidClientSecret,
        #[serde(rename = "clientSecretExpired")]
        ClientSecretExpired,
        #[serde(rename = "serverError")]
        ServerError,
        #[serde(rename = "accessDenied")]
        AccessDenied,
        #[serde(rename = "accessTokenKeyRequired")]
        AccessTokenKeyRequired,
        #[serde(rename = "invalidAccessTokenKey")]
        InvalidAccessTokenKey,
        #[serde(rename = "failedToGetAccessToken")]
        FailedToGetAccessToken,
        #[serde(rename = "invalidClientId")]
        InvalidClientId,
        #[serde(rename = "invalidClient")]
        InvalidClient,
        #[serde(rename = "invalidValidTo")]
        InvalidValidTo,
        #[serde(rename = "invalidUserId")]
        InvalidUserId,
        #[serde(rename = "failedToIssueAccessToken")]
        FailedToIssueAccessToken,
        #[serde(rename = "authorizationGrantScopeMissing")]
        AuthorizationGrantScopeMissing,
        #[serde(rename = "invalidPublicAccessTokenKey")]
        InvalidPublicAccessTokenKey,
        #[serde(rename = "invalidPublicAccessToken")]
        InvalidPublicAccessToken,
        #[serde(rename = "publicFeatureFlagNotEnabled")]
        PublicFeatureFlagNotEnabled,
        #[serde(rename = "sshPolicyDisabled")]
        SshPolicyDisabled,
        #[serde(rename = "hostAuthorizationNotFound")]
        HostAuthorizationNotFound,
        #[serde(rename = "hostAuthorizationIsNotValid")]
        HostAuthorizationIsNotValid,
        #[serde(rename = "invalidScope")]
        InvalidScope,
    }
    impl std::fmt::Display for AccessTokenError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::None => write!(f, "none"),
                Self::GrantTypeRequired => write!(f, "grantTypeRequired"),
                Self::AuthorizationGrantRequired => write!(f, "authorizationGrantRequired"),
                Self::ClientSecretRequired => write!(f, "clientSecretRequired"),
                Self::RedirectUriRequired => write!(f, "redirectUriRequired"),
                Self::InvalidAuthorizationGrant => write!(f, "invalidAuthorizationGrant"),
                Self::InvalidAuthorizationScopes => write!(f, "invalidAuthorizationScopes"),
                Self::InvalidRefreshToken => write!(f, "invalidRefreshToken"),
                Self::AuthorizationNotFound => write!(f, "authorizationNotFound"),
                Self::AuthorizationGrantExpired => write!(f, "authorizationGrantExpired"),
                Self::AccessAlreadyIssued => write!(f, "accessAlreadyIssued"),
                Self::InvalidRedirectUri => write!(f, "invalidRedirectUri"),
                Self::AccessTokenNotFound => write!(f, "accessTokenNotFound"),
                Self::InvalidAccessToken => write!(f, "invalidAccessToken"),
                Self::AccessTokenAlreadyRefreshed => write!(f, "accessTokenAlreadyRefreshed"),
                Self::InvalidClientSecret => write!(f, "invalidClientSecret"),
                Self::ClientSecretExpired => write!(f, "clientSecretExpired"),
                Self::ServerError => write!(f, "serverError"),
                Self::AccessDenied => write!(f, "accessDenied"),
                Self::AccessTokenKeyRequired => write!(f, "accessTokenKeyRequired"),
                Self::InvalidAccessTokenKey => write!(f, "invalidAccessTokenKey"),
                Self::FailedToGetAccessToken => write!(f, "failedToGetAccessToken"),
                Self::InvalidClientId => write!(f, "invalidClientId"),
                Self::InvalidClient => write!(f, "invalidClient"),
                Self::InvalidValidTo => write!(f, "invalidValidTo"),
                Self::InvalidUserId => write!(f, "invalidUserId"),
                Self::FailedToIssueAccessToken => write!(f, "failedToIssueAccessToken"),
                Self::AuthorizationGrantScopeMissing => write!(f, "authorizationGrantScopeMissing"),
                Self::InvalidPublicAccessTokenKey => write!(f, "invalidPublicAccessTokenKey"),
                Self::InvalidPublicAccessToken => write!(f, "invalidPublicAccessToken"),
                Self::PublicFeatureFlagNotEnabled => write!(f, "publicFeatureFlagNotEnabled"),
                Self::SshPolicyDisabled => write!(f, "sshPolicyDisabled"),
                Self::HostAuthorizationNotFound => write!(f, "hostAuthorizationNotFound"),
                Self::HostAuthorizationIsNotValid => write!(f, "hostAuthorizationIsNotValid"),
                Self::InvalidScope => write!(f, "invalidScope"),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationGrant {
    #[serde(rename = "grantType", default, skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<authorization_grant::GrantType>,
}
impl AuthorizationGrant {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod authorization_grant {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GrantType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "jwtBearer")]
        JwtBearer,
        #[serde(rename = "refreshToken")]
        RefreshToken,
        #[serde(rename = "implicit")]
        Implicit,
        #[serde(rename = "clientCredentials")]
        ClientCredentials,
    }
    impl std::fmt::Display for GrantType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::None => write!(f, "none"),
                Self::JwtBearer => write!(f, "jwtBearer"),
                Self::RefreshToken => write!(f, "refreshToken"),
                Self::Implicit => write!(f, "implicit"),
                Self::ClientCredentials => write!(f, "clientCredentials"),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssuedToken {
    #[serde(
        rename = "isAuthenticated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_authenticated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl IssuedToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonWebToken {
    #[serde(flatten)]
    pub issued_token: IssuedToken,
}
impl JsonWebToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returned by the List method; contains a list of personal access tokens (PATs) and the continuation token to get the next page of results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedPatTokens {
    #[doc = "Used to access the next page of results in successive API calls to list personal access tokens (PATs)"]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "The list of personal access tokens (PATs)"]
    #[serde(
        rename = "patTokens",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pat_tokens: Vec<PatToken>,
}
impl PagedPatTokens {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a personal access token (PAT) used to access Azure DevOps resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatToken {
    #[doc = "Unique guid identifier"]
    #[serde(
        rename = "authorizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_id: Option<String>,
    #[doc = "The token name"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "The token scopes for accessing Azure DevOps resources"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The organizations for which the token is valid; null if the token applies to all of the user's accessible organizations"]
    #[serde(
        rename = "targetAccounts",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_accounts: Vec<String>,
    #[doc = "The unique token string generated at creation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "The token creation date"]
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_from: Option<time::OffsetDateTime>,
    #[doc = "The token expiration date"]
    #[serde(
        rename = "validTo",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_to: Option<time::OffsetDateTime>,
}
impl PatToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encapsulates the request parameters for creating a new personal access token (PAT)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatTokenCreateRequest {
    #[doc = "True, if this personal access token (PAT) is for all of the user's accessible organizations. False, if otherwise (e.g. if the token is for a specific organization)"]
    #[serde(rename = "allOrgs", default, skip_serializing_if = "Option::is_none")]
    pub all_orgs: Option<bool>,
    #[doc = "The token name"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "The token scopes for accessing Azure DevOps resources"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The token expiration date"]
    #[serde(
        rename = "validTo",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_to: Option<time::OffsetDateTime>,
}
impl PatTokenCreateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the resulting personal access token (PAT) and the error (if any) that occurred during the operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatTokenResult {
    #[doc = "Represents a personal access token (PAT) used to access Azure DevOps resources"]
    #[serde(rename = "patToken", default, skip_serializing_if = "Option::is_none")]
    pub pat_token: Option<PatToken>,
    #[doc = "The error (if any) that occurred"]
    #[serde(
        rename = "patTokenError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pat_token_error: Option<pat_token_result::PatTokenError>,
}
impl PatTokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pat_token_result {
    use super::*;
    #[doc = "The error (if any) that occurred"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PatTokenError {
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
    impl std::fmt::Display for PatTokenError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::None => write!(f, "none"),
                Self::DisplayNameRequired => write!(f, "displayNameRequired"),
                Self::InvalidDisplayName => write!(f, "invalidDisplayName"),
                Self::InvalidValidTo => write!(f, "invalidValidTo"),
                Self::InvalidScope => write!(f, "invalidScope"),
                Self::UserIdRequired => write!(f, "userIdRequired"),
                Self::InvalidUserId => write!(f, "invalidUserId"),
                Self::InvalidUserType => write!(f, "invalidUserType"),
                Self::AccessDenied => write!(f, "accessDenied"),
                Self::FailedToIssueAccessToken => write!(f, "failedToIssueAccessToken"),
                Self::InvalidClient => write!(f, "invalidClient"),
                Self::InvalidClientType => write!(f, "invalidClientType"),
                Self::InvalidClientId => write!(f, "invalidClientId"),
                Self::InvalidTargetAccounts => write!(f, "invalidTargetAccounts"),
                Self::HostAuthorizationNotFound => write!(f, "hostAuthorizationNotFound"),
                Self::AuthorizationNotFound => write!(f, "authorizationNotFound"),
                Self::FailedToUpdateAccessToken => write!(f, "failedToUpdateAccessToken"),
                Self::SourceNotSupported => write!(f, "sourceNotSupported"),
                Self::InvalidSourceIp => write!(f, "invalidSourceIP"),
                Self::InvalidSource => write!(f, "invalidSource"),
                Self::DuplicateHash => write!(f, "duplicateHash"),
                Self::SshPolicyDisabled => write!(f, "sshPolicyDisabled"),
                Self::InvalidToken => write!(f, "invalidToken"),
                Self::TokenNotFound => write!(f, "tokenNotFound"),
                Self::InvalidAuthorizationId => write!(f, "invalidAuthorizationId"),
                Self::FailedToReadTenantPolicy => write!(f, "failedToReadTenantPolicy"),
                Self::GlobalPatPolicyViolation => write!(f, "globalPatPolicyViolation"),
                Self::FullScopePatPolicyViolation => write!(f, "fullScopePatPolicyViolation"),
                Self::PatLifespanPolicyViolation => write!(f, "patLifespanPolicyViolation"),
                Self::InvalidTokenType => write!(f, "invalidTokenType"),
                Self::InvalidAudience => write!(f, "invalidAudience"),
                Self::InvalidSubject => write!(f, "invalidSubject"),
                Self::DeploymentHostNotSupported => write!(f, "deploymentHostNotSupported"),
            }
        }
    }
}
#[doc = "Encapsulates the request parameters for updating a personal access token (PAT)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatTokenUpdateRequest {
    #[doc = "(Optional) True if this personal access token (PAT) is for all of the user's accessible organizations. False if otherwise (e.g. if the token is for a specific organization)"]
    #[serde(rename = "allOrgs", default, skip_serializing_if = "Option::is_none")]
    pub all_orgs: Option<bool>,
    #[doc = "The authorizationId identifying a single, unique personal access token (PAT)"]
    #[serde(
        rename = "authorizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_id: Option<String>,
    #[doc = "(Optional) The token name"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "(Optional) The token scopes for accessing Azure DevOps resources"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "(Optional) The token expiration date"]
    #[serde(
        rename = "validTo",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_to: Option<time::OffsetDateTime>,
}
impl PatTokenUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshTokenGrant {
    #[serde(flatten)]
    pub authorization_grant: AuthorizationGrant,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwt: Option<JsonWebToken>,
}
impl RefreshTokenGrant {
    pub fn new() -> Self {
        Self::default()
    }
}
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
