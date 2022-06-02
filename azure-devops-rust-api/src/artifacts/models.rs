#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildPackage {
    #[serde(rename = "feedName", default, skip_serializing_if = "Option::is_none")]
    pub feed_name: Option<String>,
    #[serde(
        rename = "packageDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_description: Option<String>,
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<String>,
    #[serde(
        rename = "packageVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "protocolType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    #[serde(flatten)]
    pub feed_core: FeedCore,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "badgesEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub badges_enabled: Option<bool>,
    #[serde(
        rename = "defaultViewId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_view_id: Option<String>,
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "hideDeletedPackageVersions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_deleted_package_versions: Option<bool>,
    #[serde(
        rename = "permanentDeletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permanent_deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<FeedPermission>,
    #[serde(
        rename = "scheduledPermanentDeleteDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_permanent_delete_date: Option<String>,
    #[serde(
        rename = "upstreamEnabledChangedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_enabled_changed_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedBatchData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<FeedBatchOperationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<feed_batch_data::Operation>,
}
pub mod feed_batch_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "saveCachedPackages")]
        SaveCachedPackages,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedBatchOperationData {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedChange {
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<feed_change::ChangeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed: Option<Feed>,
    #[serde(
        rename = "feedContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub feed_continuation_token: Option<i64>,
    #[serde(
        rename = "latestPackageContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_package_continuation_token: Option<i64>,
}
pub mod feed_change {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "addOrUpdate")]
        AddOrUpdate,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "permanentDelete")]
        PermanentDelete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedChangesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "feedChanges", default, skip_serializing_if = "Vec::is_empty")]
    pub feed_changes: Vec<FeedChange>,
    #[serde(
        rename = "nextFeedContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_feed_continuation_token: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedCore {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<feed_core::Capabilities>,
    #[serde(
        rename = "fullyQualifiedId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fully_qualified_id: Option<String>,
    #[serde(
        rename = "fullyQualifiedName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fully_qualified_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isReadOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_read_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(
        rename = "upstreamEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_enabled: Option<bool>,
    #[serde(
        rename = "upstreamSources",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upstream_sources: Vec<UpstreamSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<FeedView>,
    #[serde(rename = "viewId", default, skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(rename = "viewName", default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
}
pub mod feed_core {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Capabilities {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "upstreamV2")]
        UpstreamV2,
        #[serde(rename = "underMaintenance")]
        UnderMaintenance,
        #[serde(rename = "defaultCapabilities")]
        DefaultCapabilities,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedPermission {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        rename = "identityDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_descriptor: Option<IdentityDescriptor>,
    #[serde(
        rename = "identityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_id: Option<String>,
    #[serde(
        rename = "isInheritedRole",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_inherited_role: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<feed_permission::Role>,
}
pub mod feed_permission {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "reader")]
        Reader,
        #[serde(rename = "contributor")]
        Contributor,
        #[serde(rename = "administrator")]
        Administrator,
        #[serde(rename = "collaborator")]
        Collaborator,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedRetentionPolicy {
    #[serde(
        rename = "ageLimitInDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub age_limit_in_days: Option<i32>,
    #[serde(
        rename = "countLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_limit: Option<i32>,
    #[serde(
        rename = "daysToKeepRecentlyDownloadedPackages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_keep_recently_downloaded_packages: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedUpdate {
    #[serde(
        rename = "allowUpstreamNameConflict",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_upstream_name_conflict: Option<bool>,
    #[serde(
        rename = "badgesEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub badges_enabled: Option<bool>,
    #[serde(
        rename = "defaultViewId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "hideDeletedPackageVersions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_deleted_package_versions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "upstreamEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_enabled: Option<bool>,
    #[serde(
        rename = "upstreamSources",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upstream_sources: Vec<UpstreamSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedView {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<feed_view::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<feed_view::Visibility>,
}
pub mod feed_view {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "release")]
        Release,
        #[serde(rename = "implicit")]
        Implicit,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Visibility {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "collection")]
        Collection,
        #[serde(rename = "organization")]
        Organization,
        #[serde(rename = "aadTenant")]
        AadTenant,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalPermission {
    #[serde(
        rename = "identityDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_descriptor: Option<IdentityDescriptor>,
    #[serde(
        rename = "identityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<global_permission::Role>,
}
pub mod global_permission {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "feedCreator")]
        FeedCreator,
        #[serde(rename = "administrator")]
        Administrator,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(
        rename = "identityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatchDocument {
    #[serde(flatten)]
    pub vec_json_patch_operation: Vec<JsonPatchOperation>,
}
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
pub struct MinimalPackageVersion {
    #[serde(
        rename = "directUpstreamSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_upstream_source_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isCachedVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_cached_version: Option<bool>,
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(rename = "isLatest", default, skip_serializing_if = "Option::is_none")]
    pub is_latest: Option<bool>,
    #[serde(rename = "isListed", default, skip_serializing_if = "Option::is_none")]
    pub is_listed: Option<bool>,
    #[serde(
        rename = "normalizedVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub normalized_version: Option<String>,
    #[serde(
        rename = "packageDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_description: Option<String>,
    #[serde(
        rename = "publishDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publish_date: Option<String>,
    #[serde(rename = "storageId", default, skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<FeedView>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "pluginId", default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_reference::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
