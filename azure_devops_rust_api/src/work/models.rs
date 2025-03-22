// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Activity {
    #[serde(
        rename = "capacityPerDay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub capacity_per_day: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Activity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BacklogColumn {
    #[doc = "Reference to a field in a work item"]
    #[serde(
        rename = "columnFieldReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub column_field_reference: Option<WorkItemFieldReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
impl BacklogColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BacklogConfiguration {
    #[serde(
        rename = "backlogFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backlog_fields: Option<BacklogFields>,
    #[doc = "Bugs behavior"]
    #[serde(
        rename = "bugsBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bugs_behavior: Option<backlog_configuration::BugsBehavior>,
    #[doc = "Hidden Backlog"]
    #[serde(
        rename = "hiddenBacklogs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hidden_backlogs: Vec<String>,
    #[doc = "Is BugsBehavior Configured in the process"]
    #[serde(
        rename = "isBugsBehaviorConfigured",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_bugs_behavior_configured: Option<bool>,
    #[doc = "Portfolio backlog descriptors"]
    #[serde(
        rename = "portfolioBacklogs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub portfolio_backlogs: Vec<BacklogLevelConfiguration>,
    #[serde(
        rename = "requirementBacklog",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requirement_backlog: Option<BacklogLevelConfiguration>,
    #[serde(
        rename = "taskBacklog",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_backlog: Option<BacklogLevelConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Mapped states for work item types"]
    #[serde(
        rename = "workItemTypeMappedStates",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_type_mapped_states: Vec<WorkItemTypeStateInfo>,
}
impl BacklogConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backlog_configuration {
    use super::*;
    #[doc = "Bugs behavior"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BugsBehavior {
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "asRequirements")]
        AsRequirements,
        #[serde(rename = "asTasks")]
        AsTasks,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BacklogFields {
    #[doc = "Field Type (e.g. Order, Activity) to Field Reference Name map"]
    #[serde(
        rename = "typeFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_fields: Option<serde_json::Value>,
}
impl BacklogFields {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contract representing a backlog level"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BacklogLevel {
    #[doc = "Reference name of the corresponding WIT category"]
    #[serde(
        rename = "categoryReferenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub category_reference_name: Option<String>,
    #[doc = "Plural name for the backlog level"]
    #[serde(
        rename = "pluralName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub plural_name: Option<String>,
    #[doc = "Collection of work item states that are included in the plan. The server will filter to only these work item types."]
    #[serde(
        rename = "workItemStates",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_states: Vec<String>,
    #[doc = "Collection of valid workitem type names for the given backlog level"]
    #[serde(
        rename = "workItemTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_types: Vec<String>,
}
impl BacklogLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BacklogLevelConfiguration {
    #[doc = "List of fields to include in Add Panel"]
    #[serde(
        rename = "addPanelFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub add_panel_fields: Vec<WorkItemFieldReference>,
    #[doc = "Color for the backlog level"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Default list of columns for the backlog"]
    #[serde(
        rename = "columnFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub column_fields: Vec<BacklogColumn>,
    #[doc = "Reference to a work item type."]
    #[serde(
        rename = "defaultWorkItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_work_item_type: Option<WorkItemTypeReference>,
    #[doc = "Backlog Id (for Legacy Backlog Level from process config it can be categoryref name)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates whether the backlog level is hidden"]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "Backlog Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Backlog Rank (Taskbacklog is 0)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[doc = "The type of this backlog level"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<backlog_level_configuration::Type>,
    #[doc = "Max number of work items to show in the given backlog"]
    #[serde(
        rename = "workItemCountLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_count_limit: Option<i32>,
    #[doc = "Work Item types participating in this backlog as known by the project/Process, can be overridden by team settings for bugs"]
    #[serde(
        rename = "workItemTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_types: Vec<WorkItemTypeReference>,
}
impl BacklogLevelConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backlog_level_configuration {
    use super::*;
    #[doc = "The type of this backlog level"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "portfolio")]
        Portfolio,
        #[serde(rename = "requirement")]
        Requirement,
        #[serde(rename = "task")]
        Task,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BacklogLevelConfigurationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BacklogLevelConfiguration>,
}
impl BacklogLevelConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents work items in a backlog level"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BacklogLevelWorkItems {
    #[doc = "A list of work items within a backlog level"]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<WorkItemLink>,
}
impl BacklogLevelWorkItems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Board {
    #[serde(flatten)]
    pub board_reference: BoardReference,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "allowedMappings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_mappings: Option<serde_json::Value>,
    #[serde(rename = "canEdit", default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<BoardColumn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<BoardFields>,
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rows: Vec<BoardRow>,
}
impl Board {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a board badge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardBadge {
    #[doc = "The ID of the board represented by this badge."]
    #[serde(rename = "boardId", default, skip_serializing_if = "Option::is_none")]
    pub board_id: Option<String>,
    #[doc = "A link to the SVG resource."]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}
impl BoardBadge {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardCardRuleSettings {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl BoardCardRuleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardCardSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<serde_json::Value>,
}
impl BoardCardSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardChart {
    #[serde(flatten)]
    pub board_chart_reference: BoardChartReference,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "The settings for the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
impl BoardChart {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardChartReference {
    #[doc = "Name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl BoardChartReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardChartReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BoardChartReference>,
}
impl BoardChartReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardColumn {
    #[serde(
        rename = "columnType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub column_type: Option<board_column::ColumnType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isSplit", default, skip_serializing_if = "Option::is_none")]
    pub is_split: Option<bool>,
    #[serde(rename = "itemLimit", default, skip_serializing_if = "Option::is_none")]
    pub item_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "stateMappings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_mappings: Option<serde_json::Value>,
}
impl BoardColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod board_column {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ColumnType {
        #[serde(rename = "incoming")]
        Incoming,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "outgoing")]
        Outgoing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardColumnList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BoardColumn>,
}
impl BoardColumnList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardFields {
    #[doc = "An abstracted reference to a field"]
    #[serde(
        rename = "columnField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub column_field: Option<FieldReference>,
    #[doc = "An abstracted reference to a field"]
    #[serde(rename = "doneField", default, skip_serializing_if = "Option::is_none")]
    pub done_field: Option<FieldReference>,
    #[doc = "An abstracted reference to a field"]
    #[serde(rename = "rowField", default, skip_serializing_if = "Option::is_none")]
    pub row_field: Option<FieldReference>,
}
impl BoardFields {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardReference {
    #[doc = "Id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl BoardReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BoardReference>,
}
impl BoardReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardRow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl BoardRow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardRowList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BoardRow>,
}
impl BoardRowList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardSuggestedValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl BoardSuggestedValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardSuggestedValueList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BoardSuggestedValue>,
}
impl BoardSuggestedValueList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardUserSettings {
    #[serde(
        rename = "autoRefreshState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_refresh_state: Option<bool>,
}
impl BoardUserSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityContractBase {
    #[serde(flatten)]
    pub team_settings_data_contract_base: TeamSettingsDataContractBase,
    #[doc = "Collection of capacities associated with the team member"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub activities: Vec<Activity>,
    #[doc = "The days off associated with the team member"]
    #[serde(
        rename = "daysOff",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_off: Vec<DateRange>,
}
impl CapacityContractBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Expected data from PATCH"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityPatch {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub activities: Vec<Activity>,
    #[serde(
        rename = "daysOff",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_off: Vec<DateRange>,
}
impl CapacityPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Card settings, such as fields and rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CardFieldSettings {
    #[doc = "A collection of field information of additional fields on cards. The index in the collection signifies the order of the field among the additional fields. Currently unused. Should be used with User Story 691539: Card setting: additional fields"]
    #[serde(
        rename = "additionalFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_fields: Vec<FieldInfo>,
    #[doc = "Display format for the assigned to field"]
    #[serde(
        rename = "assignedToDisplayFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to_display_format: Option<card_field_settings::AssignedToDisplayFormat>,
    #[doc = "A collection of field information of rendered core fields on cards."]
    #[serde(
        rename = "coreFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub core_fields: Vec<FieldInfo>,
    #[doc = "Flag indicating whether to show assigned to field on cards. When true, AssignedToDisplayFormat will determine how the field will be displayed"]
    #[serde(
        rename = "showAssignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_assigned_to: Option<bool>,
    #[doc = "Flag indicating whether to show child rollup on cards"]
    #[serde(
        rename = "showChildRollup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_child_rollup: Option<bool>,
    #[doc = "Flag indicating whether to show empty fields on cards"]
    #[serde(
        rename = "showEmptyFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_empty_fields: Option<bool>,
    #[doc = "Flag indicating whether to show ID on cards"]
    #[serde(rename = "showId", default, skip_serializing_if = "Option::is_none")]
    pub show_id: Option<bool>,
    #[doc = "Flag indicating whether to show parent field on cards"]
    #[serde(
        rename = "showParent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_parent: Option<bool>,
    #[doc = "Flag indicating whether to show state field on cards"]
    #[serde(rename = "showState", default, skip_serializing_if = "Option::is_none")]
    pub show_state: Option<bool>,
    #[doc = "Flag indicating whether to show tags on cards"]
    #[serde(rename = "showTags", default, skip_serializing_if = "Option::is_none")]
    pub show_tags: Option<bool>,
}
impl CardFieldSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod card_field_settings {
    use super::*;
    #[doc = "Display format for the assigned to field"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AssignedToDisplayFormat {
        #[serde(rename = "avatarOnly")]
        AvatarOnly,
        #[serde(rename = "fullName")]
        FullName,
        #[serde(rename = "avatarAndFullName")]
        AvatarAndFullName,
    }
}
#[doc = "Card settings, such as fields and rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CardSettings {
    #[doc = "Card settings, such as fields and rules"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<CardFieldSettings>,
}
impl CardSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about a given backlog category"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CategoryConfiguration {
    #[doc = "Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Category Reference Name"]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "Work item types for the backlog category"]
    #[serde(
        rename = "workItemTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_types: Vec<WorkItemTypeReference>,
}
impl CategoryConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatePlan {
    #[doc = "Description of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the plan to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Plan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Type of plan to create."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<create_plan::Type>,
}
impl CreatePlan {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod create_plan {
    use super::*;
    #[doc = "Type of plan to create."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "deliveryTimelineView")]
        DeliveryTimelineView,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DateRange {
    #[doc = "End of the date range."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub end: Option<time::OffsetDateTime>,
    #[doc = "Start of the date range."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start: Option<time::OffsetDateTime>,
}
impl DateRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data contract for Data of Delivery View"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliveryViewData {
    #[serde(flatten)]
    pub plan_view_data: PlanViewData,
    #[doc = "Work item child id to parent id map"]
    #[serde(
        rename = "childIdToParentIdMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub child_id_to_parent_id_map: Option<serde_json::Value>,
    #[serde(
        rename = "criteriaStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub criteria_status: Option<TimelineCriteriaStatus>,
    #[doc = "The end date of the delivery view data"]
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "Max number of teams that can be configured for a delivery plan"]
    #[serde(
        rename = "maxExpandedTeams",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_expanded_teams: Option<i32>,
    #[doc = "Mapping between parent id, title and all the child work item ids"]
    #[serde(
        rename = "parentItemMaps",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parent_item_maps: Vec<ParentChildWiMap>,
    #[doc = "The start date for the delivery view data"]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "All the team data"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub teams: Vec<TimelineTeamData>,
    #[doc = "List of all work item ids that have a dependency but not a violation"]
    #[serde(
        rename = "workItemDependencies",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_dependencies: Vec<i32>,
    #[doc = "List of all work item ids that have a violation"]
    #[serde(
        rename = "workItemViolations",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_violations: Vec<i32>,
}
impl DeliveryViewData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of properties, specific to the DeliveryTimelineView"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliveryViewPropertyCollection {
    #[doc = "Card settings, such as fields and rules"]
    #[serde(
        rename = "cardSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub card_settings: Option<CardSettings>,
    #[doc = "Field criteria"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub criteria: Vec<FilterClause>,
    #[doc = "Markers. Will be missing/null if there are no markers."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub markers: Vec<Marker>,
    #[doc = "Card style settings"]
    #[serde(
        rename = "styleSettings",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub style_settings: Vec<Rule>,
    #[doc = "tag style settings"]
    #[serde(
        rename = "tagStyleSettings",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tag_style_settings: Vec<Rule>,
    #[doc = "Team backlog mappings"]
    #[serde(
        rename = "teamBacklogMappings",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team_backlog_mappings: Vec<TeamBacklogMapping>,
}
impl DeliveryViewPropertyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object bag storing the set of permissions relevant to this plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldInfo {
    #[doc = "The additional field display name"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "The additional field type"]
    #[serde(rename = "fieldType", default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<field_info::FieldType>,
    #[doc = "Indicates if the field definition is for an identity field."]
    #[serde(
        rename = "isIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_identity: Option<bool>,
    #[doc = "The additional field reference name"]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
}
impl FieldInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod field_info {
    use super::*;
    #[doc = "The additional field type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FieldType {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "plainText")]
        PlainText,
        #[serde(rename = "integer")]
        Integer,
        #[serde(rename = "dateTime")]
        DateTime,
        #[serde(rename = "treePath")]
        TreePath,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(rename = "double")]
        Double,
    }
}
#[doc = "An abstracted reference to a field"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldReference {
    #[doc = "fieldRefName for the field"]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "Full http link to more information about the field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl FieldReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldSetting {}
