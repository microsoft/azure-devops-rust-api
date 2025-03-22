// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Class for encapsulating the allowed and denied permissions for a given IdentityDescriptor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlEntry {
    #[doc = "The set of permission bits that represent the actions that the associated descriptor is allowed to perform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow: Option<i32>,
    #[doc = "The set of permission bits that represent the actions that the associated descriptor is not allowed to perform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny: Option<i32>,
    #[doc = "The descriptor for the user this AccessControlEntry applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "Holds the inherited and effective permission information for a given AccessControlEntry."]
    #[serde(
        rename = "extendedInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extended_info: Option<AceExtendedInformation>,
}
impl AccessControlEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlEntryList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessControlEntry>,
}
impl AccessControlEntryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The AccessControlList class is meant to associate a set of AccessControlEntries with a security token and its inheritance settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlList {
    #[doc = "Storage of permissions keyed on the identity the permission is for."]
    #[serde(
        rename = "acesDictionary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aces_dictionary: Option<serde_json::Value>,
    #[doc = "True if this ACL holds ACEs that have extended information."]
    #[serde(
        rename = "includeExtendedInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_extended_info: Option<bool>,
    #[doc = "True if the given token inherits permissions from parents."]
    #[serde(
        rename = "inheritPermissions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inherit_permissions: Option<bool>,
    #[doc = "The token that this AccessControlList is for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl AccessControlList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessControlListBody {
    pub value: Vec<AccessControlList>,
}
impl AccessControlListBody {
    pub fn new(value: Vec<AccessControlList>) -> Self {
        Self { value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlListList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessControlList>,
}
impl AccessControlListList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of AccessControlList. An AccessControlList is meant to associate a set of AccessControlEntries with a security token and its inheritance settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessControlListsCollection {}
impl AccessControlListsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Holds the inherited and effective permission information for a given AccessControlEntry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AceExtendedInformation {
    #[doc = "This is the combination of all of the explicit and inherited permissions for this identity on this token.  These are the permissions used when determining if a given user has permission to perform an action."]
    #[serde(
        rename = "effectiveAllow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_allow: Option<i32>,
    #[doc = "This is the combination of all of the explicit and inherited permissions for this identity on this token.  These are the permissions used when determining if a given user has permission to perform an action."]
    #[serde(
        rename = "effectiveDeny",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_deny: Option<i32>,
    #[doc = "These are the permissions that are inherited for this identity on this token.  If the token does not inherit permissions this will be 0.  Note that any permissions that have been explicitly set on this token for this identity, or any groups that this identity is a part of, are not included here."]
    #[serde(
        rename = "inheritedAllow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inherited_allow: Option<i32>,
    #[doc = "These are the permissions that are inherited for this identity on this token.  If the token does not inherit permissions this will be 0.  Note that any permissions that have been explicitly set on this token for this identity, or any groups that this identity is a part of, are not included here."]
    #[serde(
        rename = "inheritedDeny",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inherited_deny: Option<i32>,
}
impl AceExtendedInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionDefinition {
    #[doc = "The bit mask integer for this action. Must be a power of 2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit: Option<i32>,
    #[doc = "The localized display name for this action."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "The non-localized name for this action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The namespace that this action belongs to.  This will only be used for reading from the database."]
    #[serde(
        rename = "namespaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_id: Option<String>,
}
impl ActionDefinition {
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
#[doc = "Represents an evaluated permission."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionEvaluation {
    #[doc = "Permission bit for this evaluated permission."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<i32>,
    #[doc = "Security namespace identifier for this evaluated permission."]
    #[serde(
        rename = "securityNamespaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_namespace_id: Option<String>,
    #[doc = "Security namespace-specific token for this evaluated permission."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Permission evaluation value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
impl PermissionEvaluation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a set of evaluated permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionEvaluationBatch {
    #[doc = "True if members of the Administrators group should always pass the security check."]
    #[serde(
        rename = "alwaysAllowAdministrators",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_allow_administrators: Option<bool>,
    #[doc = "Array of permission evaluations to evaluate."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evaluations: Vec<PermissionEvaluation>,
}
impl PermissionEvaluationBatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for describing the details of a TeamFoundationSecurityNamespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityNamespaceDescription {
    #[doc = "The list of actions that this Security Namespace is responsible for securing."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<ActionDefinition>,
    #[doc = "This is the dataspace category that describes where the security information for this SecurityNamespace should be stored."]
    #[serde(
        rename = "dataspaceCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dataspace_category: Option<String>,
    #[doc = "This localized name for this namespace."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "If the security tokens this namespace will be operating on need to be split on certain character lengths to determine its elements, that length should be specified here. If not, this value will be -1."]
    #[serde(
        rename = "elementLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub element_length: Option<i32>,
    #[doc = "This is the type of the extension that should be loaded from the plugins directory for extending this security namespace."]
    #[serde(
        rename = "extensionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_type: Option<String>,
    #[doc = "If true, the security namespace is remotable, allowing another service to proxy the namespace."]
    #[serde(
        rename = "isRemotable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_remotable: Option<bool>,
    #[doc = "This non-localized for this namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unique identifier for this namespace."]
    #[serde(
        rename = "namespaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_id: Option<String>,
    #[doc = "The permission bits needed by a user in order to read security data on the Security Namespace."]
    #[serde(
        rename = "readPermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub read_permission: Option<i32>,
    #[doc = "If the security tokens this namespace will be operating on need to be split on certain characters to determine its elements that character should be specified here. If not, this value will be the null character."]
    #[serde(
        rename = "separatorValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub separator_value: Option<String>,
    #[doc = "Used to send information about the structure of the security namespace over the web service."]
    #[serde(
        rename = "structureValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub structure_value: Option<i32>,
    #[doc = "The bits reserved by system store"]
    #[serde(
        rename = "systemBitMask",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_bit_mask: Option<i32>,
    #[doc = "If true, the security service will expect an ISecurityDataspaceTokenTranslator plugin to exist for this namespace"]
    #[serde(
        rename = "useTokenTranslator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_token_translator: Option<bool>,
    #[doc = "The permission bits needed by a user in order to modify security data on the Security Namespace."]
    #[serde(
        rename = "writePermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub write_permission: Option<i32>,
}
impl SecurityNamespaceDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityNamespaceDescriptionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityNamespaceDescription>,
}
impl SecurityNamespaceDescriptionList {
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
