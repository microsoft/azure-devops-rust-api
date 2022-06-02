#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchOperationData {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatchOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<json_patch_operation::Op>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
pub mod json_patch_operation {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenMinimalPackageDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPackage {
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(
        rename = "artifactIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_index: Option<ReferenceLink>,
    #[serde(
        rename = "artifactMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_metadata: Option<ReferenceLink>,
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<ReferenceLinks>,
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pom: Option<MavenPomMetadata>,
    #[serde(
        rename = "requestedFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_file: Option<ReferenceLink>,
    #[serde(
        rename = "snapshotMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_metadata: Option<ReferenceLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<ReferenceLinks>,
    #[serde(
        rename = "versionsIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub versions_index: Option<ReferenceLink>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPackagesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<maven_packages_batch_request::Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<MavenMinimalPackageDetails>,
}
pub mod maven_packages_batch_request {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPackageVersionDeletionState {
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomBuild {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Plugin>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomCi {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifiers: Vec<MavenPomCiNotifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomCiNotifier {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomDependencyManagement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<MavenPomDependency>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomIssueManagement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomLicense {
    #[serde(flatten)]
    pub maven_pom_organization: MavenPomOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomMailingList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "otherArchives",
        default,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<MavenPomPerson>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<MavenPomDependency>,
    #[serde(
        rename = "dependencyManagement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dependency_management: Option<MavenPomDependencyManagement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub licenses: Vec<MavenPomLicense>,
    #[serde(
        rename = "mailingLists",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mailing_lists: Vec<MavenPomMailingList>,
    #[serde(
        rename = "modelVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub model_version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenPomOrganization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenRecycleBinPackageVersionDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenRepository {
    #[serde(
        rename = "uniqueVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_version: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MavenSnapshotRepository {
    #[serde(flatten)]
    pub maven_repository: MavenRepository,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "permanentlyDeletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permanently_deleted_date: Option<String>,
    #[serde(rename = "sourceChain", default, skip_serializing_if = "Vec::is_empty")]
    pub source_chain: Vec<UpstreamSourceInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<JsonPatchOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plugin {
    #[serde(flatten)]
    pub maven_pom_gav: MavenPomGav,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PluginConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfiguration {
    #[serde(
        rename = "goalPrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub goal_prefix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpstreamingBehavior {
    #[serde(
        rename = "versionsFromExternalUpstreams",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub versions_from_external_upstreams:
        Option<upstreaming_behavior::VersionsFromExternalUpstreams>,
}
pub mod upstreaming_behavior {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionsFromExternalUpstreams {
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "allowExternalVersions")]
        AllowExternalVersions,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpstreamSourceInfo {
    #[serde(
        rename = "displayLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "sourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_type: Option<upstream_source_info::SourceType>,
}
pub mod upstream_source_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "internal")]
        Internal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchDeprecateData {
    #[serde(flatten)]
    pub batch_operation_data: BatchOperationData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MinimalPackageDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpmPackagesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<npm_packages_batch_request::Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<MinimalPackageDetails>,
}
pub mod npm_packages_batch_request {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpmPackageVersionDeletionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "unpublishedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unpublished_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpmRecycleBinPackageVersionDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchListData {
    #[serde(flatten)]
    pub batch_operation_data: BatchOperationData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listed: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NuGetPackagesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<nu_get_packages_batch_request::Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<MinimalPackageDetails>,
}
pub mod nu_get_packages_batch_request {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NuGetPackageVersionDeletionState {
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NuGetRecycleBinPackageVersionDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PyPiPackagesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<py_pi_packages_batch_request::Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<MinimalPackageDetails>,
}
pub mod py_pi_packages_batch_request {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PyPiPackageVersionDeletionState {
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PyPiRecycleBinPackageVersionDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UPackPackagesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchOperationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<u_pack_packages_batch_request::Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<MinimalPackageDetails>,
}
pub mod u_pack_packages_batch_request {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UPackPackageVersionDeletionState {
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UPackRecycleBinPackageVersionDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
