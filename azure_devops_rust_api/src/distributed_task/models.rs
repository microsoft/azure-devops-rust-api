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
pub struct AgentChangeEvent {
    #[doc = "A task agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<TaskAgent>,
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
}
impl AgentChangeEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentJobRequestMessage {
    #[serde(flatten)]
    pub job_request_message: JobRequestMessage,
    #[serde(
        rename = "lockedUntil",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub locked_until: Option<time::OffsetDateTime>,
    #[serde(rename = "lockToken", default, skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<TaskInstance>,
}
impl AgentJobRequestMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolEvent {
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "An organization-level grouping of agents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPool>,
}
impl AgentPoolEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentQueueEvent {
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "An agent queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<TaskAgentQueue>,
}
impl AgentQueueEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentQueuesEvent {
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub queues: Vec<TaskAgentQueue>,
}
impl AgentQueuesEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentRefreshMessage {
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i32>,
    #[serde(
        rename = "targetVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl AgentRefreshMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthenticationSchemeReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl AuthenticationSchemeReference {
    pub fn new() -> Self {
        Self::default()
    }
}
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
pub struct AzureKeyVaultVariableGroupProviderData {
    #[serde(
        rename = "lastRefreshedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_refreshed_on: Option<time::OffsetDateTime>,
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<String>,
}
impl AzureKeyVaultVariableGroupProviderData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultVariableValue {
    #[serde(flatten)]
    pub variable_value: VariableValue,
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub expires: Option<time::OffsetDateTime>,
}
impl AzureKeyVaultVariableValue {
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
    #[doc = "Id of tenant from which azure management group belongs"]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CounterVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}
impl CounterVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSource {
    #[serde(
        rename = "authenticationScheme",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_scheme: Option<AuthenticationSchemeReference>,
    #[serde(
        rename = "endpointUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<AuthorizationHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "resourceUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_url: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceDetails {
    #[serde(
        rename = "dataSourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_name: Option<String>,
    #[serde(
        rename = "dataSourceUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<AuthorizationHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(
        rename = "resourceUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_url: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Demand {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Demand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DemandEquals {
    #[serde(flatten)]
    pub demand: Demand,
}
impl DemandEquals {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DemandExists {
    #[serde(flatten)]
    pub demand: Demand,
}
impl DemandExists {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DemandMinimumVersion {
    #[serde(flatten)]
    pub demand: Demand,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<DemandSource>,
}
impl DemandMinimumVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DemandSource {
    #[serde(
        rename = "sourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_name: Option<String>,
    #[serde(
        rename = "sourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_type: Option<demand_source::SourceType>,
    #[serde(
        rename = "sourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_version: Option<String>,
}
impl DemandSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod demand_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceType {
        #[serde(rename = "task")]
        Task,
        #[serde(rename = "feature")]
        Feature,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyBinding {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl DependencyBinding {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependsOn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGatesChangeEvent {
    #[serde(
        rename = "gateNames",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub gate_names: Vec<String>,
}
impl DeploymentGatesChangeEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGroup {
    #[serde(flatten)]
    pub deployment_group_reference: DeploymentGroupReference,
    #[doc = "Description of the deployment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Number of deployment targets in the deployment group."]
    #[serde(
        rename = "machineCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_count: Option<i32>,
    #[doc = "List of deployment targets in the deployment group."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<DeploymentMachine>,
    #[doc = "List of unique tags across all deployment targets in the deployment group."]
    #[serde(
        rename = "machineTags",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_tags: Vec<String>,
}
impl DeploymentGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to create Deployment group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGroupCreateParameter {
    #[doc = "Description of the deployment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the deployment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Identifier of the deployment pool in which deployment agents are registered."]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
}
impl DeploymentGroupCreateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Deployment pool to create Deployment group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGroupCreateParameterPoolProperty {
    #[doc = "Deployment pool identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl DeploymentGroupCreateParameterPoolProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGroupList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeploymentGroup>,
}
impl DeploymentGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment group metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGroupMetrics {
    #[doc = "Metrics columns header"]
    #[serde(
        rename = "columnsHeader",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub columns_header: Option<MetricsColumnsHeader>,
    #[doc = "Deployment group reference. This is useful for referring a deployment group in another object."]
    #[serde(
        rename = "deploymentGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_group: Option<DeploymentGroupReference>,
    #[doc = "Values of properties and the metrics. E.g. 1: total count of deployment targets for which 'TargetState' is 'offline'. E.g. 2: Average time of deployment to the deployment targets for which 'LastJobStatus' is 'passed' and 'TargetState' is 'online'."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rows: Vec<MetricsRow>,
}
impl DeploymentGroupMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment group reference. This is useful for referring a deployment group in another object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGroupReference {
    #[doc = "Deployment group identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the deployment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
}
impl DeploymentGroupReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment group update parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentGroupUpdateParameter {
    #[doc = "Description of the deployment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the deployment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DeploymentGroupUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentMachine {
    #[doc = "A task agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<TaskAgent>,
    #[doc = "Deployment target Identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Tags of the deployment target."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl DeploymentMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentMachineChangedData {
    #[serde(flatten)]
    pub deployment_machine: DeploymentMachine,
    #[serde(
        rename = "addedTags",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub added_tags: Vec<String>,
    #[serde(
        rename = "deletedTags",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deleted_tags: Vec<String>,
}
impl DeploymentMachineChangedData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentMachineGroup {
    #[serde(flatten)]
    pub deployment_machine_group_reference: DeploymentMachineGroupReference,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<DeploymentMachine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
impl DeploymentMachineGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentMachineGroupReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
}
impl DeploymentMachineGroupReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentMachineList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeploymentMachine>,
}
impl DeploymentMachineList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentMachinesChangeEvent {
    #[doc = "Deployment group reference. This is useful for referring a deployment group in another object."]
    #[serde(
        rename = "machineGroupReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_group_reference: Option<DeploymentGroupReference>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<DeploymentMachineChangedData>,
}
impl DeploymentMachinesChangeEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment pool summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentPoolSummary {
    #[doc = "List of deployment groups referring to the deployment pool."]
    #[serde(
        rename = "deploymentGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployment_groups: Vec<DeploymentGroupReference>,
    #[doc = "Number of deployment agents that are offline."]
    #[serde(
        rename = "offlineAgentsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offline_agents_count: Option<i32>,
    #[doc = "Number of deployment agents that are online."]
    #[serde(
        rename = "onlineAgentsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub online_agents_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[doc = "EnvironmentResourceReference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<EnvironmentResourceReference>,
}
impl DeploymentPoolSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment target update parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentTargetUpdateParameter {
    #[doc = "Identifier of the deployment target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl DeploymentTargetUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticLogMetadata {
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i32>,
    #[serde(rename = "agentName", default, skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "phaseName", default, skip_serializing_if = "Option::is_none")]
    pub phase_name: Option<String>,
    #[serde(
        rename = "phaseResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phase_result: Option<String>,
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
}
impl DiagnosticLogMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticAgentPoolResizedEvent {
    #[serde(rename = "newSize", default, skip_serializing_if = "Option::is_none")]
    pub new_size: Option<i32>,
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
    #[serde(rename = "poolName", default, skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    #[serde(
        rename = "previousSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_size: Option<i32>,
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<String>,
}
impl ElasticAgentPoolResizedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data and settings for an elastic node"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticNode {
    #[doc = "Distributed Task's Agent Id"]
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i32>,
    #[doc = "Summary of the state of the agent"]
    #[serde(
        rename = "agentState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_state: Option<elastic_node::AgentState>,
    #[doc = "Compute Id.  VMSS's InstanceId"]
    #[serde(rename = "computeId", default, skip_serializing_if = "Option::is_none")]
    pub compute_id: Option<String>,
    #[doc = "State of the compute host"]
    #[serde(
        rename = "computeState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compute_state: Option<elastic_node::ComputeState>,
    #[doc = "Users can force state changes to specific states (ToReimage, ToDelete, Save)"]
    #[serde(
        rename = "desiredState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_state: Option<elastic_node::DesiredState>,
    #[doc = "Unique identifier since the agent and/or VM may be null"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Computer name. Used to match a scaleset VM with an agent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Pool Id that this node belongs to"]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
    #[doc = "Last job RequestId assigned to this agent"]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
    #[doc = "State of the ElasticNode"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<elastic_node::State>,
    #[doc = "Last state change. Only updated by SQL."]
    #[serde(
        rename = "stateChangedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub state_changed_on: Option<time::OffsetDateTime>,
}
impl ElasticNode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_node {
    use super::*;
    #[doc = "Summary of the state of the agent"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AgentState {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "online")]
        Online,
        #[serde(rename = "assigned")]
        Assigned,
    }
    #[doc = "State of the compute host"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComputeState {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "healthy")]
        Healthy,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "stopped")]
        Stopped,
        #[serde(rename = "reimaging")]
        Reimaging,
        #[serde(rename = "unhealthyVm")]
        UnhealthyVm,
        #[serde(rename = "unhealthyVmssVm")]
        UnhealthyVmssVm,
    }
    #[doc = "Users can force state changes to specific states (ToReimage, ToDelete, Save)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DesiredState {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "creatingCompute")]
        CreatingCompute,
        #[serde(rename = "startingAgent")]
        StartingAgent,
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "assigned")]
        Assigned,
        #[serde(rename = "offline")]
        Offline,
        #[serde(rename = "pendingReimage")]
        PendingReimage,
        #[serde(rename = "pendingDelete")]
        PendingDelete,
        #[serde(rename = "saved")]
        Saved,
        #[serde(rename = "deletingCompute")]
        DeletingCompute,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "lost")]
        Lost,
        #[serde(rename = "reimagingCompute")]
        ReimagingCompute,
        #[serde(rename = "restartingAgent")]
        RestartingAgent,
        #[serde(rename = "failedToStartPendingDelete")]
        FailedToStartPendingDelete,
        #[serde(rename = "failedToRestartPendingDelete")]
        FailedToRestartPendingDelete,
        #[serde(rename = "failedVMPendingDelete")]
        FailedVmPendingDelete,
        #[serde(rename = "assignedPendingDelete")]
        AssignedPendingDelete,
        #[serde(rename = "retryDelete")]
        RetryDelete,
        #[serde(rename = "unhealthyVm")]
        UnhealthyVm,
        #[serde(rename = "unhealthyVmPendingDelete")]
        UnhealthyVmPendingDelete,
    }
    #[doc = "State of the ElasticNode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "creatingCompute")]
        CreatingCompute,
        #[serde(rename = "startingAgent")]
        StartingAgent,
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "assigned")]
        Assigned,
        #[serde(rename = "offline")]
        Offline,
        #[serde(rename = "pendingReimage")]
        PendingReimage,
        #[serde(rename = "pendingDelete")]
        PendingDelete,
        #[serde(rename = "saved")]
        Saved,
        #[serde(rename = "deletingCompute")]
        DeletingCompute,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "lost")]
        Lost,
        #[serde(rename = "reimagingCompute")]
        ReimagingCompute,
        #[serde(rename = "restartingAgent")]
        RestartingAgent,
        #[serde(rename = "failedToStartPendingDelete")]
        FailedToStartPendingDelete,
        #[serde(rename = "failedToRestartPendingDelete")]
        FailedToRestartPendingDelete,
        #[serde(rename = "failedVMPendingDelete")]
        FailedVmPendingDelete,
        #[serde(rename = "assignedPendingDelete")]
        AssignedPendingDelete,
        #[serde(rename = "retryDelete")]
        RetryDelete,
        #[serde(rename = "unhealthyVm")]
        UnhealthyVm,
        #[serde(rename = "unhealthyVmPendingDelete")]
        UnhealthyVmPendingDelete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticNodeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ElasticNode>,
}
impl ElasticNodeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class used for updating an elastic node where only certain members are populated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticNodeSettings {
    #[doc = "State of the ElasticNode"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<elastic_node_settings::State>,
}
impl ElasticNodeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_node_settings {
    use super::*;
    #[doc = "State of the ElasticNode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "creatingCompute")]
        CreatingCompute,
        #[serde(rename = "startingAgent")]
        StartingAgent,
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "assigned")]
        Assigned,
        #[serde(rename = "offline")]
        Offline,
        #[serde(rename = "pendingReimage")]
        PendingReimage,
        #[serde(rename = "pendingDelete")]
        PendingDelete,
        #[serde(rename = "saved")]
        Saved,
        #[serde(rename = "deletingCompute")]
        DeletingCompute,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "lost")]
        Lost,
        #[serde(rename = "reimagingCompute")]
        ReimagingCompute,
        #[serde(rename = "restartingAgent")]
        RestartingAgent,
        #[serde(rename = "failedToStartPendingDelete")]
        FailedToStartPendingDelete,
        #[serde(rename = "failedToRestartPendingDelete")]
        FailedToRestartPendingDelete,
        #[serde(rename = "failedVMPendingDelete")]
        FailedVmPendingDelete,
        #[serde(rename = "assignedPendingDelete")]
        AssignedPendingDelete,
        #[serde(rename = "retryDelete")]
        RetryDelete,
        #[serde(rename = "unhealthyVm")]
        UnhealthyVm,
        #[serde(rename = "unhealthyVmPendingDelete")]
        UnhealthyVmPendingDelete,
    }
}
#[doc = "Data and settings for an elastic pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPool {
    #[doc = "Set whether agents should be configured to run with interactive UI"]
    #[serde(
        rename = "agentInteractiveUI",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_interactive_ui: Option<bool>,
    #[doc = "Azure string representing to location of the resource"]
    #[serde(rename = "azureId", default, skip_serializing_if = "Option::is_none")]
    pub azure_id: Option<String>,
    #[doc = "Number of agents to have ready waiting for jobs"]
    #[serde(
        rename = "desiredIdle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_idle: Option<i32>,
    #[doc = "The desired size of the pool"]
    #[serde(
        rename = "desiredSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_size: Option<i32>,
    #[doc = "Maximum number of nodes that will exist in the elastic pool"]
    #[serde(
        rename = "maxCapacity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_capacity: Option<i32>,
    #[doc = "Keep nodes in the pool on failure for investigation"]
    #[serde(
        rename = "maxSavedNodeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_saved_node_count: Option<i32>,
    #[doc = "Timestamp the pool was first detected to be offline"]
    #[serde(
        rename = "offlineSince",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub offline_since: Option<time::OffsetDateTime>,
    #[doc = "Operating system type of the nodes in the pool"]
    #[serde(
        rename = "orchestrationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orchestration_type: Option<elastic_pool::OrchestrationType>,
    #[doc = "Operating system type of the nodes in the pool"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<elastic_pool::OsType>,
    #[doc = "Id of the associated TaskAgentPool"]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
    #[doc = "Discard node after each job completes"]
    #[serde(
        rename = "recycleAfterEachUse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recycle_after_each_use: Option<bool>,
    #[doc = "Id of the Service Endpoint used to connect to Azure"]
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
    #[doc = "Scope the Service Endpoint belongs to"]
    #[serde(
        rename = "serviceEndpointScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_scope: Option<String>,
    #[doc = "The number of sizing attempts executed while trying to achieve a desired size"]
    #[serde(
        rename = "sizingAttempts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sizing_attempts: Option<i32>,
    #[doc = "State of the pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<elastic_pool::State>,
    #[doc = "The minimum time in minutes to keep idle agents alive"]
    #[serde(
        rename = "timeToLiveMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_to_live_minutes: Option<i32>,
}
impl ElasticPool {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool {
    use super::*;
    #[doc = "Operating system type of the nodes in the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrchestrationType {
        #[serde(rename = "uniform")]
        Uniform,
        #[serde(rename = "flexible")]
        Flexible,
    }
    #[doc = "Operating system type of the nodes in the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        #[serde(rename = "windows")]
        Windows,
        #[serde(rename = "linux")]
        Linux,
    }
    #[doc = "State of the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "online")]
        Online,
        #[serde(rename = "offline")]
        Offline,
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "new")]
        New,
    }
}
#[doc = "Returned result from creating a new elastic pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolCreationResult {
    #[doc = "An organization-level grouping of agents."]
    #[serde(rename = "agentPool", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool: Option<TaskAgentPool>,
    #[doc = "An agent queue."]
    #[serde(
        rename = "agentQueue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_queue: Option<TaskAgentQueue>,
    #[doc = "Data and settings for an elastic pool"]
    #[serde(
        rename = "elasticPool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub elastic_pool: Option<ElasticPool>,
}
impl ElasticPoolCreationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ElasticPool>,
}
impl ElasticPoolList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log data for an Elastic Pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolLog {
    #[doc = "Log Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "E.g. error, warning, info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<elastic_pool_log::Level>,
    #[doc = "Log contents"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Operation that triggered the message being logged"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<elastic_pool_log::Operation>,
    #[doc = "Id of the associated TaskAgentPool"]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
    #[doc = "Datetime that the log occurred"]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl ElasticPoolLog {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_log {
    use super::*;
    #[doc = "E.g. error, warning, info"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "info")]
        Info,
    }
    #[doc = "Operation that triggered the message being logged"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "configurationJob")]
        ConfigurationJob,
        #[serde(rename = "sizingJob")]
        SizingJob,
        #[serde(rename = "increaseCapacity")]
        IncreaseCapacity,
        #[serde(rename = "reimage")]
        Reimage,
        #[serde(rename = "deleteVMs")]
        DeleteVMs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolLogList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ElasticPoolLog>,
}
impl ElasticPoolLogList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class used for updating an elastic pool where only certain members are populated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolSettings {
    #[doc = "Set whether agents should be configured to run with interactive UI"]
    #[serde(
        rename = "agentInteractiveUI",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_interactive_ui: Option<bool>,
    #[doc = "Azure string representing to location of the resource"]
    #[serde(rename = "azureId", default, skip_serializing_if = "Option::is_none")]
    pub azure_id: Option<String>,
    #[doc = "Number of machines to have ready waiting for jobs"]
    #[serde(
        rename = "desiredIdle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_idle: Option<i32>,
    #[doc = "Maximum number of machines that will exist in the elastic pool"]
    #[serde(
        rename = "maxCapacity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_capacity: Option<i32>,
    #[doc = "Keep machines in the pool on failure for investigation"]
    #[serde(
        rename = "maxSavedNodeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_saved_node_count: Option<i32>,
    #[doc = "Operating system type of the machines in the pool"]
    #[serde(
        rename = "orchestrationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orchestration_type: Option<elastic_pool_settings::OrchestrationType>,
    #[doc = "Operating system type of the machines in the pool"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<elastic_pool_settings::OsType>,
    #[doc = "Discard machines after each job completes"]
    #[serde(
        rename = "recycleAfterEachUse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recycle_after_each_use: Option<bool>,
    #[doc = "Id of the Service Endpoint used to connect to Azure"]
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
    #[doc = "Scope the Service Endpoint belongs to"]
    #[serde(
        rename = "serviceEndpointScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_scope: Option<String>,
    #[doc = "The minimum time in minutes to keep idle agents alive"]
    #[serde(
        rename = "timeToLiveMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_to_live_minutes: Option<i32>,
}
impl ElasticPoolSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_settings {
    use super::*;
    #[doc = "Operating system type of the machines in the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrchestrationType {
        #[serde(rename = "uniform")]
        Uniform,
        #[serde(rename = "flexible")]
        Flexible,
    }
    #[doc = "Operating system type of the machines in the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        #[serde(rename = "windows")]
        Windows,
        #[serde(rename = "linux")]
        Linux,
    }
}
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
#[doc = "Represents url of the service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointUrl {
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<DependsOn>,
    #[doc = "Gets or sets the display name of service endpoint url."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
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
#[doc = "Properties to create Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentCreateParameter {
    #[doc = "Description of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EnvironmentCreateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "EnvironmentDeploymentExecutionRecord."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDeploymentExecutionRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<TaskOrchestrationOwner>,
    #[doc = "Id of the Environment"]
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[doc = "Finish time of the environment deployment execution"]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "Id of the Environment deployment execution history record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Job Attempt"]
    #[serde(
        rename = "jobAttempt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_attempt: Option<i32>,
    #[doc = "Job name"]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskOrchestrationOwner>,
    #[doc = "Plan Id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Plan type of the environment deployment execution record"]
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[doc = "Queue time of the environment deployment execution"]
    #[serde(
        rename = "queueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queue_time: Option<time::OffsetDateTime>,
    #[doc = "Request identifier of the Environment deployment execution history record"]
    #[serde(
        rename = "requestIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_identifier: Option<String>,
    #[doc = "Resource Id"]
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<i32>,
    #[doc = "Result of the environment deployment execution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<environment_deployment_execution_record::Result>,
    #[doc = "Project Id"]
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[doc = "Service owner Id"]
    #[serde(
        rename = "serviceOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_owner: Option<String>,
    #[doc = "Stage Attempt"]
    #[serde(
        rename = "stageAttempt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_attempt: Option<i32>,
    #[doc = "Stage name"]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[doc = "Start time of the environment deployment execution"]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
}
impl EnvironmentDeploymentExecutionRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_deployment_execution_record {
    use super::*;
    #[doc = "Result of the environment deployment execution"]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDeploymentExecutionRecordList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnvironmentDeploymentExecutionRecord>,
}
impl EnvironmentDeploymentExecutionRecordList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentInstance {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Creation time of the Environment"]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Description of the Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Id of the Environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<IdentityRef>,
    #[doc = "Last modified time of the Environment"]
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[doc = "Name of the Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<EnvironmentResourceReference>,
}
impl EnvironmentInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentInstanceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnvironmentInstance>,
}
impl EnvironmentInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "EnvironmentLinkedResourceReference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentLinkedResourceReference {
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of resource."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}
impl EnvironmentLinkedResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EnvironmentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentResource {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(
        rename = "environmentReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_reference: Option<EnvironmentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<IdentityRef>,
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Tags of the Environment Resource."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Environment resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<environment_resource::Type>,
}
impl EnvironmentResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_resource {
    use super::*;
    #[doc = "Environment resource type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "virtualMachine")]
        VirtualMachine,
        #[serde(rename = "kubernetes")]
        Kubernetes,
    }
}
#[doc = "EnvironmentResourceDeploymentExecutionRecord."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentResourceDeploymentExecutionRecord {
    #[doc = "Id of the Environment"]
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[doc = "Finish time of the environment resource deployment execution"]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "Id of the Environment deployment execution history record"]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
    #[doc = "Resource Id"]
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<i32>,
    #[doc = "Result of the environment deployment execution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<environment_resource_deployment_execution_record::Result>,
    #[doc = "Start time of the environment resource deployment execution"]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
}
impl EnvironmentResourceDeploymentExecutionRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_resource_deployment_execution_record {
    use super::*;
    #[doc = "Result of the environment deployment execution"]
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
#[doc = "EnvironmentResourceReference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentResourceReference {
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Tags of the Environment Resource Reference."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<environment_resource_reference::Type>,
}
impl EnvironmentResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_resource_reference {
    use super::*;
    #[doc = "Type of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "virtualMachine")]
        VirtualMachine,
        #[serde(rename = "kubernetes")]
        Kubernetes,
    }
}
#[doc = "Properties to update Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentUpdateParameter {
    #[doc = "Description of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EnvironmentUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsConfig {}