pub mod operation_reference {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isCached", default, skip_serializing_if = "Option::is_none")]
    pub is_cached: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "normalizedName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub normalized_name: Option<String>,
    #[serde(
        rename = "protocolType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_type: Option<String>,
    #[serde(rename = "starCount", default, skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<MinimalPackageVersion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageChange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<Package>,
    #[serde(
        rename = "packageVersionChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version_change: Option<PackageVersionChange>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageChangesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        rename = "nextPackageContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_package_continuation_token: Option<i64>,
    #[serde(
        rename = "packageChanges",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_changes: Vec<PackageChange>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageDependency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<String>,
    #[serde(
        rename = "versionRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_range: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageFile {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PackageFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "protocolMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_metadata: Option<ProtocolMetadata>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageMetrics {
    #[serde(
        rename = "downloadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_count: Option<f64>,
    #[serde(
        rename = "downloadUniqueUsers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_unique_users: Option<f64>,
    #[serde(
        rename = "lastDownloaded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_downloaded: Option<String>,
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageMetricsQuery {
    #[serde(rename = "packageIds", default, skip_serializing_if = "Vec::is_empty")]
    pub package_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersion {
    #[serde(flatten)]
    pub minimal_package_version: MinimalPackageVersion,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<PackageDependency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<PackageFile>,
    #[serde(
        rename = "otherVersions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_versions: Vec<MinimalPackageVersion>,
    #[serde(
        rename = "protocolMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_metadata: Option<ProtocolMetadata>,
    #[serde(rename = "sourceChain", default, skip_serializing_if = "Vec::is_empty")]
    pub source_chain: Vec<UpstreamSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionChange {
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<package_version_change::ChangeType>,
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<i64>,
    #[serde(
        rename = "packageVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version: Option<PackageVersion>,
}
pub mod package_version_change {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "addOrUpdate")]
        AddOrUpdate,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "permanentDelete")]
        PermanentDelete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionMetrics {
    #[serde(
        rename = "downloadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_count: Option<f64>,
    #[serde(
        rename = "downloadUniqueUsers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_unique_users: Option<f64>,
    #[serde(
        rename = "lastDownloaded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_downloaded: Option<String>,
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(
        rename = "packageVersionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionMetricsQuery {
    #[serde(
        rename = "packageVersionIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_version_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionProvenance {
    #[serde(rename = "feedId", default, skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<String>,
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(
        rename = "packageVersionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(
        rename = "schemaVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schema_version: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(
        rename = "provenanceSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provenance_source: Option<String>,
    #[serde(
        rename = "publisherUserIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_user_identity: Option<String>,
    #[serde(rename = "userAgent", default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecycleBinPackageVersion {
    #[serde(flatten)]
    pub package_version: PackageVersion,
    #[serde(
        rename = "scheduledPermanentDeleteDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_permanent_delete_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveCachedPackagesData {
    #[serde(flatten)]
    pub feed_batch_operation_data: FeedBatchOperationData,
    #[serde(
        rename = "normalizedPackageNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub normalized_package_names: Vec<String>,
    #[serde(
        rename = "viewsForPromotion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub views_for_promotion: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpstreamSource {
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[serde(
        rename = "displayLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "internalUpstreamCollectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_collection_id: Option<String>,
    #[serde(
        rename = "internalUpstreamFeedId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_feed_id: Option<String>,
    #[serde(
        rename = "internalUpstreamProjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_project_id: Option<String>,
    #[serde(
        rename = "internalUpstreamViewId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<upstream_source::Status>,
    #[serde(
        rename = "statusDetails",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub status_details: Vec<UpstreamStatusDetail>,
    #[serde(
        rename = "upstreamSourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_source_type: Option<upstream_source::UpstreamSourceType>,
}
pub mod upstream_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "disabled")]
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpstreamSourceType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "internal")]
        Internal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpstreamStatusDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapper {
    #[serde(flatten)]
    pub vss_json_collection_wrapper_base: VssJsonCollectionWrapperBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapperBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Feed>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedPermissionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FeedPermission>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedViewList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FeedView>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalPermissionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GlobalPermission>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Package>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageMetricsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PackageMetrics>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PackageVersion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionMetricsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PackageVersionMetrics>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecycleBinPackageVersionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecycleBinPackageVersion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionResponse {
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(
        rename = "sessionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_name: Option<String>,
}
