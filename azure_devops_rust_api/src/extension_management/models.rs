// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcquisitionOperation {
    #[doc = "State of the AcquisitionOperation for the current user"]
    #[serde(
        rename = "operationState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_state: Option<acquisition_operation::OperationState>,
    #[doc = "AcquisitionOperationType: install, request, buy, etc..."]
    #[serde(
        rename = "operationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_type: Option<acquisition_operation::OperationType>,
    #[doc = "Optional reason to justify current state. Typically used with Disallow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "List of reasons indicating why the operation is not allowed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reasons: Vec<AcquisitionOperationDisallowReason>,
}
impl AcquisitionOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod acquisition_operation {
    use super::*;
    #[doc = "State of the AcquisitionOperation for the current user"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationState {
        #[serde(rename = "disallow")]
        Disallow,
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "completed")]
        Completed,
    }
    #[doc = "AcquisitionOperationType: install, request, buy, etc..."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationType {
        #[serde(rename = "get")]
        Get,
        #[serde(rename = "install")]
        Install,
        #[serde(rename = "buy")]
        Buy,
        #[serde(rename = "try")]
        Try,
        #[serde(rename = "request")]
        Request,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "purchaseRequest")]
        PurchaseRequest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcquisitionOperationDisallowReason {
    #[doc = "User-friendly message clarifying the reason for disallowance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Type of reason for disallowance - AlreadyInstalled, UnresolvedDemand, etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl AcquisitionOperationDisallowReason {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Market item acquisition options (install, buy, etc) for an installation target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcquisitionOptions {
    #[serde(
        rename = "defaultOperation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_operation: Option<AcquisitionOperation>,
    #[doc = "The item id that this options refer to"]
    #[serde(rename = "itemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[doc = "Operations allowed for the ItemId in this target"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<AcquisitionOperation>,
    #[doc = "Additional properties which can be added to the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The target that this options refer to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl AcquisitionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Representation of a ContributionNode that can be used for serialized to clients."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientContribution {
    #[doc = "Description of the contribution/type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Fully qualified identifier of the contribution/type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Includes is a set of contributions that should have this contribution included in their targets list."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub includes: Vec<String>,
    #[doc = "Properties/attributes of this contribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The ids of the contribution(s) that this contribution targets. (parent contributions)"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub targets: Vec<String>,
    #[doc = "Id of the Contribution Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ClientContribution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Representation of a ContributionNode that can be used for serialized to clients."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientContributionNode {
    #[doc = "List of ids for contributions which are children to the current contribution."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<String>,
    #[doc = "Representation of a ContributionNode that can be used for serialized to clients."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contribution: Option<ClientContribution>,
    #[doc = "List of ids for contributions which are parents to the current contribution."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parents: Vec<String>,
}
impl ClientContributionNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientContributionProviderDetails {
    #[doc = "Friendly name for the provider."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Unique identifier for this provider. The provider name can be used to cache the contribution data and refer back to it when looking for changes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties associated with the provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Version of contributions associated with this contribution provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ClientContributionProviderDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A client data provider are the details needed to make the data provider request from the client."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDataProviderQuery {
    #[serde(flatten)]
    pub data_provider_query: DataProviderQuery,
    #[doc = "The Id of the service instance type that should be communicated with in order to resolve the data providers from the client given the query values."]
    #[serde(
        rename = "queryServiceInstanceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_service_instance_type: Option<String>,
}
impl ClientDataProviderQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An individual contribution made by an extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Contribution {
    #[serde(flatten)]
    pub contribution_base: ContributionBase,
    #[doc = "List of constraints (filters) that should be applied to the availability of this contribution"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub constraints: Vec<ContributionConstraint>,
    #[doc = "Includes is a set of contributions that should have this contribution included in their targets list."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub includes: Vec<String>,
    #[doc = "Properties/attributes of this contribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "List of demanded claims in order for the user to see this contribution (like anonymous, public, member...)."]
    #[serde(
        rename = "restrictedTo",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restricted_to: Vec<String>,
    #[doc = "The ids of the contribution(s) that this contribution targets. (parent contributions)"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub targets: Vec<String>,
    #[doc = "Id of the Contribution Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Contribution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class shared by contributions and contribution types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContributionBase {
    #[doc = "Description of the contribution/type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Fully qualified identifier of the contribution/type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "VisibleTo can be used to restrict whom can reference a given contribution/type. This value should be a list of publishers or extensions access is restricted too.  Examples: \"ms\" - Means only the \"ms\" publisher can reference this. \"ms.vss-web\" - Means only the \"vss-web\" extension from the \"ms\" publisher can reference this."]
    #[serde(
        rename = "visibleTo",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub visible_to: Vec<String>,
}
impl ContributionBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies a constraint that can be used to dynamically include/exclude a given contribution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContributionConstraint {
    #[doc = "An optional property that can be specified to group constraints together. All constraints within a group are AND'd together (all must be evaluate to True in order for the contribution to be included). Different groups of constraints are OR'd (only one group needs to evaluate to True for the contribution to be included)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    #[doc = "Fully qualified identifier of a shared constraint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "If true, negate the result of the filter (include the contribution if the applied filter returns false instead of true)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inverse: Option<bool>,
    #[doc = "Name of the IContributionFilter plugin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties that are fed to the contribution filter class"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Constraints can be optionally be applied to one or more of the relationships defined in the contribution. If no relationships are defined then all relationships are associated with the constraint. This means the default behaviour will eliminate the contribution from the tree completely if the constraint is applied."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relationships: Vec<String>,
}
impl ContributionConstraint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A query that can be issued for contribution nodes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContributionNodeQuery {
    #[doc = "The contribution ids of the nodes to find."]
    #[serde(
        rename = "contributionIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contribution_ids: Vec<String>,
    #[doc = "Contextual information that data providers can examine when populating their data"]
    #[serde(
        rename = "dataProviderContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_provider_context: Option<DataProviderContext>,
    #[doc = "Indicator if contribution provider details should be included in the result."]
    #[serde(
        rename = "includeProviderDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_provider_details: Option<bool>,
    #[doc = "Query options tpo be used when fetching ContributionNodes"]
    #[serde(
        rename = "queryOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_options: Option<contribution_node_query::QueryOptions>,
}
impl ContributionNodeQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod contribution_node_query {
    use super::*;
    #[doc = "Query options tpo be used when fetching ContributionNodes"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryOptions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "includeSelf")]
        IncludeSelf,
        #[serde(rename = "includeChildren")]
        IncludeChildren,
        #[serde(rename = "includeSubTree")]
        IncludeSubTree,
        #[serde(rename = "includeAll")]
        IncludeAll,
        #[serde(rename = "ignoreConstraints")]
        IgnoreConstraints,
    }
}
#[doc = "Result of a contribution node query.  Wraps the resulting contribution nodes and provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContributionNodeQueryResult {
    #[doc = "Map of contribution ids to corresponding node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<serde_json::Value>,
    #[doc = "Map of provider ids to the corresponding provider details object."]
    #[serde(
        rename = "providerDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_details: Option<serde_json::Value>,
}
impl ContributionNodeQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description about a property of a contribution type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContributionPropertyDescription {
    #[doc = "Description of the property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "True if this property is required"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "The type of value used for this property"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<contribution_property_description::Type>,
}
impl ContributionPropertyDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod contribution_property_description {
    use super::*;
    #[doc = "The type of value used for this property"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "string")]
        String,
        #[serde(rename = "uri")]
        Uri,
        #[serde(rename = "guid")]
        Guid,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(rename = "integer")]
        Integer,
        #[serde(rename = "double")]
        Double,
        #[serde(rename = "dateTime")]
        DateTime,
        #[serde(rename = "dictionary")]
        Dictionary,
        #[serde(rename = "array")]
        Array,
        #[serde(rename = "object")]
        Object,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContributionProviderDetails {
    #[doc = "Friendly name for the provider."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Unique identifier for this provider. The provider name can be used to cache the contribution data and refer back to it when looking for changes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties associated with the provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Version of contributions associated with this contribution provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ContributionProviderDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A contribution type, given by a json schema"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContributionType {
    #[serde(flatten)]
    pub contribution_base: ContributionBase,
    #[doc = "Controls whether or not contributions of this type have the type indexed for queries. This allows clients to find all extensions that have a contribution of this type.  NOTE: Only TrustedPartners are allowed to specify indexed contribution types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[doc = "Friendly name of the contribution/type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the allowed properties for this contribution type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ContributionType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contextual information that data providers can examine when populating their data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProviderContext {
    #[doc = "Generic property bag that contains context-specific properties that data providers can use when populating their data dictionary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl DataProviderContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProviderExceptionDetails {
    #[doc = "The type of the exception that was thrown."]
    #[serde(
        rename = "exceptionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_type: Option<String>,
    #[doc = "Message that is associated with the exception."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The StackTrace from the exception turned into a string."]
    #[serde(
        rename = "stackTrace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<String>,
}
impl DataProviderExceptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A query that can be issued for data provider data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProviderQuery {
    #[doc = "Contextual information that data providers can examine when populating their data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<DataProviderContext>,
    #[doc = "The contribution ids of the data providers to resolve"]
    #[serde(
        rename = "contributionIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contribution_ids: Vec<String>,
}
impl DataProviderQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result structure from calls to GetDataProviderData"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProviderResult {
    #[doc = "This is the set of data providers that were requested, but either they were defined as client providers, or as remote providers that failed and may be retried by the client."]
    #[serde(
        rename = "clientProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_providers: Option<serde_json::Value>,
    #[doc = "Property bag of data keyed off of the data provider contribution id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Set of exceptions that occurred resolving the data providers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<serde_json::Value>,
    #[doc = "List of data providers resolved in the data-provider query"]
    #[serde(
        rename = "resolvedProviders",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resolved_providers: Vec<ResolvedDataProvider>,
    #[doc = "Scope name applied to this data provider result."]
    #[serde(rename = "scopeName", default, skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    #[doc = "Scope value applied to this data provider result."]
    #[serde(
        rename = "scopeValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_value: Option<String>,
    #[doc = "Property bag of shared data that was contributed to by any of the individual data providers"]
    #[serde(
        rename = "sharedData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_data: Option<serde_json::Value>,
}
impl DataProviderResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data bag that any data provider can contribute to. This shared dictionary is returned in the data provider result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProviderSharedData {}
impl DataProviderSharedData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contract for handling the extension acquisition process"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionAcquisitionRequest {
    #[doc = "How the item is being assigned"]
    #[serde(
        rename = "assignmentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assignment_type: Option<extension_acquisition_request::AssignmentType>,
    #[doc = "The id of the subscription used for purchase"]
    #[serde(rename = "billingId", default, skip_serializing_if = "Option::is_none")]
    pub billing_id: Option<String>,
    #[doc = "The marketplace id (publisherName.extensionName) for the item"]
    #[serde(rename = "itemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[doc = "The type of operation, such as install, request, purchase"]
    #[serde(
        rename = "operationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_type: Option<extension_acquisition_request::OperationType>,
    #[doc = "Additional properties which can be added to the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "How many licenses should be purchased"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}
