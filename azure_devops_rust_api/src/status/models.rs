// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseStatus {
    #[serde(flatten)]
    pub status: Status,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<OrganizationHealth>,
}
impl EnterpriseStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Geography {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Geography {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeographyWithHealth {
    #[serde(flatten)]
    pub geography: Geography,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<geography_with_health::Health>,
}
impl GeographyWithHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod geography_with_health {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Health {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "degraded")]
        Degraded,
        #[serde(rename = "advisory")]
        Advisory,
        #[serde(rename = "healthy")]
        Healthy,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveSiteEvent {
    #[serde(
        rename = "endTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub end_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub impact: Vec<LiveSiteEventImpact>,
    #[serde(
        rename = "incidentUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub incident_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<LiveSiteEventLog>,
    #[serde(
        rename = "nextUpdateTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub next_update_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<live_site_event::Severity>,
    #[serde(
        rename = "startTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<live_site_event::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl LiveSiteEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod live_site_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "degraded")]
        Degraded,
        #[serde(rename = "advisory")]
        Advisory,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "resolved")]
        Resolved,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveSiteEventAuthor {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl LiveSiteEventAuthor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveSiteEventImpact {
    #[serde(rename = "scopeName", default, skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<live_site_event_impact::ScopeType>,
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}
impl LiveSiteEventImpact {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod live_site_event_impact {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "geography")]
        Geography,
        #[serde(rename = "organization")]
        Organization,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveSiteEventLog {
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<LiveSiteEventAuthor>,
    #[serde(
        rename = "creationDateTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub creation_date_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "descriptionMd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description_md: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(
        rename = "lastUpdatedDateTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub last_updated_date_time: Option<time::OffsetDateTime>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<live_site_event_log::ScopeType>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<live_site_event_log::Type>,
}
impl LiveSiteEventLog {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod live_site_event_log {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "geography")]
        Geography,
        #[serde(rename = "organization")]
        Organization,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "event")]
        Event,
        #[serde(rename = "postmortem")]
        Postmortem,
        #[serde(rename = "notification")]
        Notification,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveSiteEventLogAttachment {
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<LiveSiteEventAuthor>,
    #[serde(
        rename = "creationDateTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub creation_date_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(rename = "eventId", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i32>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<live_site_event_log_attachment::ScopeType>,
}
impl LiveSiteEventLogAttachment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod live_site_event_log_attachment {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "geography")]
        Geography,
        #[serde(rename = "organization")]
        Organization,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveSiteEventTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "initialDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl LiveSiteEventTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveSiteEventTemplateData {
    #[serde(rename = "defaultId", default, skip_serializing_if = "Option::is_none")]
    pub default_id: Option<String>,
    #[serde(
        rename = "finalDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub final_description: Option<String>,
    #[serde(
        rename = "intermediateDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intermediate_description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<LiveSiteEventTemplate>,
}
impl LiveSiteEventTemplateData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MicroService {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scaleUnits", default, skip_serializing_if = "Vec::is_empty")]
    pub scale_units: Vec<MicroServiceScaleUnit>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<Service>,
}
impl MicroService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MicroServiceScaleUnit {
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geography: Option<Geography>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isInternal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_internal: Option<bool>,
}
impl MicroServiceScaleUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationHealth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<ServiceWithHealth>,
}
impl OrganizationHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Service {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isInternal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_internal: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<service::State>,
}
impl Service {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "hidden")]
        Hidden,
        #[serde(rename = "deleted")]
        Deleted,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceHealth {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub geographies: Vec<GeographyWithHealth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ServiceHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceStatus {
    #[serde(flatten)]
    pub status: Status,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<ServiceHealth>,
}
impl ServiceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceWithHealth {
    #[serde(flatten)]
    pub service: Service,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<service_with_health::Health>,
}
impl ServiceWithHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_with_health {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Health {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "degraded")]
        Degraded,
        #[serde(rename = "advisory")]
        Advisory,
        #[serde(rename = "healthy")]
        Healthy,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Status {
    #[serde(
        rename = "lastUpdated",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusSummary>,
}
impl Status {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<status_summary::Health>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl StatusSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod status_summary {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Health {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "degraded")]
        Degraded,
        #[serde(rename = "advisory")]
        Advisory,
        #[serde(rename = "healthy")]
        Healthy,
    }
}
#[doc = "This class is used to serialized collections as a single JSON object on the wire, to avoid serializing JSON arrays directly to the client, which can be a security hole"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VssJsonCollectionWrapper {
    #[serde(flatten)]
    pub vss_json_collection_wrapper_base: VssJsonCollectionWrapperBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VssJsonCollectionWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VssJsonCollectionWrapperBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl VssJsonCollectionWrapperBase {
    pub fn new() -> Self {
        Self::default()
    }
}
