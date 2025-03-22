// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Data required to deprecate multiple package versions. Pass this while performing NpmBatchOperationTypes.Deprecate batch operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchDeprecateData {
    #[doc = "Deprecate message that will be added to packages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl BatchDeprecateData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data required to unlist or relist multiple package versions. Pass this while performing NuGetBatchOperationTypes.List batch operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchListData {
    #[doc = "The desired listed status for the package versions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listed: Option<bool>,
}
impl BatchListData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Do not attempt to use this type to create a new BatchOperationData. This type does not contain sufficient fields to create a new batch operation data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchOperationData {}
impl BatchOperationData {
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
pub struct MavenDistributionManagement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<MavenRepository>,
    #[serde(
        rename = "snapshotRepository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_repository: Option<MavenSnapshotRepository>,
}
impl MavenDistributionManagement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identifies a particular Maven package version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenMinimalPackageDetails {
    #[doc = "Package artifact ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[doc = "Package group ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[doc = "Package version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl MavenMinimalPackageDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPackage {
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[doc = "The class to represent a REST reference link.  RFC:<http://tools>.ietf.org/html/draft-kelly-json-hal-06  The RFC is not fully implemented, additional properties are allowed on the reference link but as of yet we don't have a need for them."]
    #[serde(
        rename = "artifactIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_index: Option<ReferenceLink>,
    #[doc = "The class to represent a REST reference link.  RFC:<http://tools>.ietf.org/html/draft-kelly-json-hal-06  The RFC is not fully implemented, additional properties are allowed on the reference link but as of yet we don't have a need for them."]
    #[serde(
        rename = "artifactMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_metadata: Option<ReferenceLink>,
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<ReferenceLinks>,
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pom: Option<MavenPomMetadata>,
    #[doc = "The class to represent a REST reference link.  RFC:<http://tools>.ietf.org/html/draft-kelly-json-hal-06  The RFC is not fully implemented, additional properties are allowed on the reference link but as of yet we don't have a need for them."]
    #[serde(
        rename = "requestedFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_file: Option<ReferenceLink>,
    #[doc = "The class to represent a REST reference link.  RFC:<http://tools>.ietf.org/html/draft-kelly-json-hal-06  The RFC is not fully implemented, additional properties are allowed on the reference link but as of yet we don't have a need for them."]
    #[serde(
        rename = "snapshotMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_metadata: Option<ReferenceLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<ReferenceLinks>,
    #[doc = "The class to represent a REST reference link.  RFC:<http://tools>.ietf.org/html/draft-kelly-json-hal-06  The RFC is not fully implemented, additional properties are allowed on the reference link but as of yet we don't have a need for them."]
    #[serde(
        rename = "versionsIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub versions_index: Option<ReferenceLink>,
}
impl MavenPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deletion state of a maven package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPackageVersionDeletionState {
    #[doc = "Artifact Id of the package."]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[doc = "UTC date the package was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "Group Id of the package."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Version of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl MavenPackageVersionDeletionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A batch of operations to apply to package versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPackagesBatchRequest {
    #[doc = "Do not attempt to use this type to create a new BatchOperationData. This type does not contain sufficient fields to create a new batch operation data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[doc = "Type of operation that needs to be performed on packages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<maven_packages_batch_request::Operation>,
    #[doc = "The packages onto which the operation will be performed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub packages: Vec<MavenMinimalPackageDetails>,
}
impl MavenPackagesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maven_packages_batch_request {
    use super::*;
    #[doc = "Type of operation that needs to be performed on packages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "promote")]
        Promote,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "permanentDelete")]
        PermanentDelete,
        #[serde(rename = "restoreToFeed")]
        RestoreToFeed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomBuild {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plugins: Vec<Plugin>,
}
impl MavenPomBuild {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomCi {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notifiers: Vec<MavenPomCiNotifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl MavenPomCi {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomCiNotifier {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configuration: Vec<String>,
    #[serde(
        rename = "sendOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_on_error: Option<String>,
    #[serde(
        rename = "sendOnFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_on_failure: Option<String>,
    #[serde(
        rename = "sendOnSuccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_on_success: Option<String>,
    #[serde(
        rename = "sendOnWarning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_on_warning: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl MavenPomCiNotifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomDependency {
    #[serde(flatten)]
    pub maven_pom_gav: MavenPomGav,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl MavenPomDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomDependencyManagement {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependencies: Vec<MavenPomDependency>,
}
impl MavenPomDependencyManagement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomGav {
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl MavenPomGav {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomIssueManagement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl MavenPomIssueManagement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomLicense {
    #[serde(flatten)]
    pub maven_pom_organization: MavenPomOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
}
impl MavenPomLicense {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomMailingList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "otherArchives",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_archives: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<String>,
}
impl MavenPomMailingList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomMetadata {
    #[serde(flatten)]
    pub maven_pom_gav: MavenPomGav,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<MavenPomBuild>,
    #[serde(
        rename = "ciManagement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ci_management: Option<MavenPomCi>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contributors: Vec<MavenPomPerson>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependencies: Vec<MavenPomDependency>,
    #[serde(
        rename = "dependencyManagement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dependency_management: Option<MavenPomDependencyManagement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub developers: Vec<MavenPomPerson>,
    #[serde(
        rename = "distributionManagement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub distribution_management: Option<MavenDistributionManagement>,
    #[serde(
        rename = "inceptionYear",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inception_year: Option<String>,
    #[serde(
        rename = "issueManagement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_management: Option<MavenPomIssueManagement>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub licenses: Vec<MavenPomLicense>,
    #[serde(
        rename = "mailingLists",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mailing_lists: Vec<MavenPomMailingList>,
    #[serde(
        rename = "modelVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub model_version: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub modules: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<MavenPomOrganization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<MavenPomParent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerequisites: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scm: Option<MavenPomScm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl MavenPomMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomOrganization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl MavenPomOrganization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomParent {
    #[serde(flatten)]
    pub maven_pom_gav: MavenPomGav,
    #[serde(
        rename = "relativePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_path: Option<String>,
}
impl MavenPomParent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomPerson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(
        rename = "organizationUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub roles: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl MavenPomPerson {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenPomScm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    #[serde(
        rename = "developerConnection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub developer_connection: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl MavenPomScm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenRecycleBinPackageVersionDetails {
    #[doc = "Setting to false will undo earlier deletion and restore the package to feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
impl MavenRecycleBinPackageVersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenRepository {
    #[serde(
        rename = "uniqueVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_version: Option<bool>,
}
impl MavenRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MavenSnapshotRepository {
    #[serde(flatten)]
    pub maven_repository: MavenRepository,
}
impl MavenSnapshotRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Minimal package details required to identify a package within a protocol."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MinimalPackageDetails {
    #[doc = "Package name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Package version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl MinimalPackageDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deletion state of an npm package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NpmPackageVersionDeletionState {
    #[doc = "Name of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "UTC date the package was unpublished."]
    #[serde(
        rename = "unpublishedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub unpublished_date: Option<time::OffsetDateTime>,
    #[doc = "Version of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl NpmPackageVersionDeletionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A batch of operations to apply to package versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NpmPackagesBatchRequest {
    #[doc = "Do not attempt to use this type to create a new BatchOperationData. This type does not contain sufficient fields to create a new batch operation data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[doc = "Type of operation that needs to be performed on packages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<npm_packages_batch_request::Operation>,
    #[doc = "The packages onto which the operation will be performed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub packages: Vec<MinimalPackageDetails>,
}
impl NpmPackagesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod npm_packages_batch_request {
    use super::*;
    #[doc = "Type of operation that needs to be performed on packages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "promote")]
        Promote,
        #[serde(rename = "deprecate")]
        Deprecate,
        #[serde(rename = "unpublish")]
        Unpublish,
        #[serde(rename = "permanentDelete")]
        PermanentDelete,
        #[serde(rename = "restoreToFeed")]
        RestoreToFeed,
        #[serde(rename = "delete")]
        Delete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NpmRecycleBinPackageVersionDetails {
    #[doc = "Setting to false will undo earlier deletion and restore the package to feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
impl NpmRecycleBinPackageVersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deletion state of a NuGet package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NuGetPackageVersionDeletionState {
    #[doc = "Utc date the package was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "Name of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl NuGetPackageVersionDeletionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A batch of operations to apply to package versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NuGetPackagesBatchRequest {
    #[doc = "Do not attempt to use this type to create a new BatchOperationData. This type does not contain sufficient fields to create a new batch operation data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[doc = "Type of operation that needs to be performed on packages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<nu_get_packages_batch_request::Operation>,
    #[doc = "The packages onto which the operation will be performed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub packages: Vec<MinimalPackageDetails>,
}
impl NuGetPackagesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nu_get_packages_batch_request {
    use super::*;
    #[doc = "Type of operation that needs to be performed on packages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "promote")]
        Promote,
        #[serde(rename = "list")]
        List,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "permanentDelete")]
        PermanentDelete,
        #[serde(rename = "restoreToFeed")]
        RestoreToFeed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NuGetRecycleBinPackageVersionDetails {
    #[doc = "Setting to false will undo earlier deletion and restore the package to feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
impl NuGetRecycleBinPackageVersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Package version metadata for a Maven package"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Package {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "If and when the package was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "Package Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If and when the package was permanently deleted."]
    #[serde(
        rename = "permanentlyDeletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub permanently_deleted_date: Option<time::OffsetDateTime>,
    #[doc = "The history of upstream sources for this package. The first source in the list is the immediate source from which this package was saved."]
    #[serde(
        rename = "sourceChain",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_chain: Vec<UpstreamSourceInfo>,
    #[doc = "The version of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Package {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersionDetails {
    #[doc = "The JSON model for a JSON Patch operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<JsonPatchOperation>,
}
impl PackageVersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Plugin {
    #[serde(flatten)]
    pub maven_pom_gav: MavenPomGav,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PluginConfiguration>,
}
impl Plugin {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PluginConfiguration {
    #[serde(
        rename = "goalPrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub goal_prefix: Option<String>,
}
impl PluginConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deletion state of a Python package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PyPiPackageVersionDeletionState {
    #[doc = "UTC date the package was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "Name of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PyPiPackageVersionDeletionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A batch of operations to apply to package versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PyPiPackagesBatchRequest {
    #[doc = "Do not attempt to use this type to create a new BatchOperationData. This type does not contain sufficient fields to create a new batch operation data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[doc = "Type of operation that needs to be performed on packages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<py_pi_packages_batch_request::Operation>,
    #[doc = "The packages onto which the operation will be performed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub packages: Vec<MinimalPackageDetails>,
}
impl PyPiPackagesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod py_pi_packages_batch_request {
    use super::*;
    #[doc = "Type of operation that needs to be performed on packages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "promote")]
        Promote,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "permanentDelete")]
        PermanentDelete,
        #[serde(rename = "restoreToFeed")]
        RestoreToFeed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PyPiRecycleBinPackageVersionDetails {
    #[doc = "Setting to false will undo earlier deletion and restore the package to feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
impl PyPiRecycleBinPackageVersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class to represent a REST reference link.  RFC:<http://tools>.ietf.org/html/draft-kelly-json-hal-06  The RFC is not fully implemented, additional properties are allowed on the reference link but as of yet we don't have a need for them."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}
impl ReferenceLink {
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
#[doc = "Deletion state of a Universal package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UPackPackageVersionDeletionState {
    #[doc = "UTC date the package was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "Name of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl UPackPackageVersionDeletionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A batch of operations to apply to package versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UPackPackagesBatchRequest {
    #[doc = "Do not attempt to use this type to create a new BatchOperationData. This type does not contain sufficient fields to create a new batch operation data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[doc = "Type of operation that needs to be performed on packages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<u_pack_packages_batch_request::Operation>,
    #[doc = "The packages onto which the operation will be performed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub packages: Vec<MinimalPackageDetails>,
}
impl UPackPackagesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod u_pack_packages_batch_request {
    use super::*;
    #[doc = "Type of operation that needs to be performed on packages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "promote")]
        Promote,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "permanentDelete")]
        PermanentDelete,
        #[serde(rename = "restoreToFeed")]
        RestoreToFeed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UPackRecycleBinPackageVersionDetails {
    #[doc = "Setting to false will undo earlier deletion and restore the package to feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
impl UPackRecycleBinPackageVersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Upstream source definition, including its Identity, package type, and other associated information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamSourceInfo {
    #[doc = "Locator for connecting to the upstream source in a user friendly format, that may potentially change over time"]
    #[serde(
        rename = "displayLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_location: Option<String>,
    #[doc = "Identity of the upstream source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Locator for connecting to the upstream source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Display name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Source type, such as Public or Internal."]
    #[serde(
        rename = "sourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_type: Option<upstream_source_info::SourceType>,
}
impl UpstreamSourceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upstream_source_info {
    use super::*;
    #[doc = "Source type, such as Public or Internal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "internal")]
        Internal,
    }
}
#[doc = "Describes upstreaming behavior for a given feed/protocol/package"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamingBehavior {
    #[doc = "Indicates whether external upstream versions should be considered for this package"]
    #[serde(
        rename = "versionsFromExternalUpstreams",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub versions_from_external_upstreams:
        Option<upstreaming_behavior::VersionsFromExternalUpstreams>,
}
impl UpstreamingBehavior {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upstreaming_behavior {
    use super::*;
    #[doc = "Indicates whether external upstream versions should be considered for this package"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionsFromExternalUpstreams {
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "allowExternalVersions")]
        AllowExternalVersions,
    }
}