impl FieldSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilterClause {
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(
        rename = "logicalOperator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logical_operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl FilterClause {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilterGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
impl FilterGroup {
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
pub struct ITaskboardColumnMapping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
impl ITaskboardColumnMapping {
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
#[doc = "Capacity and teams for all teams in an iteration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IterationCapacity {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub teams: Vec<TeamCapacityTotals>,
    #[serde(
        rename = "totalIterationCapacityPerDay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_iteration_capacity_per_day: Option<f64>,
    #[serde(
        rename = "totalIterationDaysOff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_iteration_days_off: Option<i32>,
}
impl IterationCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents work items in an iteration backlog"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IterationWorkItems {
    #[serde(flatten)]
    pub team_settings_data_contract_base: TeamSettingsDataContractBase,
    #[doc = "Work item relations"]
    #[serde(
        rename = "workItemRelations",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_relations: Vec<WorkItemLink>,
}
impl IterationWorkItems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Link description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Link {
    #[doc = "Collection of link attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "Relation type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[doc = "Link url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Link {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client serialization contract for Delivery Timeline Markers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Marker {
    #[doc = "Color associated with the marker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Where the marker should be displayed on the timeline."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "Label/title for the marker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl Marker {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Member {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(
        rename = "uniqueName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Member {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParentChildWiMap {
    #[serde(
        rename = "childWorkItemIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub child_work_item_ids: Vec<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(
        rename = "workItemTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type_name: Option<String>,
}
impl ParentChildWiMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParentChildWiMapList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ParentChildWiMap>,
}
impl ParentChildWiMapList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data contract for the plan definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Plan {
    #[serde(
        rename = "createdByIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_identity: Option<IdentityRef>,
    #[doc = "Date when the plan was created"]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Description of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Id of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Date when the plan was last accessed. Default is null."]
    #[serde(
        rename = "lastAccessed",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_accessed: Option<time::OffsetDateTime>,
    #[serde(
        rename = "modifiedByIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by_identity: Option<IdentityRef>,
    #[doc = "Date when the plan was last modified. Default to CreatedDate when the plan is first created."]
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "Name of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The PlanPropertyCollection instance associated with the plan. These are dependent on the type of the plan. For example, DeliveryTimelineView, it would be of type DeliveryViewPropertyCollection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Revision of the plan. Used to safeguard users from overwriting each other's changes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "Type of the plan"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<plan::Type>,
    #[doc = "The resource url to locate the plan via rest api"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Bit flag indicating set of permissions a user has to the plan."]
    #[serde(
        rename = "userPermissions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_permissions: Option<plan::UserPermissions>,
}
impl Plan {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod plan {
    use super::*;
    #[doc = "Type of the plan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "deliveryTimelineView")]
        DeliveryTimelineView,
    }
    #[doc = "Bit flag indicating set of permissions a user has to the plan."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserPermissions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "view")]
        View,
        #[serde(rename = "edit")]
        Edit,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "manage")]
        Manage,
        #[serde(rename = "allPermissions")]
        AllPermissions,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Plan>,
}
impl PlanList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata about a plan definition that is stored in favorites service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanMetadata {
    #[serde(
        rename = "createdByIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_identity: Option<IdentityRef>,
    #[doc = "Description of plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Last modified date of the plan"]
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "Bit flag indicating set of permissions a user has to the plan."]
    #[serde(
        rename = "userPermissions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_permissions: Option<plan_metadata::UserPermissions>,
}
impl PlanMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod plan_metadata {
    use super::*;
    #[doc = "Bit flag indicating set of permissions a user has to the plan."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserPermissions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "view")]
        View,
        #[serde(rename = "edit")]
        Edit,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "manage")]
        Manage,
        #[serde(rename = "allPermissions")]
        AllPermissions,
    }
}
#[doc = "Base class for plan view data contracts. Anything common goes here."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanViewData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl PlanViewData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a single pre-defined query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PredefinedQuery {
    #[doc = "Whether or not the query returned the complete set of data or if the data was truncated."]
    #[serde(rename = "hasMore", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[doc = "Id of the query"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Localized name of the query"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The results of the query.  This will be a set of WorkItem objects with only the 'id' set.  The client is responsible for paging in the data as needed."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<WorkItem>,
    #[doc = "REST API Url to use to retrieve results for this query"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Url to use to display a page in the browser with the results of this query"]
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}
impl PredefinedQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Process Configurations for the project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessConfiguration {
    #[doc = "Details about a given backlog category"]
    #[serde(
        rename = "bugWorkItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bug_work_items: Option<CategoryConfiguration>,
    #[doc = "Details about portfolio backlogs"]
    #[serde(
        rename = "portfolioBacklogs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub portfolio_backlogs: Vec<CategoryConfiguration>,
    #[doc = "Details about a given backlog category"]
    #[serde(
        rename = "requirementBacklog",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requirement_backlog: Option<CategoryConfiguration>,
    #[doc = "Details about a given backlog category"]
    #[serde(
        rename = "taskBacklog",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_backlog: Option<CategoryConfiguration>,
    #[doc = "Type fields for the process configuration"]
    #[serde(
        rename = "typeFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_fields: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessConfiguration {
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
#[doc = "Represents a reorder request for one or more work items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReorderOperation {
    #[doc = "IDs of the work items to be reordered.  Must be valid WorkItem Ids."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<i32>,
    #[doc = "IterationPath for reorder operation. This is only used when we reorder from the Iteration Backlog"]
    #[serde(
        rename = "iterationPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_path: Option<String>,
    #[doc = "ID of the work item that should be after the reordered items. Can use 0 to specify the end of the list."]
    #[serde(rename = "nextId", default, skip_serializing_if = "Option::is_none")]
    pub next_id: Option<i32>,
    #[doc = "Parent ID for all of the work items involved in this operation. Can use 0 to indicate the items don't have a parent."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    #[doc = "ID of the work item that should be before the reordered items. Can use 0 to specify the beginning of the list."]
    #[serde(
        rename = "previousId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_id: Option<i32>,
}
impl ReorderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reorder result for a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReorderResult {
    #[doc = "The ID of the work item that was reordered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The updated order value of the work item that was reordered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
}
impl ReorderResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReorderResultList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReorderResult>,
}
impl ReorderResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rule {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub clauses: Vec<FilterClause>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Attribute>,
}
impl Rule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the taskbord column"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskboardColumn {
    #[doc = "Column ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Work item type states mapped to this column to support auto state update when column is updated."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<ITaskboardColumnMapping>,
    #[doc = "Column name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Column position relative to other columns in the same board"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}
