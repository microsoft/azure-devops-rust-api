// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Approval {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Identities which are not allowed to approve."]
    #[serde(
        rename = "blockedApprovers",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_approvers: Vec<IdentityRef>,
    #[doc = "Date on which approval got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Order in which approvers will be actionable."]
    #[serde(
        rename = "executionOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_order: Option<approval::ExecutionOrder>,
    #[doc = "Unique identifier of the approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Instructions for the approvers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[doc = "Date on which approval was last modified."]
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[doc = "Minimum number of approvers that should approve for the entire approval to be considered approved."]
    #[serde(
        rename = "minRequiredApprovers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_required_approvers: Option<i32>,
    #[doc = "Current user permissions for approval object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<approval::Permissions>,
    #[doc = "Overall status of the approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<approval::Status>,
    #[doc = "List of steps associated with the approval."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub steps: Vec<ApprovalStep>,
}
impl Approval {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval {
    use super::*;
    #[doc = "Order in which approvers will be actionable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionOrder {
        #[serde(rename = "anyOrder")]
        AnyOrder,
        #[serde(rename = "inSequence")]
        InSequence,
    }
    #[doc = "Current user permissions for approval object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "view")]
        View,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "reassign")]
        Reassign,
        #[serde(rename = "resourceAdmin")]
        ResourceAdmin,
        #[serde(rename = "queueBuild")]
        QueueBuild,
    }
    #[doc = "Overall status of the approval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "uninitiated")]
        Uninitiated,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalCheckConfiguration {
    #[serde(flatten)]
    pub check_configuration: CheckConfiguration,
    #[doc = "Config to create a new approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<ApprovalConfigSettings>,
}
impl ApprovalCheckConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalCompletedNotificationEvent {
    #[serde(flatten)]
    pub approval_notification_event_base: ApprovalNotificationEventBase,
}
impl ApprovalCompletedNotificationEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Config to create a new approval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalConfig {
    #[doc = "Ordered list of approvers."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub approvers: Vec<IdentityRef>,
    #[doc = "Identities which are not allowed to approve."]
    #[serde(
        rename = "blockedApprovers",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_approvers: Vec<IdentityRef>,
    #[doc = "Order in which approvers will be actionable."]
    #[serde(
        rename = "executionOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_order: Option<approval_config::ExecutionOrder>,
    #[doc = "Instructions for the approver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[doc = "Minimum number of approvers that should approve for the entire approval to be considered approved. Defaults to all."]
    #[serde(
        rename = "minRequiredApprovers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_required_approvers: Option<i32>,
}
impl ApprovalConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval_config {
    use super::*;
    #[doc = "Order in which approvers will be actionable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionOrder {
        #[serde(rename = "anyOrder")]
        AnyOrder,
        #[serde(rename = "inSequence")]
        InSequence,
    }
}
#[doc = "Config to create a new approval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalConfigSettings {
    #[serde(flatten)]
    pub approval_config: ApprovalConfig,
    #[doc = "Determines whether check requester can approve the check."]
    #[serde(
        rename = "requesterCannotBeApprover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requester_cannot_be_approver: Option<bool>,
}
impl ApprovalConfigSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Approval>,
}
impl ApprovalList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data for notification base class for approval events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalNotificationEventBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<Approval>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl ApprovalNotificationEventBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to create a new approval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalRequest {
    #[doc = "Unique identifier with which the approval is to be registered."]
    #[serde(
        rename = "approvalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_id: Option<String>,
    #[doc = "Config to create a new approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ApprovalConfig>,
}
impl ApprovalRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data for a single approval step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalStep {
    #[serde(
        rename = "actualApprover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_approver: Option<IdentityRef>,
    #[serde(
        rename = "assignedApprover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_approver: Option<IdentityRef>,
    #[doc = "Comment associated with this step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "History of the approval step"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub history: Vec<ApprovalStepHistory>,
    #[doc = "Timestamp at which this step was initiated."]
    #[serde(
        rename = "initiatedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub initiated_on: Option<time::OffsetDateTime>,
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<IdentityRef>,
    #[doc = "Timestamp at which this step was last modified."]
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[doc = "Order in which the approvers are allowed to approve."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Current user permissions for step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<approval_step::Permissions>,
    #[doc = "Current status of this step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<approval_step::Status>,
}
impl ApprovalStep {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval_step {
    use super::*;
    #[doc = "Current user permissions for step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "view")]
        View,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "reassign")]
        Reassign,
        #[serde(rename = "resourceAdmin")]
        ResourceAdmin,
        #[serde(rename = "queueBuild")]
        QueueBuild,
    }
    #[doc = "Current status of this step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "uninitiated")]
        Uninitiated,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "Data for a single approval step history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalStepHistory {
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<IdentityRef>,
    #[doc = "Comment associated with this step history."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Timestamp at which this step history was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
}
impl ApprovalStepHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data to update an approval object or its individual step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalUpdateParameters {
    #[doc = "ID of the approval to be updated."]
    #[serde(
        rename = "approvalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_id: Option<String>,
    #[serde(
        rename = "assignedApprover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_approver: Option<IdentityRef>,
    #[doc = "Gets or sets comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "reassignTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reassign_to: Option<IdentityRef>,
    #[doc = "Gets or sets status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<approval_update_parameters::Status>,
}
impl ApprovalUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval_update_parameters {
    use super::*;
    #[doc = "Gets or sets status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "uninitiated")]
        Uninitiated,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalsQueryParameters {
    #[doc = "Query approvals based on list of approval IDs."]
    #[serde(
        rename = "approvalIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub approval_ids: Vec<String>,
    #[doc = "Query approvals based on approval status"]
    #[serde(
        rename = "approverStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approver_status: Option<approvals_query_parameters::ApproverStatus>,
    #[doc = "Query approvals based on how many approvals to return."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "Query approvals based on list of approver IDs."]
    #[serde(
        rename = "userIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_ids: Vec<String>,
}
impl ApprovalsQueryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approvals_query_parameters {
    use super::*;
    #[doc = "Query approvals based on approval status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApproverStatus {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "uninitiated")]
        Uninitiated,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckConfiguration {
    #[serde(flatten)]
    pub check_configuration_ref: CheckConfigurationRef,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Time when check got configured."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Is check disabled."]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "An issue (error, warning) associated with a check configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<CheckIssue>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Time when configured check was modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Timeout in minutes for the check."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}
impl CheckConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckConfigurationData {
    #[serde(
        rename = "checkConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_configuration: Option<CheckConfiguration>,
    #[doc = "Definition Ref Id of the particular check."]
    #[serde(
        rename = "definitionRefId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_ref_id: Option<String>,
}
impl CheckConfigurationData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckConfigurationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CheckConfiguration>,
}
impl CheckConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckConfigurationRef {
    #[doc = "Check configuration id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<CheckType>,
    #[doc = "The URL from which one can fetch the configured check."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CheckConfigurationRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckData {
    #[doc = "List of check configuration data"]
    #[serde(
        rename = "checkConfigurationDataList",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub check_configuration_data_list: Vec<CheckConfigurationData>,
    #[doc = "List of check definitions"]
    #[serde(
        rename = "checkDefinitions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub check_definitions: Vec<CheckDefinitionData>,
    #[doc = "List of default check settings"]
    #[serde(
        rename = "defaultCheckSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_check_settings: Option<serde_json::Value>,
    #[doc = "List of time zones."]
    #[serde(
        rename = "timeZoneList",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub time_zone_list: Vec<TimeZone>,
}
impl CheckData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckDefinitionData {
    #[doc = "Flag to allow multiple configurations of a particular check on a resource."]
    #[serde(
        rename = "allowMultipleConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_multiple_configurations: Option<bool>,
    #[doc = "Details about the check"]
    #[serde(
        rename = "checkDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_definition: Option<serde_json::Value>,
    #[doc = "Check DefinitionRef Id"]
    #[serde(
        rename = "definitionRefId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_ref_id: Option<String>,
    #[doc = "Description about the check"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<CheckIcon>,
    #[doc = "Name of the check"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Check UI contribution Dependencies"]
    #[serde(
        rename = "uiContributionDependencies",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ui_contribution_dependencies: Vec<String>,
    #[doc = "Check UI contribution Type"]
    #[serde(
        rename = "uiContributionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ui_contribution_type: Option<String>,
}
impl CheckDefinitionData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckIcon {
    #[doc = "Asset Location of the icon"]
    #[serde(
        rename = "assetLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_location: Option<String>,
    #[doc = "Name of the icon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Url of the icon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CheckIcon {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An issue (error, warning) associated with a check configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckIssue {
    #[doc = "Short summary of the check - its name and resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A more detailed description of issue."]
    #[serde(
        rename = "detailedMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_message: Option<String>,
    #[doc = "A description of issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The type (error, warning) of the issue."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<check_issue::Type>,
}
impl CheckIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_issue {
    use super::*;
    #[doc = "The type (error, warning) of the issue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "warning")]
        Warning,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckIssueData {
    #[doc = "List of default check issue settings"]
    #[serde(
        rename = "defaultCheckIssueSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_check_issue_settings: Option<serde_json::Value>,
    #[doc = "List of resources with check issues"]
    #[serde(
        rename = "resourcesWithCheckIssuesList",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources_with_check_issues_list: Vec<ResourceCheckIssue>,
}
impl CheckIssueData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckRun {
    #[serde(flatten)]
    pub check_run_result: CheckRunResult,
    #[serde(
        rename = "checkConfigurationRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_configuration_ref: Option<CheckConfigurationRef>,
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl CheckRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckRunResult {
    #[serde(
        rename = "resultMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<check_run_result::Status>,
}
impl CheckRunResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_run_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckSuite {
    #[serde(flatten)]
    pub check_suite_ref: CheckSuiteRef,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "List of check runs associated with the given check suite request."]
    #[serde(
        rename = "checkRuns",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub check_runs: Vec<CheckRun>,
    #[doc = "Completed date of the given check suite request"]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Optional message for the given check suite request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Overall check runs status for the given suite request. This is check suite status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<check_suite::Status>,
}
impl CheckSuite {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_suite {
    use super::*;
    #[doc = "Overall check runs status for the given suite request. This is check suite status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckSuiteRef {
    #[doc = "Evaluation context for the check suite request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[doc = "Unique suite id generated by the pipeline orchestrator for the pipeline check runs request on the list of resources Pipeline orchestrator will used this identifier to map the check requests on a stage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl CheckSuiteRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckSuiteRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<Resource>,
}
impl CheckSuiteRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckType {
    #[doc = "Gets or sets check type id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the check type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CheckType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericCheckConfiguration {
    #[serde(flatten)]
    pub check_configuration: CheckConfiguration,
    #[doc = "Settings for the generic check configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
impl GenericCheckConfiguration {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(
        rename = "authorizedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorized_by: Option<IdentityRef>,
    #[serde(
        rename = "authorizedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub authorized_on: Option<time::OffsetDateTime>,
}
impl Permission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelinePermission {
    #[serde(flatten)]
    pub permission: Permission,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl PipelinePermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineProcessResources {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<PipelineResourceReference>,
}
impl PipelineProcessResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineResourceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(
        rename = "authorizedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorized_by: Option<String>,
    #[serde(
        rename = "authorizedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub authorized_on: Option<time::OffsetDateTime>,
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PipelineResourceReference {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A check configuration issue (error, warning) associated with a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceCheckIssue {
    #[doc = "An issue (error, warning) associated with a check configuration."]
    #[serde(
        rename = "checkIssue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_issue: Option<CheckIssue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}
impl ResourceCheckIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePipelinePermissions {
    #[serde(
        rename = "allPipelines",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_pipelines: Option<Permission>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pipelines: Vec<PipelinePermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}
impl ResourcePipelinePermissions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePipelinePermissionsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourcePipelinePermissions>,
}
impl ResourcePipelinePermissionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Config to facilitate task check"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskCheckConfig {
    #[serde(
        rename = "definitionRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_ref: Option<TaskCheckDefinitionReference>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[serde(
        rename = "linkedVariableGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub linked_variable_group: Option<String>,
    #[serde(
        rename = "retryInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_interval: Option<i32>,
}
impl TaskCheckConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskCheckConfiguration {
    #[serde(flatten)]
    pub check_configuration: CheckConfiguration,
    #[doc = "Config to facilitate task check"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<TaskCheckConfig>,
}
impl TaskCheckConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskCheckDefinitionReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl TaskCheckDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeZone {
    #[doc = "Display name of the time zone."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Id of the time zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl TimeZone {
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
