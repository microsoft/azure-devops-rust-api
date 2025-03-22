// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuditActionInfo {
    #[doc = "The action id for the event, i.e Git.CreateRepo, Project.RenameProject"]
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[doc = "Area of Azure DevOps the action occurred"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[doc = "Type of action executed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<audit_action_info::Category>,
}
impl AuditActionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod audit_action_info {
    use super::*;
    #[doc = "Type of action executed"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "modify")]
        Modify,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "access")]
        Access,
        #[serde(rename = "execute")]
        Execute,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuditActionInfoList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AuditActionInfo>,
}
impl AuditActionInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuditLogEntry {
    #[doc = "The action if for the event, i.e Git.CreateRepo, Project.RenameProject"]
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[doc = "ActivityId"]
    #[serde(
        rename = "activityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_id: Option<String>,
    #[doc = "The Actor's Client Id (if actor is a service principal)"]
    #[serde(
        rename = "actorClientId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_client_id: Option<String>,
    #[doc = "The Actor's CUID"]
    #[serde(rename = "actorCUID", default, skip_serializing_if = "Option::is_none")]
    pub actor_cuid: Option<String>,
    #[doc = "The Actor's UPN"]
    #[serde(rename = "actorUPN", default, skip_serializing_if = "Option::is_none")]
    pub actor_upn: Option<String>,
    #[doc = "The Actor's User Id (if actor is a user)"]
    #[serde(
        rename = "actorUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_user_id: Option<String>,
    #[doc = "Type of authentication used by the author"]
    #[serde(
        rename = "authenticationMechanism",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_mechanism: Option<String>,
    #[doc = "This allows us to group things together, like one user action that caused a cascade of event entries (project creation)."]
    #[serde(
        rename = "correlationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub correlation_id: Option<String>,
    #[doc = "External data such as CUIDs, item names, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "EventId, should be unique"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "IP Address where the event was originated"]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "When specified, the id of the project this event is associated to"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The organization Id (Organization is the only scope currently supported)"]
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[doc = "The type of the scope (Organization is only scope currently supported)"]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<audit_log_entry::ScopeType>,
    #[doc = "The time when the event occurred in UTC"]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The user agent from the request"]
    #[serde(rename = "userAgent", default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl AuditLogEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod audit_log_entry {
    use super::*;
    #[doc = "The type of the scope (Organization is only scope currently supported)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "deployment")]
        Deployment,
        #[serde(rename = "enterprise")]
        Enterprise,
        #[serde(rename = "organization")]
        Organization,
        #[serde(rename = "project")]
        Project,
    }
}
#[doc = "The object returned when the audit log is queried. It contains the log and the information needed to query more audit entries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuditLogQueryResult {
    #[doc = "The continuation token to pass to get the next set of results"]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "The list of audit log entries"]
    #[serde(
        rename = "decoratedAuditLogEntries",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub decorated_audit_log_entries: Vec<DecoratedAuditLogEntry>,
    #[doc = "True when there are more matching results to be fetched, false otherwise."]
    #[serde(rename = "hasMore", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
impl AuditLogQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents an audit stream"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuditStream {
    #[doc = "Inputs used to communicate with external service. Inputs could be url, a connection string, a token, etc."]
    #[serde(
        rename = "consumerInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_inputs: Option<serde_json::Value>,
    #[doc = "Type of the consumer, i.e. splunk, azureEventHub, etc."]
    #[serde(
        rename = "consumerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_type: Option<String>,
    #[doc = "The time when the stream was created"]
    #[serde(
        rename = "createdTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Used to identify individual streams"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Unique stream identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Status of the stream, Enabled, Disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<audit_stream::Status>,
    #[doc = "Reason for the current stream status, i.e. Disabled by the system, Invalid credentials, etc."]
    #[serde(
        rename = "statusReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_reason: Option<String>,
    #[doc = "The time when the stream was last updated"]
    #[serde(
        rename = "updatedTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub updated_time: Option<time::OffsetDateTime>,
}
impl AuditStream {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod audit_stream {
    use super::*;
    #[doc = "Status of the stream, Enabled, Disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabledByUser")]
        DisabledByUser,
        #[serde(rename = "disabledBySystem")]
        DisabledBySystem,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "backfilling")]
        Backfilling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuditStreamList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AuditStream>,
}
impl AuditStreamList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DecoratedAuditLogEntry {
    #[doc = "The action id for the event, i.e Git.CreateRepo, Project.RenameProject"]
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[doc = "ActivityId"]
    #[serde(
        rename = "activityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_id: Option<String>,
    #[doc = "The Actor's Client Id (if actor is a service principal)"]
    #[serde(
        rename = "actorClientId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_client_id: Option<String>,
    #[doc = "The Actor's CUID"]
    #[serde(rename = "actorCUID", default, skip_serializing_if = "Option::is_none")]
    pub actor_cuid: Option<String>,
    #[doc = "DisplayName of the user who initiated the action"]
    #[serde(
        rename = "actorDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_display_name: Option<String>,
    #[doc = "URL of Actor's Profile image"]
    #[serde(
        rename = "actorImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_image_url: Option<String>,
    #[doc = "The Actor's UPN"]
    #[serde(rename = "actorUPN", default, skip_serializing_if = "Option::is_none")]
    pub actor_upn: Option<String>,
    #[doc = "The Actor's User Id (if actor is a user)"]
    #[serde(
        rename = "actorUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_user_id: Option<String>,
    #[doc = "Area of Azure DevOps the action occurred"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[doc = "Type of authentication used by the actor"]
    #[serde(
        rename = "authenticationMechanism",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_mechanism: Option<String>,
    #[doc = "Type of action executed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<decorated_audit_log_entry::Category>,
    #[doc = "DisplayName of the category"]
    #[serde(
        rename = "categoryDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub category_display_name: Option<String>,
    #[doc = "This allows related audit entries to be grouped together. Generally this occurs when a single action causes a cascade of audit entries. For example, project creation."]
    #[serde(
        rename = "correlationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub correlation_id: Option<String>,
    #[doc = "External data such as CUIDs, item names, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Decorated details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "EventId - Needs to be unique per service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "IP Address where the event was originated"]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "When specified, the id of the project this event is associated to"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "When specified, the name of the project this event is associated to"]
    #[serde(
        rename = "projectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    #[doc = "DisplayName of the scope"]
    #[serde(
        rename = "scopeDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_display_name: Option<String>,
    #[doc = "The organization Id (Organization is the only scope currently supported)"]
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[doc = "The type of the scope (Organization is only scope currently supported)"]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<decorated_audit_log_entry::ScopeType>,
    #[doc = "The time when the event occurred in UTC"]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The user agent from the request"]
    #[serde(rename = "userAgent", default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl DecoratedAuditLogEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod decorated_audit_log_entry {
    use super::*;
    #[doc = "Type of action executed"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "modify")]
        Modify,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "access")]
        Access,
        #[serde(rename = "execute")]
        Execute,
    }
    #[doc = "The type of the scope (Organization is only scope currently supported)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "deployment")]
        Deployment,
        #[serde(rename = "enterprise")]
        Enterprise,
        #[serde(rename = "organization")]
        Organization,
        #[serde(rename = "project")]
        Project,
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
