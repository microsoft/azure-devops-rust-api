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
}
#[doc = "Container class for changed identities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangedIdentities {
    #[doc = "Changed Identities"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identities: Vec<Identity>,
    #[doc = "More data available, set to true if pagesize is specified."]
    #[serde(rename = "moreData", default, skip_serializing_if = "Option::is_none")]
    pub more_data: Option<bool>,
    #[doc = "Context class for changed identities"]
    #[serde(
        rename = "sequenceContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence_context: Option<ChangedIdentitiesContext>,
}
impl ChangedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Context class for changed identities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangedIdentitiesContext {
    #[doc = "Last Group SequenceId"]
    #[serde(
        rename = "groupSequenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_sequence_id: Option<i32>,
    #[doc = "Last Identity SequenceId"]
    #[serde(
        rename = "identitySequenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_sequence_id: Option<i32>,
    #[doc = "Last Group OrganizationIdentitySequenceId"]
    #[serde(
        rename = "organizationIdentitySequenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_identity_sequence_id: Option<i32>,
    #[doc = "Page size"]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
impl ChangedIdentitiesContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateScopeInfo {
    #[serde(
        rename = "adminGroupDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_group_description: Option<String>,
    #[serde(
        rename = "adminGroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_group_name: Option<String>,
    #[serde(rename = "creatorId", default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(
        rename = "parentScopeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_scope_id: Option<String>,
    #[serde(rename = "scopeName", default, skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<create_scope_info::ScopeType>,
}
impl CreateScopeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod create_scope_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "serviceHost")]
        ServiceHost,
        #[serde(rename = "teamProject")]
        TeamProject,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrameworkIdentityInfo {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(
        rename = "identityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_type: Option<framework_identity_info::IdentityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl FrameworkIdentityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod framework_identity_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IdentityType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "serviceIdentity")]
        ServiceIdentity,
        #[serde(rename = "aggregateIdentity")]
        AggregateIdentity,
        #[serde(rename = "importedIdentity")]
        ImportedIdentity,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupMembership {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "queriedId", default, skip_serializing_if = "Option::is_none")]
    pub queried_id: Option<String>,
}
impl GroupMembership {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[serde(flatten)]
    pub identity_base: IdentityBase,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base Identity class to allow \"trimmed\" identity class in the GetConnectionData API Makes sure that on-the-wire representations of the derived classes are compatible with each other (e.g. Server responds with PublicIdentity object while client deserializes it as Identity object) Derived classes should not have additional \\[DataMember\\] properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityBase {
    #[doc = "The custom display name for the identity (if any). Setting this property to an empty string will clear the existing custom display name. Setting this property to null will not affect the existing persisted value (since null values do not get sent over the wire or to the database)"]
    #[serde(
        rename = "customDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_display_name: Option<String>,
    #[doc = "Identity descriptor"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "Identity Identifier. Also called Storage Key, or VSID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "True if the identity has a membership in any Azure Devops group in the organization."]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "True if the identity is a group."]
    #[serde(
        rename = "isContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_container: Option<bool>,
    #[serde(rename = "masterId", default, skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    #[doc = "Id of the members of the identity (groups only)."]
    #[serde(
        rename = "memberIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub member_ids: Vec<String>,
    #[serde(
        rename = "memberOf",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub member_of: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub members: Vec<String>,
    #[serde(
        rename = "metaTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub meta_type_id: Option<i32>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "The display name for the identity as specified by the source identity provider."]
    #[serde(
        rename = "providerDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_display_name: Option<String>,
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<i32>,
    #[serde(
        rename = "socialDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub social_descriptor: Option<String>,
    #[doc = "Subject descriptor of a Graph entity."]
    #[serde(
        rename = "subjectDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_descriptor: Option<String>,
    #[serde(
        rename = "uniqueUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_user_id: Option<i32>,
}
impl IdentityBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityBatchInfo {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub descriptors: Vec<String>,
    #[serde(
        rename = "identityIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identity_ids: Vec<String>,
    #[serde(
        rename = "includeRestrictedVisibility",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_restricted_visibility: Option<bool>,
    #[serde(
        rename = "propertyNames",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_names: Vec<String>,
    #[serde(
        rename = "queryMembership",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_membership: Option<identity_batch_info::QueryMembership>,
    #[serde(
        rename = "socialDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub social_descriptors: Vec<String>,
    #[serde(
        rename = "subjectDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subject_descriptors: Vec<String>,
}
impl IdentityBatchInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_batch_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryMembership {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "expanded")]
        Expanded,
        #[serde(rename = "expandedUp")]
        ExpandedUp,
        #[serde(rename = "expandedDown")]
        ExpandedDown,
    }
}
#[doc = "An Identity descriptor is a wrapper for the identity type (Windows SID, Passport) along with a unique identifier such as the SID or PUID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityDescriptor {
    #[doc = "The unique identifier for this identity, not exceeding 256 chars, which will be persisted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "Type of descriptor (for example, Windows, Passport, etc.)."]
    #[serde(
        rename = "identityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_type: Option<String>,
}
impl IdentityDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Identity>,
}
impl IdentityList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityRightsTransferData {
    #[serde(
        rename = "userPrincipalNameMappings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_principal_name_mappings: Option<serde_json::Value>,
}
impl IdentityRightsTransferData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrators: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "isGlobal", default, skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[serde(
        rename = "localScopeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<identity_scope::ScopeType>,
    #[serde(
        rename = "securingHostId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub securing_host_id: Option<String>,
    #[serde(
        rename = "subjectDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_descriptor: Option<String>,
}
impl IdentityScope {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_scope {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "serviceHost")]
        ServiceHost,
        #[serde(rename = "teamProject")]
        TeamProject,
    }
}
#[doc = "Identity information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentitySelf {
    #[doc = "The UserPrincipalName (UPN) of the account. This value comes from the source provider."]
    #[serde(
        rename = "accountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    #[doc = "The display name. For AAD accounts with multiple tenants this is the display name of the profile in the home tenant."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "This represents the name of the container of origin. For AAD accounts this is the tenantID of the home tenant. For MSA accounts this is the string \"Windows Live ID\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "This is the VSID of the home tenant profile. If the profile is signed into the home tenant or if the profile has no tenants then this Id is the same as the Id returned by the profile/profiles/me endpoint. Going forward it is recommended that you use the combined values of Origin, OriginId and Domain to uniquely identify a user rather than this Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of source provider for the origin identifier. For MSA accounts this is \"msa\". For AAD accounts this is \"aad\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The unique identifier from the system of origin. If there are multiple tenants this is the unique identifier of the account in the home tenant. (For MSA this is the PUID in hex notation, for AAD this is the object id.)"]
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[doc = "For AAD accounts this is all of the tenants that this account is a member of."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tenants: Vec<TenantInfo>,
}
impl IdentitySelf {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentitySnapshot {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<Identity>,
    #[serde(
        rename = "identityIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identity_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub memberships: Vec<GroupMembership>,
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<IdentityScope>,
}
impl IdentitySnapshot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityUpdateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<bool>,
}
impl IdentityUpdateData {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Represents a JSON object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[doc = "Gets the node type for this JToken."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl JObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON model for JSON Patch Operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPatchDocument {}
impl JsonPatchDocument {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON model for a JSON Patch operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPatchOperation {
    #[doc = "The path to copy from for the Move/Copy operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "The patch operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<json_patch_operation::Op>,
    #[doc = "The path for the operation. In the case of an array, a zero based index can be used to specify the position in the array (e.g. /biscuits/0/name). The \"-\" character can be used instead of an index to insert at the end of the array (e.g. /biscuits/-)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The value for the operation. This is either a primitive or a JToken."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl JsonPatchOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod json_patch_operation {
    use super::*;
    #[doc = "The patch operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "replace")]
        Replace,
        #[serde(rename = "move")]
        Move,
        #[serde(rename = "copy")]
        Copy,
        #[serde(rename = "test")]
        Test,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedIdentities {
    #[serde(
        rename = "continuationToken",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub continuation_token: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identities: Vec<Identity>,
}
impl PagedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertiesCollection {
    #[doc = "The count of properties in the collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<serde_json::Value>,
    #[doc = "The set of keys in the collection."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub keys: Vec<String>,
    #[doc = "The set of values in the collection."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl PropertiesCollection {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwapIdentityInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id2: Option<String>,
}
impl SwapIdentityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantInfo {
    #[serde(
        rename = "homeTenant",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_tenant: Option<bool>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(
        rename = "tenantName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tenant_name: Option<String>,
    #[serde(
        rename = "verifiedDomains",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub verified_domains: Vec<String>,
}
impl TenantInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class is used to serialize collections as a single JSON object on the wire."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VssJsonCollectionWrapper {
    #[serde(flatten)]
    pub vss_json_collection_wrapper_base: VssJsonCollectionWrapperBase,
    #[doc = "The serialized item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VssJsonCollectionWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VssJsonCollectionWrapperBase {
    #[doc = "The number of serialized items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl VssJsonCollectionWrapperBase {
    pub fn new() -> Self {
        Self::default()
    }
}
