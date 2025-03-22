// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadGraphMember {
    #[serde(flatten)]
    pub graph_member: GraphMember,
    #[doc = "The short, generally unique name for the user in the backing directory. For AAD users, this corresponds to the mail nickname, which is often but not necessarily similar to the part of the user's mail address before the @ sign. For GitHub users, this corresponds to the GitHub user handle."]
    #[serde(
        rename = "directoryAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub directory_alias: Option<String>,
    #[doc = "When true, the group has been deleted in the identity provider"]
    #[serde(
        rename = "isDeletedInOrigin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deleted_in_origin: Option<bool>,
    #[doc = "The meta type of the user in the origin, such as \"member\", \"guest\", etc. See UserMetaType for the set of possible values."]
    #[serde(rename = "metaType", default, skip_serializing_if = "Option::is_none")]
    pub meta_type: Option<String>,
}
impl AadGraphMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "License assigned to a user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessLevel {
    #[doc = "Type of Account License (e.g. Express, Stakeholder etc.). To use the AccountLicenseType, LicensingSource should be defined as 'account' in the request body."]
    #[serde(
        rename = "accountLicenseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_license_type: Option<access_level::AccountLicenseType>,
    #[doc = "Assignment Source of the License (e.g. Group, Unknown etc."]
    #[serde(
        rename = "assignmentSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assignment_source: Option<access_level::AssignmentSource>,
    #[doc = "Display name of the License"]
    #[serde(
        rename = "licenseDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub license_display_name: Option<String>,
    #[doc = "Licensing Source (e.g. Account. MSDN etc.)"]
    #[serde(
        rename = "licensingSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub licensing_source: Option<access_level::LicensingSource>,
    #[doc = "Type of MSDN License (e.g. Visual Studio Professional, Visual Studio Enterprise etc.). To use the MsdnLicenseType, LicensingSource should be defined as 'msdn' in the request body."]
    #[serde(
        rename = "msdnLicenseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub msdn_license_type: Option<access_level::MsdnLicenseType>,
    #[doc = "User status in the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<access_level::Status>,
    #[doc = "Status message."]
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
}
impl AccessLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_level {
    use super::*;
    #[doc = "Type of Account License (e.g. Express, Stakeholder etc.). To use the AccountLicenseType, LicensingSource should be defined as 'account' in the request body."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountLicenseType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "earlyAdopter")]
        EarlyAdopter,
        #[serde(rename = "express")]
        Express,
        #[serde(rename = "professional")]
        Professional,
        #[serde(rename = "advanced")]
        Advanced,
        #[serde(rename = "stakeholder")]
        Stakeholder,
    }
    #[doc = "Assignment Source of the License (e.g. Group, Unknown etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AssignmentSource {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "groupRule")]
        GroupRule,
    }
    #[doc = "Licensing Source (e.g. Account. MSDN etc.)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LicensingSource {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "account")]
        Account,
        #[serde(rename = "msdn")]
        Msdn,
        #[serde(rename = "profile")]
        Profile,
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "trial")]
        Trial,
    }
    #[doc = "Type of MSDN License (e.g. Visual Studio Professional, Visual Studio Enterprise etc.). To use the MsdnLicenseType, LicensingSource should be defined as 'msdn' in the request body."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MsdnLicenseType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "eligible")]
        Eligible,
        #[serde(rename = "professional")]
        Professional,
        #[serde(rename = "platforms")]
        Platforms,
        #[serde(rename = "testProfessional")]
        TestProfessional,
        #[serde(rename = "premium")]
        Premium,
        #[serde(rename = "ultimate")]
        Ultimate,
        #[serde(rename = "enterprise")]
        Enterprise,
    }
    #[doc = "User status in the account"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "pendingDisabled")]
        PendingDisabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseOperationResult {
    #[doc = "List of error codes paired with their corresponding error messages"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<serde_json::Value>,
    #[doc = "Success status of the operation"]
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
}
impl BaseOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntitlementBase {
    #[doc = "License assigned to a user"]
    #[serde(
        rename = "accessLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_level: Option<AccessLevel>,
    #[doc = "\\[Readonly\\] Date the member was added to the collection."]
    #[serde(
        rename = "dateCreated",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date_created: Option<time::OffsetDateTime>,
    #[doc = "\\[Readonly\\] GroupEntitlements that this member belongs to."]
    #[serde(
        rename = "groupAssignments",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_assignments: Vec<GroupEntitlement>,
    #[doc = "The unique identifier which matches the Id of the Identity associated with the GraphMember."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "\\[Readonly\\] Date the member last accessed the collection."]
    #[serde(
        rename = "lastAccessedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_accessed_date: Option<time::OffsetDateTime>,
    #[doc = "Relation between a project and the member's effective permissions in that project."]
    #[serde(
        rename = "projectEntitlements",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub project_entitlements: Vec<ProjectEntitlement>,
}
impl EntitlementBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntitlementOperationResultBase {
    #[doc = "List of error codes paired with their corresponding error messages."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<serde_json::Value>,
    #[doc = "Success status of the operation."]
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[doc = "Resulting entitlement property.  For specific implementations, see also: <seealso cref=\"T:Microsoft.VisualStudio.Services.MemberEntitlementManagement.WebApi.ServicePrincipalEntitlementOperationResult\" /><seealso cref=\"T:Microsoft.VisualStudio.Services.MemberEntitlementManagement.WebApi.UserEntitlementOperationResult\" />"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl EntitlementOperationResultBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An extension assigned to a user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Extension {
    #[doc = "Assignment source for this extension. I.e. explicitly assigned or from a group rule."]
    #[serde(
        rename = "assignmentSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assignment_source: Option<extension::AssignmentSource>,
    #[doc = "Gallery Id of the Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Friendly name of this extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Source of this extension assignment. Ex: msdn, account, none, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<extension::Source>,
}
impl Extension {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension {
    use super::*;
    #[doc = "Assignment source for this extension. I.e. explicitly assigned or from a group rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AssignmentSource {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "groupRule")]
        GroupRule,
    }
    #[doc = "Source of this extension assignment. Ex: msdn, account, none, etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "account")]
        Account,
        #[serde(rename = "msdn")]
        Msdn,
        #[serde(rename = "profile")]
        Profile,
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "trial")]
        Trial,
    }
}
#[doc = "Summary of Extensions in the organization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionSummaryData {
    #[serde(flatten)]
    pub summary_data: SummaryData,
    #[doc = "Count of Extension Licenses assigned to users through msdn."]
    #[serde(
        rename = "assignedThroughSubscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_through_subscription: Option<i32>,
    #[doc = "Gallery Id of the Extension"]
    #[serde(
        rename = "extensionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_id: Option<String>,
    #[doc = "Friendly name of this extension"]
    #[serde(
        rename = "extensionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_name: Option<String>,
    #[doc = "Whether its a Trial Version."]
    #[serde(
        rename = "isTrialVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_trial_version: Option<bool>,
    #[doc = "Minimum License Required for the Extension."]
    #[serde(
        rename = "minimumLicenseRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_license_required: Option<extension_summary_data::MinimumLicenseRequired>,
    #[doc = "Days remaining for the Trial to expire."]
    #[serde(
        rename = "remainingTrialDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_trial_days: Option<i32>,
    #[doc = "Date on which the Trial expires."]
    #[serde(
        rename = "trialExpiryDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub trial_expiry_date: Option<time::OffsetDateTime>,
}
impl ExtensionSummaryData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extension_summary_data {
    use super::*;
    #[doc = "Minimum License Required for the Extension."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MinimumLicenseRequired {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "express")]
        Express,
        #[serde(rename = "advanced")]
        Advanced,
        #[serde(rename = "advancedPlus")]
        AdvancedPlus,
        #[serde(rename = "stakeholder")]
        Stakeholder,
    }
}
#[doc = "Graph group entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGroup {
    #[serde(flatten)]
    pub graph_member: GraphMember,
    #[doc = "A short phrase to help human readers disambiguate groups with similar names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl GraphGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphMember {
    #[serde(flatten)]
    pub graph_subject: GraphSubject,
    #[doc = "This represents the name of the container of origin for a graph member. (For MSA this is \"Windows Live ID\", for AD the name of the domain, for AAD the tenantID of the directory, for VSTS groups the ScopeId, etc)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The email address of record for a given graph member. This may be different than the principal name."]
    #[serde(
        rename = "mailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mail_address: Option<String>,
    #[doc = "This is the PrincipalName of this graph member from the source provider. The source provider may change this field over time and it is not guaranteed to be immutable for the life of the graph member by VSTS."]
    #[serde(
        rename = "principalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_name: Option<String>,
}
impl GraphMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphServicePrincipal {
    #[serde(flatten)]
    pub aad_graph_member: AadGraphMember,
    #[serde(
        rename = "applicationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_id: Option<String>,
}
impl GraphServicePrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Top-level graph entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubject {
    #[serde(flatten)]
    pub graph_subject_base: GraphSubjectBase,
    #[doc = "[Internal Use Only] The legacy descriptor is here in case you need to access old version IMS using identity descriptor."]
    #[serde(
        rename = "legacyDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_descriptor: Option<String>,
    #[doc = "The type of source provider for the origin identifier (ex:AD, AAD, MSA)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The unique identifier from the system of origin. Typically a sid, object id or Guid. Linking and unlinking operations can cause this value to change for a user because the user is not backed by a different provider and has a different unique id in the new provider."]
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[doc = "This field identifies the type of the graph subject (ex: Group, Scope, User)."]
    #[serde(
        rename = "subjectKind",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_kind: Option<String>,
}
impl GraphSubject {
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
pub struct GraphUser {
    #[serde(flatten)]
    pub aad_graph_member: AadGraphMember,
}
impl GraphUser {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Project Group (e.g. Contributor, Reader etc.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Group {
    #[doc = "Display Name of the Group"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Group Type"]
    #[serde(rename = "groupType", default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<group::GroupType>,
}
impl Group {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod group {
    use super::*;
    #[doc = "Group Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GroupType {
        #[serde(rename = "projectStakeholder")]
        ProjectStakeholder,
        #[serde(rename = "projectReader")]
        ProjectReader,
        #[serde(rename = "projectContributor")]
        ProjectContributor,
        #[serde(rename = "projectAdministrator")]
        ProjectAdministrator,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[doc = "A group entity with additional properties including its license, extensions, and project membership"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupEntitlement {
    #[doc = "Graph group entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GraphGroup>,
    #[doc = "The unique identifier which matches the Id of the GraphMember."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "\\[Readonly\\] The last time the group licensing rule was executed (regardless of whether any changes were made)."]
    #[serde(
        rename = "lastExecuted",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_executed: Option<time::OffsetDateTime>,
    #[doc = "License assigned to a user"]
    #[serde(
        rename = "licenseRule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub license_rule: Option<AccessLevel>,
    #[doc = "Group members. Only used when creating a new group."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub members: Vec<UserEntitlement>,
    #[doc = "Relation between a project and the member's effective permissions in that project."]
    #[serde(
        rename = "projectEntitlements",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub project_entitlements: Vec<ProjectEntitlement>,
    #[doc = "The status of the group rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<group_entitlement::Status>,
}
impl GroupEntitlement {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod group_entitlement {
    use super::*;
    #[doc = "The status of the group rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "applyPending")]
        ApplyPending,
        #[serde(rename = "applied")]
        Applied,
        #[serde(rename = "incompatible")]
        Incompatible,
        #[serde(rename = "unableToApply")]
        UnableToApply,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupEntitlementList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GroupEntitlement>,
}
impl GroupEntitlementList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupEntitlementOperationReference {
    #[serde(flatten)]
    pub operation_reference: OperationReference,
    #[doc = "Operation completed with success or failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[doc = "True if all operations were successful."]
    #[serde(
        rename = "haveResultsSucceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub have_results_succeeded: Option<bool>,
    #[doc = "List of results for each operation."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<GroupOperationResult>,
}
impl GroupEntitlementOperationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupOperationResult {
    #[serde(flatten)]
    pub base_operation_result: BaseOperationResult,
    #[doc = "Identifier of the Group being acted upon"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "A group entity with additional properties including its license, extensions, and project membership"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<GroupEntitlement>,
}
impl GroupOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Group option to add a user to"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupOption {
    #[doc = "License assigned to a user"]
    #[serde(
        rename = "accessLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_level: Option<AccessLevel>,
    #[doc = "Project Group (e.g. Contributor, Reader etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}
impl GroupOption {
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
#[doc = "Summary of Licenses in the organization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseSummaryData {
    #[serde(flatten)]
    pub summary_data: SummaryData,
    #[doc = "Type of Account License. To use the AccountLicenseType, LicensingSource should be defined as 'account' in the request body."]
    #[serde(
        rename = "accountLicenseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_license_type: Option<license_summary_data::AccountLicenseType>,
    #[doc = "Count of Disabled Licenses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<i32>,
    #[doc = "Designates if this license quantity can be changed through purchase"]
    #[serde(
        rename = "isPurchasable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_purchasable: Option<bool>,
    #[doc = "Name of the License."]
    #[serde(
        rename = "licenseName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub license_name: Option<String>,
    #[doc = "Type of MSDN License. To use the MsdnLicenseType, LicensingSource should be defined as 'msdn' in the request body."]
    #[serde(
        rename = "msdnLicenseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub msdn_license_type: Option<license_summary_data::MsdnLicenseType>,
    #[doc = "Specifies the date when billing will charge for paid licenses"]
    #[serde(
        rename = "nextBillingDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub next_billing_date: Option<time::OffsetDateTime>,
    #[doc = "Source of the License."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<license_summary_data::Source>,
    #[doc = "Total license count after next billing cycle"]
    #[serde(
        rename = "totalAfterNextBillingDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_after_next_billing_date: Option<i32>,
}
impl LicenseSummaryData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod license_summary_data {
    use super::*;
    #[doc = "Type of Account License. To use the AccountLicenseType, LicensingSource should be defined as 'account' in the request body."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountLicenseType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "earlyAdopter")]
        EarlyAdopter,
        #[serde(rename = "express")]
        Express,
        #[serde(rename = "professional")]
        Professional,
        #[serde(rename = "advanced")]
        Advanced,
        #[serde(rename = "stakeholder")]
        Stakeholder,
    }
    #[doc = "Type of MSDN License. To use the MsdnLicenseType, LicensingSource should be defined as 'msdn' in the request body."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MsdnLicenseType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "eligible")]
        Eligible,
        #[serde(rename = "professional")]
        Professional,
        #[serde(rename = "platforms")]
        Platforms,
        #[serde(rename = "testProfessional")]
        TestProfessional,
        #[serde(rename = "premium")]
        Premium,
        #[serde(rename = "ultimate")]
        Ultimate,
        #[serde(rename = "enterprise")]
        Enterprise,
    }
    #[doc = "Source of the License."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "account")]
        Account,
        #[serde(rename = "msdn")]
        Msdn,
        #[serde(rename = "profile")]
        Profile,
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "trial")]
        Trial,
    }
}
#[doc = "Deprecated: Use UserEntitlement instead"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlement {
    #[serde(flatten)]
    pub user_entitlement: UserEntitlement,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<GraphMember>,
}
impl MemberEntitlement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An AAD member entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlement2 {
    #[serde(flatten)]
    pub entitlement_base: EntitlementBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<AadGraphMember>,
}
impl MemberEntitlement2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlement2OperationReference {
    #[serde(flatten)]
    pub operation_reference: OperationReference,
    #[doc = "Operation completed with success or failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[doc = "True if all operations were successful."]
    #[serde(
        rename = "haveResultsSucceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub have_results_succeeded: Option<bool>,
    #[doc = "List of results for each operation."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<MemberEntitlement2OperationResult>,
}
impl MemberEntitlement2OperationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlement2OperationResult {
    #[serde(flatten)]
    pub entitlement_operation_result_base: EntitlementOperationResultBase,
    #[doc = "Identifier of the Member being acted upon."]
    #[serde(rename = "memberId", default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}
impl MemberEntitlement2OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlement2PatchResponse {
    #[serde(flatten)]
    pub member_entitlement2_response_base: MemberEntitlement2ResponseBase,
    #[doc = "List of results for each operation"]
    #[serde(
        rename = "operationResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operation_results: Vec<MemberEntitlement2OperationResult>,
}
impl MemberEntitlement2PatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlement2PostResponse {
    #[serde(flatten)]
    pub member_entitlement2_response_base: MemberEntitlement2ResponseBase,
    #[serde(
        rename = "operationResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_result: Option<MemberEntitlement2OperationResult>,
}
impl MemberEntitlement2PostResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlement2ResponseBase {
    #[doc = "True if all operations were successful."]
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[doc = "An AAD member entity"]
    #[serde(
        rename = "memberEntitlement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub member_entitlement: Option<MemberEntitlement2>,
}
impl MemberEntitlement2ResponseBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlementOperationReference {
    #[serde(flatten)]
    pub operation_reference: OperationReference,
    #[doc = "Operation completed with success or failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[doc = "True if all operations were successful"]
    #[serde(
        rename = "haveResultsSucceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub have_results_succeeded: Option<bool>,
    #[doc = "List of results for each operation"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<OperationResult>,
}
impl MemberEntitlementOperationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlementsPatchResponse {
    #[serde(flatten)]
    pub member_entitlements_response_base: MemberEntitlementsResponseBase,
    #[doc = "List of results for each operation"]
    #[serde(
        rename = "operationResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operation_results: Vec<OperationResult>,
}
impl MemberEntitlementsPatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlementsPostResponse {
    #[serde(flatten)]
    pub member_entitlements_response_base: MemberEntitlementsResponseBase,
    #[serde(
        rename = "operationResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_result: Option<OperationResult>,
}
impl MemberEntitlementsPostResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberEntitlementsResponseBase {
    #[doc = "True if all operations were successful."]
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[doc = "Deprecated: Use UserEntitlement instead"]
    #[serde(
        rename = "memberEntitlement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub member_entitlement: Option<MemberEntitlement>,
}
impl MemberEntitlementsResponseBase {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "List of error codes paired with their corresponding error messages."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<serde_json::Value>,
    #[doc = "Success status of the operation."]
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[doc = "Identifier of the Member being acted upon."]
    #[serde(rename = "memberId", default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[doc = "Deprecated: Use UserEntitlement instead"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<MemberEntitlement>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A page of users"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedGraphMemberList {
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<UserEntitlement>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub members: Vec<UserEntitlement>,
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i32>,
}
impl PagedGraphMemberList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A page of user entitlements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedUserEntitlementsList {
    #[doc = "The continuation token for next page of data. Can be null, if no more data exists."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "The requested user entitlement items."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<UserEntitlement>,
    #[doc = "The total count of the existing user entitlement items."]
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i32>,
}
impl PagedUserEntitlementsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Relation between a project and the user's effective permissions in that project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEntitlement {
    #[doc = "Assignment Source (e.g. Group or Unknown)."]
    #[serde(
        rename = "assignmentSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assignment_source: Option<project_entitlement::AssignmentSource>,
    #[doc = "Project Group (e.g. Contributor, Reader etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[doc = "Whether the user is inheriting permissions to a project through a Azure DevOps or AAD group membership."]
    #[serde(
        rename = "projectPermissionInherited",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_permission_inherited: Option<project_entitlement::ProjectPermissionInherited>,
    #[doc = "A reference to a project"]
    #[serde(
        rename = "projectRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_ref: Option<ProjectRef>,
    #[doc = "Team Ref."]
    #[serde(
        rename = "teamRefs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team_refs: Vec<TeamRef>,
}
impl ProjectEntitlement {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_entitlement {
    use super::*;
    #[doc = "Assignment Source (e.g. Group or Unknown)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AssignmentSource {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "groupRule")]
        GroupRule,
    }
    #[doc = "Whether the user is inheriting permissions to a project through a Azure DevOps or AAD group membership."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProjectPermissionInherited {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "notInherited")]
        NotInherited,
        #[serde(rename = "inherited")]
        Inherited,
    }
}
#[doc = "A reference to a project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectRef {
    #[doc = "Project ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Project Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProjectRef {
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
pub struct ServicePrincipalEntitlement {
    #[serde(flatten)]
    pub entitlement_base: EntitlementBase,
    #[serde(
        rename = "servicePrincipal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_principal: Option<GraphServicePrincipal>,
}
impl ServicePrincipalEntitlement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalEntitlementOperationReference {
    #[serde(flatten)]
    pub operation_reference: OperationReference,
    #[doc = "Operation completed with success or failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[doc = "True if all operations were successful."]
    #[serde(
        rename = "haveResultsSucceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub have_results_succeeded: Option<bool>,
    #[doc = "List of results for each operation."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<ServicePrincipalEntitlementOperationResult>,
}
impl ServicePrincipalEntitlementOperationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalEntitlementOperationResult {
    #[serde(flatten)]
    pub entitlement_operation_result_base: EntitlementOperationResultBase,
    #[doc = "Identifier of the ServicePrincipal being acted upon."]
    #[serde(
        rename = "servicePrincipalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_principal_id: Option<String>,
}
impl ServicePrincipalEntitlementOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalEntitlementsPatchResponse {
    #[serde(flatten)]
    pub service_principal_entitlements_response_base: ServicePrincipalEntitlementsResponseBase,
    #[serde(
        rename = "operationResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operation_results: Vec<ServicePrincipalEntitlementOperationResult>,
}
impl ServicePrincipalEntitlementsPatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalEntitlementsPostResponse {
    #[serde(flatten)]
    pub service_principal_entitlements_response_base: ServicePrincipalEntitlementsResponseBase,
    #[serde(
        rename = "operationResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_result: Option<ServicePrincipalEntitlementOperationResult>,
}
impl ServicePrincipalEntitlementsPostResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalEntitlementsResponseBase {
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[serde(
        rename = "servicePrincipalEntitlement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_principal_entitlement: Option<ServicePrincipalEntitlement>,
}
impl ServicePrincipalEntitlementsResponseBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryData {
    #[doc = "Count of Licenses already assigned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<i32>,
    #[doc = "Available Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<i32>,
    #[doc = "Quantity"]
    #[serde(
        rename = "includedQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub included_quantity: Option<i32>,
    #[doc = "Total Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
impl SummaryData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to a team"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamRef {
    #[doc = "Team ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Team Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TeamRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A user entity with additional properties including their license, extensions, and project membership"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEntitlement {
    #[serde(flatten)]
    pub entitlement_base: EntitlementBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<GraphUser>,
}
impl UserEntitlement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEntitlementOperationReference {
    #[serde(flatten)]
    pub operation_reference: OperationReference,
    #[doc = "Operation completed with success or failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[doc = "True if all operations were successful."]
    #[serde(
        rename = "haveResultsSucceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub have_results_succeeded: Option<bool>,
    #[doc = "List of results for each operation."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<UserEntitlementOperationResult>,
}
impl UserEntitlementOperationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEntitlementOperationResult {
    #[serde(flatten)]
    pub entitlement_operation_result_base: EntitlementOperationResultBase,
    #[doc = "Identifier of the Member being acted upon."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl UserEntitlementOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEntitlementsPatchResponse {
    #[serde(flatten)]
    pub user_entitlements_response_base: UserEntitlementsResponseBase,
    #[doc = "List of results for each operation."]
    #[serde(
        rename = "operationResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operation_results: Vec<UserEntitlementOperationResult>,
}
impl UserEntitlementsPatchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEntitlementsPostResponse {
    #[serde(flatten)]
    pub user_entitlements_response_base: UserEntitlementsResponseBase,
    #[serde(
        rename = "operationResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_result: Option<UserEntitlementOperationResult>,
}
impl UserEntitlementsPostResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEntitlementsResponseBase {
    #[doc = "True if all operations were successful."]
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[doc = "A user entity with additional properties including their license, extensions, and project membership"]
    #[serde(
        rename = "userEntitlement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_entitlement: Option<UserEntitlement>,
}
impl UserEntitlementsResponseBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of licenses and extensions assigned to users in the organization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsersSummary {
    #[doc = "Available Access Levels"]
    #[serde(
        rename = "availableAccessLevels",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_access_levels: Vec<AccessLevel>,
    #[doc = "License assigned to a user"]
    #[serde(
        rename = "defaultAccessLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_access_level: Option<AccessLevel>,
    #[doc = "Group Options"]
    #[serde(
        rename = "groupOptions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_options: Vec<GroupOption>,
    #[doc = "Summary of Licenses in the organization"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub licenses: Vec<LicenseSummaryData>,
    #[doc = "Summary of Projects in the organization"]
    #[serde(
        rename = "projectRefs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub project_refs: Vec<ProjectRef>,
}
impl UsersSummary {
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
