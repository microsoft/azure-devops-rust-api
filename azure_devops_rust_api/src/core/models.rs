// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CategorizedWebApiTeams {
    #[doc = "Teams that the user is a member of."]
    #[serde(
        rename = "myTeams",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub my_teams: Vec<WebApiTeam>,
    #[doc = "Teams that the user can read but is not member of."]
    #[serde(
        rename = "otherReadableTeams",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_readable_teams: Vec<WebApiTeam>,
}
impl CategorizedWebApiTeams {
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
pub struct IdentityData {
    #[serde(
        rename = "identityIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identity_ids: Vec<String>,
}
impl IdentityData {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Reference for an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationReference {
    #[doc = "Unique identifier for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Unique identifier for the plugin."]
    #[serde(rename = "pluginId", default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[doc = "The current status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_reference::Status>,
    #[doc = "URL to get the full operation object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl OperationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_reference {
    use super::*;
    #[doc = "The current status of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "cancelled")]
        Cancelled,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Process {
    #[serde(flatten)]
    pub process_reference: ProcessReference,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<process::Type>,
}
impl Process {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "inherited")]
        Inherited,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Process>,
}
impl ProcessList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the image data for project avatar."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectAvatar {
    #[doc = "The avatar image represented as a byte array."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image: Vec<String>,
}
impl ProjectAvatar {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information describing a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectInfo {
    #[doc = "The abbreviated name of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[doc = "The description of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The id of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The time that this project was last updated."]
    #[serde(
        rename = "lastUpdateTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "The name of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A set of name-value pairs storing additional property data related to the project."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub properties: Vec<ProjectProperty>,
    #[doc = "The current revision of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[doc = "The current state of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<project_info::State>,
    #[doc = "A Uri that can be used to refer to this project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The version number of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "Indicates whom the project is visible to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<project_info::Visibility>,
}
impl ProjectInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_info {
    use super::*;
    #[doc = "The current state of the project."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "wellFormed")]
        WellFormed,
        #[serde(rename = "createPending")]
        CreatePending,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "unchanged")]
        Unchanged,
        #[serde(rename = "deleted")]
        Deleted,
    }
    #[doc = "Indicates whom the project is visible to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Visibility {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "public")]
        Public,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectMessage {
    #[doc = "Contains information describing a project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectInfo>,
    #[serde(
        rename = "projectChangeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_change_type: Option<project_message::ProjectChangeType>,
    #[serde(
        rename = "shouldInvalidateSystemStore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub should_invalidate_system_store: Option<bool>,
}
impl ProjectMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_message {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProjectChangeType {
        #[serde(rename = "modified")]
        Modified,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "added")]
        Added,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectProperties {
    #[doc = "The team project Id"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The collection of team project properties"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub properties: Vec<ProjectProperty>,
}
impl ProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A named value associated with a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectProperty {
    #[doc = "The name of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl ProjectProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectPropertyList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProjectProperty>,
}
impl ProjectPropertyList {
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
pub struct Proxy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ProxyAuthorization>,
    #[doc = "This is a description string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The friendly name of the server"]
    #[serde(
        rename = "friendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<String>,
    #[serde(
        rename = "globalDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_default: Option<bool>,
    #[doc = "This is a string representation of the site that the proxy server is located in (e.g. \"NA-WA-RED\")"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(
        rename = "siteDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub site_default: Option<bool>,
    #[doc = "The URL of the proxy server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Proxy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyAuthorization {
    #[doc = "Gets or sets the endpoint used to obtain access tokens from the configured token service."]
    #[serde(
        rename = "authorizationUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_url: Option<String>,
    #[doc = "Gets or sets the client identifier for this proxy."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Gets or sets the user identity to authorize for on-prem."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "Represents the public key portion of an RSA asymmetric key."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<PublicKey>,
}
impl ProxyAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the public key portion of an RSA asymmetric key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicKey {
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
impl PublicKey {
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
#[doc = "The Team Context for an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamContext {
    #[doc = "The team project Id or name.  Ignored if ProjectId is set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[doc = "The Team Project ID.  Required if Project is not set."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The Team Id or name.  Ignored if TeamId is set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[doc = "The Team Id"]
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}
impl TeamContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityRef>,
    #[serde(
        rename = "isTeamAdmin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_team_admin: Option<bool>,
}
impl TeamMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMemberList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TeamMember>,
}
impl TeamMemberList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Team Project object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProject {
    #[serde(flatten)]
    pub team_project_reference: TeamProjectReference,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Set of capabilities this project has (such as process template & version control)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(
        rename = "defaultTeam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team: Option<WebApiTeamRef>,
}
impl TeamProject {
    pub fn new(team_project_reference: TeamProjectReference) -> Self {
        Self {
            team_project_reference,
            links: None,
            capabilities: None,
            default_team: None,
        }
    }
}
#[doc = "Data contract for a TeamProjectCollection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamProjectCollection {
    #[serde(flatten)]
    pub team_project_collection_reference: TeamProjectCollectionReference,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Project collection description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Process customization type on this collection. It can be Xml or Inherited."]
    #[serde(
        rename = "processCustomizationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub process_customization_type: Option<team_project_collection::ProcessCustomizationType>,
    #[doc = "Project collection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl TeamProjectCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod team_project_collection {
    use super::*;
    #[doc = "Process customization type on this collection. It can be Xml or Inherited."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProcessCustomizationType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "xml")]
        Xml,
        #[serde(rename = "inherited")]
        Inherited,
    }
}
#[doc = "Reference object for a TeamProjectCollection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamProjectCollectionReference {
    #[doc = "Collection avatar Url."]
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[doc = "Collection Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Collection Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Collection REST Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TeamProjectCollectionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a shallow reference to a TeamProject."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProjectReference {
    #[doc = "Project abbreviation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[doc = "Url to default team identity image."]
    #[serde(
        rename = "defaultTeamImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team_image_url: Option<String>,
    #[doc = "The project's description (if any)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Project identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Project last update time."]
    #[serde(
        rename = "lastUpdateTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "Project name."]
    pub name: String,
    #[doc = "Project revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[doc = "Project state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<team_project_reference::State>,
    #[doc = "Url to the full version of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Project visibility."]
    pub visibility: team_project_reference::Visibility,
}
impl TeamProjectReference {
    pub fn new(name: String, visibility: team_project_reference::Visibility) -> Self {
        Self {
            abbreviation: None,
            default_team_image_url: None,
            description: None,
            id: None,
            last_update_time: None,
            name,
            revision: None,
            state: None,
            url: None,
            visibility,
        }
    }
}
pub mod team_project_reference {
    use super::*;
    #[doc = "Project state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "wellFormed")]
        WellFormed,
        #[serde(rename = "createPending")]
        CreatePending,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "unchanged")]
        Unchanged,
        #[serde(rename = "deleted")]
        Deleted,
    }
    #[doc = "Project visibility."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Visibility {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "organization")]
        Organization,
        #[serde(rename = "unchanged")]
        Unchanged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamProjectReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TeamProjectReference>,
}
impl TeamProjectReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A data transfer object that stores the metadata associated with the creation of temporary data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemporaryDataCreatedDto {
    #[serde(flatten)]
    pub temporary_data_dto: TemporaryDataDto,
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TemporaryDataCreatedDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A data transfer object that stores the metadata associated with the temporary data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemporaryDataDto {
    #[serde(
        rename = "expirationSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl TemporaryDataDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Updateable properties for a WebApiTeam."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateTeam {
    #[doc = "New description for the team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "New name for the team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl UpdateTeam {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiConnectedService {
    #[serde(flatten)]
    pub web_api_connected_service_ref: WebApiConnectedServiceRef,
    #[serde(
        rename = "authenticatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticated_by: Option<IdentityRef>,
    #[doc = "Extra description on the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly Name of service connection"]
    #[serde(
        rename = "friendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<String>,
    #[doc = "Id/Name of the connection service. For Ex: Subscription Id for Azure Connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The kind of service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[doc = "Optional uri to connect directly to the service such as<https://windows>.azure.com"]
    #[serde(
        rename = "serviceUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_uri: Option<String>,
}
impl WebApiConnectedService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiConnectedServiceDetails {
    #[serde(flatten)]
    pub web_api_connected_service_ref: WebApiConnectedServiceRef,
    #[serde(
        rename = "connectedServiceMetaData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_service_meta_data: Option<WebApiConnectedService>,
    #[doc = "Credential info"]
    #[serde(
        rename = "credentialsXml",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credentials_xml: Option<String>,
    #[doc = "Optional uri to connect directly to the service such as<https://windows>.azure.com"]
    #[serde(rename = "endPoint", default, skip_serializing_if = "Option::is_none")]
    pub end_point: Option<String>,
}
impl WebApiConnectedServiceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiConnectedServiceRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WebApiConnectedServiceRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The representation of data needed to create a tag definition which is sent across the wire."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiCreateTagRequestData {
    #[doc = "Name of the tag definition that will be created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WebApiCreateTagRequestData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebApiProject {
    #[serde(flatten)]
    pub team_project_reference: TeamProjectReference,
    #[doc = "Set of capabilities this project has"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<WebApiProjectCollectionRef>,
    #[serde(
        rename = "defaultTeam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team: Option<WebApiTeamRef>,
}
impl WebApiProject {
    pub fn new(team_project_reference: TeamProjectReference) -> Self {
        Self {
            team_project_reference,
            capabilities: None,
            collection: None,
            default_team: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiProjectCollection {
    #[serde(flatten)]
    pub web_api_project_collection_ref: WebApiProjectCollectionRef,
    #[doc = "Project collection description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Project collection state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl WebApiProjectCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiProjectCollectionRef {
    #[doc = "Collection Tfs Url (Host Url)"]
    #[serde(
        rename = "collectionUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_url: Option<String>,
    #[doc = "Collection Guid"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Collection Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Collection REST Url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WebApiProjectCollectionRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The representation of a tag definition which is sent across the wire."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiTagDefinition {
    #[doc = "Whether or not the tag definition is active."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "ID of the tag definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the tag definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource URL for the Tag Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WebApiTagDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiTeam {
    #[serde(flatten)]
    pub web_api_team_ref: WebApiTeamRef,
    #[doc = "Team description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Identity REST API Url to this team"]
    #[serde(
        rename = "identityUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_url: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "projectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
}
impl WebApiTeam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiTeamList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WebApiTeam>,
}
impl WebApiTeamList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiTeamRef {
    #[doc = "Team (Identity) Guid. A Team Foundation ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Team name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Team REST API Url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WebApiTeamRef {
    pub fn new() -> Self {
        Self::default()
    }
}
