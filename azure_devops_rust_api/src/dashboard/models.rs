// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Copy options of a Dashboard."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CopyDashboardOptions {
    #[doc = "Dashboard Scope. Can be either Project or Project_Team"]
    #[serde(
        rename = "copyDashboardScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_dashboard_scope: Option<copy_dashboard_options::CopyDashboardScope>,
    #[doc = "When this flag is set to true,option to select the folder to copy Queries of copy dashboard will appear."]
    #[serde(
        rename = "copyQueriesFlag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_queries_flag: Option<bool>,
    #[doc = "Description of the dashboard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the dashboard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the project. Provided by service at creation time."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Path to which the queries should be copied of copy dashboard"]
    #[serde(
        rename = "queryFolderPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_folder_path: Option<String>,
    #[doc = "Refresh interval of dashboard"]
    #[serde(
        rename = "refreshInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_interval: Option<i32>,
    #[doc = "ID of the team. Provided by service at creation time"]
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}
impl CopyDashboardOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod copy_dashboard_options {
    use super::*;
    #[doc = "Dashboard Scope. Can be either Project or Project_Team"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CopyDashboardScope {
        #[serde(rename = "collection_User")]
        CollectionUser,
        #[serde(rename = "project_Team")]
        ProjectTeam,
        #[serde(rename = "project")]
        Project,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CopyDashboardResponse {
    #[doc = "Model of a Dashboard."]
    #[serde(
        rename = "copiedDashboard",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copied_dashboard: Option<Dashboard>,
    #[doc = "Copy options of a Dashboard."]
    #[serde(
        rename = "copyDashboardOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_dashboard_options: Option<CopyDashboardOptions>,
}
impl CopyDashboardResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model of a Dashboard."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dashboard {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Entity to which the dashboard is scoped."]
    #[serde(
        rename = "dashboardScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dashboard_scope: Option<dashboard::DashboardScope>,
    #[doc = "Description of the dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Server defined version tracking value, used for edit collision detection."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "ID of the group for a dashboard. For team-scoped dashboards, this is the unique identifier for the team associated with the dashboard. For project-scoped dashboards this property is empty."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "ID of the Dashboard. Provided by service at creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Dashboard Last Accessed Date."]
    #[serde(
        rename = "lastAccessedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_accessed_date: Option<time::OffsetDateTime>,
    #[doc = "Id of the person who modified Dashboard."]
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<String>,
    #[doc = "Dashboard's last modified date."]
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "Name of the Dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the owner for a dashboard. For team-scoped dashboards, this is the unique identifier for the team associated with the dashboard. For project-scoped dashboards, this is the unique identifier for the user identity associated with the dashboard."]
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[doc = "Position of the dashboard, within a dashboard group. If unset at creation time, position is decided by the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[doc = "Interval for client to automatically refresh the dashboard. Expressed in minutes."]
    #[serde(
        rename = "refreshInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_interval: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The set of Widgets on the dashboard."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub widgets: Vec<Widget>,
}
impl Dashboard {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dashboard {
    use super::*;
    #[doc = "Entity to which the dashboard is scoped."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DashboardScope {
        #[serde(rename = "collection_User")]
        CollectionUser,
        #[serde(rename = "project_Team")]
        ProjectTeam,
        #[serde(rename = "project")]
        Project,
    }
}
#[doc = "Describes a list of dashboards associated to an owner. Currently, teams own dashboard groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardGroup {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "A list of Dashboards held by the Dashboard Group"]
    #[serde(
        rename = "dashboardEntries",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dashboard_entries: Vec<DashboardGroupEntry>,
    #[doc = "Deprecated: The old permission model describing the level of permissions for the current team. Pre-M125."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<dashboard_group::Permission>,
    #[doc = "A permissions bit mask describing the security permissions of the current team for dashboards. When this permission is the value None, use GroupMemberPermission. Permissions are evaluated based on the presence of a value other than None, else the GroupMemberPermission will be saved."]
    #[serde(
        rename = "teamDashboardPermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_dashboard_permission: Option<dashboard_group::TeamDashboardPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl DashboardGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dashboard_group {
    use super::*;
    #[doc = "Deprecated: The old permission model describing the level of permissions for the current team. Pre-M125."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permission {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "edit")]
        Edit,
        #[serde(rename = "manage")]
        Manage,
        #[serde(rename = "managePermissions")]
        ManagePermissions,
    }
    #[doc = "A permissions bit mask describing the security permissions of the current team for dashboards. When this permission is the value None, use GroupMemberPermission. Permissions are evaluated based on the presence of a value other than None, else the GroupMemberPermission will be saved."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TeamDashboardPermission {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "read")]
        Read,
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "edit")]
        Edit,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "managePermissions")]
        ManagePermissions,
    }
}
#[doc = "Dashboard group entry, wrapping around Dashboard (needed?)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardGroupEntry {
    #[serde(flatten)]
    pub dashboard: Dashboard,
}
impl DashboardGroupEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response from RestAPI when saving and editing DashboardGroupEntry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardGroupEntryResponse {
    #[serde(flatten)]
    pub dashboard_group_entry: DashboardGroupEntry,
}
impl DashboardGroupEntryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Dashboard>,
}
impl DashboardList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DashboardResponse {
    #[serde(flatten)]
    pub dashboard_group_entry: DashboardGroupEntry,
}
impl DashboardResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lightbox configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LightboxOptions {
    #[doc = "Height of desired lightbox, in pixels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[doc = "Set to true to allow lightbox resizing, false to disallow lightbox resizing, defaults to false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resizable: Option<bool>,
    #[doc = "Width of desired lightbox, in pixels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
impl LightboxOptions {
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
#[doc = "versioning for an artifact as described at:<http://semver>.org/, of the form major.minor.patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SemanticVersion {
    #[doc = "Major version when you make incompatible API changes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,
    #[doc = "Minor version when you add functionality in a backwards-compatible manner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,
    #[doc = "Patch version when you make backwards-compatible bug fixes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<i32>,
}
impl SemanticVersion {
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
#[doc = "Widget data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Widget {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Refers to the allowed sizes for the widget. This gets populated when user wants to configure the widget"]
    #[serde(
        rename = "allowedSizes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_sizes: Vec<WidgetSize>,
    #[doc = "Read-Only Property from Dashboard Service. Indicates if settings are blocked for the current user."]
    #[serde(
        rename = "areSettingsBlockedForUser",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub are_settings_blocked_for_user: Option<bool>,
    #[doc = "Refers to unique identifier of a feature artifact. Used for pinning+unpinning a specific artifact."]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(
        rename = "configurationContributionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_contribution_id: Option<String>,
    #[serde(
        rename = "configurationContributionRelativeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_contribution_relative_id: Option<String>,
    #[serde(
        rename = "contentUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_uri: Option<String>,
    #[doc = "The id of the underlying contribution defining the supplied Widget Configuration."]
    #[serde(
        rename = "contributionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contribution_id: Option<String>,
    #[doc = "Model of a Dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<Dashboard>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(
        rename = "isNameConfigurable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_name_configurable: Option<bool>,
    #[doc = "Lightbox configuration"]
    #[serde(
        rename = "lightboxOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lightbox_options: Option<LightboxOptions>,
    #[serde(
        rename = "loadingImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loading_image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<WidgetPosition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[doc = "versioning for an artifact as described at:<http://semver>.org/, of the form major.minor.patch."]
    #[serde(
        rename = "settingsVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub settings_version: Option<SemanticVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<WidgetSize>,
    #[serde(rename = "typeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Widget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Widget>,
}
impl WidgetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contribution based information describing Dashboard Widgets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetMetadata {
    #[doc = "Sizes supported by the Widget."]
    #[serde(
        rename = "allowedSizes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_sizes: Vec<WidgetSize>,
    #[doc = "Opt-in boolean that indicates if the widget requires the Analytics Service to function. Widgets requiring the analytics service are hidden from the catalog if the Analytics Service is not available."]
    #[serde(
        rename = "analyticsServiceRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_service_required: Option<bool>,
    #[doc = "Resource for an icon in the widget catalog."]
    #[serde(
        rename = "catalogIconUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub catalog_icon_url: Option<String>,
    #[doc = "Opt-in URL string pointing at widget information. Defaults to extension marketplace URL if omitted"]
    #[serde(
        rename = "catalogInfoUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub catalog_info_url: Option<String>,
    #[doc = "The id of the underlying contribution defining the supplied Widget custom configuration UI. Null if custom configuration UI is not available."]
    #[serde(
        rename = "configurationContributionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_contribution_id: Option<String>,
    #[doc = "The relative id of the underlying contribution defining the supplied Widget custom configuration UI. Null if custom configuration UI is not available."]
    #[serde(
        rename = "configurationContributionRelativeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_contribution_relative_id: Option<String>,
    #[doc = "Indicates if the widget requires configuration before being added to dashboard."]
    #[serde(
        rename = "configurationRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_required: Option<bool>,
    #[doc = "Uri for the widget content to be loaded from ."]
    #[serde(
        rename = "contentUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_uri: Option<String>,
    #[doc = "The id of the underlying contribution defining the supplied Widget."]
    #[serde(
        rename = "contributionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contribution_id: Option<String>,
    #[doc = "Optional default settings to be copied into widget settings."]
    #[serde(
        rename = "defaultSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_settings: Option<String>,
    #[doc = "Summary information describing the widget."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Widgets can be disabled by the app store.  We'll need to gracefully handle for: - persistence (Allow) - Requests (Tag as disabled, and provide context)"]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Opt-out boolean that indicates if the widget supports widget name/title configuration. Widgets ignoring the name should set it to false in the manifest."]
    #[serde(
        rename = "isNameConfigurable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_name_configurable: Option<bool>,
    #[doc = "Opt-out boolean indicating if the widget is hidden from the catalog. Commonly, this is used to allow developers to disable creation of a deprecated widget. A widget must have a functional default state, or have a configuration experience, in order to be visible from the catalog."]
    #[serde(
        rename = "isVisibleFromCatalog",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_visible_from_catalog: Option<bool>,
    #[doc = "Keywords associated with this widget, non-filterable and invisible"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub keywords: Vec<String>,
    #[doc = "Lightbox configuration"]
    #[serde(
        rename = "lightboxOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lightbox_options: Option<LightboxOptions>,
    #[doc = "Resource for a loading placeholder image on dashboard"]
    #[serde(
        rename = "loadingImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loading_image_url: Option<String>,
    #[doc = "User facing name of the widget type. Each widget must use a unique value here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Publisher Name of this kind of widget."]
    #[serde(
        rename = "publisherName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_name: Option<String>,
    #[doc = "Data contract required for the widget to function and to work in its container."]
    #[serde(
        rename = "supportedScopes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_scopes: Vec<serde_json::Value>,
    #[doc = "Tags associated with this widget, visible on each widget and filterable."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Contribution target IDs"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub targets: Vec<String>,
    #[doc = "Deprecated: locally unique developer-facing id of this kind of widget. ContributionId provides a globally unique identifier for widget types."]
    #[serde(rename = "typeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
}
impl WidgetMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetMetadataResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Contribution based information describing Dashboard Widgets."]
    #[serde(
        rename = "widgetMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub widget_metadata: Option<WidgetMetadata>,
}
impl WidgetMetadataResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetPosition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row: Option<i32>,
}
impl WidgetPosition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response from RestAPI when saving and editing Widget"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetResponse {
    #[serde(flatten)]
    pub widget: Widget,
}
impl WidgetResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetSize {
    #[doc = "The Width of the widget, expressed in dashboard grid columns."]
    #[serde(
        rename = "columnSpan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub column_span: Option<i32>,
    #[doc = "The height of the widget, expressed in dashboard grid rows."]
    #[serde(rename = "rowSpan", default, skip_serializing_if = "Option::is_none")]
    pub row_span: Option<i32>,
}
impl WidgetSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetTypesResponse {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(
        rename = "widgetTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub widget_types: Vec<WidgetMetadata>,
}
impl WidgetTypesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wrapper class to support HTTP header generation using CreateResponse, ClientHeaderParameter and ClientResponseType in WidgetV2Controller"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetsVersionedList {
    #[serde(
        rename = "eTag",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub e_tag: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub widgets: Vec<Widget>,
}
impl WidgetsVersionedList {
    pub fn new() -> Self {
        Self::default()
    }
}