impl ExtensionAcquisitionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_acquisition_request {
    use super::*;
    #[doc = "How the item is being assigned"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AssignmentType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "me")]
        Me,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "The type of operation, such as install, request, purchase"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationType {
        #[serde(rename = "get")]
        Get,
        #[serde(rename = "install")]
        Install,
        #[serde(rename = "buy")]
        Buy,
        #[serde(rename = "try")]
        Try,
        #[serde(rename = "request")]
        Request,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "purchaseRequest")]
        PurchaseRequest,
    }
}
#[doc = "Audit log for an extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionAuditLog {
    #[doc = "Collection of audit log entries"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entries: Vec<ExtensionAuditLogEntry>,
    #[doc = "Extension that the change was made for"]
    #[serde(
        rename = "extensionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_name: Option<String>,
    #[doc = "Publisher that the extension is part of"]
    #[serde(
        rename = "publisherName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_name: Option<String>,
}
impl ExtensionAuditLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An audit log entry for an extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionAuditLogEntry {
    #[doc = "Change that was made to extension"]
    #[serde(
        rename = "auditAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audit_action: Option<String>,
    #[doc = "Date at which the change was made"]
    #[serde(
        rename = "auditDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub audit_date: Option<time::OffsetDateTime>,
    #[doc = "Extra information about the change"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<IdentityRef>,
}
impl ExtensionAuditLogEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionAuthorization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
}
impl ExtensionAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionBadge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "imgUri", default, skip_serializing_if = "Option::is_none")]
    pub img_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