impl TaskboardColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the state to column mapping per work item type This allows auto state update when the column changes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskboardColumnMapping {
    #[doc = "State of the work item type mapped to the column"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Work Item Type name who's state is mapped to the column"]
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
impl TaskboardColumnMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskboardColumns {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<TaskboardColumn>,
    #[doc = "Are the columns cutomized for this team"]
    #[serde(
        rename = "isCustomized",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_customized: Option<bool>,
    #[doc = "Specifies if the referenced WIT and State is valid"]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "Details of validation failure if the state to column mapping is invalid"]
    #[serde(
        rename = "validationMesssage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub validation_messsage: Option<String>,
}
impl TaskboardColumns {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Column value of a work item in the taskboard"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskboardWorkItemColumn {
    #[doc = "Work item column value in the taskboard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[doc = "Work item column id in the taskboard"]
    #[serde(rename = "columnId", default, skip_serializing_if = "Option::is_none")]
    pub column_id: Option<String>,
    #[doc = "Work Item state value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Work item id"]
    #[serde(
        rename = "workItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_id: Option<i32>,
}
impl TaskboardWorkItemColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskboardWorkItemColumnList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TaskboardWorkItemColumn>,
}
impl TaskboardWorkItemColumnList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mapping of teams to the corresponding work item category"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamBacklogMapping {
    #[serde(
        rename = "categoryReferenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub category_reference_name: Option<String>,
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}
impl TeamBacklogMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents team member capacity with totals aggregated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamCapacity {
    #[serde(
        rename = "teamMembers",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team_members: Vec<TeamMemberCapacityIdentityRef>,
    #[serde(
        rename = "totalCapacityPerDay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_capacity_per_day: Option<f64>,
    #[serde(
        rename = "totalDaysOff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_days_off: Option<i32>,
}
impl TeamCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Team information with total capacity and days off"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamCapacityTotals {
    #[serde(
        rename = "teamCapacityPerDay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_capacity_per_day: Option<f64>,
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(
        rename = "teamTotalDaysOff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_total_days_off: Option<i32>,
}
impl TeamCapacityTotals {
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
#[doc = "Represents a single TeamFieldValue"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamFieldValue {
    #[serde(
        rename = "includeChildren",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_children: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TeamFieldValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Essentially a collection of team field values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamFieldValues {
    #[serde(flatten)]
    pub team_settings_data_contract_base: TeamSettingsDataContractBase,
    #[doc = "The default team field value"]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
    #[doc = "An abstracted reference to a field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldReference>,
    #[doc = "Collection of all valid team field values"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<TeamFieldValue>,
}
impl TeamFieldValues {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Expected data from PATCH"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamFieldValuesPatch {
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<TeamFieldValue>,
}
impl TeamFieldValuesPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamIterationAttributes {
    #[doc = "Finish date of the iteration. Date-only, correct unadjusted at midnight in UTC."]
    #[serde(
        rename = "finishDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_date: Option<time::OffsetDateTime>,
    #[doc = "Start date of the iteration. Date-only, correct unadjusted at midnight in UTC."]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Time frame of the iteration, such as past, current or future."]
    #[serde(rename = "timeFrame", default, skip_serializing_if = "Option::is_none")]
    pub time_frame: Option<team_iteration_attributes::TimeFrame>,
}
impl TeamIterationAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod team_iteration_attributes {
    use super::*;
    #[doc = "Time frame of the iteration, such as past, current or future."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeFrame {
        #[serde(rename = "past")]
        Past,
        #[serde(rename = "current")]
        Current,
        #[serde(rename = "future")]
        Future,
    }
}
#[doc = "Represents capacity for a specific team member"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMemberCapacity {
    #[serde(flatten)]
    pub capacity_contract_base: CapacityContractBase,
    #[serde(
        rename = "teamMember",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_member: Option<Member>,
}
impl TeamMemberCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents capacity for a specific team member"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMemberCapacityIdentityRef {
    #[serde(flatten)]
    pub capacity_contract_base: CapacityContractBase,
    #[serde(
        rename = "teamMember",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_member: Option<IdentityRef>,
}
impl TeamMemberCapacityIdentityRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMemberCapacityIdentityRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TeamMemberCapacityIdentityRef>,
}
impl TeamMemberCapacityIdentityRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data contract for TeamSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSetting {
    #[serde(flatten)]
    pub team_settings_data_contract_base: TeamSettingsDataContractBase,
    #[doc = "Represents a shallow ref for a single iteration."]
    #[serde(
        rename = "backlogIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backlog_iteration: Option<TeamSettingsIteration>,
    #[doc = "Information about categories that are visible on the backlog."]
    #[serde(
        rename = "backlogVisibilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backlog_visibilities: Option<serde_json::Value>,
    #[doc = "BugsBehavior (Off, AsTasks, AsRequirements, ...)"]
    #[serde(
        rename = "bugsBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bugs_behavior: Option<team_setting::BugsBehavior>,
    #[doc = "Represents a shallow ref for a single iteration."]
    #[serde(
        rename = "defaultIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_iteration: Option<TeamSettingsIteration>,
    #[doc = "Default Iteration macro (if any)"]
    #[serde(
        rename = "defaultIterationMacro",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_iteration_macro: Option<String>,
    #[doc = "Days that the team is working"]
    #[serde(
        rename = "workingDays",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub working_days: Vec<serde_json::Value>,
}
impl TeamSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod team_setting {
    use super::*;
    #[doc = "BugsBehavior (Off, AsTasks, AsRequirements, ...)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BugsBehavior {
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "asRequirements")]
        AsRequirements,
        #[serde(rename = "asTasks")]
        AsTasks,
    }
}
#[doc = "Base class for TeamSettings data contracts. Anything common goes here."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSettingsDataContractBase {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TeamSettingsDataContractBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSettingsDaysOff {
    #[serde(flatten)]
    pub team_settings_data_contract_base: TeamSettingsDataContractBase,
    #[serde(
        rename = "daysOff",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_off: Vec<DateRange>,
}
impl TeamSettingsDaysOff {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSettingsDaysOffPatch {
    #[serde(
        rename = "daysOff",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_off: Vec<DateRange>,
}
impl TeamSettingsDaysOffPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a shallow ref for a single iteration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSettingsIteration {
    #[serde(flatten)]
    pub team_settings_data_contract_base: TeamSettingsDataContractBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TeamIterationAttributes>,
    #[doc = "Id of the iteration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the iteration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Relative path of the iteration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl TeamSettingsIteration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSettingsIterationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TeamSettingsIteration>,
}
impl TeamSettingsIterationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data contract for what we expect to receive when PATCH"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamSettingsPatch {
    #[serde(
        rename = "backlogIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backlog_iteration: Option<String>,
    #[serde(
        rename = "backlogVisibilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backlog_visibilities: Option<serde_json::Value>,
    #[serde(
        rename = "bugsBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bugs_behavior: Option<team_settings_patch::BugsBehavior>,
    #[serde(
        rename = "defaultIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_iteration: Option<String>,
    #[serde(
        rename = "defaultIterationMacro",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_iteration_macro: Option<String>,
    #[serde(
        rename = "workingDays",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub working_days: Vec<serde_json::Value>,
}
impl TeamSettingsPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod team_settings_patch {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BugsBehavior {
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "asRequirements")]
        AsRequirements,
        #[serde(rename = "asTasks")]
        AsTasks,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineCriteriaStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<timeline_criteria_status::Type>,
}
impl TimelineCriteriaStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod timeline_criteria_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "invalidFilterClause")]
        InvalidFilterClause,
        #[serde(rename = "unknown")]
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineIterationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<timeline_iteration_status::Type>,
}
impl TimelineIterationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod timeline_iteration_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "isOverlapping")]
        IsOverlapping,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineTeamData {
    #[doc = "Contract representing a backlog level"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backlog: Option<BacklogLevel>,
    #[doc = "The field reference names of the work item data"]
    #[serde(
        rename = "fieldReferenceNames",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_reference_names: Vec<String>,
    #[doc = "The id of the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Was iteration and work item data retrieved for this team. <remarks> Teams with IsExpanded false have not had their iteration, work item, and field related data queried and will never contain this data. If true then these items are queried and, if there are items in the queried range, there will be data. </remarks>"]
    #[serde(
        rename = "isExpanded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_expanded: Option<bool>,
    #[doc = "The iteration data, including the work items, in the queried date range."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub iterations: Vec<TimelineTeamIteration>,
    #[doc = "The name of the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The order by field name of this team"]
    #[serde(
        rename = "orderByField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_by_field: Option<String>,
    #[doc = "The field reference names of the partially paged work items, such as ID, WorkItemType"]
    #[serde(
        rename = "partiallyPagedFieldReferenceNames",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub partially_paged_field_reference_names: Vec<String>,
    #[serde(
        rename = "partiallyPagedWorkItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub partially_paged_work_items: Vec<Vec<serde_json::Value>>,
    #[doc = "The project id the team belongs team"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Work item types for which we will collect roll up data on the client side"]
    #[serde(
        rename = "rollupWorkItemTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rollup_work_item_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TimelineTeamStatus>,
    #[doc = "The team field default value"]
    #[serde(
        rename = "teamFieldDefaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_field_default_value: Option<String>,
    #[doc = "The team field name of this team"]
    #[serde(
        rename = "teamFieldName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_field_name: Option<String>,
    #[doc = "The team field values"]
    #[serde(
        rename = "teamFieldValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team_field_values: Vec<TeamFieldValue>,
    #[doc = "Work items associated with the team that are not under any of the team's iterations"]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<Vec<serde_json::Value>>,
    #[doc = "Colors for the work item types."]
    #[serde(
        rename = "workItemTypeColors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_type_colors: Vec<WorkItemColor>,
}
impl TimelineTeamData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineTeamIteration {
    #[doc = "The iteration CSS Node Id"]
    #[serde(rename = "cssNodeId", default, skip_serializing_if = "Option::is_none")]
    pub css_node_id: Option<String>,
    #[doc = "The end date of the iteration"]
    #[serde(
        rename = "finishDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_date: Option<time::OffsetDateTime>,
    #[doc = "The iteration name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "All the partially paged workitems in this iteration."]
    #[serde(
        rename = "partiallyPagedWorkItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub partially_paged_work_items: Vec<Vec<serde_json::Value>>,
    #[doc = "The iteration path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The start date of the iteration"]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TimelineIterationStatus>,
    #[doc = "The work items that have been paged in this iteration"]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<Vec<serde_json::Value>>,
}
impl TimelineTeamIteration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineTeamStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<timeline_team_status::Type>,
}
impl TimelineTeamStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod timeline_team_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "doesntExistOrAccessDenied")]
        DoesntExistOrAccessDenied,
        #[serde(rename = "maxTeamsExceeded")]
        MaxTeamsExceeded,
        #[serde(rename = "maxTeamFieldsExceeded")]
        MaxTeamFieldsExceeded,
        #[serde(rename = "backlogInError")]
        BacklogInError,
        #[serde(rename = "missingTeamFieldValue")]
        MissingTeamFieldValue,
        #[serde(rename = "noIterationsExist")]
        NoIterationsExist,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdatePlan {
    #[doc = "Description of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the plan to create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Plan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Revision of the plan that was updated - the value used here should match the one the server gave the client in the Plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "Type of the plan"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<update_plan::Type>,
}
impl UpdatePlan {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_plan {
    use super::*;
    #[doc = "Type of the plan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "deliveryTimelineView")]
        DeliveryTimelineView,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateTaskboardColumn {
    #[doc = "Column ID, keep it null for new column"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Work item type states mapped to this column to support auto state update when column is updated."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<TaskboardColumnMapping>,
    #[doc = "Column name is required"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Column position relative to other columns in the same board"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}
impl UpdateTaskboardColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateTaskboardWorkItemColumn {
    #[serde(rename = "newColumn", default, skip_serializing_if = "Option::is_none")]
    pub new_column: Option<String>,
}
impl UpdateTaskboardWorkItemColumn {
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
#[doc = "Describes a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItem {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "Represents the reference to a specific version of a comment on a Work Item."]
    #[serde(
        rename = "commentVersionRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_version_ref: Option<WorkItemCommentVersionRef>,
    #[doc = "Map of field and values for the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    #[doc = "The work item ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Relations of the work item."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relations: Vec<WorkItemRelation>,
    #[doc = "Revision number of the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev: Option<i32>,
}
impl WorkItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Work item color and icon."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemColor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(
        rename = "primaryColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_color: Option<String>,
    #[serde(
        rename = "workItemTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type_name: Option<String>,
}
impl WorkItemColor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the reference to a specific version of a comment on a Work Item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemCommentVersionRef {
    #[serde(flatten)]
    pub work_item_tracking_resource_reference: WorkItemTrackingResourceReference,
    #[doc = "The id assigned to the comment."]
    #[serde(rename = "commentId", default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i32>,
    #[doc = "\\[Internal\\] The work item revision where this comment was originally added."]
    #[serde(
        rename = "createdInRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_in_revision: Option<i32>,
    #[doc = "\\[Internal\\] Specifies whether comment was deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "\\[Internal\\] The text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The version number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl WorkItemCommentVersionRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a field in a work item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemFieldReference {
    #[doc = "The friendly name of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reference name of the field."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "The REST URL of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemFieldReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A link between two work items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemLink {
    #[doc = "The type of link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[doc = "Contains reference to a work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<WorkItemReference>,
    #[doc = "Contains reference to a work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<WorkItemReference>,
}
impl WorkItemLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains reference to a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemReference {
    #[doc = "Work item ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "REST API URL of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemRelation {
    #[serde(flatten)]
    pub link: Link,
}
impl WorkItemRelation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for WIT REST resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTrackingResource {
    #[serde(flatten)]
    pub work_item_tracking_resource_reference: WorkItemTrackingResourceReference,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
impl WorkItemTrackingResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for work item tracking resource references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTrackingResourceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemTrackingResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeReference {
    #[serde(flatten)]
    pub work_item_tracking_resource_reference: WorkItemTrackingResourceReference,
    #[doc = "Name of the work item type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WorkItemTypeReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeStateInfo {
    #[doc = "State name to state category map"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub states: Option<serde_json::Value>,
    #[doc = "Work Item type name"]
    #[serde(
        rename = "workItemTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type_name: Option<String>,
}
impl WorkItemTypeStateInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attribute {}
impl Attribute {
    pub fn new() -> Self {
        Self::default()
    }
}
