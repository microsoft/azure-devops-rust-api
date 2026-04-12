// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadOauthTokenRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl AadOauthTokenRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadOauthTokenResult {
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    #[serde(
        rename = "refreshTokenCache",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token_cache: Option<String>,
}
impl AadOauthTokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthConfiguration {
    #[serde(flatten)]
    pub o_auth_configuration: OAuthConfiguration,
    #[doc = "Gets or sets parameters contained in configuration object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl AuthConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the authentication scheme to be used for authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthenticationSchemeReference {
    #[doc = "Gets or sets the key and value of the fields used for authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Gets or sets the type of authentication scheme of an endpoint."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl AuthenticationSchemeReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the header of the REST request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationHeader {
    #[doc = "Gets or sets the name of authorization header."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the value of authorization header."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AuthorizationHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAppService {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureAppService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultPermission {
    #[serde(flatten)]
    pub azure_resource_permission: AzureResourcePermission,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<String>,
}
impl AzureKeyVaultPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMlWorkspace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureMlWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Management Group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureManagementGroup {
    #[doc = "Display name of azure management group"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Id of azure management group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure management group name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Id of tenant from which azure management group belogs"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl AzureManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure management group query result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureManagementGroupQueryResult {
    #[doc = "Error message in case of an exception"]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "List of azure management groups"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureManagementGroup>,
}
impl AzureManagementGroupQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[serde(
        rename = "resourceProvider",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_provider: Option<String>,
}
impl AzurePermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourcePermission {
    #[serde(flatten)]
    pub azure_permission: AzurePermission,
    #[serde(
        rename = "resourceGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_group: Option<String>,
}
impl AzureResourcePermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureRoleAssignmentPermission {
    #[serde(flatten)]
    pub azure_permission: AzurePermission,
    #[serde(
        rename = "roleAssignmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub role_assignment_id: Option<String>,
}
impl AzureRoleAssignmentPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSpnOperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
}
impl AzureSpnOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSubscription {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        rename = "subscriptionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_id: Option<String>,
    #[serde(
        rename = "subscriptionTenantId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_tenant_id: Option<String>,
    #[serde(
        rename = "subscriptionTenantName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_tenant_name: Option<String>,
}
impl AzureSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSubscriptionQueryResult {
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureSubscription>,
}
impl AzureSubscriptionQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the client certificate to be used for the endpoint request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientCertificate {
    #[doc = "Gets or sets the value of client certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ClientCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the data sources for this endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSource {
    #[doc = "Specifies the authentication scheme to be used for authentication."]
    #[serde(
        rename = "authenticationScheme",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_scheme: Option<AuthenticationSchemeReference>,
    #[doc = "Gets or sets the pagination format supported by this data source(ContinuationToken/SkipTop)."]
    #[serde(
        rename = "callbackContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_context_template: Option<String>,
    #[doc = "Gets or sets the template to check if subsequent call is needed."]
    #[serde(
        rename = "callbackRequiredTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_required_template: Option<String>,
    #[doc = "Gets or sets the endpoint url of the data source."]
    #[serde(
        rename = "endpointUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_url: Option<String>,
    #[doc = "Gets or sets the authorization headers of the request."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<AuthorizationHeader>,
    #[doc = "Gets or sets the initial value of the query params."]
    #[serde(
        rename = "initialContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_context_template: Option<String>,
    #[doc = "Gets or sets the name of the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the request content of the endpoint request."]
    #[serde(
        rename = "requestContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_content: Option<String>,
    #[doc = "Gets or sets the request method of the endpoint request."]
    #[serde(
        rename = "requestVerb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_verb: Option<String>,
    #[doc = "Gets or sets the resource url of the endpoint request."]
    #[serde(
        rename = "resourceUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_url: Option<String>,
    #[doc = "Gets or sets the result selector to filter the response of the endpoint request."]
    #[serde(
        rename = "resultSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_selector: Option<String>,
}
impl DataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the data source binding of the endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceBinding {
    #[serde(flatten)]
    pub data_source_binding_base: DataSourceBindingBase,
}
impl DataSourceBinding {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents binding of data source for the service endpoint request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceBindingBase {
    #[doc = "Pagination format supported by this data source(ContinuationToken/SkipTop)."]
    #[serde(
        rename = "callbackContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_context_template: Option<String>,
    #[doc = "Subsequent calls needed?"]
    #[serde(
        rename = "callbackRequiredTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_required_template: Option<String>,
    #[doc = "Gets or sets the name of the data source."]
    #[serde(
        rename = "dataSourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_name: Option<String>,
    #[doc = "Gets or sets the endpoint Id."]
    #[serde(
        rename = "endpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<String>,
    #[doc = "Gets or sets the url of the service endpoint."]
    #[serde(
        rename = "endpointUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_url: Option<String>,
    #[doc = "Gets or sets the authorization headers."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<AuthorizationHeader>,
    #[doc = "Defines the initial value of the query params"]
    #[serde(
        rename = "initialContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_context_template: Option<String>,
    #[doc = "Gets or sets the parameters for the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets http request body"]
    #[serde(
        rename = "requestContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_content: Option<String>,
    #[doc = "Gets or sets http request verb"]
    #[serde(
        rename = "requestVerb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_verb: Option<String>,
    #[doc = "Gets or sets the result selector."]
    #[serde(
        rename = "resultSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_selector: Option<String>,
    #[doc = "Gets or sets the result template."]
    #[serde(
        rename = "resultTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_template: Option<String>,
    #[doc = "Gets or sets the target of the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl DataSourceBindingBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents details of the service endpoint data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceDetails {
    #[doc = "Gets or sets the data source name."]
    #[serde(
        rename = "dataSourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_name: Option<String>,
    #[doc = "Gets or sets the data source url."]
    #[serde(
        rename = "dataSourceUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_url: Option<String>,
    #[doc = "Gets or sets the request headers."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<AuthorizationHeader>,
    #[doc = "Gets or sets the initialization context used for the initial call to the data source"]
    #[serde(
        rename = "initialContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_context_template: Option<String>,
    #[doc = "Gets the parameters of data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the data source request content."]
    #[serde(
        rename = "requestContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_content: Option<String>,
    #[doc = "Gets or sets the data source request verb. Get/Post are the only implemented types"]
    #[serde(
        rename = "requestVerb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_verb: Option<String>,
    #[doc = "Gets or sets the resource url of data source."]
    #[serde(
        rename = "resourceUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_url: Option<String>,
    #[doc = "Gets or sets the result selector."]
    #[serde(
        rename = "resultSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_selector: Option<String>,
}
impl DataSourceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the details of the input on which a given input is dependent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyBinding {
    #[doc = "Gets or sets the value of the field on which url is dependent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Gets or sets the corresponding value of url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl DependencyBinding {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the dependency data for the endpoint inputs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyData {
    #[doc = "Gets or sets the category of dependency data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[doc = "Gets or sets the key-value pair to specify properties and their values."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub map: Vec<serde_json::Value>,
}
impl DependencyData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the inputs on which any given input is dependent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependsOn {
    #[doc = "Gets or sets the ID of the field on which URL's value is dependent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[doc = "Gets or sets key-value pair containing other's field value and corresponding url value."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub map: Vec<DependencyBinding>,
}
impl DependsOn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the authorization used for service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointAuthorization {
    #[doc = "Gets or sets the parameters for the selected authorization scheme."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the scheme used for service endpoint authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}
impl EndpointAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointOperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
}
impl EndpointOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents url of the service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointUrl {
    #[doc = "Represents the inputs on which any given input is dependent."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<DependsOn>,
    #[doc = "Gets or sets the display name of service endpoint url."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the format of the url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "Gets or sets the help text of service endpoint url."]
    #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    #[doc = "Gets or sets the visibility of service endpoint url."]
    #[serde(rename = "isVisible", default, skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<String>,
    #[doc = "Gets or sets the value of service endpoint url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EndpointUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubjectBase {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "The descriptor is the primary way to reference the graph subject while the system is running. This field will uniquely identify the same graph subject across both Accounts and Organizations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "This is the non-unique display name of the graph subject. To change this field, you must alter its value in the source provider."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "This url is the full route to the source resource of this graph subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GraphSubjectBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the public url of the help documentation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelpLink {
    #[doc = "Gets or sets the help text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "Gets or sets the public url of the help documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl HelpLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityRef {
    #[serde(flatten)]
    pub graph_subject_base: GraphSubjectBase,
    #[doc = "Deprecated - Can be retrieved by querying the Graph user referenced in the \"self\" entry of the IdentityRef \"_links\" dictionary"]
    #[serde(
        rename = "directoryAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub directory_alias: Option<String>,
    pub id: String,
    #[doc = "Deprecated - Available in the \"avatar\" entry of the IdentityRef \"_links\" dictionary"]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[doc = "Deprecated - Can be retrieved by querying the Graph membership state referenced in the \"membershipState\" entry of the GraphUser \"_links\" dictionary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[doc = "Deprecated - Can be inferred from the subject type of the descriptor (Descriptor.IsAadUserType/Descriptor.IsAadGroupType)"]
    #[serde(
        rename = "isAadIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_aad_identity: Option<bool>,
    #[doc = "Deprecated - Can be inferred from the subject type of the descriptor (Descriptor.IsGroupType)"]
    #[serde(
        rename = "isContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_container: Option<bool>,
    #[serde(
        rename = "isDeletedInOrigin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deleted_in_origin: Option<bool>,
    #[doc = "Deprecated - not in use in most preexisting implementations of ToIdentityRef"]
    #[serde(
        rename = "profileUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_url: Option<String>,
    #[doc = "Deprecated - use Domain+PrincipalName instead"]
    #[serde(
        rename = "uniqueName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_name: Option<String>,
}
impl IdentityRef {
    pub fn new(id: String) -> Self {
        Self {
            graph_subject_base: GraphSubjectBase::default(),
            directory_alias: None,
            id,
            image_url: None,
            inactive: None,
            is_aad_identity: None,
            is_container: None,
            is_deleted_in_origin: None,
            profile_url: None,
            unique_name: None,
        }
    }
}
#[doc = "Describes an input for subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputDescriptor {
    #[doc = "The ids of all inputs that the value of this input is dependent on."]
    #[serde(
        rename = "dependencyInputIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependency_input_ids: Vec<String>,
    #[doc = "Description of what this input is used for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The group localized name to which this input belongs and can be shown as a header for the container that will include all the inputs in the group."]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[doc = "If true, the value information for this input is dynamic and should be fetched when the value of dependency inputs change."]
    #[serde(
        rename = "hasDynamicValueInformation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_dynamic_value_information: Option<bool>,
    #[doc = "Identifier for the subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Mode in which the value of this input should be entered"]
    #[serde(rename = "inputMode", default, skip_serializing_if = "Option::is_none")]
    pub input_mode: Option<input_descriptor::InputMode>,
    #[doc = "Gets whether this input is confidential, such as for a password or application key"]
    #[serde(
        rename = "isConfidential",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_confidential: Option<bool>,
    #[doc = "Localized name which can be shown as a label for the subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Custom properties for the input which can be used by the service provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Underlying data type for the input value. When this value is specified, InputMode, Validation and Values are optional."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets whether this input is included in the default generated action description."]
    #[serde(
        rename = "useInDefaultDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_in_default_description: Option<bool>,
    #[doc = "Describes what values are valid for a subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<InputValidation>,
    #[doc = "A hint for input value. It can be used in the UI as the input placeholder."]
    #[serde(rename = "valueHint", default, skip_serializing_if = "Option::is_none")]
    pub value_hint: Option<String>,
    #[doc = "Information about the possible/allowed values for a given subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<InputValues>,
}
impl InputDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod input_descriptor {
    use super::*;
    #[doc = "Mode in which the value of this input should be entered"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InputMode {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "textBox")]
        TextBox,
        #[serde(rename = "passwordBox")]
        PasswordBox,
        #[serde(rename = "combo")]
        Combo,
        #[serde(rename = "radioButtons")]
        RadioButtons,
        #[serde(rename = "checkBox")]
        CheckBox,
        #[serde(rename = "textArea")]
        TextArea,
    }
}
#[doc = "Describes what values are valid for a subscription input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValidation {
    #[doc = "Gets or sets the data type to validate."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<input_validation::DataType>,
    #[doc = "Gets or sets if this is a required field."]
    #[serde(
        rename = "isRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
    #[doc = "Gets or sets the maximum length of this descriptor."]
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[doc = "Gets or sets the minimum value for this descriptor."]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[doc = "Gets or sets the minimum length of this descriptor."]
    #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[doc = "Gets or sets the minimum value for this descriptor."]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    #[doc = "Gets or sets the pattern to validate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Gets or sets the error on pattern mismatch."]
    #[serde(
        rename = "patternMismatchErrorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pattern_mismatch_error_message: Option<String>,
}
impl InputValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod input_validation {
    use super::*;
    #[doc = "Gets or sets the data type to validate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "string")]
        String,
        #[serde(rename = "number")]
        Number,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(rename = "guid")]
        Guid,
        #[serde(rename = "uri")]
        Uri,
    }
}
#[doc = "Information about a single value for an input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValue {
    #[doc = "Any other data about this input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "The text to show for the display of this value"]
    #[serde(
        rename = "displayValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_value: Option<String>,
    #[doc = "The value to store for this input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl InputValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the possible/allowed values for a given subscription input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValues {
    #[doc = "The default value to use for this input"]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
    #[doc = "Error information related to a subscription input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<InputValuesError>,
    #[doc = "The id of the input"]
    #[serde(rename = "inputId", default, skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[doc = "Should this input be disabled"]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Should the value be restricted to one of the values in the PossibleValues (True) or are the values in PossibleValues just a suggestion (False)"]
    #[serde(
        rename = "isLimitedToPossibleValues",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_limited_to_possible_values: Option<bool>,
    #[doc = "Should this input be made read-only"]
    #[serde(
        rename = "isReadOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_read_only: Option<bool>,
    #[doc = "Possible values that this input can take"]
    #[serde(
        rename = "possibleValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub possible_values: Vec<InputValue>,
}
impl InputValues {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error information related to a subscription input value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValuesError {
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl InputValuesError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a JSON object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JObject {
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<JToken>,
    #[doc = "Gets the node type for this JToken."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl JObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an abstract JSON token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JToken {
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first: Option<Box<JToken>>,
    #[doc = "Gets a value indicating whether this token has child tokens."]
    #[serde(rename = "hasValues", default, skip_serializing_if = "Option::is_none")]
    pub has_values: Option<bool>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<JToken>>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Option<Box<JToken>>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<JToken>>,
    #[doc = "Gets or sets the parent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[doc = "Gets the path of the JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<Box<JToken>>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<Box<JToken>>,
    #[doc = "Gets the node type for this JToken."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl JToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2TokenResult {
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(
        rename = "errorDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_description: Option<String>,
    #[serde(rename = "expiresIn", default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    #[serde(rename = "issuedAt", default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(
        rename = "refreshToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl OAuth2TokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthConfiguration {
    #[doc = "Gets or sets the ClientId"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Gets or sets the ClientSecret"]
    #[serde(
        rename = "clientSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_secret: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets or sets the time when config was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the type of the endpoint."]
    #[serde(
        rename = "endpointType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_type: Option<String>,
    #[doc = "Gets or sets the unique identifier of this field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Gets or sets the time when variable group was modified"]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl OAuthConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthConfigurationParams {
    #[doc = "Gets or sets the ClientId"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Gets or sets the ClientSecret"]
    #[serde(
        rename = "clientSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_secret: Option<String>,
    #[doc = "Gets or sets the type of the endpoint."]
    #[serde(
        rename = "endpointType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_type: Option<String>,
    #[doc = "Gets or sets the name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl OAuthConfigurationParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthEndpointStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
}
impl OAuthEndpointStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Parameter {
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Parameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectReference {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProjectReference {
    pub fn new(id: String) -> Self {
        Self { id, name: None }
    }
}
#[doc = "The class to represent a collection of REST reference links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLinks {
    #[doc = "The readonly view of the links.  Because Reference links are readonly, we only want to expose them as read only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
impl ReferenceLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specify the properties for refreshing the endpoint authentication object being queried"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshAuthenticationParameters {
    #[doc = "EndpointId which needs new authentication params"]
    #[serde(
        rename = "endpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<String>,
    #[doc = "Scope of the token requested. For GitHub marketplace apps, scope contains repository Ids"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scope: Vec<i32>,
    #[doc = "The requested endpoint authentication should be valid for _ minutes. Authentication params will not be refreshed if the token contained in endpoint already has active token."]
    #[serde(
        rename = "tokenValidityInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub token_validity_in_minutes: Option<i64>,
}
impl RefreshAuthenticationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents template to transform the result data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResultTransformationDetails {
    #[doc = "Gets or sets the template for callback parameters"]
    #[serde(
        rename = "callbackContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_context_template: Option<String>,
    #[doc = "Gets or sets the template to decide whether to callback or not"]
    #[serde(
        rename = "callbackRequiredTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_required_template: Option<String>,
    #[doc = "Gets or sets the template for result transformation."]
    #[serde(
        rename = "resultTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_template: Option<String>,
}
impl ResultTransformationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an endpoint which may be used by an orchestration job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    #[serde(
        rename = "administratorsGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub administrators_group: Option<IdentityRef>,
    #[doc = "Represents the authorization used for service endpoint."]
    pub authorization: EndpointAuthorization,
    #[serde(rename = "createdBy")]
    pub created_by: IdentityRef,
    pub data: serde_json::Value,
    #[doc = "Gets or sets the description of endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "This is a deprecated field."]
    #[serde(
        rename = "groupScopeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_scope_id: Option<String>,
    #[doc = "Gets or sets the identifier of this endpoint."]
    pub id: String,
    #[doc = "EndPoint state indicator"]
    #[serde(rename = "isReady")]
    pub is_ready: bool,
    #[doc = "Indicates whether service endpoint is shared with other projects or not."]
    #[serde(rename = "isShared")]
    pub is_shared: bool,
    #[doc = "Gets or sets the friendly name of the endpoint."]
    pub name: String,
    #[doc = "Error message during creation/deletion of endpoint"]
    #[serde(
        rename = "operationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_status: Option<serde_json::Value>,
    #[doc = "Owner of the endpoint Supported values are \"library\", \"agentcloud\""]
    pub owner: String,
    #[serde(
        rename = "readersGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub readers_group: Option<IdentityRef>,
    #[doc = "All other project references where the service endpoint is shared."]
    #[serde(
        rename = "serviceEndpointProjectReferences",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_endpoint_project_references: Vec<ServiceEndpointProjectReference>,
    #[doc = "Gets or sets the type of the endpoint."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Gets or sets the url of the endpoint."]
    pub url: String,
}
impl ServiceEndpoint {
    pub fn new(
        authorization: EndpointAuthorization,
        created_by: IdentityRef,
        data: serde_json::Value,
        id: String,
        is_ready: bool,
        is_shared: bool,
        name: String,
        owner: String,
        type_: String,
        url: String,
    ) -> Self {
        Self {
            administrators_group: None,
            authorization,
            created_by,
            data,
            description: None,
            group_scope_id: None,
            id,
            is_ready,
            is_shared,
            name,
            operation_status: None,
            owner,
            readers_group: None,
            service_endpoint_project_references: Vec::new(),
            type_,
            url,
        }
    }
}
#[doc = "Represents the authentication scheme used to authenticate the endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointAuthenticationScheme {
    #[doc = "Gets or sets the authorization headers of service endpoint authentication scheme."]
    #[serde(
        rename = "authorizationHeaders",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authorization_headers: Vec<AuthorizationHeader>,
    #[doc = "Gets or sets the Authorization url required to authenticate using OAuth2"]
    #[serde(
        rename = "authorizationUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_url: Option<String>,
    #[doc = "Gets or sets the certificates of service endpoint authentication scheme."]
    #[serde(
        rename = "clientCertificates",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub client_certificates: Vec<ClientCertificate>,
    #[doc = "Gets or sets the data source bindings of the endpoint."]
    #[serde(
        rename = "dataSourceBindings",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_source_bindings: Vec<DataSourceBinding>,
    #[doc = "Gets or sets the display name for the service endpoint authentication scheme."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the input descriptors for the service endpoint authentication scheme."]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "Gets or sets the properties of service endpoint authentication scheme."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Gets or sets whether this auth scheme requires OAuth2 configuration or not."]
    #[serde(
        rename = "requiresOAuth2Configuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_o_auth2_configuration: Option<bool>,
    #[doc = "Gets or sets the scheme for service endpoint authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}
impl ServiceEndpointAuthenticationScheme {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents details of the service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointDetails {
    #[doc = "Represents the authorization used for service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<EndpointAuthorization>,
    #[doc = "Gets or sets the data of service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Gets or sets the type of service endpoint."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the connection url of service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ServiceEndpointDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents service endpoint execution data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointExecutionData {
    #[doc = "Represents execution owner of the service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<ServiceEndpointExecutionOwner>,
    #[doc = "Gets the finish time of service endpoint execution."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the Id of service endpoint execution data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Represents execution owner of the service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<ServiceEndpointExecutionOwner>,
    #[doc = "Gets the additional details about the instance that used the service endpoint."]
    #[serde(
        rename = "ownerDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_details: Option<String>,
    #[doc = "Gets the plan type of service endpoint execution data."]
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[doc = "Gets the result of service endpoint execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<service_endpoint_execution_data::Result>,
    #[doc = "Gets the start time of service endpoint execution."]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
}
impl ServiceEndpointExecutionData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_endpoint_execution_data {
    use super::*;
    #[doc = "Gets the result of service endpoint execution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "succeededWithIssues")]
        SucceededWithIssues,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
}
#[doc = "Represents execution owner of the service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointExecutionOwner {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Gets or sets the Id of service endpoint execution owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets the name of service endpoint execution owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ServiceEndpointExecutionOwner {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the details of service endpoint execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointExecutionRecord {
    #[doc = "Represents service endpoint execution data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ServiceEndpointExecutionData>,
    #[doc = "Gets the Id of service endpoint."]
    #[serde(
        rename = "endpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<String>,
}
impl ServiceEndpointExecutionRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointExecutionRecordList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServiceEndpointExecutionRecord>,
}
impl ServiceEndpointExecutionRecordList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointExecutionRecordsInput {
    #[doc = "Represents service endpoint execution data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ServiceEndpointExecutionData>,
    #[serde(
        rename = "endpointIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoint_ids: Vec<String>,
}
impl ServiceEndpointExecutionRecordsInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServiceEndpoint>,
}
impl ServiceEndpointList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointOAuthConfigurationReference {
    #[serde(
        rename = "configurationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_id: Option<String>,
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
    #[serde(
        rename = "serviceEndpointProjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_project_id: Option<String>,
}
impl ServiceEndpointOAuthConfigurationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceEndpointProjectReference {
    #[doc = "Gets or sets description of the service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets name of the service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectReference")]
    pub project_reference: ProjectReference,
}
impl ServiceEndpointProjectReference {
    pub fn new(project_reference: ProjectReference) -> Self {
        Self {
            description: None,
            name: None,
            project_reference,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointRequest {
    #[doc = "Represents details of the service endpoint data source."]
    #[serde(
        rename = "dataSourceDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_details: Option<DataSourceDetails>,
    #[doc = "Represents template to transform the result data."]
    #[serde(
        rename = "resultTransformationDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_transformation_details: Option<ResultTransformationDetails>,
    #[doc = "Represents details of the service endpoint."]
    #[serde(
        rename = "serviceEndpointDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_details: Option<ServiceEndpointDetails>,
}
impl ServiceEndpointRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents result of the service endpoint request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointRequestResult {
    #[doc = "Gets or sets the parameters used to make subsequent calls to the data source"]
    #[serde(
        rename = "callbackContextParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_context_parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the flat that decides if another call to the data source is to be made"]
    #[serde(
        rename = "callbackRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_required: Option<bool>,
    #[doc = "Gets or sets the error message of the service endpoint request result."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<JToken>,
    #[doc = "Gets or sets the status code of the service endpoint request result."]
    #[serde(
        rename = "statusCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_code: Option<service_endpoint_request_result::StatusCode>,
}
impl ServiceEndpointRequestResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_endpoint_request_result {
    use super::*;
    #[doc = "Gets or sets the status code of the service endpoint request result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusCode {
        #[serde(rename = "continue")]
        Continue,
        #[serde(rename = "switchingProtocols")]
        SwitchingProtocols,
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "accepted")]
        Accepted,
        #[serde(rename = "nonAuthoritativeInformation")]
        NonAuthoritativeInformation,
        #[serde(rename = "noContent")]
        NoContent,
        #[serde(rename = "resetContent")]
        ResetContent,
        #[serde(rename = "partialContent")]
        PartialContent,
        #[serde(rename = "multipleChoices")]
        MultipleChoices,
        #[serde(rename = "ambiguous")]
        Ambiguous,
        #[serde(rename = "movedPermanently")]
        MovedPermanently,
        #[serde(rename = "moved")]
        Moved,
        #[serde(rename = "found")]
        Found,
        #[serde(rename = "redirect")]
        Redirect,
        #[serde(rename = "seeOther")]
        SeeOther,
        #[serde(rename = "redirectMethod")]
        RedirectMethod,
        #[serde(rename = "notModified")]
        NotModified,
        #[serde(rename = "useProxy")]
        UseProxy,
        #[serde(rename = "unused")]
        Unused,
        #[serde(rename = "temporaryRedirect")]
        TemporaryRedirect,
        #[serde(rename = "redirectKeepVerb")]
        RedirectKeepVerb,
        #[serde(rename = "badRequest")]
        BadRequest,
        #[serde(rename = "unauthorized")]
        Unauthorized,
        #[serde(rename = "paymentRequired")]
        PaymentRequired,
        #[serde(rename = "forbidden")]
        Forbidden,
        #[serde(rename = "notFound")]
        NotFound,
        #[serde(rename = "methodNotAllowed")]
        MethodNotAllowed,
        #[serde(rename = "notAcceptable")]
        NotAcceptable,
        #[serde(rename = "proxyAuthenticationRequired")]
        ProxyAuthenticationRequired,
        #[serde(rename = "requestTimeout")]
        RequestTimeout,
        #[serde(rename = "conflict")]
        Conflict,
        #[serde(rename = "gone")]
        Gone,
        #[serde(rename = "lengthRequired")]
        LengthRequired,
        #[serde(rename = "preconditionFailed")]
        PreconditionFailed,
        #[serde(rename = "requestEntityTooLarge")]
        RequestEntityTooLarge,
        #[serde(rename = "requestUriTooLong")]
        RequestUriTooLong,
        #[serde(rename = "unsupportedMediaType")]
        UnsupportedMediaType,
        #[serde(rename = "requestedRangeNotSatisfiable")]
        RequestedRangeNotSatisfiable,
        #[serde(rename = "expectationFailed")]
        ExpectationFailed,
        #[serde(rename = "upgradeRequired")]
        UpgradeRequired,
        #[serde(rename = "internalServerError")]
        InternalServerError,
        #[serde(rename = "notImplemented")]
        NotImplemented,
        #[serde(rename = "badGateway")]
        BadGateway,
        #[serde(rename = "serviceUnavailable")]
        ServiceUnavailable,
        #[serde(rename = "gatewayTimeout")]
        GatewayTimeout,
        #[serde(rename = "httpVersionNotSupported")]
        HttpVersionNotSupported,
    }
}
#[doc = "Represents type of the service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointType {
    #[doc = "Authentication scheme of service endpoint type."]
    #[serde(
        rename = "authenticationSchemes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authentication_schemes: Vec<ServiceEndpointAuthenticationScheme>,
    #[doc = "Data sources of service endpoint type."]
    #[serde(
        rename = "dataSources",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_sources: Vec<DataSource>,
    #[doc = "Dependency data of service endpoint type."]
    #[serde(
        rename = "dependencyData",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependency_data: Vec<DependencyData>,
    #[doc = "Gets or sets the description of service endpoint type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the display name of service endpoint type."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Represents url of the service endpoint."]
    #[serde(
        rename = "endpointUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_url: Option<EndpointUrl>,
    #[doc = "Specifies the public url of the help documentation."]
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<HelpLink>,
    #[doc = "Gets or sets the help text shown at the endpoint create dialog."]
    #[serde(
        rename = "helpMarkDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub help_mark_down: Option<String>,
    #[doc = "Gets or sets the icon url of service endpoint type."]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "Input descriptor of service endpoint type."]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "Gets or sets the name of service endpoint type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Trusted hosts of a service endpoint type."]
    #[serde(
        rename = "trustedHosts",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trusted_hosts: Vec<String>,
    #[doc = "Gets or sets the ui contribution id of service endpoint type."]
    #[serde(
        rename = "uiContributionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ui_contribution_id: Option<String>,
}
impl ServiceEndpointType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServiceEndpointType>,
}
impl ServiceEndpointTypeList {
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
