// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildPackage {
    #[doc = "Display name of the feed."]
    #[serde(rename = "feedName", default, skip_serializing_if = "Option::is_none")]
    pub feed_name: Option<String>,
    #[doc = "Package version description."]
    #[serde(
        rename = "packageDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_description: Option<String>,
    #[doc = "Display name of the package."]
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<String>,
    #[doc = "Version of the package."]
    #[serde(
        rename = "packageVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version: Option<String>,
    #[doc = "TFS project id."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Type of the package."]
    #[serde(
        rename = "protocolType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_type: Option<String>,
}
impl BuildPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This holds the configuration for the ManifestTool. The values in this file are populated from the command line, config file and default."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "additionalComponentDetectorArgs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_component_detector_args: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "buildComponentPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_component_path: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "buildDropPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_drop_path: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "buildListFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_list_file: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "catalogFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub catalog_file_path: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "configFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub config_file_path: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "dockerImagesToScan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub docker_images_to_scan: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "externalDocumentReferenceListFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_document_reference_list_file: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "hashAlgorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hash_algorithm: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "ignoreMissing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_missing: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "manifestDirPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_dir_path: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "manifestInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_info: Option<ConfigurationSetting>,
    #[doc = "The action currently being performed by the manifest tool."]
    #[serde(
        rename = "manifestToolAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_tool_action: Option<configuration::ManifestToolAction>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "packageVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "rootPathFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub root_path_filter: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "telemetryFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub telemetry_file_path: Option<ConfigurationSetting>,
    #[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
    #[serde(
        rename = "validateSignature",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub validate_signature: Option<ConfigurationSetting>,
}
impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration {
    use super::*;
    #[doc = "The action currently being performed by the manifest tool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ManifestToolAction {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "validate")]
        Validate,
        #[serde(rename = "generate")]
        Generate,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "Encapsulates a configuration setting to provide metadata about the setting source and type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationSetting {
    #[doc = "The source where this setting came from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The actual value of the setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ConfigurationSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Feed {
    #[serde(flatten)]
    pub feed_core: FeedCore,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "If set, this feed supports generation of package badges."]
    #[serde(
        rename = "badgesEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub badges_enabled: Option<bool>,
    #[doc = "The view that the feed administrator has indicated is the default experience for readers."]
    #[serde(
        rename = "defaultViewId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_view_id: Option<String>,
    #[doc = "The date that this feed was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "A description for the feed.  Descriptions must not exceed 255 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "If set, the feed will hide all deleted/unpublished versions"]
    #[serde(
        rename = "hideDeletedPackageVersions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_deleted_package_versions: Option<bool>,
    #[doc = "The date that this feed was permanently deleted."]
    #[serde(
        rename = "permanentDeletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub permanent_deleted_date: Option<time::OffsetDateTime>,
    #[doc = "Explicit permissions for the feed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permissions: Vec<FeedPermission>,
    #[doc = "The date that this feed is scheduled to be permanently deleted."]
    #[serde(
        rename = "scheduledPermanentDeleteDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub scheduled_permanent_delete_date: Option<time::OffsetDateTime>,
    #[doc = "If set, time that the UpstreamEnabled property was changed. Will be null if UpstreamEnabled was never changed after Feed creation."]
    #[serde(
        rename = "upstreamEnabledChangedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub upstream_enabled_changed_date: Option<time::OffsetDateTime>,
    #[doc = "The URL of the base feed in GUID form."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Feed {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedBatchData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<FeedBatchOperationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<feed_batch_data::Operation>,
}
impl FeedBatchData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod feed_batch_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        #[serde(rename = "saveCachedPackages")]
        SaveCachedPackages,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedBatchOperationData {}
