// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Detailed report of permissions for a set of groups and users over a set of security namespaces"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionsReport {
    #[doc = "Error if the report creation failed or empty if successful"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the report which typically includes the requestor's display name"]
    #[serde(
        rename = "reportName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_name: Option<String>,
    #[serde(
        rename = "reportStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_status: Option<permissions_report::ReportStatus>,
    #[serde(
        rename = "reportStatusLastUpdatedTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub report_status_last_updated_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "requestedTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub requested_time: Option<time::OffsetDateTime>,
    #[doc = "User who requested the report be created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requestor: Option<String>,
}
impl PermissionsReport {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod permissions_report {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReportStatus {
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completedWithErrors")]
        CompletedWithErrors,
        #[serde(rename = "completedSuccessfully")]
        CompletedSuccessfully,
        #[serde(rename = "deleted")]
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionsReportList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PermissionsReport>,
}
impl PermissionsReportList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details for creating a permissions report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionsReportRequest {
    #[doc = "List of groups and users to fetch permissions on.  An empty list will fetch all groups and users in the organization"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub descriptors: Vec<String>,
    #[doc = "Name of the report to create, make it unique"]
    #[serde(
        rename = "reportName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_name: Option<String>,
    #[doc = "List of resources to fetch permisions on"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<PermissionsReportResource>,
}
impl PermissionsReportRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifics of the resource for the permissions report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionsReportResource {
    #[doc = "GUID, Name, or ref for the specified resource type"]
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<String>,
    #[doc = "For repo resource type, resource name is the repo name"]
    #[serde(
        rename = "resourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_name: Option<String>,
    #[doc = "Specify the type of resource to report permissions on"]
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<permissions_report_resource::ResourceType>,
}
impl PermissionsReportResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod permissions_report_resource {
    use super::*;
    #[doc = "Specify the type of resource to report permissions on"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceType {
        #[serde(rename = "repo")]
        Repo,
        #[serde(rename = "ref")]
        Ref,
        #[serde(rename = "projectGit")]
        ProjectGit,
        #[serde(rename = "release")]
        Release,
        #[serde(rename = "tfvc")]
        Tfvc,
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
