// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "The full policy configuration with settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyConfiguration {
    #[serde(flatten)]
    pub versioned_policy_configuration_ref: VersionedPolicyConfigurationRef,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "The date and time when the policy was created."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether the policy is blocking."]
    #[serde(
        rename = "isBlocking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_blocking: Option<bool>,
    #[doc = "Indicates whether the policy has been (soft) deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Indicates whether the policy is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "If set, this policy requires \"Manage Enterprise Policies\" permission to create, edit, or delete."]
    #[serde(
        rename = "isEnterpriseManaged",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_enterprise_managed: Option<bool>,
    #[doc = "The policy configuration settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
impl PolicyConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyConfigurationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyConfiguration>,
}
impl PolicyConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy configuration reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyConfigurationRef {
    #[doc = "The policy configuration ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Policy type reference."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<PolicyTypeRef>,
    #[doc = "The URL where the policy configuration can be retrieved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PolicyConfigurationRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This record encapsulates the current state of a policy as it applies to one specific pull request. Each pull request has a unique PolicyEvaluationRecord for each pull request which the policy applies to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvaluationRecord {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "A string which uniquely identifies the target of a policy evaluation."]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[doc = "Time when this policy finished evaluating on this pull request."]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "The full policy configuration with settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PolicyConfiguration>,
    #[doc = "Internal context data of this policy evaluation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[doc = "Guid which uniquely identifies this evaluation record (one policy running on one pull request)."]
    #[serde(
        rename = "evaluationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evaluation_id: Option<String>,
    #[doc = "Time when this policy was first evaluated on this pull request."]
    #[serde(
        rename = "startedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_date: Option<time::OffsetDateTime>,
    #[doc = "Status of the policy (Running, Approved, Failed, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<policy_evaluation_record::Status>,
}
impl PolicyEvaluationRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_evaluation_record {
    use super::*;
    #[doc = "Status of the policy (Running, Approved, Failed, etc.)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "notApplicable")]
        NotApplicable,
        #[serde(rename = "broken")]
        Broken,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvaluationRecordList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyEvaluationRecord>,
}
impl PolicyEvaluationRecordList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User-friendly policy type with description (used for querying policy types)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyType {
    #[serde(flatten)]
    pub policy_type_ref: PolicyTypeRef,
    #[doc = "Links"]
    #[serde(rename = "_links")]
    pub links: serde_json::Value,
    #[doc = "Detailed description of the policy type."]
    pub description: String,
}
impl PolicyType {
    pub fn new(
        policy_type_ref: PolicyTypeRef,
        links: serde_json::Value,
        description: String,
    ) -> Self {
        Self {
            policy_type_ref,
            links,
            description,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyType>,
}
impl PolicyTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy type reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyTypeRef {
    #[doc = "Display name of the policy type."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The policy type ID."]
    pub id: String,
    #[doc = "The URL where the policy type can be retrieved."]
    pub url: String,
}
impl PolicyTypeRef {
    pub fn new(display_name: String, id: String, url: String) -> Self {
        Self {
            display_name,
            id,
            url,
        }
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
#[doc = "A particular revision for a policy configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionedPolicyConfigurationRef {
    #[serde(flatten)]
    pub policy_configuration_ref: PolicyConfigurationRef,
    #[doc = "The policy configuration revision ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl VersionedPolicyConfigurationRef {
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