impl EventsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressionValidationItem {
    #[serde(flatten)]
    pub validation_item: ValidationItem,
}
impl ExpressionValidationItem {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelpLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl HelpLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputBindingContext {
    #[doc = "Value of the input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl InputBindingContext {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValidationItem {
    #[serde(flatten)]
    pub validation_item: ValidationItem,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<InputBindingContext>,
}
impl InputValidationItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValidationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
}
impl InputValidationRequest {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "An issue (error, warning) associated with a pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Issue {
    #[doc = "The category of the issue. <br />Example: Code - refers to compilation errors <br />Example: General - refers to generic errors"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "A dictionary containing details about the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "A description of issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The type (error, warning) of the issue."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<issue::Type>,
}
impl Issue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod issue {
    use super::*;
    #[doc = "The type (error, warning) of the issue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "warning")]
        Warning,
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
pub struct JobAssignedEvent {
    #[serde(flatten)]
    pub job_event: JobEvent,
    #[doc = "A job request for an agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<TaskAgentJobRequest>,
}
impl JobAssignedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCancelMessage {
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl JobCancelMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCanceledEvent {
    #[serde(flatten)]
    pub job_event: JobEvent,
    #[doc = "The reason for job cancellation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The job's timeout interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl JobCanceledEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCompletedEvent {
    #[serde(flatten)]
    pub job_event: JobEvent,
    #[doc = "Indicates whether the agent is in the process of shutting down."]
    #[serde(
        rename = "agentShuttingDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_shutting_down: Option<bool>,
    #[doc = "The ID of the request."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
    #[doc = "The result of the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<job_completed_event::Result>,
}
impl JobCompletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_completed_event {
    use super::*;
    #[doc = "The result of the request."]
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
#[doc = "Represents the context of variables and vectors for a job request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobEnvironment {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<ServiceEndpoint>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mask: Vec<MaskHint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(
        rename = "secureFiles",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub secure_files: Vec<SecureFile>,
    #[doc = "Represents an endpoint which may be used by an orchestration job."]
    #[serde(
        rename = "systemConnection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_connection: Option<ServiceEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl JobEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A pipeline job event to be processed by the execution plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobEvent {
    #[doc = "The ID of the pipeline job affected by the event."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The name of the pipeline job event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl JobEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobEventConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl JobEventConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobEventsConfig {
    #[serde(
        rename = "jobAssigned",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_assigned: Option<JobEventConfig>,
    #[serde(
        rename = "jobCompleted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_completed: Option<JobEventConfig>,
    #[serde(
        rename = "jobStarted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_started: Option<JobEventConfig>,
}
impl JobEventsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobMetadataEvent {
    #[serde(flatten)]
    pub job_event: JobEvent,
    #[doc = "A message to be sent to an agent currently running the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<JobMetadataMessage>,
}
impl JobMetadataEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A message to be sent to an agent currently running the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobMetadataMessage {
    #[doc = "The id of the job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The agent's frequency of posting lines to the logs console expressed in milliseconds. There are 2 modes: Slow (10 seconds) and Fast (half a second)."]
    #[serde(
        rename = "postLinesFrequencyMillis",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_lines_frequency_millis: Option<i32>,
}
impl JobMetadataMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an option that may affect the way an agent runs the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Gets the id of the option."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl JobOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRequestMessage {
    #[doc = "Represents the context of variables and vectors for a job request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<JobEnvironment>,
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(
        rename = "jobRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_ref_name: Option<String>,
    #[serde(
        rename = "messageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TaskOrchestrationPlanReference>,
    #[doc = "A reference to a timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<TimelineReference>,
}
impl JobRequestMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStartedEvent {
    #[serde(flatten)]
    pub job_event: JobEvent,
}
impl JobStartedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesResource {
    #[serde(flatten)]
    pub environment_resource: EnvironmentResource,
    #[serde(
        rename = "clusterName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
}
impl KubernetesResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesResourceCreateParameters {
    #[serde(
        rename = "clusterName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Tags of the kubernetes resource."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl KubernetesResourceCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesResourceCreateParametersExistingEndpoint {
    #[serde(flatten)]
    pub kubernetes_resource_create_parameters: KubernetesResourceCreateParameters,
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
}
impl KubernetesResourceCreateParametersExistingEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesResourceCreateParametersNewEndpoint {
    #[serde(flatten)]
    pub kubernetes_resource_create_parameters: KubernetesResourceCreateParameters,
    #[doc = "Represents an endpoint which may be used by an orchestration job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<ServiceEndpoint>,
}
impl KubernetesResourceCreateParametersNewEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesResourcePatchParameters {
    #[serde(
        rename = "authorizationParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_parameters: Option<serde_json::Value>,
    #[doc = "Provider type (CustomProvider or AzureKubernetesServiceProvider) of the resource to be updated"]
    #[serde(
        rename = "providerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_type: Option<String>,
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<i32>,
}
impl KubernetesResourcePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a purchase of resource units in a secondary marketplace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplacePurchasedLicense {
    #[doc = "The Marketplace display name."]
    #[serde(
        rename = "marketplaceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub marketplace_name: Option<String>,
    #[doc = "The name of the identity making the purchase as seen by the marketplace"]
    #[serde(
        rename = "purchaserName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub purchaser_name: Option<String>,
    #[doc = "The quantity purchased."]
    #[serde(
        rename = "purchaseUnitCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_unit_count: Option<i32>,
}
impl MarketplacePurchasedLicense {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaskHint {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<mask_hint::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MaskHint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mask_hint {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "variable")]
        Variable,
        #[serde(rename = "regex")]
        Regex,
    }
}
#[doc = "Meta data for a metrics column."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsColumnMetaData {
    #[doc = "Name."]
    #[serde(
        rename = "columnName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub column_name: Option<String>,
    #[doc = "Data type."]
    #[serde(
        rename = "columnValueType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub column_value_type: Option<String>,
}
impl MetricsColumnMetaData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metrics columns header"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsColumnsHeader {
    #[doc = "Properties of deployment group for which metrics are provided. E.g. 1: LastJobStatus E.g. 2: TargetState"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<MetricsColumnMetaData>,
    #[doc = "The types of metrics. E.g. 1: total count of deployment targets. E.g. 2: Average time of deployment to the deployment targets."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics: Vec<MetricsColumnMetaData>,
}
impl MetricsColumnsHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metrics row."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsRow {
    #[doc = "The values of the properties mentioned as 'Dimensions' in column header. E.g. 1: For a property 'LastJobStatus' - metrics will be provided for 'passed', 'failed', etc. E.g. 2: For a property 'TargetState' - metrics will be provided for 'online', 'offline' targets."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<String>,
    #[doc = "Metrics in serialized format. Should be deserialized based on the data type provided in header."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics: Vec<String>,
}
impl MetricsRow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a downloadable package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageMetadata {
    #[doc = "The date the package was created"]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "A direct link to download the package."]
    #[serde(
        rename = "downloadUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_url: Option<String>,
    #[doc = "The UI uses this to display instructions, i.e. \"unzip MyAgent.zip\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[doc = "MD5 hash as a base64 string"]
    #[serde(rename = "hashValue", default, skip_serializing_if = "Option::is_none")]
    pub hash_value: Option<String>,
    #[doc = "A link to documentation"]
    #[serde(rename = "infoUrl", default, skip_serializing_if = "Option::is_none")]
    pub info_url: Option<String>,
    #[doc = "The platform (win7, linux, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[doc = "The type of package (e.g. \"agent\")"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<PackageVersion>,
}
impl PackageMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<i32>,
}
impl PackageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanEnvironment {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mask: Vec<MaskHint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl PlanEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProjectReference {
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
pub struct PublishTaskGroupMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "parentDefinitionRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_definition_revision: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(
        rename = "taskGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_group_id: Option<String>,
    #[serde(
        rename = "taskGroupRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_group_revision: Option<i32>,
}
impl PublishTaskGroupMetadata {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceFilterOptions {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identities: Vec<IdentityRef>,
    #[serde(
        rename = "resourceTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_types: Vec<String>,
}
impl ResourceFilterOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceFilters {
    #[serde(
        rename = "createdBy",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub created_by: Vec<String>,
    #[serde(
        rename = "resourceType",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_type: Vec<String>,
    #[serde(
        rename = "searchText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_text: Option<String>,
}
impl ResourceFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resources include Service Connections, Variable Groups and Secure Files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceItem {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets or sets description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets icon url of the resource."]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "Gets or sets Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates whether resource is shared with other projects or not."]
    #[serde(rename = "isShared", default, skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[doc = "Gets or sets name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets internal properties of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Gets or sets resource type."]
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<String>,
}
impl ResourceItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLimit {
    #[serde(
        rename = "failedToReachAllProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_to_reach_all_providers: Option<bool>,
    #[serde(rename = "hostId", default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[serde(rename = "isHosted", default, skip_serializing_if = "Option::is_none")]
    pub is_hosted: Option<bool>,
    #[serde(rename = "isPremium", default, skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    #[serde(
        rename = "parallelismTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parallelism_tag: Option<String>,
    #[serde(
        rename = "resourceLimitsData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_limits_data: Option<serde_json::Value>,
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i32>,
    #[serde(
        rename = "totalMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_minutes: Option<i32>,
}
impl ResourceLimit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request for a resource's exclusive lock"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLockRequest {
    #[doc = "The date/time this request was assigned."]
    #[serde(
        rename = "assignTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub assign_time: Option<time::OffsetDateTime>,
    #[doc = "The ID of the check run waiting on this request"]
    #[serde(
        rename = "checkRunId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_run_id: Option<String>,
    #[doc = "The ID of the pipeline that requested this resource"]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "The date/time this request was finished."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The behavior this request should exhibit in relation to other lock requests"]
    #[serde(rename = "lockType", default, skip_serializing_if = "Option::is_none")]
    pub lock_type: Option<resource_lock_request::LockType>,
    #[doc = "Attempt of the graph node"]
    #[serde(
        rename = "nodeAttempt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_attempt: Option<i32>,
    #[doc = "Name of the graph node (currently stage) requesting this resource"]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Internal ID for the orchestration plan connected with this request."]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "The ID of the project of the check run and definition exist in"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The date/time this request was queued."]
    #[serde(
        rename = "queueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queue_time: Option<time::OffsetDateTime>,
    #[doc = "ID of the request."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
    #[doc = "The id of the resource"]
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<String>,
    #[doc = "The type of the resource"]
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<String>,
    #[doc = "The result of this request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<resource_lock_request::Status>,
}
impl ResourceLockRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_lock_request {
    use super::*;
    #[doc = "The behavior this request should exhibit in relation to other lock requests"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LockType {
        #[serde(rename = "runLatest")]
        RunLatest,
        #[serde(rename = "sequential")]
        Sequential,
    }
    #[doc = "The result of this request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inUse")]
        InUse,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "abandoned")]
        Abandoned,
        #[serde(rename = "waitingOnChecks")]
        WaitingOnChecks,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    #[serde(
        rename = "resourceLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_limit: Option<ResourceLimit>,
    #[serde(
        rename = "runningRequests",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub running_requests: Vec<TaskAgentJobRequest>,
    #[serde(rename = "usedCount", default, skip_serializing_if = "Option::is_none")]
    pub used_count: Option<i32>,
    #[serde(
        rename = "usedMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub used_minutes: Option<i32>,
}
impl ResourceUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesHubData {
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[serde(
        rename = "hasProjectLevelManagePermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_project_level_manage_permission: Option<bool>,
    #[serde(
        rename = "resourceFilterOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_filter_options: Option<ResourceFilterOptions>,
    #[serde(
        rename = "resourceFilters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_filters: Option<ResourceFilters>,
    #[serde(
        rename = "resourceItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_items: Vec<ResourceItem>,
}
impl ResourcesHubData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResultTransformationDetails {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureFile {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
}
impl SecureFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureFileEvent {
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "secureFiles",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub secure_files: Vec<SecureFile>,
}
impl SecureFileEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SendJobResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<JobEventsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl SendJobResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerExecutionDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<EventsConfig>,
    #[serde(
        rename = "handlerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub handler_name: Option<String>,
}
impl ServerExecutionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTaskRequestMessage {
    #[serde(flatten)]
    pub job_request_message: JobRequestMessage,
    #[serde(
        rename = "taskDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_definition: Option<TaskDefinition>,
    #[serde(
        rename = "taskInstance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_instance: Option<TaskInstance>,
}
impl ServerTaskRequestMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an endpoint which may be used by an orchestration job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpoint {
    #[serde(
        rename = "administratorsGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub administrators_group: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<EndpointAuthorization>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Gets or sets the description of endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "groupScopeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_scope_id: Option<String>,
    #[doc = "Gets or sets the identifier of this endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "EndPoint state indicator"]
    #[serde(rename = "isReady", default, skip_serializing_if = "Option::is_none")]
    pub is_ready: Option<bool>,
    #[doc = "Indicates whether service endpoint is shared with other projects or not."]
    #[serde(rename = "isShared", default, skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[doc = "Gets or sets the friendly name of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Error message during creation/deletion of endpoint"]
    #[serde(
        rename = "operationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_status: Option<serde_json::Value>,
    #[doc = "Gets or sets the owner of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(
        rename = "readersGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub readers_group: Option<IdentityRef>,
    #[doc = "Gets or sets the type of the endpoint."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the url of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ServiceEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[doc = "Gets or sets the certificates of service endpoint authentication scheme."]
    #[serde(
        rename = "clientCertificates",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub client_certificates: Vec<ClientCertificate>,
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
    #[doc = "Gets or sets the scheme for service endpoint authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}
impl ServiceEndpointAuthenticationScheme {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<EndpointAuthorization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<TaskOrchestrationOwner>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskOrchestrationOwner>,
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
pub struct ServiceEndpointRequest {
    #[serde(
        rename = "dataSourceDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_details: Option<DataSourceDetails>,
    #[serde(
        rename = "resultTransformationDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_transformation_details: Option<ResultTransformationDetails>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointRequestResult {
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<JToken>,
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
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<HelpLink>,
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
#[doc = "A task agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgent {
    #[serde(flatten)]
    pub task_agent_reference: TaskAgentReference,
    #[serde(
        rename = "assignedAgentCloudRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_agent_cloud_request: Option<TaskAgentCloudRequest>,
    #[doc = "A job request for an agent."]
    #[serde(
        rename = "assignedRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_request: Option<TaskAgentJobRequest>,
    #[doc = "Provides data necessary for authorizing the agent using OAuth 2.0 authentication flows."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<TaskAgentAuthorization>,
    #[doc = "Date on which this agent was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "A job request for an agent."]
    #[serde(
        rename = "lastCompletedRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_completed_request: Option<TaskAgentJobRequest>,
    #[doc = "Maximum job parallelism allowed for this agent."]
    #[serde(
        rename = "maxParallelism",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallelism: Option<i32>,
    #[doc = "Details about an agent update."]
    #[serde(
        rename = "pendingUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_update: Option<TaskAgentUpdate>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Date on which the last connectivity status change occurred."]
    #[serde(
        rename = "statusChangedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub status_changed_on: Option<time::OffsetDateTime>,
    #[doc = "System-defined capabilities supported by this agent's host. Warning: To set capabilities use the PUT method, PUT will completely overwrite existing capabilities."]
    #[serde(
        rename = "systemCapabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_capabilities: Option<serde_json::Value>,
    #[doc = "User-defined capabilities supported by this agent's host. Warning: To set capabilities use the PUT method, PUT will completely overwrite existing capabilities."]
    #[serde(
        rename = "userCapabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_capabilities: Option<serde_json::Value>,
}
impl TaskAgent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides data necessary for authorizing the agent using OAuth 2.0 authentication flows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentAuthorization {
    #[doc = "Endpoint used to obtain access tokens from the configured token service."]
    #[serde(
        rename = "authorizationUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_url: Option<String>,
    #[doc = "Client identifier for this agent."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Represents the public key portion of an RSA asymmetric key."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<TaskAgentPublicKey>,
}
impl TaskAgentAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentCloud {
    #[doc = "Gets or sets a AcquireAgentEndpoint using which a request can be made to acquire new agent"]
    #[serde(
        rename = "acquireAgentEndpoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub acquire_agent_endpoint: Option<String>,
    #[serde(
        rename = "acquisitionTimeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub acquisition_timeout: Option<i32>,
    #[serde(
        rename = "agentCloudId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_cloud_id: Option<i32>,
    #[serde(
        rename = "getAccountParallelismEndpoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub get_account_parallelism_endpoint: Option<String>,
    #[serde(
        rename = "getAgentDefinitionEndpoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub get_agent_definition_endpoint: Option<String>,
    #[serde(
        rename = "getAgentRequestStatusEndpoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub get_agent_request_status_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Signifies that this Agent Cloud is internal and should not be user-manageable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(
        rename = "maxParallelism",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallelism: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "releaseAgentEndpoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_agent_endpoint: Option<String>,
    #[serde(
        rename = "sharedSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_secret: Option<String>,
    #[doc = "Gets or sets the type of the endpoint."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl TaskAgentCloud {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentCloudList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskAgentCloud>,
}
impl TaskAgentCloudList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentCloudRequest {
    #[doc = "A reference to an agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<TaskAgentReference>,
    #[serde(
        rename = "agentCloudId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_cloud_id: Option<i32>,
    #[serde(
        rename = "agentConnectedTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub agent_connected_time: Option<time::OffsetDateTime>,
    #[serde(rename = "agentData", default, skip_serializing_if = "Option::is_none")]
    pub agent_data: Option<serde_json::Value>,
    #[serde(
        rename = "agentSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_specification: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[serde(
        rename = "provisionedTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub provisioned_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "provisionRequestTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub provision_request_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "releaseRequestTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub release_request_time: Option<time::OffsetDateTime>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl TaskAgentCloudRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentCloudRequestList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskAgentCloudRequest>,
}
impl TaskAgentCloudRequestList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentCloudType {
    #[doc = "Gets or sets the display name of agent cloud type."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the input descriptors"]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "Gets or sets the name of agent cloud type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TaskAgentCloudType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentCloudTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskAgentCloudType>,
}
impl TaskAgentCloudTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentDowngrade {
    #[serde(flatten)]
    pub task_agent_update_reason: TaskAgentUpdateReason,
}
impl TaskAgentDowngrade {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "sidecarContainers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sidecar_containers: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub steps: Vec<TaskAgentJobStep>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variables: Vec<TaskAgentJobVariable>,
}
impl TaskAgentJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A job request for an agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentJobRequest {
    #[serde(
        rename = "agentSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_specification: Option<serde_json::Value>,
    #[doc = "The date/time this request was assigned."]
    #[serde(
        rename = "assignTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub assign_time: Option<time::OffsetDateTime>,
    #[doc = "Additional data about the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<TaskOrchestrationOwner>,
    #[doc = "A list of demands required to fulfill this request."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[doc = "The date/time this request was finished."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The host which triggered this request."]
    #[serde(rename = "hostId", default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[doc = "ID of the job resulting from this request."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Name of the job resulting from this request."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "The deadline for the agent to renew the lock."]
    #[serde(
        rename = "lockedUntil",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub locked_until: Option<time::OffsetDateTime>,
    #[serde(
        rename = "matchedAgents",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub matched_agents: Vec<TaskAgentReference>,
    #[serde(
        rename = "matchesAllAgentsInPool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub matches_all_agents_in_pool: Option<bool>,
    #[serde(
        rename = "orchestrationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orchestration_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskOrchestrationOwner>,
    #[serde(rename = "planGroup", default, skip_serializing_if = "Option::is_none")]
    pub plan_group: Option<String>,
    #[doc = "Internal ID for the orchestration plan connected with this request."]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Internal detail representing the type of orchestration plan."]
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[doc = "The ID of the pool this request targets"]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "The ID of the queue this request targets"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[doc = "The date/time this request was queued."]
    #[serde(
        rename = "queueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queue_time: Option<time::OffsetDateTime>,
    #[doc = "The date/time this request was receieved by an agent."]
    #[serde(
        rename = "receiveTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub receive_time: Option<time::OffsetDateTime>,
    #[doc = "ID of the request."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
    #[doc = "A reference to an agent."]
    #[serde(
        rename = "reservedAgent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reserved_agent: Option<TaskAgentReference>,
    #[doc = "The result of this request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<task_agent_job_request::Result>,
    #[doc = "Scope of the pipeline; matches the project ID."]
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[doc = "The service which owns this request."]
    #[serde(
        rename = "serviceOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_owner: Option<String>,
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
    #[serde(
        rename = "userDelayed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_delayed: Option<bool>,
}
impl TaskAgentJobRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_job_request {
    use super::*;
    #[doc = "The result of this request."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentJobStep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "retryCountOnTaskFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_count_on_task_failure: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskAgentJobTask>,
    #[serde(
        rename = "timeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_in_minutes: Option<i32>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<task_agent_job_step::Type>,
}
impl TaskAgentJobStep {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_job_step {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "task")]
        Task,
        #[serde(rename = "action")]
        Action,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentJobTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl TaskAgentJobTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentJobVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TaskAgentJobVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskAgent>,
}
impl TaskAgentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentManualUpdate {
    #[serde(flatten)]
    pub task_agent_update_reason: TaskAgentUpdateReason,
}
impl TaskAgentManualUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides a contract for receiving messages from the task orchestrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentMessage {
    #[doc = "Gets or sets the body of the message. If the <c>IV</c> property is provided the body will need to be decrypted using the <c>TaskAgentSession.EncryptionKey</c> value in addition to the <c>IV</c>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Gets or sets the initialization vector used to encrypt this message."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub iv: Vec<String>,
    #[doc = "Gets or sets the message identifier."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[doc = "Gets or sets the message type, describing the data contract found in <c>TaskAgentMessage.Body</c>."]
    #[serde(
        rename = "messageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<String>,
}
impl TaskAgentMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentMinAgentVersionRequiredUpdate {
    #[serde(flatten)]
    pub task_agent_update_reason: TaskAgentUpdateReason,
    #[serde(
        rename = "jobDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_definition: Option<TaskOrchestrationOwner>,
    #[serde(rename = "jobOwner", default, skip_serializing_if = "Option::is_none")]
    pub job_owner: Option<TaskOrchestrationOwner>,
    #[serde(
        rename = "minAgentVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_agent_version: Option<Demand>,
}
impl TaskAgentMinAgentVersionRequiredUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An organization-level grouping of agents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPool {
    #[serde(flatten)]
    pub task_agent_pool_reference: TaskAgentPoolReference,
    #[doc = "The ID of the associated agent cloud."]
    #[serde(
        rename = "agentCloudId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_cloud_id: Option<i32>,
    #[doc = "Whether or not a queue should be automatically provisioned for each project collection."]
    #[serde(
        rename = "autoProvision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_provision: Option<bool>,
    #[doc = "Whether or not the pool should autosize itself based on the Agent Cloud Provider settings."]
    #[serde(rename = "autoSize", default, skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<bool>,
    #[doc = "Whether or not agents in this pool are allowed to automatically update"]
    #[serde(
        rename = "autoUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_update: Option<bool>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "The date/time of the pool creation."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Target parallelism - Only applies to agent pools that are backed by pool providers. It will be null for regular pools."]
    #[serde(
        rename = "targetSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_size: Option<i32>,
}
impl TaskAgentPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskAgentPool>,
}
impl TaskAgentPoolList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolMaintenanceDefinition {
    #[doc = "Enable maintenance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Maintenance job timeout per agent"]
    #[serde(
        rename = "jobTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_timeout_in_minutes: Option<i32>,
    #[doc = "Max percentage of agents within a pool running maintenance job at given time"]
    #[serde(
        rename = "maxConcurrentAgentsPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrent_agents_percentage: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<TaskAgentPoolMaintenanceOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[serde(
        rename = "retentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_policy: Option<TaskAgentPoolMaintenanceRetentionPolicy>,
    #[serde(
        rename = "scheduleSetting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_setting: Option<TaskAgentPoolMaintenanceSchedule>,
}
impl TaskAgentPoolMaintenanceDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolMaintenanceJob {
    #[doc = "The maintenance definition for the maintenance job"]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "The total error counts during the maintenance job"]
    #[serde(
        rename = "errorCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_count: Option<i32>,
    #[doc = "Time that the maintenance job was completed"]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "Id of the maintenance job"]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i32>,
    #[doc = "The log download url for the maintenance job"]
    #[serde(
        rename = "logsDownloadUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logs_download_url: Option<String>,
    #[doc = "Orchestration/Plan Id for the maintenance job"]
    #[serde(
        rename = "orchestrationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orchestration_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[doc = "Time that the maintenance job was queued"]
    #[serde(
        rename = "queueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queue_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<IdentityRef>,
    #[doc = "The maintenance job result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<task_agent_pool_maintenance_job::Result>,
    #[doc = "Time that the maintenance job was started"]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Status of the maintenance job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_agent_pool_maintenance_job::Status>,
    #[serde(
        rename = "targetAgents",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_agents: Vec<TaskAgentPoolMaintenanceJobTargetAgent>,
    #[doc = "The total warning counts during the maintenance job"]
    #[serde(
        rename = "warningCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_count: Option<i32>,
}
impl TaskAgentPoolMaintenanceJob {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_pool_maintenance_job {
    use super::*;
    #[doc = "The maintenance job result"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
    }
    #[doc = "Status of the maintenance job"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "queued")]
        Queued,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolMaintenanceJobTargetAgent {
    #[doc = "A reference to an agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<TaskAgentReference>,
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<task_agent_pool_maintenance_job_target_agent::Result>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_agent_pool_maintenance_job_target_agent::Status>,
}
impl TaskAgentPoolMaintenanceJobTargetAgent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_pool_maintenance_job_target_agent {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "queued")]
        Queued,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolMaintenanceOptions {
    #[doc = "time to consider a System.DefaultWorkingDirectory is stale"]
    #[serde(
        rename = "workingDirectoryExpirationInDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_directory_expiration_in_days: Option<i32>,
}
impl TaskAgentPoolMaintenanceOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolMaintenanceRetentionPolicy {
    #[doc = "Number of records to keep for maintenance job executed with this definition."]
    #[serde(
        rename = "numberOfHistoryRecordsToKeep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_history_records_to_keep: Option<i32>,
}
impl TaskAgentPoolMaintenanceRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolMaintenanceSchedule {
    #[doc = "Days for a build (flags enum for days of the week)"]
    #[serde(
        rename = "daysToBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_build: Option<task_agent_pool_maintenance_schedule::DaysToBuild>,
    #[doc = "The Job Id of the Scheduled job that will queue the pool maintenance job."]
    #[serde(
        rename = "scheduleJobId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_job_id: Option<String>,
    #[doc = "Local timezone hour to start"]
    #[serde(
        rename = "startHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_hours: Option<i32>,
    #[doc = "Local timezone minute to start"]
    #[serde(
        rename = "startMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_minutes: Option<i32>,
    #[doc = "Time zone of the build schedule (string representation of the time zone id)"]
    #[serde(
        rename = "timeZoneId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_zone_id: Option<String>,
}
impl TaskAgentPoolMaintenanceSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_pool_maintenance_schedule {
    use super::*;
    #[doc = "Days for a build (flags enum for days of the week)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DaysToBuild {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "monday")]
        Monday,
        #[serde(rename = "tuesday")]
        Tuesday,
        #[serde(rename = "wednesday")]
        Wednesday,
        #[serde(rename = "thursday")]
        Thursday,
        #[serde(rename = "friday")]
        Friday,
        #[serde(rename = "saturday")]
        Saturday,
        #[serde(rename = "sunday")]
        Sunday,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets a value indicating whether or not this pool is managed by the service."]
    #[serde(rename = "isHosted", default, skip_serializing_if = "Option::is_none")]
    pub is_hosted: Option<bool>,
    #[doc = "Determines whether the pool is legacy."]
    #[serde(rename = "isLegacy", default, skip_serializing_if = "Option::is_none")]
    pub is_legacy: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Additional pool settings and details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<task_agent_pool_reference::Options>,
    #[doc = "Gets or sets the type of the pool"]
    #[serde(rename = "poolType", default, skip_serializing_if = "Option::is_none")]
    pub pool_type: Option<task_agent_pool_reference::PoolType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Gets the current size of the pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
impl TaskAgentPoolReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_pool_reference {
    use super::*;
    #[doc = "Additional pool settings and details"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Options {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "elasticPool")]
        ElasticPool,
        #[serde(rename = "singleUseAgents")]
        SingleUseAgents,
        #[serde(rename = "preserveAgentOnJobFailure")]
        PreserveAgentOnJobFailure,
    }
    #[doc = "Gets or sets the type of the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PoolType {
        #[serde(rename = "automation")]
        Automation,
        #[serde(rename = "deployment")]
        Deployment,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolStatus {
    #[serde(flatten)]
    pub task_agent_pool_reference: TaskAgentPoolReference,
    #[doc = "Number of requests queued and assigned to an agent. Not running yet."]
    #[serde(
        rename = "assignedRequestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_request_count: Option<i32>,
    #[doc = "Number of queued requests which are not assigned to any agents"]
    #[serde(
        rename = "queuedRequestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queued_request_count: Option<i32>,
    #[doc = "Number of currently running requests"]
    #[serde(
        rename = "runningRequestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub running_request_count: Option<i32>,
}
impl TaskAgentPoolStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPoolSummary {
    #[doc = "Metrics columns header"]
    #[serde(
        rename = "columnsHeader",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub columns_header: Option<MetricsColumnsHeader>,
    #[serde(
        rename = "deploymentGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployment_groups: Vec<DeploymentGroupReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub queues: Vec<TaskAgentQueue>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rows: Vec<MetricsRow>,
}
impl TaskAgentPoolSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the public key portion of an RSA asymmetric key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentPublicKey {
    #[doc = "Gets or sets the exponent for the public key."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exponent: Vec<String>,
    #[doc = "Gets or sets the modulus for the public key."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub modulus: Vec<String>,
}
impl TaskAgentPublicKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An agent queue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentQueue {
    #[doc = "ID of the queue"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the queue"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[doc = "Project ID"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl TaskAgentQueue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentQueueList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskAgentQueue>,
}
impl TaskAgentQueueList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to an agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentReference {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "This agent's access point."]
    #[serde(
        rename = "accessPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_point: Option<String>,
    #[doc = "Whether or not this agent should run jobs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Identifier of the agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Agent OS."]
    #[serde(
        rename = "osDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub os_description: Option<String>,
    #[doc = "Provisioning state of this agent."]
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[doc = "Whether or not the agent is online."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_agent_reference::Status>,
    #[doc = "Agent version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl TaskAgentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_reference {
    use super::*;
    #[doc = "Whether or not the agent is online."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "offline")]
        Offline,
        #[serde(rename = "online")]
        Online,
    }
}
#[doc = "Represents a session for performing message exchanges from an agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentSession {
    #[doc = "A reference to an agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<TaskAgentReference>,
    #[doc = "Represents a symmetric key used for message-level encryption for communication sent to an agent."]
    #[serde(
        rename = "encryptionKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_key: Option<TaskAgentSessionKey>,
    #[doc = "Gets or sets the owner name of this session. Generally this will be the machine of origination."]
    #[serde(rename = "ownerName", default, skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[doc = "Gets the unique identifier for this session."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(
        rename = "systemCapabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_capabilities: Option<serde_json::Value>,
}
impl TaskAgentSession {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a symmetric key used for message-level encryption for communication sent to an agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentSessionKey {
    #[doc = "Gets or sets a value indicating whether or not the key value is encrypted. If this value is true, the Value property should be decrypted using the <c>RSA</c> key exchanged with the server during registration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[doc = "Gets or sets the symmetric key value."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
}
impl TaskAgentSessionKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about an agent update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentUpdate {
    #[doc = "Current state of this agent update."]
    #[serde(
        rename = "currentState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<TaskAgentUpdateReason>,
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<IdentityRef>,
    #[doc = "Date on which this update was requested."]
    #[serde(
        rename = "requestTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub request_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "sourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_version: Option<PackageVersion>,
    #[serde(
        rename = "targetVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_version: Option<PackageVersion>,
}
impl TaskAgentUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAgentUpdateReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<task_agent_update_reason::Code>,
}
impl TaskAgentUpdateReason {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_agent_update_reason {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Code {
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "minAgentVersionRequired")]
        MinAgentVersionRequired,
        #[serde(rename = "downgrade")]
        Downgrade,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAssignedEvent {
    #[serde(flatten)]
    pub task_event: TaskEvent,
}
impl TaskAssignedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskAttachment {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(
        rename = "lastChangedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_changed_by: Option<String>,
    #[serde(
        rename = "lastChangedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_on: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recordId", default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(
        rename = "timelineId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl TaskAttachment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskCommandRestrictions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<task_command_restrictions::Mode>,
}
impl TaskCommandRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_command_restrictions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        #[serde(rename = "any")]
        Any,
        #[serde(rename = "restricted")]
        Restricted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskCompletedEvent {
    #[serde(flatten)]
    pub task_event: TaskEvent,
    #[doc = "The api request was no delivered successfully"]
    #[serde(
        rename = "deliveryFailed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_failed: Option<bool>,
    #[doc = "The result of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<task_completed_event::Result>,
}
impl TaskCompletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_completed_event {
    use super::*;
    #[doc = "The result of the task."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskDefinition {
    #[serde(
        rename = "agentExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_execution: Option<TaskExecution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(
        rename = "contentsUploaded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contents_uploaded: Option<bool>,
    #[serde(
        rename = "contributionIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contribution_identifier: Option<String>,
    #[serde(
        rename = "contributionVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contribution_version: Option<String>,
    #[serde(
        rename = "dataSourceBindings",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_source_bindings: Vec<DataSourceBinding>,
    #[serde(
        rename = "definitionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<serde_json::Value>,
    #[serde(
        rename = "friendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<TaskGroupDefinition>,
    #[serde(
        rename = "helpMarkDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub help_mark_down: Option<String>,
    #[serde(rename = "helpUrl", default, skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
    #[serde(rename = "hostType", default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<String>,
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<TaskInputDefinition>,
    #[serde(
        rename = "instanceNameFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_name_format: Option<String>,
    #[serde(
        rename = "minimumAgentVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_agent_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "outputVariables",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output_variables: Vec<TaskOutputVariable>,
    #[serde(
        rename = "packageLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_location: Option<String>,
    #[serde(
        rename = "packageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_type: Option<String>,
    #[serde(
        rename = "postJobExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_job_execution: Option<serde_json::Value>,
    #[serde(
        rename = "preJobExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_job_execution: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(
        rename = "releaseNotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<TaskRestrictions>,
    #[serde(
        rename = "runsOn",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub runs_on: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub satisfies: Vec<String>,
    #[serde(
        rename = "serverOwned",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_owned: Option<bool>,
    #[serde(
        rename = "showEnvironmentVariables",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_environment_variables: Option<bool>,
    #[serde(
        rename = "sourceDefinitions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_definitions: Vec<TaskSourceDefinition>,
    #[serde(
        rename = "sourceLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<TaskVersion>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub visibility: Vec<String>,
}
impl TaskDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskDefinitionEndpoint {
    #[doc = "An ID that identifies a service connection to be used for authenticating endpoint requests."]
    #[serde(
        rename = "connectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_id: Option<String>,
    #[doc = "An Json based keyselector to filter response returned by fetching the endpoint <c>Url</c>.A Json based keyselector must be prefixed with \"jsonpath:\". KeySelector can be used to specify the filter to get the keys for the values specified with Selector. <example> The following keyselector defines an Json for extracting nodes named 'ServiceName'. <code> endpoint.KeySelector = \"jsonpath://ServiceName\"; </code></example>"]
    #[serde(
        rename = "keySelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_selector: Option<String>,
    #[doc = "The scope as understood by Connected Services. Essentially, a project-id for now."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "An XPath/Json based selector to filter response returned by fetching the endpoint <c>Url</c>. An XPath based selector must be prefixed with the string \"xpath:\". A Json based selector must be prefixed with \"jsonpath:\". <example> The following selector defines an XPath for extracting nodes named 'ServiceName'. <code> endpoint.Selector = \"xpath://ServiceName\"; </code></example>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[doc = "TaskId that this endpoint belongs to."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "URL to GET."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TaskDefinitionEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskDefinitionReference {
    #[doc = "Gets or sets the definition type. Values can be 'task' or 'metaTask'."]
    #[serde(
        rename = "definitionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_type: Option<String>,
    #[doc = "Gets or sets the unique identifier of task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the version specification of task."]
    #[serde(
        rename = "versionSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_spec: Option<String>,
}
impl TaskDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskEvent {
    #[serde(flatten)]
    pub job_event: JobEvent,
    #[doc = "The ID of the task definition."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}
impl TaskEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskExecution {
    #[doc = "A reference to a task."]
    #[serde(rename = "execTask", default, skip_serializing_if = "Option::is_none")]
    pub exec_task: Option<TaskReference>,
    #[doc = "If a task is going to run code, then this provides the type/script etc... information by platform. For example, it might look like. net45: { typeName: \"Microsoft.TeamFoundation.Automation.Tasks.PowerShellTask\", assemblyName: \"Microsoft.TeamFoundation.Automation.Tasks.PowerShell.dll\" } net20: { typeName: \"Microsoft.TeamFoundation.Automation.Tasks.PowerShellTask\", assemblyName: \"Microsoft.TeamFoundation.Automation.Tasks.PowerShell.dll\" } java: { jar: \"powershelltask.tasks.automation.teamfoundation.microsoft.com\", } node: { script: \"powershellhost.js\", }"]
    #[serde(
        rename = "platformInstructions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_instructions: Option<serde_json::Value>,
}
impl TaskExecution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroup {
    #[serde(flatten)]
    pub task_definition: TaskDefinition,
    #[doc = "Gets or sets comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets or sets date on which it got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets as 'true' to indicate as deleted, 'false' otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Gets or sets date on which it got modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Gets or sets parent task group Id. This is used while creating a draft task group."]
    #[serde(
        rename = "parentDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_definition_id: Option<String>,
    #[doc = "Gets or sets revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "Gets or sets the tasks."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<TaskGroupStep>,
}
impl TaskGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupCreateParameter {
    #[doc = "Sets author name of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "Sets category of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Sets description of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Sets friendly name of the task group."]
    #[serde(
        rename = "friendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<String>,
    #[doc = "Sets url icon of the task group."]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "Sets input for the task group."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<TaskInputDefinition>,
    #[doc = "Sets display name of the task group."]
    #[serde(
        rename = "instanceNameFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_name_format: Option<String>,
    #[doc = "Sets name of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Sets parent task group Id. This is used while creating a draft task group."]
    #[serde(
        rename = "parentDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_definition_id: Option<String>,
    #[doc = "Sets RunsOn of the task group. Value can be 'Agent', 'Server' or 'DeploymentGroup'."]
    #[serde(
        rename = "runsOn",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub runs_on: Vec<String>,
    #[doc = "Sets tasks for the task group."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<TaskGroupStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<TaskVersion>,
}
impl TaskGroupCreateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupDefinition {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        rename = "isExpanded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_expanded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[serde(
        rename = "visibleRule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visible_rule: Option<String>,
}
impl TaskGroupDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskGroup>,
}
impl TaskGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupPublishPreviewParameter {
    #[serde(flatten)]
    pub task_group_update_properties_base: TaskGroupUpdatePropertiesBase,
    #[doc = "This is to disable previous versions of task group upon publish"]
    #[serde(
        rename = "disablePriorVersions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_prior_versions: Option<bool>,
    #[doc = "Denotes if task group is in preview"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[doc = "This is the revision of task group that is getting published"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<TaskVersion>,
}
impl TaskGroupPublishPreviewParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupRestoreParameter {
    #[serde(flatten)]
    pub task_group_update_properties_base: TaskGroupUpdatePropertiesBase,
    #[doc = "This is to restore deleted Task Group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<bool>,
}
impl TaskGroupRestoreParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupRevision {
    #[serde(rename = "changedBy", default, skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<IdentityRef>,
    #[serde(
        rename = "changedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub changed_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<task_group_revision::ChangeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i32>,
    #[serde(
        rename = "majorVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub major_version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(
        rename = "taskGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_group_id: Option<String>,
}
impl TaskGroupRevision {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_group_revision {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "undelete")]
        Undelete,
    }
}
#[doc = "Represents tasks in the task group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupStep {
    #[doc = "Gets or sets as 'true' to run the task always, 'false' otherwise."]
    #[serde(rename = "alwaysRun", default, skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    #[doc = "Gets or sets condition for the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Gets or sets as 'true' to continue on error, 'false' otherwise."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "Gets or sets the display name."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets or sets as task is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets dictionary of environment variables."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    #[doc = "Gets or sets dictionary of inputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Gets or sets the maximum number of retries"]
    #[serde(
        rename = "retryCountOnTaskFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_count_on_task_failure: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskDefinitionReference>,
    #[doc = "Gets or sets the maximum time, in minutes, that a task is allowed to execute on agent before being cancelled by server. A zero value indicates an infinite timeout."]
    #[serde(
        rename = "timeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_in_minutes: Option<i32>,
}
impl TaskGroupStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupUpdateParameter {
    #[doc = "Sets author name of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "Sets category of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Sets comment of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Sets description of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Sets friendly name of the task group."]
    #[serde(
        rename = "friendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<String>,
    #[doc = "Sets url icon of the task group."]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "Sets the unique identifier of this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Sets input for the task group."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<TaskInputDefinition>,
    #[doc = "Sets display name of the task group."]
    #[serde(
        rename = "instanceNameFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_name_format: Option<String>,
    #[doc = "Sets name of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets parent task group Id. This is used while creating a draft task group."]
    #[serde(
        rename = "parentDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_definition_id: Option<String>,
    #[doc = "Sets revision of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "Sets RunsOn of the task group. Value can be 'Agent', 'Server' or 'DeploymentGroup'."]
    #[serde(
        rename = "runsOn",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub runs_on: Vec<String>,
    #[doc = "Sets tasks for the task group."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<TaskGroupStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<TaskVersion>,
}
impl TaskGroupUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskGroupUpdatePropertiesBase {
    #[doc = "Comment for this update request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
impl TaskGroupUpdatePropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskHubLicenseDetails {
    #[serde(
        rename = "enterpriseUsersCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise_users_count: Option<i32>,
    #[serde(
        rename = "failedToReachAllProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_to_reach_all_providers: Option<bool>,
    #[serde(
        rename = "freeHostedLicenseCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub free_hosted_license_count: Option<i32>,
    #[serde(
        rename = "freeLicenseCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub free_license_count: Option<i32>,
    #[serde(
        rename = "hasLicenseCountEverUpdated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_license_count_ever_updated: Option<bool>,
    #[serde(
        rename = "hostedAgentMinutesFreeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hosted_agent_minutes_free_count: Option<i32>,
    #[serde(
        rename = "hostedAgentMinutesUsedCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hosted_agent_minutes_used_count: Option<i32>,
    #[serde(
        rename = "hostedLicensesArePremium",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hosted_licenses_are_premium: Option<bool>,
    #[serde(
        rename = "msdnUsersCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub msdn_users_count: Option<i32>,
    #[doc = "Microsoft-hosted licenses purchased from VSTS directly."]
    #[serde(
        rename = "purchasedHostedLicenseCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub purchased_hosted_license_count: Option<i32>,
    #[doc = "Self-hosted licenses purchased from VSTS directly."]
    #[serde(
        rename = "purchasedLicenseCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub purchased_license_count: Option<i32>,
    #[serde(
        rename = "totalHostedLicenseCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_hosted_license_count: Option<i32>,
    #[serde(
        rename = "totalLicenseCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_license_count: Option<i32>,
    #[serde(
        rename = "totalPrivateLicenseCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_private_license_count: Option<i32>,
}
impl TaskHubLicenseDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskHubOidcToken {
    #[serde(rename = "oidcToken", default, skip_serializing_if = "Option::is_none")]
    pub oidc_token: Option<String>,
}
impl TaskHubOidcToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskInputDefinition {
    #[serde(flatten)]
    pub task_input_definition_base: TaskInputDefinitionBase,
}
impl TaskInputDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskInputDefinitionBase {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aliases: Vec<String>,
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(
        rename = "helpMarkDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub help_mark_down: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<TaskInputValidation>,
    #[serde(
        rename = "visibleRule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visible_rule: Option<String>,
}
impl TaskInputDefinitionBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskInputValidation {
    #[doc = "Conditional expression"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "Message explaining how user can correct if validation fails"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl TaskInputValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskInstance {
    #[serde(flatten)]
    pub task_reference: TaskReference,
    #[serde(rename = "alwaysRun", default, skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    #[serde(
        rename = "instanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_id: Option<String>,
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[serde(
        rename = "retryCountOnTaskFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_count_on_task_failure: Option<i32>,
    #[serde(
        rename = "timeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_in_minutes: Option<i32>,
}
impl TaskInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A task log connected to a timeline record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskLog {
    #[serde(flatten)]
    pub task_log_reference: TaskLogReference,
    #[doc = "The time of the task log creation."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The REST URL of the task log when indexed."]
    #[serde(
        rename = "indexLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_location: Option<String>,
    #[doc = "The time of the last modification of the task log."]
    #[serde(
        rename = "lastChangedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_on: Option<time::OffsetDateTime>,
    #[doc = "The number of the task log lines."]
    #[serde(rename = "lineCount", default, skip_serializing_if = "Option::is_none")]
    pub line_count: Option<i64>,
    #[doc = "The path of the task log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl TaskLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to a task log. This class contains information about the output printed to the timeline record's logs console during pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskLogReference {
    #[doc = "The ID of the task log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The REST URL of the task log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TaskLogReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationContainer {
    #[serde(flatten)]
    pub task_orchestration_item: TaskOrchestrationItem,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<TaskOrchestrationItem>,
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(
        rename = "maxConcurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrency: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback: Option<Box<TaskOrchestrationContainer>>,
}
impl TaskOrchestrationContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationItem {
    #[serde(rename = "itemType", default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<task_orchestration_item::ItemType>,
}
impl TaskOrchestrationItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_orchestration_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ItemType {
        #[serde(rename = "container")]
        Container,
        #[serde(rename = "job")]
        Job,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationJob {
    #[serde(flatten)]
    pub task_orchestration_item: TaskOrchestrationItem,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[serde(rename = "executeAs", default, skip_serializing_if = "Option::is_none")]
    pub execute_as: Option<IdentityRef>,
    #[serde(
        rename = "executionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_mode: Option<String>,
    #[serde(
        rename = "executionTimeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_timeout: Option<String>,
    #[serde(
        rename = "instanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<TaskInstance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl TaskOrchestrationJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationOwner {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TaskOrchestrationOwner {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationPlan {
    #[serde(flatten)]
    pub task_orchestration_plan_reference: TaskOrchestrationPlanReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<PlanEnvironment>,
    #[doc = "A reference to a task log. This class contains information about the output printed to the timeline record's logs console during pipeline run."]
    #[serde(
        rename = "expandedYaml",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expanded_yaml: Option<TaskLogReference>,
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation: Option<TaskOrchestrationContainer>,
    #[doc = "A reference to a task log. This class contains information about the output printed to the timeline record's logs console during pipeline run."]
    #[serde(
        rename = "initializationLog",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initialization_log: Option<TaskLogReference>,
    #[serde(
        rename = "requestedById",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by_id: Option<String>,
    #[serde(
        rename = "requestedForId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_for_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<task_orchestration_plan::Result>,
    #[serde(
        rename = "resultCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_code: Option<String>,
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<task_orchestration_plan::State>,
    #[doc = "A reference to a timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<TimelineReference>,
}
impl TaskOrchestrationPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_orchestration_plan {
    use super::*;
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "throttled")]
        Throttled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationPlanGroup {
    #[serde(rename = "planGroup", default, skip_serializing_if = "Option::is_none")]
    pub plan_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(
        rename = "runningRequests",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub running_requests: Vec<TaskAgentJobRequest>,
}
impl TaskOrchestrationPlanGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationPlanGroupsQueueMetrics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_orchestration_plan_groups_queue_metrics::Status>,
}
impl TaskOrchestrationPlanGroupsQueueMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_orchestration_plan_groups_queue_metrics {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationPlanReference {
    #[serde(
        rename = "artifactLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_location: Option<String>,
    #[serde(
        rename = "artifactUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<TaskOrchestrationOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskOrchestrationOwner>,
    #[serde(rename = "planGroup", default, skip_serializing_if = "Option::is_none")]
    pub plan_group: Option<String>,
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(
        rename = "scopeIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl TaskOrchestrationPlanReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationQueuedPlan {
    #[serde(
        rename = "assignTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub assign_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<TaskOrchestrationOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskOrchestrationOwner>,
    #[serde(rename = "planGroup", default, skip_serializing_if = "Option::is_none")]
    pub plan_group: Option<String>,
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
    #[serde(
        rename = "queuePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_position: Option<i32>,
    #[serde(
        rename = "queueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queue_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "scopeIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_identifier: Option<String>,
}
impl TaskOrchestrationQueuedPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationQueuedPlanGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<TaskOrchestrationOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskOrchestrationOwner>,
    #[serde(rename = "planGroup", default, skip_serializing_if = "Option::is_none")]
    pub plan_group: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plans: Vec<TaskOrchestrationQueuedPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(
        rename = "queuePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_position: Option<i32>,
}
impl TaskOrchestrationQueuedPlanGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOutputVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TaskOutputVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskPackageMetadata {
    #[doc = "Gets the name of the package."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets the url of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Gets the version of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl TaskPackageMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskReference {
    #[doc = "The ID of the task definition. Corresponds to the id value of task.json file. <br />Example: CmdLineV2 { \"id\": \"D9BAFED4-0B18-4F58-968D-86655B4D2CE9\" }"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A dictionary of inputs specific to a task definition. Corresponds to inputs value of task.json file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "The name of the task definition. Corresponds to the name value of task.json file. <br />Example: CmdLineV2 { \"name\": \"CmdLine\" }"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the task definition. Corresponds to the version value of task.json file. <br />Example: CmdLineV2 { \"version\": { \"Major\": 2, \"Minor\": 212, \"Patch\": 0 } }"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl TaskReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskRestrictions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<TaskCommandRestrictions>,
    #[serde(
        rename = "settableVariables",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub settable_variables: Option<TaskVariableRestrictions>,
}
impl TaskRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskSourceDefinition {
    #[serde(flatten)]
    pub task_source_definition_base: TaskSourceDefinitionBase,
}
impl TaskSourceDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskSourceDefinitionBase {
    #[serde(rename = "authKey", default, skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(
        rename = "keySelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl TaskSourceDefinitionBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskStartedEvent {
    #[serde(flatten)]
    pub task_event: TaskEvent,
}
impl TaskStartedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskVariableRestrictions {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed: Vec<String>,
}
impl TaskVariableRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskVersion {
    #[serde(rename = "isTest", default, skip_serializing_if = "Option::is_none")]
    pub is_test: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<i32>,
}
impl TaskVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Timeline {
    #[serde(flatten)]
    pub timeline_reference: TimelineReference,
    #[serde(
        rename = "lastChangedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_changed_by: Option<String>,
    #[serde(
        rename = "lastChangedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_on: Option<time::OffsetDateTime>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub records: Vec<TimelineRecord>,
}
impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An attempt to update a TimelineRecord."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineAttempt {
    #[doc = "The attempt of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "The unique identifier for the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "The record identifier located within the specified timeline."]
    #[serde(rename = "recordId", default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[doc = "The timeline identifier which owns the record representing this attempt."]
    #[serde(
        rename = "timelineId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_id: Option<String>,
}
impl TimelineAttempt {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detailed information about the execution of different operations during pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineRecord {
    #[doc = "The specification of an agent running a pipeline job, in binary format. Applicable when record is of type Job. <br />Example: { \"VMImage\" : \"windows-2019\" }"]
    #[serde(
        rename = "agentSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_specification: Option<serde_json::Value>,
    #[doc = "The number of record attempts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "The ID connecting all records updated at the same time. This value is taken from timeline's ChangeId."]
    #[serde(rename = "changeId", default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<i32>,
    #[doc = "A string that indicates the current operation."]
    #[serde(
        rename = "currentOperation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_operation: Option<String>,
    #[doc = "A reference to a timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<TimelineReference>,
    #[doc = "The number of errors produced by this operation."]
    #[serde(
        rename = "errorCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_count: Option<i32>,
    #[doc = "The finish time of the record."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The ID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "String identifier that is consistent across attempts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "The list of issues produced by this operation."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub issues: Vec<Issue>,
    #[doc = "The time the record was last modified."]
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "The REST URL of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "A reference to a task log. This class contains information about the output printed to the timeline record's logs console during pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<TaskLogReference>,
    #[doc = "The name of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An ordinal value relative to other records within the timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "The ID of the record's parent. <br />Example: Stage is a parent of a Phase, Phase is a parent of a Job, Job is a parent of a Task."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The percentage of record completion."]
    #[serde(
        rename = "percentComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percent_complete: Option<i32>,
    #[doc = "The previous record attempts."]
    #[serde(
        rename = "previousAttempts",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub previous_attempts: Vec<TimelineAttempt>,
    #[doc = "The ID of the queue which connects projects to agent pools on which the operation ran on. Applicable when record is of type Job."]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[doc = "Name of the referenced record."]
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[doc = "The result of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<timeline_record::Result>,
    #[doc = "Evaluation of predefined conditions upon completion of record's operation. <br />Example: Evaluating `succeeded()`, Result = True <br />Example: Evaluating `and(succeeded(), eq(variables['system.debug'], False))`, Result = False"]
    #[serde(
        rename = "resultCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_code: Option<String>,
    #[doc = "The start time of the record."]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The state of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<timeline_record::State>,
    #[doc = "A reference to a task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskReference>,
    #[doc = "The type of operation being tracked by the record. <br />Example: Stage, Phase, Job, Task..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The variables of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
    #[doc = "The number of warnings produced by this operation."]
    #[serde(
        rename = "warningCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_count: Option<i32>,
    #[doc = "The name of the agent running the operation. Applicable when record is of type Job."]
    #[serde(
        rename = "workerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_name: Option<String>,
}
impl TimelineRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod timeline_record {
    use super::*;
    #[doc = "The result of the record."]
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
    #[doc = "The state of the record."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineRecordFeedLinesWrapper {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "endLine", default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    #[serde(rename = "startLine", default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    #[serde(rename = "stepId", default, skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
}
impl TimelineRecordFeedLinesWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineRecordList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TimelineRecord>,
}
impl TimelineRecordList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to a timeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineReference {
    #[doc = "The change ID."]
    #[serde(rename = "changeId", default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<i32>,
    #[doc = "The ID of the timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The REST URL of the timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TimelineReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationItem {
    #[doc = "Tells whether the current input is valid or not"]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "Reason for input validation failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Type of validation item"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Value to validate. The conditional expression to validate for the input for \"expression\" type Eg:eq(variables['Build.SourceBranch'], 'refs/heads/master');eq(value, 'refs/heads/master')"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ValidationItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A variable group is a collection of related variables."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroup {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets or sets the time when variable group was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets description of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets id of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Indicates whether variable group is shared with other projects or not."]
    #[serde(rename = "isShared", default, skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
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
    #[doc = "Gets or sets name of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Defines provider data of the variable group."]
    #[serde(
        rename = "providerData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_data: Option<VariableGroupProviderData>,
    #[doc = "Gets or sets type of the variable group."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "all project references where the variable group is shared with other projects."]
    #[serde(
        rename = "variableGroupProjectReferences",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_group_project_references: Vec<VariableGroupProjectReference>,
    #[doc = "Gets or sets variables contained in the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl VariableGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroupList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VariableGroup>,
}
impl VariableGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroupParameters {
    #[doc = "Sets description of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Sets name of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Defines provider data of the variable group."]
    #[serde(
        rename = "providerData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_data: Option<VariableGroupProviderData>,
    #[doc = "Sets type of the variable group."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(
        rename = "variableGroupProjectReferences",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_group_project_references: Vec<VariableGroupProjectReference>,
    #[doc = "Sets variables contained in the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl VariableGroupParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A variable group reference is a shallow reference to variable group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroupProjectReference {
    #[doc = "Gets or sets description of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets name of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "projectReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_reference: Option<ProjectReference>,
}
impl VariableGroupProjectReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines provider data of the variable group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroupProviderData {}
impl VariableGroupProviderData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A wrapper class for a generic variable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableValue {
    #[doc = "Indicates whether the variable can be changed during script's execution runtime."]
    #[serde(
        rename = "isReadOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_read_only: Option<bool>,
    #[doc = "Indicates whether the variable should be encrypted at rest."]
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[doc = "The value of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VariableValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachine {
    #[doc = "A task agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<TaskAgent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl VirtualMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineGroup {
    #[serde(flatten)]
    pub environment_resource: EnvironmentResource,
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<i32>,
}
impl VirtualMachineGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineGroupCreateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl VirtualMachineGroupCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineResource {
    #[serde(flatten)]
    pub environment_resource: EnvironmentResource,
    #[doc = "A task agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<TaskAgent>,
}
impl VirtualMachineResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineResourceCreateParameters {
    #[serde(
        rename = "virtualMachineResource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_machine_resource: Option<VirtualMachineResource>,
}
impl VirtualMachineResourceCreateParameters {
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