impl FeedBatchOperationData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container that encapsulates the state of the feed after a create, update, or delete."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedChange {
    #[doc = "The type of operation."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<feed_change::ChangeType>,
    #[doc = "A container for artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed: Option<Feed>,
    #[doc = "A token that identifies the next change in the log of changes."]
    #[serde(
        rename = "feedContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub feed_continuation_token: Option<i64>,
    #[doc = "A token that identifies the latest package change for this feed.  This can be used to quickly determine if there have been any changes to packages in a specific feed."]
    #[serde(
        rename = "latestPackageContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_package_continuation_token: Option<i64>,
}
impl FeedChange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod feed_change {
    use super::*;
    #[doc = "The type of operation."]
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
#[doc = "A result set containing the feed changes for the range that was requested."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedChangesResponse {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "The number of changes in this set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "A container that encapsulates the state of the feed after a create, update, or delete."]
    #[serde(
        rename = "feedChanges",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub feed_changes: Vec<FeedChange>,
    #[doc = "When iterating through the log of changes this value indicates the value that should be used for the next continuation token."]
    #[serde(
        rename = "nextFeedContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_feed_continuation_token: Option<i64>,
}
impl FeedChangesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that contains all of the settings for a specific feed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedCore {
    #[doc = "Supported capabilities of a feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<feed_core::Capabilities>,
    #[doc = "This will either be the feed GUID or the feed GUID and view GUID depending on how the feed was accessed."]
    #[serde(
        rename = "fullyQualifiedId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fully_qualified_id: Option<String>,
    #[doc = "Full name of the view, in feed@view format."]
    #[serde(
        rename = "fullyQualifiedName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fully_qualified_name: Option<String>,
    #[doc = "A GUID that uniquely identifies this feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "If set, all packages in the feed are immutable.  It is important to note that feed views are immutable; therefore, this flag will always be set for views."]
    #[serde(
        rename = "isReadOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_read_only: Option<bool>,
    #[doc = "A name for the feed. feed names must follow these rules: <list type=\"bullet\"><item><description> Must not exceed 64 characters </description></item><item><description> Must not contain whitespaces </description></item><item><description> Must not start with an underscore or a period </description></item><item><description> Must not end with a period </description></item><item><description> Must not contain any of the following illegal characters: <!\\[CDATA\\[ @, ~, ;, {, }, \\\\, +, =, <, >, |, /, \\\\\\\\, ?, :, &, $, *, \\\", #, \\[, \\] \\]\\]></description></item></list>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[doc = "This should always be true. Setting to false will override all sources in UpstreamSources."]
    #[serde(
        rename = "upstreamEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_enabled: Option<bool>,
    #[doc = "A list of sources that this feed will fetch packages from.  An empty list indicates that this feed will not search any additional sources for packages."]
    #[serde(
        rename = "upstreamSources",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upstream_sources: Vec<UpstreamSource>,
    #[doc = "A view on top of a feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<FeedView>,
    #[doc = "View Id."]
    #[serde(rename = "viewId", default, skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[doc = "View name."]
    #[serde(rename = "viewName", default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
}
impl FeedCore {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod feed_core {
    use super::*;
    #[doc = "Supported capabilities of a feed."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedIdsResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "projectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
}
impl FeedIdsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Feed>,
}
impl FeedList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Permissions for a feed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedPermission {
    #[doc = "Display name for the identity."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Identity associated with this role."]
    #[serde(
        rename = "identityDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_descriptor: Option<String>,
    #[doc = "Id of the identity associated with this role."]
    #[serde(
        rename = "identityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_id: Option<String>,
    #[doc = "Boolean indicating whether the role is inherited or set directly."]
    #[serde(
        rename = "isInheritedRole",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_inherited_role: Option<bool>,
    #[doc = "The role for this identity on a feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<feed_permission::Role>,
}
impl FeedPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod feed_permission {
    use super::*;
    #[doc = "The role for this identity on a feed."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedPermissionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeedPermission>,
}
impl FeedPermissionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Retention policy settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedRetentionPolicy {
    #[doc = "This attribute is deprecated and is not honoured by retention"]
    #[serde(
        rename = "ageLimitInDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub age_limit_in_days: Option<i32>,
    #[doc = "Maximum versions to preserve per package and package type."]
    #[serde(
        rename = "countLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_limit: Option<i32>,
    #[doc = "Number of days to preserve a package version after its latest download."]
    #[serde(
        rename = "daysToKeepRecentlyDownloadedPackages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_keep_recently_downloaded_packages: Option<i32>,
}
impl FeedRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update a feed definition with these new values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedUpdate {
    #[doc = "If set, the feed will allow upload of packages that exist on the upstream"]
    #[serde(
        rename = "allowUpstreamNameConflict",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_upstream_name_conflict: Option<bool>,
    #[doc = "If set, this feed supports generation of package badges."]
    #[serde(
        rename = "badgesEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub badges_enabled: Option<bool>,
    #[doc = "The view that the feed administrator has indicated is the default experience for readers."]
    #[serde(
        rename = "defaultViewId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_view_id: Option<String>,
    #[doc = "A description for the feed.  Descriptions must not exceed 255 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "If set, feed will hide all deleted/unpublished versions"]
    #[serde(
        rename = "hideDeletedPackageVersions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_deleted_package_versions: Option<bool>,
    #[doc = "A GUID that uniquely identifies this feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A name for the feed. feed names must follow these rules: <list type=\"bullet\"><item><description> Must not exceed 64 characters </description></item><item><description> Must not contain whitespaces </description></item><item><description> Must not start with an underscore or a period </description></item><item><description> Must not end with a period </description></item><item><description> Must not contain any of the following illegal characters: <!\\[CDATA\\[ @, ~, ;, {, }, \\\\, +, =, <, >, |, /, \\\\\\\\, ?, :, &, $, *, \\\", #, \\[, \\] \\]\\]></description></item></list>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If set, the feed can proxy packages from an upstream feed"]
    #[serde(
        rename = "upstreamEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_enabled: Option<bool>,
    #[doc = "A list of sources that this feed will fetch packages from.  An empty list indicates that this feed will not search any additional sources for packages."]
    #[serde(
        rename = "upstreamSources",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upstream_sources: Vec<UpstreamSource>,
}
impl FeedUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A view on top of a feed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedView {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Id of the view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of view."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<feed_view::Type>,
    #[doc = "Url of the view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Visibility status of the view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<feed_view::Visibility>,
}
impl FeedView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod feed_view {
    use super::*;
    #[doc = "Type of view."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "release")]
        Release,
        #[serde(rename = "implicit")]
        Implicit,
    }
    #[doc = "Visibility status of the view."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedViewList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FeedView>,
}
impl FeedViewList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Used to provide the filename and hash of the SBOM file to be added to the catalog file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileHash {
    #[doc = "The filename of the SBOM."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "The string hash of the SBOM file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[doc = "The HashAlgorithmName used to generate the hash of the file."]
    #[serde(
        rename = "hashAlgorithmName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hash_algorithm_name: Option<String>,
}
impl FileHash {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Permissions for feed service-wide operations such as the creation of new feeds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalPermission {
    #[doc = "Identity of the user with the provided Role."]
    #[serde(
        rename = "identityDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_descriptor: Option<String>,
    #[doc = "IdentityId corresponding to the IdentityDescriptor"]
    #[serde(
        rename = "identityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_id: Option<String>,
    #[doc = "Role associated with the Identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<global_permission::Role>,
}
impl GlobalPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod global_permission {
    use super::*;
    #[doc = "Role associated with the Identity."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalPermissionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GlobalPermission>,
}
impl GlobalPermissionList {
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
#[doc = "Defines a manifest name and version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestInfo {
    #[doc = "The name of the manifest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the manifest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ManifestInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Core data about any package, including its id and version information and basic state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MinimalPackageVersion {
    #[doc = "Upstream source this package was ingested from."]
    #[serde(
        rename = "directUpstreamSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_upstream_source_id: Option<String>,
    #[doc = "Id for the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "\\[Obsolete\\] Used for legacy scenarios and may be removed in future versions."]
    #[serde(
        rename = "isCachedVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_cached_version: Option<bool>,
    #[doc = "True if this package has been deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "True if this is the latest version of the package by package type sort order."]
    #[serde(rename = "isLatest", default, skip_serializing_if = "Option::is_none")]
    pub is_latest: Option<bool>,
    #[doc = "(NuGet and Cargo Only) True if this package is listed."]
    #[serde(rename = "isListed", default, skip_serializing_if = "Option::is_none")]
    pub is_listed: Option<bool>,
    #[doc = "Normalized version using normalization rules specific to a package type."]
    #[serde(
        rename = "normalizedVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub normalized_version: Option<String>,
    #[doc = "Package description."]
    #[serde(
        rename = "packageDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_description: Option<String>,
    #[doc = "UTC Date the package was published to the service."]
    #[serde(
        rename = "publishDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub publish_date: Option<time::OffsetDateTime>,
    #[doc = "Internal storage id."]
    #[serde(rename = "storageId", default, skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<String>,
    #[doc = "Display version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "List of views containing this package version."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub views: Vec<FeedView>,
}
impl MinimalPackageVersion {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A package, which is a container for one or more package versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Package {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Id of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Used for legacy scenarios and may be removed in future versions."]
    #[serde(rename = "isCached", default, skip_serializing_if = "Option::is_none")]
    pub is_cached: Option<bool>,
    #[doc = "The display name of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The normalized name representing the identity of this package within its package type."]
    #[serde(
        rename = "normalizedName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub normalized_name: Option<String>,
    #[doc = "Type of the package."]
    #[serde(
        rename = "protocolType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_type: Option<String>,
    #[doc = "\\[Obsolete\\] - this field is unused and will be removed in a future release."]
    #[serde(rename = "starCount", default, skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i32>,
    #[doc = "Url for this package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "All versions for this package within its feed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub versions: Vec<MinimalPackageVersion>,
}
impl Package {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single change to a feed's packages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageChange {
    #[doc = "A package, which is a container for one or more package versions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<Package>,
    #[doc = "A change to a single package version."]
    #[serde(
        rename = "packageVersionChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version_change: Option<PackageVersionChange>,
}
impl PackageChange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of change operations to a feed's packages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageChangesResponse {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Number of changes in this batch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Token that should be used in future calls for this feed to retrieve new changes."]
    #[serde(
        rename = "nextPackageContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_package_continuation_token: Option<i64>,
    #[doc = "List of changes."]
    #[serde(
        rename = "packageChanges",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_changes: Vec<PackageChange>,
}
impl PackageChangesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dependency on another package version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageDependency {
    #[doc = "Dependency package group (an optional classification within some package types)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[doc = "Dependency package name."]
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<String>,
    #[doc = "Dependency package version range."]
    #[serde(
        rename = "versionRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_range: Option<String>,
}
impl PackageDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A package file for a specific package version, only relevant to package types that contain multiple files per version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageFile {
    #[doc = "Hierarchical representation of files."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<PackageFile>,
    #[doc = "File name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Extended metadata for a specific package type."]
    #[serde(
        rename = "protocolMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_metadata: Option<ProtocolMetadata>,
}
impl PackageFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Package>,
}
impl PackageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "All metrics for a certain package id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageMetrics {
    #[doc = "Total count of downloads per package id."]
    #[serde(
        rename = "downloadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_count: Option<f64>,
    #[doc = "Number of downloads per unique user per package id."]
    #[serde(
        rename = "downloadUniqueUsers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_unique_users: Option<f64>,
    #[doc = "UTC date and time when package was last downloaded."]
    #[serde(
        rename = "lastDownloaded",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_downloaded: Option<time::OffsetDateTime>,
    #[doc = "Package id."]
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
}
impl PackageMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageMetricsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PackageMetrics>,
}
impl PackageMetricsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query to get package metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageMetricsQuery {
    #[doc = "List of package ids"]
    #[serde(
        rename = "packageIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_ids: Vec<String>,
}
impl PackageMetricsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A specific version of a package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersion {
    #[serde(flatten)]
    pub minimal_package_version: MinimalPackageVersion,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Package version author."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "UTC date that this package version was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "List of dependencies for this package version."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependencies: Vec<PackageDependency>,
    #[doc = "Package version description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Files associated with this package version, only relevant for multi-file package types."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<PackageFile>,
    #[doc = "Other versions of this package."]
    #[serde(
        rename = "otherVersions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_versions: Vec<MinimalPackageVersion>,
    #[doc = "Extended metadata for a specific package type."]
    #[serde(
        rename = "protocolMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_metadata: Option<ProtocolMetadata>,
    #[doc = "List of upstream sources through which a package version moved to land in this feed."]
    #[serde(
        rename = "sourceChain",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_chain: Vec<UpstreamSource>,
    #[doc = "Package version summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Package version tags."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Package version url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PackageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A change to a single package version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersionChange {
    #[doc = "The type of change that was performed."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<package_version_change::ChangeType>,
    #[doc = "Token marker for this change, allowing the caller to send this value back to the service and receive changes beyond this one."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<i64>,
    #[doc = "A specific version of a package."]
    #[serde(
        rename = "packageVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version: Option<PackageVersion>,
}
impl PackageVersionChange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod package_version_change {
    use super::*;
    #[doc = "The type of change that was performed."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PackageVersion>,
}
impl PackageVersionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "All metrics for a certain package version id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersionMetrics {
    #[doc = "Total count of downloads per package version id."]
    #[serde(
        rename = "downloadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_count: Option<f64>,
    #[doc = "Number of downloads per unique user per package version id."]
    #[serde(
        rename = "downloadUniqueUsers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_unique_users: Option<f64>,
    #[doc = "UTC date and time when package version was last downloaded."]
    #[serde(
        rename = "lastDownloaded",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_downloaded: Option<time::OffsetDateTime>,
    #[doc = "Package id."]
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[doc = "Package version id."]
    #[serde(
        rename = "packageVersionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version_id: Option<String>,
}
impl PackageVersionMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersionMetricsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PackageVersionMetrics>,
}
impl PackageVersionMetricsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query to get package version metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersionMetricsQuery {
    #[doc = "List of package version ids"]
    #[serde(
        rename = "packageVersionIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_version_ids: Vec<String>,
}
impl PackageVersionMetricsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provenance for a published package version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageVersionProvenance {
    #[doc = "Name or Id of the feed."]
    #[serde(rename = "feedId", default, skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<String>,
    #[doc = "Id of the package (GUID Id, not name)."]
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[doc = "Id of the package version (GUID Id, not name)."]
    #[serde(
        rename = "packageVersionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_version_id: Option<String>,
    #[doc = "Data about the origin of a published package"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
}
impl PackageVersionProvenance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectReference {
    #[doc = "Gets or sets id of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets name of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets visibility of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}
impl ProjectReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extended metadata for a specific package type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtocolMetadata {
    #[doc = "Extended metadata for a specific package type, formatted to the associated schema version definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Schema version."]
    #[serde(
        rename = "schemaVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schema_version: Option<i32>,
}
impl ProtocolMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data about the origin of a published package"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Provenance {
    #[doc = "Other provenance data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Type of provenance source, for example \"InternalBuild\", \"InternalRelease\""]
    #[serde(
        rename = "provenanceSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provenance_source: Option<String>,
    #[doc = "Identity of user that published the package"]
    #[serde(
        rename = "publisherUserIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_user_identity: Option<String>,
    #[doc = "HTTP User-Agent used when pushing the package."]
    #[serde(rename = "userAgent", default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl Provenance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single package version within the recycle bin."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecycleBinPackageVersion {
    #[serde(flatten)]
    pub package_version: PackageVersion,
    #[doc = "UTC date on which the package will automatically be removed from the recycle bin and permanently deleted."]
    #[serde(
        rename = "scheduledPermanentDeleteDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub scheduled_permanent_delete_date: Option<time::OffsetDateTime>,
}
impl RecycleBinPackageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecycleBinPackageVersionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecycleBinPackageVersion>,
}
impl RecycleBinPackageVersionList {
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
#[doc = "Represents a SBOM file object and contains additional properties related to the file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbomFile {
    #[doc = "The size of the SBOM file in bytes."]
    #[serde(
        rename = "fileSizeInBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_size_in_bytes: Option<i64>,
    #[doc = "The path where the final generated SBOM is placed."]
    #[serde(
        rename = "sbomFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sbom_file_path: Option<String>,
    #[doc = "Defines a manifest name and version."]
    #[serde(
        rename = "sbomFormatName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sbom_format_name: Option<ManifestInfo>,
}
impl SbomFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The telemetry that is logged to a file/console for the given SBOM execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SbomTelemetry {
    #[doc = "All available bsi data from the task build execution which includes build and system environment variables like repository and build information."]
    #[serde(rename = "bsiData", default, skip_serializing_if = "Option::is_none")]
    pub bsi_data: Option<serde_json::Value>,
    #[doc = "The source of the bsi data."]
    #[serde(rename = "bsiSource", default, skip_serializing_if = "Option::is_none")]
    pub bsi_source: Option<String>,
    #[doc = "The end to end results of the extension task."]
    #[serde(
        rename = "e2ETaskResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub e2_e_task_result: Option<String>,
    #[doc = "This holds the configuration for the ManifestTool. The values in this file are populated from the command line, config file and default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Configuration>,
    #[doc = "The result of the execution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "A list of the SBOM formats and related file properties that was used in the generation/validation of the SBOM."]
    #[serde(
        rename = "sbomFormatsUsed",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sbom_formats_used: Vec<SbomFile>,
    #[doc = "Any internal switches and their value that were used during the execution. A switch can be something that was provided through a configuraiton or an environment variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switches: Option<serde_json::Value>,
    #[doc = "Error messages that came from the extension task."]
    #[serde(
        rename = "taskErrorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_error_message: Option<String>,
    #[doc = "The name of the task that logged SBOM telemetry"]
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[doc = "The unique id for this telemetry"]
    #[serde(
        rename = "telemetryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub telemetry_id: Option<String>,
    #[doc = "The result of the tool as a numeric value."]
    #[serde(
        rename = "toolExecutionResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_execution_result: Option<i32>,
}
impl SbomTelemetry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaveCachedPackagesData {
    #[serde(
        rename = "normalizedPackageNames",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub normalized_package_names: Vec<String>,
    #[serde(
        rename = "viewsForPromotion",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub views_for_promotion: Vec<String>,
}
impl SaveCachedPackagesData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionRequest {
    #[doc = "Generic property bag to store data about the session"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "The feed name or id for the session"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed: Option<String>,
    #[doc = "The type of session If a known value is provided, the Data dictionary will be validated for the presence of properties required by that type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl SessionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionResponse {
    #[doc = "The unique identifier for the session"]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "The name for the session"]
    #[serde(
        rename = "sessionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_name: Option<String>,
}
impl SessionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base reponse object for all responses from the signing api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignResponseBase {
    #[doc = "The customer correlation id that is sent to ESRP for correlating the current request to ESRP."]
    #[serde(
        rename = "customerCorrelationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_correlation_id: Option<String>,
    #[doc = "If this is an error response, it will have more information about the error."]
    #[serde(rename = "errorInfo", default, skip_serializing_if = "Option::is_none")]
    pub error_info: Option<String>,
    #[doc = "The result of the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<sign_response_base::Result>,
}
impl SignResponseBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sign_response_base {
    use super::*;
    #[doc = "The result of the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "failure")]
        Failure,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "failCanRetry")]
        FailCanRetry,
    }
}
#[doc = "The response returned by the sign status api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignStatusResponse {
    #[serde(flatten)]
    pub sign_response_base: SignResponseBase,
    #[doc = "The pre-signed download url used to download the signed catalog file."]
    #[serde(
        rename = "downloadUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_url: Option<String>,
}
impl SignStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Upstream source definition, including its Identity, package type, and other associated information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamSource {
    #[doc = "UTC date that this upstream was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
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
    #[doc = "For an internal upstream type, track the Azure DevOps organization that contains it."]
    #[serde(
        rename = "internalUpstreamCollectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_collection_id: Option<String>,
    #[doc = "For an internal upstream type, track the feed id being referenced."]
    #[serde(
        rename = "internalUpstreamFeedId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_feed_id: Option<String>,
    #[doc = "For an internal upstream type, track the project of the feed being referenced."]
    #[serde(
        rename = "internalUpstreamProjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_project_id: Option<String>,
    #[doc = "For an internal upstream type, track the view of the feed being referenced."]
    #[serde(
        rename = "internalUpstreamViewId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_upstream_view_id: Option<String>,
    #[doc = "Consistent locator for connecting to the upstream source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Display name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Package type associated with the upstream source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "The identity of the service endpoint that holds credentials to use when accessing the upstream."]
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
    #[doc = "Specifies the projectId of the Service Endpoint."]
    #[serde(
        rename = "serviceEndpointProjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_project_id: Option<String>,
    #[doc = "Specifies the status of the upstream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<upstream_source::Status>,
    #[doc = "Provides a human-readable reason for the status of the upstream."]
    #[serde(
        rename = "statusDetails",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub status_details: Vec<UpstreamStatusDetail>,
    #[doc = "Source type, such as Public or Internal."]
    #[serde(
        rename = "upstreamSourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_source_type: Option<upstream_source::UpstreamSourceType>,
}
impl UpstreamSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upstream_source {
    use super::*;
    #[doc = "Specifies the status of the upstream."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "disabled")]
        Disabled,
    }
    #[doc = "Source type, such as Public or Internal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpstreamSourceType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "internal")]
        Internal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamStatusDetail {
    #[doc = "Provides a human-readable reason for the status of the upstream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl UpstreamStatusDetail {
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
