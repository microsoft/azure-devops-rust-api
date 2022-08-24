// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentGroup {
    #[doc = ""]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Time agent group was created"]
    #[serde(
        rename = "creationTime",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Id of the agent group"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The name of the agent group"]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(
        rename = "machineAccessData",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_access_data: Vec<AgentGroupAccessData>,
    #[doc = "This can eventually evolve as the ultimate JSON file that user can use to configure their machine(s) against CLT"]
    #[serde(
        rename = "machineConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_configuration: Option<WebApiUserLoadTestMachineInput>,
    #[doc = "Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl AgentGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentGroupAccessData {
    #[doc = "Type Specific details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "Access string"]
    #[serde(
        rename = "storageConnectionString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_connection_string: Option<String>,
    #[doc = "Endpoint for the service"]
    #[serde(
        rename = "storageEndPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_end_point: Option<String>,
    #[doc = "Identifier for the storage (eg. table name)"]
    #[serde(
        rename = "storageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_name: Option<String>,
    #[doc = "Type of the store (table, queue, blob)"]
    #[serde(
        rename = "storageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_type: Option<String>,
}
impl AgentGroupAccessData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Application {
    #[doc = "Unique Id of the Application Component"]
    #[serde(
        rename = "applicationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_id: Option<String>,
    #[doc = "Description of the Application component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Name of the Application component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Path identifier of the Application component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Character used to separate paths for counters"]
    #[serde(
        rename = "pathSeperator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_seperator: Option<String>,
    #[doc = "Type identifier of the Application component under test"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Version of the Application Component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationCounters {
    #[doc = "The unique Id of the Application that the counter belongs"]
    #[serde(
        rename = "applicationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_id: Option<String>,
    #[doc = "Description of autCounter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The unique Id for the AutCounter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Whether the autCounter is a default counter or not"]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Name of the AutCounter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Path of the the autcounter wrt to hierarchy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl ApplicationCounters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationCountersList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationCounters>,
}
impl ApplicationCountersList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
}
impl ApplicationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationType {
    #[doc = "Helper link url"]
    #[serde(
        rename = "actionUriLink",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_uri_link: Option<String>,
    #[doc = "The link that points to aut results site"]
    #[serde(
        rename = "autPortalLink",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aut_portal_link: Option<String>,
    #[doc = "true if application results collection is enabled for this tenant"]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "the max no. of application components allowed for collection per run"]
    #[serde(
        rename = "maxComponentsAllowedForCollection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_components_allowed_for_collection: Option<i32>,
    #[doc = "The max no. of counters that can be collected per aut"]
    #[serde(
        rename = "maxCountersAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_counters_allowed: Option<i32>,
    #[doc = "Application Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ApplicationType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationType>,
}
impl ApplicationTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrowserMix {
    #[serde(
        rename = "browserName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_name: Option<String>,
    #[serde(
        rename = "browserPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_percentage: Option<f32>,
}
impl BrowserMix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CltCustomerIntelligenceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl CltCustomerIntelligenceData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CounterGroup {
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CounterGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CounterInstanceSamples {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        rename = "counterInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_instance_id: Option<String>,
    #[doc = "The time of next refresh"]
    #[serde(
        rename = "nextRefreshTime",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub next_refresh_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<CounterSample>,
}
impl CounterInstanceSamples {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CounterSample {
    #[serde(rename = "baseValue", default, skip_serializing_if = "Option::is_none")]
    pub base_value: Option<i64>,
    #[serde(
        rename = "computedValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_value: Option<f32>,
    #[serde(
        rename = "counterFrequency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_frequency: Option<i64>,
    #[serde(
        rename = "counterInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_instance_id: Option<String>,
    #[serde(
        rename = "counterType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_type: Option<String>,
    #[serde(
        rename = "intervalEndDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub interval_end_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "intervalNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_number: Option<i32>,
    #[serde(rename = "rawValue", default, skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<i64>,
    #[serde(
        rename = "systemFrequency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_frequency: Option<i64>,
    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<i64>,
}
impl CounterSample {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CounterSampleQueryDetails {
    #[doc = "The instanceId for which samples are required"]
    #[serde(
        rename = "counterInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_instance_id: Option<String>,
    #[serde(
        rename = "fromInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_interval: Option<i32>,
    #[serde(
        rename = "toInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub to_interval: Option<i32>,
}
impl CounterSampleQueryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CounterSamplesResult {
    #[doc = "Count of the samples"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Maximum number of samples returned in this object"]
    #[serde(
        rename = "maxBatchSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_batch_size: Option<i32>,
    #[doc = "Count of the samples"]
    #[serde(
        rename = "totalSamplesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_samples_count: Option<i32>,
    #[doc = "The result samples"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<CounterInstanceSamples>,
}
impl CounterSamplesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Diagnostics {
    #[serde(
        rename = "diagnosticStoreConnectionString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub diagnostic_store_connection_string: Option<String>,
    #[serde(
        rename = "lastModifiedTime",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "relativePathToDiagnosticFiles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_path_to_diagnostic_files: Option<String>,
}
impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DropAccessData {
    #[serde(
        rename = "dropContainerUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_container_url: Option<String>,
    #[doc = "The SaSkey to use for the drop."]
    #[serde(rename = "sasKey", default, skip_serializing_if = "Option::is_none")]
    pub sas_key: Option<String>,
}
impl DropAccessData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(
        rename = "lastErrorDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_error_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "messageText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(
        rename = "scenarioName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_name: Option<String>,
    #[serde(
        rename = "stackTrace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<String>,
    #[serde(
        rename = "testCaseName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_name: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphSubjectBase {
    #[doc = ""]
    #[serde(rename = "_links")]
    pub links: ReferenceLinks,
    pub descriptor: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub url: String,
}
impl GraphSubjectBase {
    pub fn new(
        links: ReferenceLinks,
        descriptor: String,
        display_name: String,
        url: String,
    ) -> Self {
        Self {
            links,
            descriptor,
            display_name,
            url,
        }
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityRef {
    #[serde(flatten)]
    pub graph_subject_base: GraphSubjectBase,
    #[serde(
        rename = "directoryAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub directory_alias: Option<String>,
    pub id: String,
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[serde(
        rename = "isAadIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_aad_identity: Option<bool>,
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
    #[serde(
        rename = "profileUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_url: Option<String>,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
}
impl IdentityRef {
    pub fn new(graph_subject_base: GraphSubjectBase, id: String, unique_name: String) -> Self {
        Self {
            graph_subject_base,
            directory_alias: None,
            id,
            image_url: None,
            inactive: None,
            is_aad_identity: None,
            is_container: None,
            is_deleted_in_origin: None,
            profile_url: None,
            unique_name,
        }
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadGenerationGeoLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}
impl LoadGenerationGeoLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTest {}
impl LoadTest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestDefinition {
    #[serde(
        rename = "agentCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_count: Option<i32>,
    #[serde(rename = "browserMixs", default, skip_serializing_if = "Vec::is_empty")]
    pub browser_mixs: Vec<BrowserMix>,
    #[serde(rename = "coreCount", default, skip_serializing_if = "Option::is_none")]
    pub core_count: Option<i32>,
    #[serde(
        rename = "coresPerAgent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cores_per_agent: Option<i32>,
    #[serde(
        rename = "loadGenerationGeoLocations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_generation_geo_locations: Vec<LoadGenerationGeoLocation>,
    #[serde(
        rename = "loadPatternName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_pattern_name: Option<String>,
    #[serde(
        rename = "loadTestName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_test_name: Option<String>,
    #[serde(rename = "maxVusers", default, skip_serializing_if = "Option::is_none")]
    pub max_vusers: Option<i32>,
    #[serde(
        rename = "runDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_duration: Option<i32>,
    #[serde(
        rename = "samplingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sampling_rate: Option<i32>,
    #[serde(rename = "thinkTime", default, skip_serializing_if = "Option::is_none")]
    pub think_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,
}
impl LoadTestDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl LoadTestErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestRunDetails {
    #[serde(flatten)]
    pub load_test_run_settings: LoadTestRunSettings,
    #[serde(
        rename = "virtualUserCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_user_count: Option<i32>,
}
impl LoadTestRunDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestRunSettings {
    #[serde(
        rename = "agentCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_count: Option<i32>,
    #[serde(rename = "coreCount", default, skip_serializing_if = "Option::is_none")]
    pub core_count: Option<i32>,
    #[serde(
        rename = "coresPerAgent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cores_per_agent: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(
        rename = "loadGeneratorMachinesType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_generator_machines_type: Option<load_test_run_settings::LoadGeneratorMachinesType>,
    #[serde(
        rename = "samplingInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sampling_interval: Option<i32>,
    #[serde(
        rename = "warmUpDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warm_up_duration: Option<i32>,
}
impl LoadTestRunSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod load_test_run_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadGeneratorMachinesType {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "cltLoadAgent")]
        CltLoadAgent,
        #[serde(rename = "userLoadAgent")]
        UserLoadAgent,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OverridableRunSettings {
    #[serde(
        rename = "loadGeneratorMachinesType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_generator_machines_type: Option<overridable_run_settings::LoadGeneratorMachinesType>,
    #[doc = ""]
    #[serde(
        rename = "staticAgentRunSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub static_agent_run_settings: Option<StaticAgentRunSetting>,
}
impl OverridableRunSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod overridable_run_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadGeneratorMachinesType {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "cltLoadAgent")]
        CltLoadAgent,
        #[serde(rename = "userLoadAgent")]
        UserLoadAgent,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageSummary {
    #[serde(
        rename = "averagePageTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_page_time: Option<f64>,
    #[serde(rename = "pageUrl", default, skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
    #[serde(
        rename = "percentagePagesMeetingGoal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percentage_pages_meeting_goal: Option<i32>,
    #[serde(
        rename = "percentileData",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub percentile_data: Vec<SummaryPercentileData>,
    #[serde(
        rename = "scenarioName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_name: Option<String>,
    #[serde(rename = "testName", default, skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    #[serde(
        rename = "totalPages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_pages: Option<i32>,
}
impl PageSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
impl ReferenceLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestSummary {
    #[serde(
        rename = "averageResponseTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_response_time: Option<f64>,
    #[serde(
        rename = "failedRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_requests: Option<i32>,
    #[serde(
        rename = "passedRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub passed_requests: Option<i32>,
    #[serde(
        rename = "percentileData",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub percentile_data: Vec<SummaryPercentileData>,
    #[serde(
        rename = "requestsPerSec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requests_per_sec: Option<f64>,
    #[serde(
        rename = "requestUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_url: Option<String>,
    #[serde(
        rename = "scenarioName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_name: Option<String>,
    #[serde(rename = "testName", default, skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    #[serde(
        rename = "totalRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_requests: Option<i32>,
}
impl RequestSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScenarioSummary {
    #[serde(
        rename = "maxUserLoad",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_user_load: Option<i32>,
    #[serde(
        rename = "minUserLoad",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_user_load: Option<i32>,
    #[serde(
        rename = "scenarioName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_name: Option<String>,
}
impl ScenarioSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticAgentRunSetting {
    #[serde(
        rename = "loadGeneratorMachinesType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_generator_machines_type: Option<static_agent_run_setting::LoadGeneratorMachinesType>,
    #[serde(
        rename = "staticAgentGroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub static_agent_group_name: Option<String>,
}
impl StaticAgentRunSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_agent_run_setting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadGeneratorMachinesType {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "cltLoadAgent")]
        CltLoadAgent,
        #[serde(rename = "userLoadAgent")]
        UserLoadAgent,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        rename = "errorDetailList",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_detail_list: Vec<ErrorDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<i32>,
    #[serde(
        rename = "subTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_type_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl SubType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryPercentileData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentile: Option<i32>,
    #[serde(
        rename = "percentileValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percentile_value: Option<f64>,
}
impl SummaryPercentileData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantDetails {
    #[doc = "Access details"]
    #[serde(
        rename = "accessDetails",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub access_details: Vec<AgentGroupAccessData>,
    #[doc = "Tenant Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Static machines configured for local runs"]
    #[serde(
        rename = "staticMachines",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub static_machines: Vec<WebApiTestMachine>,
    #[doc = "This can eventually evolve as the ultimate JSON file that user can use to configure their machine(s) against CLT"]
    #[serde(
        rename = "userLoadAgentInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_load_agent_input: Option<WebApiUserLoadTestMachineInput>,
    #[serde(
        rename = "userLoadAgentResourcesUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_load_agent_resources_uri: Option<String>,
    #[doc = "The list of valid geo-lcations for tenant"]
    #[serde(
        rename = "validGeoLocations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_geo_locations: Vec<String>,
}
impl TenantDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestDefinition {
    #[serde(flatten)]
    pub test_definition_basic: TestDefinitionBasic,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Geo location from where load is generated"]
    #[serde(
        rename = "loadGenerationGeoLocations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_generation_geo_locations: Vec<LoadGenerationGeoLocation>,
    #[serde(
        rename = "loadTestDefinitionSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_test_definition_source: Option<String>,
    #[doc = ""]
    #[serde(
        rename = "runSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_settings: Option<LoadTestRunSettings>,
    #[doc = ""]
    #[serde(
        rename = "staticAgentRunSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub static_agent_run_settings: Option<StaticAgentRunSetting>,
    #[doc = ""]
    #[serde(
        rename = "testDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_details: Option<LoadTest>,
}
impl TestDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestDefinitionBasic {
    #[doc = ""]
    #[serde(
        rename = "accessData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_data: Option<DropAccessData>,
    #[doc = ""]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[serde(
        rename = "createdDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = ""]
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<IdentityRef>,
    #[serde(
        rename = "lastModifiedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "loadTestType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_test_type: Option<test_definition_basic::LoadTestType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestDefinitionBasic {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_definition_basic {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadTestType {
        #[serde(rename = "visualStudioLoadTest")]
        VisualStudioLoadTest,
        #[serde(rename = "jMeter")]
        JMeter,
        #[serde(rename = "oldLoadTestFile")]
        OldLoadTestFile,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestDefinitionBasicList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestDefinitionBasic>,
}
impl TestDefinitionBasicList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestDrop {
    #[doc = ""]
    #[serde(
        rename = "accessData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_data: Option<DropAccessData>,
    #[doc = "Time at which the drop is created"]
    #[serde(
        rename = "createdDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Identifies the type of drop"]
    #[serde(rename = "dropType", default, skip_serializing_if = "Option::is_none")]
    pub drop_type: Option<String>,
    #[doc = "Drop Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = ""]
    #[serde(
        rename = "loadTestDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_test_definition: Option<LoadTestDefinition>,
    #[doc = "Test Run Id"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
}
impl TestDrop {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An abstracted reference to some other resource. This class is used to provide the load test data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestDropRef {
    #[doc = "Id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestDropRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResults {
    #[doc = "The uri to the test run results file."]
    #[serde(
        rename = "cloudLoadTestSolutionUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_load_test_solution_url: Option<String>,
    #[serde(
        rename = "counterGroups",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub counter_groups: Vec<CounterGroup>,
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[doc = "The uri to the test run results file."]
    #[serde(
        rename = "resultsUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_url: Option<String>,
}
impl TestResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsSummary {
    #[doc = ""]
    #[serde(
        rename = "overallPageSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_page_summary: Option<PageSummary>,
    #[doc = ""]
    #[serde(
        rename = "overallRequestSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_request_summary: Option<RequestSummary>,
    #[doc = ""]
    #[serde(
        rename = "overallScenarioSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_scenario_summary: Option<ScenarioSummary>,
    #[doc = ""]
    #[serde(
        rename = "overallTestSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_test_summary: Option<TestSummary>,
    #[doc = ""]
    #[serde(
        rename = "overallTransactionSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_transaction_summary: Option<TransactionSummary>,
    #[serde(
        rename = "topSlowPages",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_slow_pages: Vec<PageSummary>,
    #[serde(
        rename = "topSlowRequests",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_slow_requests: Vec<RequestSummary>,
    #[serde(
        rename = "topSlowTests",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_slow_tests: Vec<TestSummary>,
    #[serde(
        rename = "topSlowTransactions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_slow_transactions: Vec<TransactionSummary>,
}
impl TestResultsSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRun {
    #[serde(flatten)]
    pub test_run_basic: TestRunBasic,
    #[doc = ""]
    #[serde(
        rename = "abortMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub abort_message: Option<TestRunAbortMessage>,
    #[doc = "true if aut counter collection could not start due to some critical error for this run."]
    #[serde(
        rename = "autInitializationError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aut_initialization_error: Option<bool>,
    #[doc = "Whether run is chargeable or not Its chargeable once we configured agent and sent start signal"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chargeable: Option<bool>,
    #[doc = "Whether run is chargeable or not The Charged VUser Minutes for the RUN"]
    #[serde(
        rename = "chargedVUserminutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub charged_v_userminutes: Option<i32>,
    #[doc = "Test run description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets the time when the test run execution finished"]
    #[serde(
        rename = "executionFinishedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub execution_finished_date: Option<time::OffsetDateTime>,
    #[doc = "Gets the time when the test run warmup finished(if warmup was specified) and load test started"]
    #[serde(
        rename = "executionStartedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub execution_started_date: Option<time::OffsetDateTime>,
    #[doc = "Gets the time when the test run was queued"]
    #[serde(
        rename = "queuedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub queued_date: Option<time::OffsetDateTime>,
    #[doc = "Retention state of the run"]
    #[serde(
        rename = "retentionState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_state: Option<test_run::RetentionState>,
    #[serde(
        rename = "runSourceIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_source_identifier: Option<String>,
    #[doc = "The uri to the run source."]
    #[serde(
        rename = "runSourceUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_source_url: Option<String>,
    #[doc = ""]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<IdentityRef>,
    #[doc = "When the test run started execution."]
    #[serde(
        rename = "startedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_date: Option<time::OffsetDateTime>,
    #[doc = ""]
    #[serde(rename = "stoppedBy", default, skip_serializing_if = "Option::is_none")]
    pub stopped_by: Option<IdentityRef>,
    #[doc = "SubState is more granular description of the state"]
    #[serde(rename = "subState", default, skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<test_run::SubState>,
    #[doc = ""]
    #[serde(
        rename = "supersedeRunSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supersede_run_settings: Option<OverridableRunSettings>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the load test data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(rename = "testDrop", default, skip_serializing_if = "Option::is_none")]
    pub test_drop: Option<TestDropRef>,
    #[doc = ""]
    #[serde(
        rename = "testSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_settings: Option<TestSettings>,
    #[doc = "Gets the time when the test run warmup started"]
    #[serde(
        rename = "warmUpStartedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub warm_up_started_date: Option<time::OffsetDateTime>,
    #[doc = "The uri to the vso detailed result."]
    #[serde(
        rename = "webResultUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_result_url: Option<String>,
}
impl TestRun {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_run {
    use super::*;
    #[doc = "Retention state of the run"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RetentionState {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "markedForDeletion")]
        MarkedForDeletion,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "retain")]
        Retain,
    }
    #[doc = "SubState is more granular description of the state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SubState {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "validatingTestRun")]
        ValidatingTestRun,
        #[serde(rename = "acquiringResources")]
        AcquiringResources,
        #[serde(rename = "configuringAgents")]
        ConfiguringAgents,
        #[serde(rename = "executingSetupScript")]
        ExecutingSetupScript,
        #[serde(rename = "warmingUp")]
        WarmingUp,
        #[serde(rename = "runningTest")]
        RunningTest,
        #[serde(rename = "executingCleanupScript")]
        ExecutingCleanupScript,
        #[serde(rename = "collectingResults")]
        CollectingResults,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "partialSuccess")]
        PartialSuccess,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunAbortMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<String>,
    #[serde(
        rename = "loggedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub logged_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl TestRunAbortMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunBasic {
    #[doc = ""]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets the creation time of the test run"]
    #[serde(
        rename = "createdDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = ""]
    #[serde(rename = "deletedBy", default, skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<IdentityRef>,
    #[doc = "Gets the deleted time of the test run"]
    #[serde(
        rename = "deletedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "Gets the finish time of the test run"]
    #[serde(
        rename = "finishedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub finished_date: Option<time::OffsetDateTime>,
    #[doc = "Gets the unique identifier for the test run definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "loadGenerationGeoLocations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_generation_geo_locations: Vec<LoadGenerationGeoLocation>,
    #[doc = "Gets the load test file of the test run definition."]
    #[serde(
        rename = "loadTestFileName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_test_file_name: Option<String>,
    #[doc = "Gets the name of the test run definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the number of the test run (unique within a tenant)"]
    #[serde(rename = "runNumber", default, skip_serializing_if = "Option::is_none")]
    pub run_number: Option<i32>,
    #[doc = "Test run source like Ibiza,VSO,BuildVNext, etc."]
    #[serde(rename = "runSource", default, skip_serializing_if = "Option::is_none")]
    pub run_source: Option<String>,
    #[doc = ""]
    #[serde(
        rename = "runSpecificDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_specific_details: Option<LoadTestRunDetails>,
    #[doc = "Run type like VisualStudioLoadTest or JMeterLoadTest"]
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<test_run_basic::RunType>,
    #[doc = "State of the test run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<test_run_basic::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestRunBasic {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_run_basic {
    use super::*;
    #[doc = "Run type like VisualStudioLoadTest or JMeterLoadTest"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunType {
        #[serde(rename = "visualStudioLoadTest")]
        VisualStudioLoadTest,
        #[serde(rename = "jMeterLoadTest")]
        JMeterLoadTest,
    }
    #[doc = "State of the test run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "error")]
        Error,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunCounterInstance {
    #[doc = "CategoryName for this counter"]
    #[serde(
        rename = "categoryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub category_name: Option<String>,
    #[doc = "Combination of source and SourceInstanceId"]
    #[serde(
        rename = "counterInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_instance_id: Option<String>,
    #[doc = "Name of the counter Eg: Errors/Sec"]
    #[serde(
        rename = "counterName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_name: Option<String>,
    #[doc = "Units for this counter. Empty string for mere numbers"]
    #[serde(
        rename = "counterUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counter_units: Option<String>,
    #[doc = "Instance Name Eg: _Avg,_Total etc"]
    #[serde(
        rename = "instanceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_name: Option<String>,
    #[doc = "true if this counter instance is a default counter"]
    #[serde(
        rename = "isPreselectedCounter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_preselected_counter: Option<bool>,
    #[doc = "Machine from where this counter was collected Used in case of machine specific counters like - Agent CPU and memory etc."]
    #[serde(
        rename = "machineName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_name: Option<String>,
    #[doc = "Counter Groups to which this counter instance is part of"]
    #[serde(
        rename = "partOfCounterGroups",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub part_of_counter_groups: Vec<String>,
    #[doc = ""]
    #[serde(
        rename = "summaryData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub summary_data: Option<WebInstanceSummaryData>,
    #[doc = "A unique name for this counter instance"]
    #[serde(
        rename = "uniqueName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_name: Option<String>,
}
impl TestRunCounterInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunCounterInstanceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestRunCounterInstance>,
}
impl TestRunCounterInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunMessage {
    #[doc = "Agent Id"]
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(
        rename = "loggedDate",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub logged_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Message Id"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(
        rename = "messageSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_source: Option<test_run_message::MessageSource>,
    #[serde(
        rename = "messageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<test_run_message::MessageType>,
    #[doc = "Id of the test run"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestRunMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_run_message {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MessageSource {
        #[serde(rename = "setupScript")]
        SetupScript,
        #[serde(rename = "cleanupScript")]
        CleanupScript,
        #[serde(rename = "validation")]
        Validation,
        #[serde(rename = "other")]
        Other,
        #[serde(rename = "autCounterCollection")]
        AutCounterCollection,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MessageType {
        #[serde(rename = "info")]
        Info,
        #[serde(rename = "output")]
        Output,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "critical")]
        Critical,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunMessageList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestRunMessage>,
}
impl TestRunMessageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSettings {
    #[doc = "Cleanup command"]
    #[serde(
        rename = "cleanupCommand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cleanup_command: Option<String>,
    #[doc = "Processor Architecture chosen"]
    #[serde(
        rename = "hostProcessPlatform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_process_platform: Option<test_settings::HostProcessPlatform>,
    #[doc = "Setup command"]
    #[serde(
        rename = "setupCommand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub setup_command: Option<String>,
}
impl TestSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_settings {
    use super::*;
    #[doc = "Processor Architecture chosen"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostProcessPlatform {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "msil")]
        Msil,
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "ia64")]
        Ia64,
        #[serde(rename = "amd64")]
        Amd64,
        #[serde(rename = "arm")]
        Arm,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSummary {
    #[serde(
        rename = "averageTestTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_test_time: Option<f64>,
    #[serde(
        rename = "failedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_tests: Option<i32>,
    #[serde(
        rename = "passedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub passed_tests: Option<i32>,
    #[serde(
        rename = "percentileData",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub percentile_data: Vec<SummaryPercentileData>,
    #[serde(
        rename = "scenarioName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_name: Option<String>,
    #[serde(rename = "testName", default, skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    #[serde(
        rename = "totalTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_tests: Option<i32>,
}
impl TestSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionSummary {
    #[serde(
        rename = "averageResponseTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_response_time: Option<f64>,
    #[serde(
        rename = "averageTransactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_transaction_time: Option<f64>,
    #[serde(
        rename = "percentileData",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub percentile_data: Vec<SummaryPercentileData>,
    #[serde(
        rename = "scenarioName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_name: Option<String>,
    #[serde(rename = "testName", default, skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    #[serde(
        rename = "totalTransactions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transactions: Option<i32>,
    #[serde(
        rename = "transactionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_name: Option<String>,
}
impl TransactionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Type {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<i32>,
    #[serde(rename = "subTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_types: Vec<SubType>,
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Type {
    pub fn new() -> Self {
        Self::default()
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
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiLoadTestMachineInput {
    #[serde(
        rename = "machineGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_group_id: Option<String>,
    #[serde(
        rename = "machineType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_type: Option<web_api_load_test_machine_input::MachineType>,
    #[doc = ""]
    #[serde(
        rename = "setupConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub setup_configuration: Option<WebApiSetupParamaters>,
    #[serde(
        rename = "supportedRunTypes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_run_types: Vec<serde_json::Value>,
}
impl WebApiLoadTestMachineInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_api_load_test_machine_input {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MachineType {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "cltLoadAgent")]
        CltLoadAgent,
        #[serde(rename = "userLoadAgent")]
        UserLoadAgent,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiSetupParamaters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
}
impl WebApiSetupParamaters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiTestMachine {
    #[serde(
        rename = "lastHeartBeat",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_heart_beat: Option<time::OffsetDateTime>,
    #[serde(
        rename = "machineName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl WebApiTestMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This can eventually evolve as the ultimate JSON file that user can use to configure their machine(s) against CLT"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiUserLoadTestMachineInput {
    #[serde(flatten)]
    pub web_api_load_test_machine_input: WebApiLoadTestMachineInput,
    #[serde(
        rename = "agentGroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_group_name: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(
        rename = "userLoadAgentResourcesUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_load_agent_resources_uri: Option<String>,
    #[serde(
        rename = "vstsAccountUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vsts_account_uri: Option<String>,
}
impl WebApiUserLoadTestMachineInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebInstanceSummaryData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}
impl WebInstanceSummaryData {
    pub fn new() -> Self {
        Self::default()
    }
}