impl ExtensionBadge {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a single collection for extension data documents"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionDataCollection {
    #[doc = "The name of the collection"]
    #[serde(
        rename = "collectionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_name: Option<String>,
    #[doc = "A list of documents belonging to the collection"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<serde_json::Value>,
    #[doc = "The type of the collection's scope, such as Default or User"]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    #[doc = "The value of the collection's scope, such as Current or Me"]
    #[serde(
        rename = "scopeValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_value: Option<String>,
}
impl ExtensionDataCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a query to receive a set of extension data collections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionDataCollectionQuery {
    #[doc = "A list of collections to query"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub collections: Vec<ExtensionDataCollection>,
}
impl ExtensionDataCollectionQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<PublishedExtension>,
    #[doc = "The current version of the extension that was updated"]
    #[serde(
        rename = "extensionVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<ExtensionHost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ExtensionEventUrls>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "The type of update that was made"]
    #[serde(
        rename = "updateType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_type: Option<extension_event::UpdateType>,
}
impl ExtensionEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_event {
    use super::*;
    #[doc = "The type of update that was made"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateType {
        #[serde(rename = "installed")]
        Installed,
        #[serde(rename = "uninstalled")]
        Uninstalled,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "versionUpdated")]
        VersionUpdated,
        #[serde(rename = "actionRequired")]
        ActionRequired,
        #[serde(rename = "actionResolved")]
        ActionResolved,
    }
}
#[doc = "Base class for an event callback for an extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionEventCallback {
    #[doc = "The uri of the endpoint that is hit when an event occurs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl ExtensionEventCallback {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of event callbacks - endpoints called when particular extension events occur."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionEventCallbackCollection {
    #[doc = "Base class for an event callback for an extension"]
    #[serde(
        rename = "postDisable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_disable: Option<ExtensionEventCallback>,
    #[doc = "Base class for an event callback for an extension"]
    #[serde(
        rename = "postEnable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_enable: Option<ExtensionEventCallback>,
    #[doc = "Base class for an event callback for an extension"]
    #[serde(
        rename = "postInstall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_install: Option<ExtensionEventCallback>,
    #[doc = "Base class for an event callback for an extension"]
    #[serde(
        rename = "postUninstall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_uninstall: Option<ExtensionEventCallback>,
    #[doc = "Base class for an event callback for an extension"]
    #[serde(
        rename = "postUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_update: Option<ExtensionEventCallback>,
    #[doc = "Base class for an event callback for an extension"]
    #[serde(
        rename = "preInstall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_install: Option<ExtensionEventCallback>,
    #[doc = "Base class for an event callback for an extension"]
    #[serde(
        rename = "versionCheck",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_check: Option<ExtensionEventCallback>,
}
impl ExtensionEventCallbackCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionEventUrls {
    #[serde(flatten)]
    pub extension_urls: ExtensionUrls,
    #[doc = "Url of the extension management page"]
    #[serde(
        rename = "manageExtensionsPage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manage_extensions_page: Option<String>,
}
impl ExtensionEventUrls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionFile {
    #[serde(rename = "assetType", default, skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl ExtensionFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionHost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExtensionHost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the component pieces of an extensions fully qualified name, along with the fully qualified name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionIdentifier {
    #[doc = "The ExtensionName component part of the fully qualified ExtensionIdentifier"]
    #[serde(
        rename = "extensionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_name: Option<String>,
    #[doc = "The PublisherName component part of the fully qualified ExtensionIdentifier"]
    #[serde(
        rename = "publisherName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_name: Option<String>,
}
impl ExtensionIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "How an extension should handle including contributions based on licensing"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionLicensing {
    #[doc = "A list of contributions which deviate from the default licensing behavior"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub overrides: Vec<LicensingOverride>,
}
impl ExtensionLicensing {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for extension properties which are shared by the extension manifest and the extension model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionManifest {
    #[doc = "Uri used as base for other relative uri's defined in extension"]
    #[serde(rename = "baseUri", default, skip_serializing_if = "Option::is_none")]
    pub base_uri: Option<String>,
    #[doc = "List of shared constraints defined by this extension"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub constraints: Vec<ContributionConstraint>,
    #[doc = "List of contributions made by this extension"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contributions: Vec<Contribution>,
    #[doc = "List of contribution types defined by this extension"]
    #[serde(
        rename = "contributionTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contribution_types: Vec<ContributionType>,
    #[doc = "List of explicit demands required by this extension"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<String>,
    #[doc = "Collection of event callbacks - endpoints called when particular extension events occur."]
    #[serde(
        rename = "eventCallbacks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_callbacks: Option<ExtensionEventCallbackCollection>,
    #[doc = "Secondary location that can be used as base for other relative uri's defined in extension"]
    #[serde(
        rename = "fallbackBaseUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_base_uri: Option<String>,
    #[doc = "Language Culture Name set by the Gallery"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "How an extension should handle including contributions based on licensing"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub licensing: Option<ExtensionLicensing>,
    #[doc = "Version of the extension manifest format/content"]
    #[serde(
        rename = "manifestVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_version: Option<f64>,
    #[doc = "Default user claims applied to all contributions (except the ones which have been specified restrictedTo explicitly) to control the visibility of a contribution."]
    #[serde(
        rename = "restrictedTo",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restricted_to: Vec<String>,
    #[doc = "List of all oauth scopes required by this extension"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
    #[doc = "The ServiceInstanceType(Guid) of the VSTS service that must be available to an account in order for the extension to be installed"]
    #[serde(
        rename = "serviceInstanceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_instance_type: Option<String>,
}
impl ExtensionManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy with a set of permissions on extension operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionPolicy {
    #[doc = "Permissions on 'Install' operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install: Option<extension_policy::Install>,
    #[doc = "Permission on 'Request' operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<extension_policy::Request>,
}
impl ExtensionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_policy {
    use super::*;
    #[doc = "Permissions on 'Install' operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Install {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "preview")]
        Preview,
        #[serde(rename = "released")]
        Released,
        #[serde(rename = "firstParty")]
        FirstParty,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "Permission on 'Request' operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Request {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "preview")]
        Preview,
        #[serde(rename = "released")]
        Released,
        #[serde(rename = "firstParty")]
        FirstParty,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "A request for an extension (to be installed or have a license assigned)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionRequest {
    #[doc = "Required message supplied if the request is rejected"]
    #[serde(
        rename = "rejectMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_message: Option<String>,
    #[doc = "Date at which the request was made"]
    #[serde(
        rename = "requestDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub request_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<IdentityRef>,
    #[doc = "Optional message supplied by the requester justifying the request"]
    #[serde(
        rename = "requestMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_message: Option<String>,
    #[doc = "Represents the state of the request"]
    #[serde(
        rename = "requestState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_state: Option<extension_request::RequestState>,
    #[doc = "Date at which the request was resolved"]
    #[serde(
        rename = "resolveDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub resolve_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "resolvedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolved_by: Option<IdentityRef>,
}
impl ExtensionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_request {
    use super::*;
    #[doc = "Represents the state of the request"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RequestState {
        #[serde(rename = "open")]
        Open,
        #[serde(rename = "accepted")]
        Accepted,
        #[serde(rename = "rejected")]
        Rejected,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionRequestEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<PublishedExtension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<ExtensionHost>,
    #[doc = "Name of the collection for which the extension was requested"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ExtensionRequestUrls>,
    #[doc = "A request for an extension (to be installed or have a license assigned)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<ExtensionRequest>,
    #[doc = "The type of update that was made"]
    #[serde(
        rename = "updateType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_type: Option<extension_request_event::UpdateType>,
}
impl ExtensionRequestEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_request_event {
    use super::*;
    #[doc = "The type of update that was made"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateType {
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "deleted")]
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionRequestUrls {
    #[serde(flatten)]
    pub extension_urls: ExtensionUrls,
    #[doc = "Link to view the extension request"]
    #[serde(
        rename = "requestPage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_page: Option<String>,
}
impl ExtensionRequestUrls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionRequestsEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<PublishedExtension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<ExtensionHost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ExtensionRequestUrls>,
    #[doc = "The extension request object"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub requests: Vec<ExtensionRequest>,
    #[doc = "The type of update that was made"]
    #[serde(
        rename = "updateType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_type: Option<extension_requests_event::UpdateType>,
}
impl ExtensionRequestsEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_requests_event {
    use super::*;
    #[doc = "The type of update that was made"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateType {
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "deleted")]
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionShare {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isOrg", default, skip_serializing_if = "Option::is_none")]
    pub is_org: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ExtensionShare {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of an extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionState {
    #[serde(flatten)]
    pub installed_extension_state: InstalledExtensionState,
    #[serde(
        rename = "extensionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_name: Option<String>,
    #[doc = "The time at which the version was last checked"]
    #[serde(
        rename = "lastVersionCheck",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_version_check: Option<time::OffsetDateTime>,
    #[serde(
        rename = "publisherName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ExtensionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionStatistic {
    #[serde(
        rename = "statisticName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub statistic_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl ExtensionStatistic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionUrls {
    #[doc = "Url of the extension icon"]
    #[serde(
        rename = "extensionIcon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_icon: Option<String>,
    #[doc = "Link to view the extension details page"]
    #[serde(
        rename = "extensionPage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_page: Option<String>,
}
impl ExtensionUrls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionVersion {
    #[serde(rename = "assetUri", default, skip_serializing_if = "Option::is_none")]
    pub asset_uri: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub badges: Vec<ExtensionBadge>,
    #[serde(
        rename = "fallbackAssetUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_asset_uri: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<ExtensionFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated: Option<time::OffsetDateTime>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub properties: Vec<serde_json::Value>,
    #[serde(
        rename = "targetPlatform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_platform: Option<String>,
    #[serde(
        rename = "validationResultMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub validation_result_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "versionDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_description: Option<String>,
}
impl ExtensionVersion {
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
pub struct InstallationTarget {
    #[serde(
        rename = "extensionVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_version: Option<String>,
    #[serde(
        rename = "productArchitecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_architecture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(
        rename = "targetPlatform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_platform: Option<String>,
    #[serde(
        rename = "targetVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_version: Option<String>,
}
impl InstallationTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a VSTS extension along with its installation state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstalledExtension {
    #[serde(flatten)]
    pub extension_manifest: ExtensionManifest,
    #[doc = "The friendly extension id for this extension - unique for a given publisher."]
    #[serde(
        rename = "extensionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_id: Option<String>,
    #[doc = "The display name of the extension."]
    #[serde(
        rename = "extensionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_name: Option<String>,
    #[doc = "This is the set of files available from the extension."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<ExtensionFile>,
    #[doc = "Extension flags relevant to contribution consumers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[doc = "The state of an installed extension"]
    #[serde(
        rename = "installState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub install_state: Option<InstalledExtensionState>,
    #[doc = "This represents the date/time the extensions was last updated in the gallery. This doesnt mean this version was updated the value represents changes to any and all versions of the extension."]
    #[serde(
        rename = "lastPublished",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_published: Option<time::OffsetDateTime>,
    #[doc = "Unique id of the publisher of this extension"]
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[doc = "The display name of the publisher"]
    #[serde(
        rename = "publisherName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_name: Option<String>,
    #[doc = "Unique id for this extension (the same id is used for all versions of a single extension)"]
    #[serde(
        rename = "registrationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_id: Option<String>,
    #[doc = "Version of this extension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl InstalledExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstalledExtensionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InstalledExtension>,
}
impl InstalledExtensionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstalledExtensionQuery {
    #[serde(
        rename = "assetTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub asset_types: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub monikers: Vec<ExtensionIdentifier>,
}
impl InstalledExtensionQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of an installed extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstalledExtensionState {
    #[doc = "States of an installed extension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[doc = "List of installation issues"]
    #[serde(
        rename = "installationIssues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub installation_issues: Vec<InstalledExtensionStateIssue>,
    #[doc = "The time at which this installation was last updated"]
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated: Option<time::OffsetDateTime>,
}
impl InstalledExtensionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an installation issue"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstalledExtensionStateIssue {
    #[doc = "The error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Source of the installation issue, for example  \"Demands\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Installation issue type (Warning, Error)"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<installed_extension_state_issue::Type>,
}
impl InstalledExtensionStateIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod installed_extension_state_issue {
    use super::*;
    #[doc = "Installation issue type (Warning, Error)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
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
#[doc = "Maps a contribution to a licensing behavior"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicensingOverride {
    #[doc = "How the inclusion of this contribution should change based on licensing"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<licensing_override::Behavior>,
    #[doc = "Fully qualified contribution id which we want to define licensing behavior for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl LicensingOverride {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod licensing_override {
    use super::*;
    #[doc = "How the inclusion of this contribution should change based on licensing"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Behavior {
        #[serde(rename = "onlyIfLicensed")]
        OnlyIfLicensed,
        #[serde(rename = "onlyIfUnlicensed")]
        OnlyIfUnlicensed,
        #[serde(rename = "alwaysInclude")]
        AlwaysInclude,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishedExtension {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<String>,
    #[serde(
        rename = "deploymentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_type: Option<published_extension::DeploymentType>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        rename = "extensionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_id: Option<String>,
    #[serde(
        rename = "extensionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(
        rename = "installationTargets",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub installation_targets: Vec<InstallationTarget>,
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated: Option<time::OffsetDateTime>,
    #[serde(
        rename = "longDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_description: Option<String>,
    #[doc = "Check if Extension is in conflict list or not. Taking as String and not as boolean because we don't want end customer to see this flag and by making it Boolean it is coming as false for all the cases."]
    #[serde(
        rename = "presentInConflictList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub present_in_conflict_list: Option<String>,
    #[doc = "Date on which the extension was first uploaded."]
    #[serde(
        rename = "publishedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "High-level information about the publisher, like id's and names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<PublisherFacts>,
    #[doc = "Date on which the extension first went public."]
    #[serde(
        rename = "releaseDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub release_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "sharedWith",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shared_with: Vec<ExtensionShare>,
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub statistics: Vec<ExtensionStatistic>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub versions: Vec<ExtensionVersion>,
}
impl PublishedExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod published_extension {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeploymentType {
        #[serde(rename = "exe")]
        Exe,
        #[serde(rename = "msi")]
        Msi,
        #[serde(rename = "vsix")]
        Vsix,
        #[serde(rename = "referralLink")]
        ReferralLink,
    }
}
#[doc = "High-level information about the publisher, like id's and names"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublisherFacts {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(
        rename = "isDomainVerified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_domain_verified: Option<bool>,
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[serde(
        rename = "publisherName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_name: Option<String>,
}
impl PublisherFacts {
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
#[doc = "A request for an extension (to be installed or have a license assigned)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestedExtension {
    #[doc = "The unique name of the extension"]
    #[serde(
        rename = "extensionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_name: Option<String>,
    #[doc = "A list of each request for the extension"]
    #[serde(
        rename = "extensionRequests",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extension_requests: Vec<ExtensionRequest>,
    #[doc = "DisplayName of the publisher that owns the extension being published."]
    #[serde(
        rename = "publisherDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_display_name: Option<String>,
    #[doc = "Represents the Publisher of the requested extension"]
    #[serde(
        rename = "publisherName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_name: Option<String>,
    #[doc = "The total number of requests for an extension"]
    #[serde(
        rename = "requestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_count: Option<i32>,
}
impl RequestedExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Entry for a specific data provider's resulting data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolvedDataProvider {
    #[doc = "The total time the data provider took to resolve its data (in milliseconds)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResolvedDataProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Scope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the extension"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedExtension {
    #[doc = "Unique Identifier for this extension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[doc = "Unique Identifier for this publisher"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Supported version for this extension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SupportedExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the extension policy applied to a given user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserExtensionPolicy {
    #[doc = "User display name that this policy refers to"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Policy with a set of permissions on extension operations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ExtensionPolicy>,
    #[doc = "User id that this policy refers to"]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl UserExtensionPolicy {
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
