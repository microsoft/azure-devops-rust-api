// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents a queue for running builds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPoolQueue {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "The ID of the queue."]
    pub id: i32,
    #[doc = "The name of the queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Represents a reference to an agent pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<TaskAgentPoolReference>,
    #[doc = "The full http link to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl AgentPoolQueue {
    pub fn new(id: i32) -> Self {
        Self {
            links: None,
            id,
            name: None,
            pool: None,
            url: None,
        }
    }
}
#[doc = "Represents a reference to an agent queue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolQueueReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The ID of the queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl AgentPoolQueueReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes how a phase should run against an agent queue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentPoolQueueTarget {
    #[serde(flatten)]
    pub phase_target: PhaseTarget,
    #[doc = "Specification of the agent defined by the pool provider."]
    #[serde(
        rename = "agentSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_specification: Option<AgentSpecification>,
    #[doc = "Enables scripts and other processes launched while executing phase to access the OAuth token"]
    #[serde(
        rename = "allowScriptsAuthAccessOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_scripts_auth_access_option: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[doc = "Additional options for running phases against an agent queue."]
    #[serde(
        rename = "executionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_options: Option<AgentTargetExecutionOptions>,
    #[doc = "Represents a queue for running builds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<AgentPoolQueue>,
}
impl AgentPoolQueueTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specification of the agent defined by the pool provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentSpecification {
    #[doc = "Agent specification unique identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl AgentSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional options for running phases against an agent queue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentTargetExecutionOptions {
    #[doc = "Indicates the type of execution options."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
}
impl AgentTargetExecutionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedResultsAnalysis {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(
        rename = "notReportedResultsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_reported_results_by_outcome: Option<serde_json::Value>,
    #[serde(
        rename = "previousContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_context: Option<TestResultsContext>,
    #[serde(
        rename = "resultsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_by_outcome: Option<serde_json::Value>,
    #[serde(
        rename = "resultsDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_difference: Option<AggregatedResultsDifference>,
    #[serde(
        rename = "runSummaryByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary_by_outcome: Option<serde_json::Value>,
    #[serde(
        rename = "runSummaryByState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary_by_state: Option<serde_json::Value>,
    #[serde(
        rename = "totalTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_tests: Option<i32>,
}
impl AggregatedResultsAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedResultsByOutcome {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(
        rename = "groupByField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_field: Option<String>,
    #[serde(
        rename = "groupByValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<aggregated_results_by_outcome::Outcome>,
    #[serde(
        rename = "rerunResultCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rerun_result_count: Option<i32>,
}
impl AggregatedResultsByOutcome {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aggregated_results_by_outcome {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Outcome {
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "inconclusive")]
        Inconclusive,
        #[serde(rename = "timeout")]
        Timeout,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "blocked")]
        Blocked,
        #[serde(rename = "notExecuted")]
        NotExecuted,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "notApplicable")]
        NotApplicable,
        #[serde(rename = "paused")]
        Paused,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "notImpacted")]
        NotImpacted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedResultsDifference {
    #[serde(
        rename = "increaseInDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_duration: Option<String>,
    #[serde(
        rename = "increaseInFailures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_failures: Option<i32>,
    #[serde(
        rename = "increaseInNonImpactedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_non_impacted_tests: Option<i32>,
    #[serde(
        rename = "increaseInOtherTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_other_tests: Option<i32>,
    #[serde(
        rename = "increaseInPassedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_passed_tests: Option<i32>,
    #[serde(
        rename = "increaseInTotalTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_total_tests: Option<i32>,
}
impl AggregatedResultsDifference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedRunsByOutcome {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<aggregated_runs_by_outcome::Outcome>,
    #[serde(rename = "runsCount", default, skip_serializing_if = "Option::is_none")]
    pub runs_count: Option<i32>,
}
impl AggregatedRunsByOutcome {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aggregated_runs_by_outcome {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Outcome {
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "notImpacted")]
        NotImpacted,
        #[serde(rename = "others")]
        Others,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedRunsByState {
    #[serde(
        rename = "resultsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_by_outcome: Option<serde_json::Value>,
    #[serde(rename = "runsCount", default, skip_serializing_if = "Option::is_none")]
    pub runs_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<aggregated_runs_by_state::State>,
}
impl AggregatedRunsByState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aggregated_runs_by_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "waiting")]
        Waiting,
        #[serde(rename = "needsInvestigation")]
        NeedsInvestigation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactResource {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Type-specific data about the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "A link to download the resource."]
    #[serde(
        rename = "downloadUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_url: Option<String>,
    #[doc = "Type-specific properties of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The type of the resource: File container, version control folder, UNC path, etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The full http link to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ArtifactResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociatedWorkItem {
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<String>,
    #[doc = "Id of associated the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "REST Url of the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
impl AssociatedWorkItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an attachment to a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attachment {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "The name of the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Attachment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachmentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Attachment>,
}
impl AttachmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationHeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AuthorizationHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data representation of a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Build {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Specification of the agent defined by the pool provider."]
    #[serde(
        rename = "agentSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_specification: Option<AgentSpecification>,
    #[doc = "Append Commit Message To BuildNumber in UI."]
    #[serde(
        rename = "appendCommitMessageToRunName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub append_commit_message_to_run_name: Option<bool>,
    #[doc = "The build number/name of the build."]
    #[serde(
        rename = "buildNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_number: Option<String>,
    #[doc = "The build number revision."]
    #[serde(
        rename = "buildNumberRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_number_revision: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<BuildController>,
    #[doc = "Represents a reference to a definition."]
    pub definition: DefinitionReference,
    #[doc = "Indicates whether the build has been deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "deletedBy", default, skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<IdentityRef>,
    #[doc = "The date the build was deleted."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[doc = "The description of how the build was deleted."]
    #[serde(
        rename = "deletedReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_reason: Option<String>,
    #[doc = "A list of demands that represents the agent capabilities required by this build."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[doc = "The time that the build was completed."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The ID of the build."]
    pub id: i32,
    #[serde(
        rename = "lastChangedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_changed_by: Option<IdentityRef>,
    #[doc = "The date the build was last changed."]
    #[serde(
        rename = "lastChangedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_date: Option<time::OffsetDateTime>,
    #[doc = "Represents a reference to a build log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<BuildLogReference>,
    #[doc = "Represents a reference to an orchestration plan."]
    #[serde(
        rename = "orchestrationPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orchestration_plan: Option<TaskOrchestrationPlanReference>,
    #[doc = "The parameters for the build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[doc = "Orchestration plans associated with the build (build, cleanup)"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plans: Vec<TaskOrchestrationPlanReference>,
    #[doc = "The build's priority."]
    pub priority: build::Priority,
    #[doc = "Represents a shallow reference to a TeamProject."]
    pub project: TeamProjectReference,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "The quality of the xaml build (good, bad, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[doc = "Represents a queue for running builds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<AgentPoolQueue>,
    #[doc = "Additional options for queueing the build."]
    #[serde(
        rename = "queueOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_options: Option<build::QueueOptions>,
    #[doc = "The current position of the build in the queue."]
    #[serde(
        rename = "queuePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_position: Option<i32>,
    #[doc = "The time that the build was queued."]
    #[serde(
        rename = "queueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queue_time: Option<time::OffsetDateTime>,
    #[doc = "The reason that the build was created."]
    pub reason: build::Reason,
    #[doc = "Represents a repository used by a build definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<BuildRepository>,
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<IdentityRef>,
    #[serde(
        rename = "requestedFor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_for: Option<IdentityRef>,
    #[doc = "The build result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<build::Result>,
    #[doc = "Indicates whether the build is retained by a release."]
    #[serde(
        rename = "retainedByRelease",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retained_by_release: Option<bool>,
    #[doc = "The source branch."]
    #[serde(
        rename = "sourceBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_branch: Option<String>,
    #[doc = "The source version."]
    #[serde(
        rename = "sourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_version: Option<String>,
    #[doc = "The time that the build was started."]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The status of the build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<build::Status>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Parameters to template expression evaluation"]
    #[serde(
        rename = "templateParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_parameters: Option<serde_json::Value>,
    #[doc = "Data representation of a build."]
    #[serde(
        rename = "triggeredByBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub triggered_by_build: Option<Box<Build>>,
    #[doc = "Sourceprovider-specific information about what triggered the build"]
    #[serde(
        rename = "triggerInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_info: Option<serde_json::Value>,
    #[doc = "The URI of the build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The REST URL of the build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(
        rename = "validationResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_results: Vec<BuildRequestValidationResult>,
}
impl Build {
    pub fn new(
        definition: DefinitionReference,
        id: i32,
        priority: build::Priority,
        project: TeamProjectReference,
        reason: build::Reason,
    ) -> Self {
        Self {
            links: None,
            agent_specification: None,
            append_commit_message_to_run_name: None,
            build_number: None,
            build_number_revision: None,
            controller: None,
            definition,
            deleted: None,
            deleted_by: None,
            deleted_date: None,
            deleted_reason: None,
            demands: Vec::new(),
            finish_time: None,
            id,
            last_changed_by: None,
            last_changed_date: None,
            logs: None,
            orchestration_plan: None,
            parameters: None,
            plans: Vec::new(),
            priority,
            project,
            properties: None,
            quality: None,
            queue: None,
            queue_options: None,
            queue_position: None,
            queue_time: None,
            reason,
            repository: None,
            requested_by: None,
            requested_for: None,
            result: None,
            retained_by_release: None,
            source_branch: None,
            source_version: None,
            start_time: None,
            status: None,
            tags: Vec::new(),
            template_parameters: None,
            triggered_by_build: None,
            trigger_info: None,
            uri: None,
            url: None,
            validation_results: Vec::new(),
        }
    }
}
pub mod build {
    use super::*;
    #[doc = "The build's priority."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Priority {
        #[serde(rename = "low")]
        Low,
        #[serde(rename = "belowNormal")]
        BelowNormal,
        #[serde(rename = "normal")]
        Normal,
        #[serde(rename = "aboveNormal")]
        AboveNormal,
        #[serde(rename = "high")]
        High,
    }
    #[doc = "Additional options for queueing the build."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueueOptions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "doNotRun")]
        DoNotRun,
    }
    #[doc = "The reason that the build was created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "individualCI")]
        IndividualCi,
        #[serde(rename = "batchedCI")]
        BatchedCi,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "scheduleForced")]
        ScheduleForced,
        #[serde(rename = "userCreated")]
        UserCreated,
        #[serde(rename = "validateShelveset")]
        ValidateShelveset,
        #[serde(rename = "checkInShelveset")]
        CheckInShelveset,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "resourceTrigger")]
        ResourceTrigger,
        #[serde(rename = "triggered")]
        Triggered,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "The build result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
    }
    #[doc = "The status of the build."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "postponed")]
        Postponed,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildAgent {
    #[serde(
        rename = "buildDirectory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_directory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<XamlBuildControllerReference>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        rename = "messageQueueUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_queue_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "reservedForBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reserved_for_build: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<XamlBuildServerReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<build_agent::Status>,
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
    #[serde(
        rename = "updatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub updated_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl BuildAgent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_agent {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unavailable")]
        Unavailable,
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "offline")]
        Offline,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildAgentReference {
    #[doc = "Id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the linked resource (definition name, controller name, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl BuildAgentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an artifact produced by a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildArtifact {
    #[doc = "The artifact ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The name of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ArtifactResource>,
    #[doc = "The artifact source, which will be the ID of the job that produced this artifact. If an artifact is associated with multiple sources, this points to the first source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl BuildArtifact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildArtifactList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildArtifact>,
}
impl BuildArtifactList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a build badge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildBadge {
    #[doc = "The ID of the build represented by this badge."]
    #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<i32>,
    #[doc = "A link to the SVG resource."]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}
impl BuildBadge {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildCompletedEvent {
    #[serde(flatten)]
    pub build_updated_event: BuildUpdatedEvent,
    #[doc = "Changes associated with a build used for build notifications"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changes: Vec<Change>,
    #[doc = "Represents a pull request object.  These are retrieved from Source Providers."]
    #[serde(
        rename = "pullRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request: Option<PullRequest>,
    #[serde(
        rename = "testResults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_results: Option<AggregatedResultsAnalysis>,
    #[doc = "Timeline records associated with a build used for build notifications"]
    #[serde(
        rename = "timelineRecords",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub timeline_records: Vec<TimelineRecord>,
    #[doc = "Work items associated with a build used for build notifications"]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<AssociatedWorkItem>,
}
impl BuildCompletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a build completion trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildCompletionTrigger {
    #[serde(flatten)]
    pub build_trigger: BuildTrigger,
    #[serde(
        rename = "branchFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_filters: Vec<String>,
    #[doc = "Represents a reference to a definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<DefinitionReference>,
    #[serde(
        rename = "requiresSuccessfulBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_successful_build: Option<bool>,
}
impl BuildCompletionTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildController {
    #[serde(flatten)]
    pub xaml_build_controller_reference: XamlBuildControllerReference,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "The date the controller was created."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The description of the controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicates whether the controller is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The status of the controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<build_controller::Status>,
    #[doc = "The date the controller was last updated."]
    #[serde(
        rename = "updatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub updated_date: Option<time::OffsetDateTime>,
    #[doc = "The controller's URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl BuildController {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_controller {
    use super::*;
    #[doc = "The status of the controller."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unavailable")]
        Unavailable,
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "offline")]
        Offline,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildControllerList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildController>,
}
impl BuildControllerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildDefinition {
    #[serde(flatten)]
    pub build_definition_reference: BuildDefinitionReference,
    #[doc = "Indicates whether badges are enabled for this definition."]
    #[serde(
        rename = "badgeEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub badge_enabled: Option<bool>,
    #[doc = "The build number format."]
    #[serde(
        rename = "buildNumberFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_number_format: Option<String>,
    #[doc = "A save-time comment for the definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The drop location for the definition."]
    #[serde(
        rename = "dropLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_location: Option<String>,
    #[doc = "The job authorization scope for builds queued against this definition."]
    #[serde(
        rename = "jobAuthorizationScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_authorization_scope: Option<build_definition::JobAuthorizationScope>,
    #[doc = "The job cancel timeout (in minutes) for builds cancelled by user for this definition."]
    #[serde(
        rename = "jobCancelTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_cancel_timeout_in_minutes: Option<i32>,
    #[doc = "The job execution timeout (in minutes) for builds queued against this definition."]
    #[serde(
        rename = "jobTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_timeout_in_minutes: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub options: Vec<BuildOption>,
    #[doc = "Represents a build process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<BuildProcess>,
    #[serde(
        rename = "processParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub process_parameters: Option<ProcessParameters>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Represents a repository used by a build definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<BuildRepository>,
    #[serde(
        rename = "retentionRules",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub retention_rules: Vec<RetentionPolicy>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub triggers: Vec<BuildTrigger>,
    #[serde(
        rename = "variableGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_groups: Vec<VariableGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl BuildDefinition {
    pub fn new(build_definition_reference: BuildDefinitionReference) -> Self {
        Self {
            build_definition_reference,
            badge_enabled: None,
            build_number_format: None,
            comment: None,
            demands: Vec::new(),
            description: None,
            drop_location: None,
            job_authorization_scope: None,
            job_cancel_timeout_in_minutes: None,
            job_timeout_in_minutes: None,
            options: Vec::new(),
            process: None,
            process_parameters: None,
            properties: None,
            repository: None,
            retention_rules: Vec::new(),
            tags: Vec::new(),
            triggers: Vec::new(),
            variable_groups: Vec::new(),
            variables: None,
        }
    }
}
pub mod build_definition {
    use super::*;
    #[doc = "The job authorization scope for builds queued against this definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobAuthorizationScope {
        #[serde(rename = "projectCollection")]
        ProjectCollection,
        #[serde(rename = "project")]
        Project,
    }
}
#[doc = "For back-compat with extensions that use the old Steps format instead of Process and Phases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildDefinition32 {
    #[serde(flatten)]
    pub build_definition_reference3_2: BuildDefinitionReference32,
    #[doc = "Indicates whether badges are enabled for this definition"]
    #[serde(
        rename = "badgeEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub badge_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub build: Vec<BuildDefinitionStep>,
    #[doc = "The build number format"]
    #[serde(
        rename = "buildNumberFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_number_format: Option<String>,
    #[doc = "The comment entered when saving the definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[doc = "The description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The drop location for the definition"]
    #[serde(
        rename = "dropLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_location: Option<String>,
    #[doc = "The job authorization scope for builds which are queued against this definition"]
    #[serde(
        rename = "jobAuthorizationScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_authorization_scope: Option<build_definition3_2::JobAuthorizationScope>,
    #[doc = "The job cancel timeout in minutes for builds which are cancelled by user for this definition"]
    #[serde(
        rename = "jobCancelTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_cancel_timeout_in_minutes: Option<i32>,
    #[doc = "The job execution timeout in minutes for builds which are queued against this definition"]
    #[serde(
        rename = "jobTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_timeout_in_minutes: Option<i32>,
    #[doc = "Data representation of a build."]
    #[serde(
        rename = "latestBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_build: Option<Build>,
    #[doc = "Data representation of a build."]
    #[serde(
        rename = "latestCompletedBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_completed_build: Option<Build>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub options: Vec<BuildOption>,
    #[serde(
        rename = "processParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub process_parameters: Option<ProcessParameters>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Represents a repository used by a build definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<BuildRepository>,
    #[serde(
        rename = "retentionRules",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub retention_rules: Vec<RetentionPolicy>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub triggers: Vec<BuildTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl BuildDefinition32 {
    pub fn new(build_definition_reference3_2: BuildDefinitionReference32) -> Self {
        Self {
            build_definition_reference3_2,
            badge_enabled: None,
            build: Vec::new(),
            build_number_format: None,
            comment: None,
            demands: Vec::new(),
            description: None,
            drop_location: None,
            job_authorization_scope: None,
            job_cancel_timeout_in_minutes: None,
            job_timeout_in_minutes: None,
            latest_build: None,
            latest_completed_build: None,
            options: Vec::new(),
            process_parameters: None,
            properties: None,
            repository: None,
            retention_rules: Vec::new(),
            tags: Vec::new(),
            triggers: Vec::new(),
            variables: None,
        }
    }
}
pub mod build_definition3_2 {
    use super::*;
    #[doc = "The job authorization scope for builds which are queued against this definition"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobAuthorizationScope {
        #[serde(rename = "projectCollection")]
        ProjectCollection,
        #[serde(rename = "project")]
        Project,
    }
}
#[doc = "Represents a reference to a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildDefinitionReference {
    #[serde(flatten)]
    pub definition_reference: DefinitionReference,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "authoredBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authored_by: Option<IdentityRef>,
    #[doc = "Represents a reference to a definition."]
    #[serde(rename = "draftOf", default, skip_serializing_if = "Option::is_none")]
    pub draft_of: Option<DefinitionReference>,
    #[doc = "The list of drafts associated with this definition, if this is not a draft definition."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub drafts: Vec<DefinitionReference>,
    #[doc = "Data representation of a build."]
    #[serde(
        rename = "latestBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_build: Option<Build>,
    #[doc = "Data representation of a build."]
    #[serde(
        rename = "latestCompletedBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_completed_build: Option<Build>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics: Vec<BuildMetric>,
    #[doc = "The quality of the definition document (draft, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<build_definition_reference::Quality>,
    #[doc = "Represents a queue for running builds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<AgentPoolQueue>,
}
impl BuildDefinitionReference {
    pub fn new(definition_reference: DefinitionReference) -> Self {
        Self {
            definition_reference,
            links: None,
            authored_by: None,
            draft_of: None,
            drafts: Vec::new(),
            latest_build: None,
            latest_completed_build: None,
            metrics: Vec::new(),
            quality: None,
            queue: None,
        }
    }
}
pub mod build_definition_reference {
    use super::*;
    #[doc = "The quality of the definition document (draft, etc.)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Quality {
        #[serde(rename = "definition")]
        Definition,
        #[serde(rename = "draft")]
        Draft,
    }
}
#[doc = "For back-compat with extensions that use the old Steps format instead of Process and Phases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildDefinitionReference32 {
    #[serde(flatten)]
    pub definition_reference: DefinitionReference,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "authoredBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authored_by: Option<IdentityRef>,
    #[doc = "Represents a reference to a definition."]
    #[serde(rename = "draftOf", default, skip_serializing_if = "Option::is_none")]
    pub draft_of: Option<DefinitionReference>,
    #[doc = "The list of drafts associated with this definition, if this is not a draft definition."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub drafts: Vec<DefinitionReference>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics: Vec<BuildMetric>,
    #[doc = "The quality of the definition document (draft, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<build_definition_reference3_2::Quality>,
    #[doc = "Represents a queue for running builds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<AgentPoolQueue>,
}
impl BuildDefinitionReference32 {
    pub fn new(definition_reference: DefinitionReference) -> Self {
        Self {
            definition_reference,
            links: None,
            authored_by: None,
            draft_of: None,
            drafts: Vec::new(),
            metrics: Vec::new(),
            quality: None,
            queue: None,
        }
    }
}
pub mod build_definition_reference3_2 {
    use super::*;
    #[doc = "The quality of the definition document (draft, etc.)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Quality {
        #[serde(rename = "definition")]
        Definition,
        #[serde(rename = "draft")]
        Draft,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildDefinitionReference>,
}
impl BuildDefinitionReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a revision of a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionRevision {
    #[serde(rename = "changedBy", default, skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<IdentityRef>,
    #[doc = "The date and time that the definition was changed."]
    #[serde(
        rename = "changedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub changed_date: Option<time::OffsetDateTime>,
    #[doc = "The change type (add, edit, delete)."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<build_definition_revision::ChangeType>,
    #[doc = "The comment associated with the change."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "A link to the definition at this revision."]
    #[serde(
        rename = "definitionUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_url: Option<String>,
    #[doc = "The name of the definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl BuildDefinitionRevision {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_definition_revision {
    use super::*;
    #[doc = "The change type (add, edit, delete)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "delete")]
        Delete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionRevisionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildDefinitionRevision>,
}
impl BuildDefinitionRevisionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionSourceProvider {
    #[doc = "Uri of the associated definition"]
    #[serde(
        rename = "definitionUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_uri: Option<String>,
    #[doc = "fields associated with this build definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    #[doc = "Id of this source provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The lst time this source provider was modified"]
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "Name of the source provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Which trigger types are supported by this definition source provider"]
    #[serde(
        rename = "supportedTriggerTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_trigger_types: Option<build_definition_source_provider::SupportedTriggerTypes>,
}
impl BuildDefinitionSourceProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_definition_source_provider {
    use super::*;
    #[doc = "Which trigger types are supported by this definition source provider"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SupportedTriggerTypes {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "batchedContinuousIntegration")]
        BatchedContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "gatedCheckIn")]
        GatedCheckIn,
        #[serde(rename = "batchedGatedCheckIn")]
        BatchedGatedCheckIn,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "Represents a step in a build phase."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionStep {
    #[doc = "Indicates whether this step should run even if a previous step fails."]
    #[serde(rename = "alwaysRun", default, skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    #[doc = "A condition that determines whether this step should run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Indicates whether the phase should continue even if this step fails."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "The display name for this step."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Indicates whether the step is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "The reference name for this step."]
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[doc = "Number of retries."]
    #[serde(
        rename = "retryCountOnTaskFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_count_on_task_failure: Option<i32>,
    #[doc = "A reference to a task definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskDefinitionReference>,
    #[doc = "The time, in minutes, that this step is allowed to run."]
    #[serde(
        rename = "timeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_in_minutes: Option<i32>,
}
impl BuildDefinitionStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a template from which new build definitions can be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionTemplate {
    #[doc = "Indicates whether the template can be deleted."]
    #[serde(rename = "canDelete", default, skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[doc = "The template category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "An optional hosted agent queue for the template to use by default."]
    #[serde(
        rename = "defaultHostedQueue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_hosted_queue: Option<String>,
    #[doc = "A description of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icons: Option<serde_json::Value>,
    #[doc = "The ID of the task whose icon is used when showing this template in the UI."]
    #[serde(
        rename = "iconTaskId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_task_id: Option<String>,
    #[doc = "The ID of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Represents a build definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<BuildDefinition>,
}
impl BuildDefinitionTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "For back-compat with extensions that use the old Steps format instead of Process and Phases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionTemplate32 {
    #[serde(rename = "canDelete", default, skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(
        rename = "defaultHostedQueue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_hosted_queue: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icons: Option<serde_json::Value>,
    #[serde(
        rename = "iconTaskId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "For back-compat with extensions that use the old Steps format instead of Process and Phases"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<BuildDefinition32>,
}
impl BuildDefinitionTemplate32 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionTemplateList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildDefinitionTemplate>,
}
impl BuildDefinitionTemplateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a variable used by a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionVariable {
    #[doc = "Indicates whether the value can be set at queue time."]
    #[serde(
        rename = "allowOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_override: Option<bool>,
    #[doc = "Indicates whether the variable's value is a secret."]
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[doc = "The value of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl BuildDefinitionVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDeletedEvent {
    #[serde(flatten)]
    pub realtime_build_event: RealtimeBuildEvent,
    #[doc = "Data representation of a build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}
impl BuildDeletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDeployment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<BuildSummary>,
    #[serde(
        rename = "sourceBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_build: Option<XamlBuildReference>,
}
impl BuildDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildEvent {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl BuildEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Build>,
}
impl BuildList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a build log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildLog {
    #[serde(flatten)]
    pub build_log_reference: BuildLogReference,
    #[doc = "The date and time the log was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The date and time the log was last changed."]
    #[serde(
        rename = "lastChangedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_on: Option<time::OffsetDateTime>,
    #[doc = "The number of lines in the log."]
    #[serde(rename = "lineCount", default, skip_serializing_if = "Option::is_none")]
    pub line_count: Option<i64>,
}
impl BuildLog {
    pub fn new(build_log_reference: BuildLogReference) -> Self {
        Self {
            build_log_reference,
            created_on: None,
            last_changed_on: None,
            line_count: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildLogList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildLog>,
}
impl BuildLogList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a build log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildLogReference {
    #[doc = "The ID of the log."]
    pub id: i32,
    #[doc = "The type of the log location."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "A full link to the log resource."]
    pub url: String,
}
impl BuildLogReference {
    pub fn new(id: i32, type_: String, url: String) -> Self {
        Self { id, type_, url }
    }
}
#[doc = "Represents metadata about builds in the system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildMetric {
    #[doc = "The date for the scope."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "The value."]
    #[serde(rename = "intValue", default, skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i32>,
    #[doc = "The name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl BuildMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildMetricList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildMetric>,
}
impl BuildMetricList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the application of an optional behavior to a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildOption {
    #[doc = "Represents a reference to a build option definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<BuildOptionDefinitionReference>,
    #[doc = "Indicates whether the behavior is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
}
impl BuildOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an optional behavior that can be applied to a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildOptionDefinition {
    #[serde(flatten)]
    pub build_option_definition_reference: BuildOptionDefinitionReference,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of input groups defined for the build option."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<BuildOptionGroupDefinition>,
    #[doc = "The list of inputs defined for the build option."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<BuildOptionInputDefinition>,
    #[doc = "The name of the build option."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A value that indicates the relative order in which the behavior should be applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i32>,
}
impl BuildOptionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildOptionDefinitionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildOptionDefinition>,
}
impl BuildOptionDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a build option definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildOptionDefinitionReference {
    #[doc = "The ID of the referenced build option."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl BuildOptionDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a group of inputs for a build option."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildOptionGroupDefinition {
    #[doc = "The name of the group to display in the UI."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Indicates whether the group is initially displayed as expanded in the UI."]
    #[serde(
        rename = "isExpanded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_expanded: Option<bool>,
    #[doc = "The internal name of the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl BuildOptionGroupDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an input for a build option."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildOptionInputDefinition {
    #[doc = "The default value."]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
    #[doc = "The name of the input group that this input belongs to."]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<serde_json::Value>,
    #[doc = "The label for the input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The name of the input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "Indicates whether the input is required to have a value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "Indicates the type of the input value."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<build_option_input_definition::Type>,
    #[doc = "The rule that is applied to determine whether the input is visible in the UI."]
    #[serde(
        rename = "visibleRule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visible_rule: Option<String>,
}
impl BuildOptionInputDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_option_input_definition {
    use super::*;
    #[doc = "Indicates the type of the input value."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(rename = "stringList")]
        StringList,
        #[serde(rename = "radio")]
        Radio,
        #[serde(rename = "pickList")]
        PickList,
        #[serde(rename = "multiLine")]
        MultiLine,
        #[serde(rename = "branchFilter")]
        BranchFilter,
    }
}
#[doc = "Represents a build process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildProcess {
    #[doc = "The type of the process."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
}
impl BuildProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents resources used by a build process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildProcessResources {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<ServiceEndpointReference>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<SecureFileReference>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub queues: Vec<AgentPoolQueueReference>,
    #[serde(
        rename = "variableGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_groups: Vec<VariableGroupReference>,
}
impl BuildProcessResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildProcessTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "fileExists",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_exists: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(
        rename = "serverPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_path: Option<String>,
    #[serde(
        rename = "supportedReasons",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_reasons: Option<build_process_template::SupportedReasons>,
    #[serde(
        rename = "teamProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_project: Option<String>,
    #[serde(
        rename = "templateType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_type: Option<build_process_template::TemplateType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl BuildProcessTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_process_template {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SupportedReasons {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "individualCI")]
        IndividualCi,
        #[serde(rename = "batchedCI")]
        BatchedCi,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "scheduleForced")]
        ScheduleForced,
        #[serde(rename = "userCreated")]
        UserCreated,
        #[serde(rename = "validateShelveset")]
        ValidateShelveset,
        #[serde(rename = "checkInShelveset")]
        CheckInShelveset,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "resourceTrigger")]
        ResourceTrigger,
        #[serde(rename = "triggered")]
        Triggered,
        #[serde(rename = "all")]
        All,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TemplateType {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "upgrade")]
        Upgrade,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildQueuedEvent {
    #[serde(flatten)]
    pub build_updated_event: BuildUpdatedEvent,
}
impl BuildQueuedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildReference {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "The build number."]
    #[serde(
        rename = "buildNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_number: Option<String>,
    #[doc = "Indicates whether the build has been deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[doc = "The time that the build was completed."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The ID of the build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The time that the build was queued."]
    #[serde(
        rename = "queueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queue_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "requestedFor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_for: Option<IdentityRef>,
    #[doc = "The build result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<build_reference::Result>,
    #[doc = "The time that the build was started."]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The build status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<build_reference::Status>,
}
impl BuildReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_reference {
    use super::*;
    #[doc = "The build result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
    }
    #[doc = "The build status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "postponed")]
        Postponed,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "Represents information about a build report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildReportMetadata {
    #[doc = "The Id of the build."]
    #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<i32>,
    #[doc = "The content of the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The type of the report."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl BuildReportMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a repository used by a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildRepository {
    #[doc = "Indicates whether to checkout submodules."]
    #[serde(
        rename = "checkoutSubmodules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub checkout_submodules: Option<bool>,
    #[doc = "Indicates whether to clean the target folder when getting code from the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clean: Option<String>,
    #[doc = "The name of the default branch."]
    #[serde(
        rename = "defaultBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_branch: Option<String>,
    #[doc = "The ID of the repository."]
    pub id: String,
    #[doc = "The friendly name of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The root folder."]
    #[serde(
        rename = "rootFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub root_folder: Option<String>,
    #[doc = "The type of the repository."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The URL of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl BuildRepository {
    pub fn new(id: String) -> Self {
        Self {
            checkout_submodules: None,
            clean: None,
            default_branch: None,
            id,
            name: None,
            properties: None,
            root_folder: None,
            type_: None,
            url: None,
        }
    }
}
#[doc = "Represents the result of validating a build request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildRequestValidationResult {
    #[doc = "The message associated with the result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<build_request_validation_result::Result>,
}
impl BuildRequestValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_request_validation_result {
    use super::*;
    #[doc = "The result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
    }
}
#[doc = "Represents information about resources used by builds in the system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildResourceUsage {
    #[doc = "The number of build agents."]
    #[serde(
        rename = "distributedTaskAgents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub distributed_task_agents: Option<i32>,
    #[doc = "The number of paid private agent slots."]
    #[serde(
        rename = "paidPrivateAgentSlots",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paid_private_agent_slots: Option<i32>,
    #[doc = "The total usage."]
    #[serde(
        rename = "totalUsage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_usage: Option<i32>,
    #[doc = "The number of XAML controllers."]
    #[serde(
        rename = "xamlControllers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub xaml_controllers: Option<i32>,
}
impl BuildResourceUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A historical overview of build retention information. This includes a list of snapshots taken about build retention usage, and a list of builds that have exceeded the default 30 day retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildRetentionHistory {
    #[doc = "A list of builds that are older than the default retention policy, but are not marked as retained. Something is causing these builds to not get cleaned up."]
    #[serde(
        rename = "buildRetentionSamples",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub build_retention_samples: Vec<BuildRetentionSample>,
}
impl BuildRetentionHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A snapshot of build retention information. This class takes a sample at the given time. It provides information about retained builds, files associated with those retained builds, and number of files being retained."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildRetentionSample {
    #[doc = "Summary of retention by build"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builds: Option<String>,
    #[doc = "List of build definitions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definitions: Option<String>,
    #[doc = "Summary of files consumed by retained builds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    #[doc = "The date and time when the sample was taken"]
    #[serde(
        rename = "sampleTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub sample_time: Option<time::OffsetDateTime>,
}
impl BuildRetentionSample {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildServer {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub agents: Vec<BuildAgentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<XamlBuildControllerReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isVirtual", default, skip_serializing_if = "Option::is_none")]
    pub is_virtual: Option<bool>,
    #[serde(
        rename = "messageQueueUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_queue_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "requireClientCertificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_client_certificates: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<build_server::Status>,
    #[serde(
        rename = "statusChangedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub status_changed_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl BuildServer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_server {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "online")]
        Online,
        #[serde(rename = "offline")]
        Offline,
    }
}
#[doc = "Represents system-wide build settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildSettings {
    #[doc = "The number of days to keep records of deleted builds."]
    #[serde(
        rename = "daysToKeepDeletedBuildsBeforeDestroy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_keep_deleted_builds_before_destroy: Option<i32>,
    #[doc = "Represents a retention policy for a build definition."]
    #[serde(
        rename = "defaultRetentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_retention_policy: Option<RetentionPolicy>,
    #[doc = "Represents a retention policy for a build definition."]
    #[serde(
        rename = "maximumRetentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_retention_policy: Option<RetentionPolicy>,
}
impl BuildSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<XamlBuildReference>,
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "keepForever",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_forever: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<build_summary::Reason>,
    #[serde(
        rename = "requestedFor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_for: Option<IdentityRef>,
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<build_summary::Status>,
}
impl BuildSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_summary {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "individualCI")]
        IndividualCi,
        #[serde(rename = "batchedCI")]
        BatchedCi,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "scheduleForced")]
        ScheduleForced,
        #[serde(rename = "userCreated")]
        UserCreated,
        #[serde(rename = "validateShelveset")]
        ValidateShelveset,
        #[serde(rename = "checkInShelveset")]
        CheckInShelveset,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "resourceTrigger")]
        ResourceTrigger,
        #[serde(rename = "triggered")]
        Triggered,
        #[serde(rename = "all")]
        All,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "postponed")]
        Postponed,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildTagsAddedEvent {
    #[serde(flatten)]
    pub build_updated_event: BuildUpdatedEvent,
    #[serde(
        rename = "allTags",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_tags: Vec<String>,
    #[serde(
        rename = "newTags",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub new_tags: Vec<String>,
}
impl BuildTagsAddedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a trigger for a buld definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildTrigger {
    #[doc = "The type of the trigger."]
    #[serde(
        rename = "triggerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_type: Option<build_trigger::TriggerType>,
}
impl BuildTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_trigger {
    use super::*;
    #[doc = "The type of the trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TriggerType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "batchedContinuousIntegration")]
        BatchedContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "gatedCheckIn")]
        GatedCheckIn,
        #[serde(rename = "batchedGatedCheckIn")]
        BatchedGatedCheckIn,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildUpdatedEvent {
    #[serde(flatten)]
    pub realtime_build_event: RealtimeBuildEvent,
    #[doc = "Data representation of a build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}
impl BuildUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a workspace mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildWorkspace {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<MappingDetails>,
}
impl BuildWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildsDeletedEvent {
    #[serde(flatten)]
    pub builds_deleted_event1: BuildsDeletedEvent1,
}
impl BuildsDeletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildsDeletedEvent1 {
    #[serde(
        rename = "buildIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub build_ids: Vec<i32>,
    #[doc = "The ID of the definition."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "The ID of the project."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl BuildsDeletedEvent1 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a change associated with a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Change {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[doc = "The location of a user-friendly representation of the resource."]
    #[serde(
        rename = "displayUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_uri: Option<String>,
    #[doc = "The identifier for the change. For a commit, this would be the SHA1. For a TFVC changeset, this would be the changeset ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The location of the full representation of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The description of the change. This might be a commit message or changeset description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Indicates whether the message was truncated."]
    #[serde(
        rename = "messageTruncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_truncated: Option<bool>,
    #[doc = "The person or process that pushed the change."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pusher: Option<String>,
    #[doc = "The timestamp for the change."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The type of change. \"commit\", \"changeset\", etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Change {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Change>,
}
impl ChangeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsoleLogEvent {
    #[serde(flatten)]
    pub realtime_build_event: RealtimeBuildEvent,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub lines: Vec<String>,
    #[serde(
        rename = "stepRecordId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub step_record_id: Option<String>,
    #[serde(
        rename = "timelineId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_id: Option<String>,
    #[serde(
        rename = "timelineRecordId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_record_id: Option<String>,
}
impl ConsoleLogEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinuousDeploymentDefinition {
    #[serde(
        rename = "connectedService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_service: Option<WebApiConnectedServiceRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<XamlDefinitionReference>,
    #[serde(rename = "gitBranch", default, skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    #[serde(
        rename = "hostedServiceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hosted_service_name: Option<String>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
    #[serde(
        rename = "storageAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_account_name: Option<String>,
    #[serde(
        rename = "subscriptionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webspace: Option<String>,
}
impl ContinuousDeploymentDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a continuous integration (CI) trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinuousIntegrationTrigger {
    #[serde(flatten)]
    pub build_trigger: BuildTrigger,
    #[doc = "Indicates whether changes should be batched while another CI build is running."]
    #[serde(
        rename = "batchChanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub batch_changes: Option<bool>,
    #[serde(
        rename = "branchFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_filters: Vec<String>,
    #[doc = "The maximum number of simultaneous CI builds that will run per branch."]
    #[serde(
        rename = "maxConcurrentBuildsPerBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrent_builds_per_branch: Option<i32>,
    #[serde(
        rename = "pathFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub path_filters: Vec<String>,
    #[doc = "The polling interval, in seconds."]
    #[serde(
        rename = "pollingInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub polling_interval: Option<i32>,
    #[doc = "The ID of the job used to poll an external repository."]
    #[serde(
        rename = "pollingJobId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub polling_job_id: Option<String>,
    #[serde(
        rename = "settingsSourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub settings_source_type: Option<i32>,
}
impl ContinuousIntegrationTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents binding of data source for the service endpoint request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceBindingBase {
    #[doc = "Pagination format supported by this data source(ContinuationToken/SkipTop)."]
    #[serde(
        rename = "callbackContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_context_template: Option<String>,
    #[doc = "Subsequent calls needed?"]
    #[serde(
        rename = "callbackRequiredTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_required_template: Option<String>,
    #[doc = "Gets or sets the name of the data source."]
    #[serde(
        rename = "dataSourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_name: Option<String>,
    #[doc = "Gets or sets the endpoint Id."]
    #[serde(
        rename = "endpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<String>,
    #[doc = "Gets or sets the url of the service endpoint."]
    #[serde(
        rename = "endpointUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_url: Option<String>,
    #[doc = "Gets or sets the authorization headers."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<AuthorizationHeader>,
    #[doc = "Defines the initial value of the query params"]
    #[serde(
        rename = "initialContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_context_template: Option<String>,
    #[doc = "Gets or sets the parameters for the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets http request body"]
    #[serde(
        rename = "requestContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_content: Option<String>,
    #[doc = "Gets or sets http request verb"]
    #[serde(
        rename = "requestVerb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_verb: Option<String>,
    #[doc = "Gets or sets the result selector."]
    #[serde(
        rename = "resultSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_selector: Option<String>,
    #[doc = "Gets or sets the result template."]
    #[serde(
        rename = "resultTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_template: Option<String>,
    #[doc = "Gets or sets the target of the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl DataSourceBindingBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefinitionReference {
    #[doc = "The date this version of the definition was created."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The ID of the referenced definition."]
    pub id: i32,
    #[doc = "The name of the referenced definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The folder path of the definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    pub project: TeamProjectReference,
    #[doc = "A value that indicates whether builds can be queued against this definition."]
    #[serde(rename = "queueStatus")]
    pub queue_status: definition_reference::QueueStatus,
    #[doc = "The definition revision number."]
    pub revision: i32,
    #[doc = "The type of the definition."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<definition_reference::Type>,
    #[doc = "The definition's URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The REST URL of the definition."]
    pub url: String,
}
impl DefinitionReference {
    pub fn new(
        id: i32,
        project: TeamProjectReference,
        queue_status: definition_reference::QueueStatus,
        revision: i32,
        url: String,
    ) -> Self {
        Self {
            created_date: None,
            id,
            name: None,
            path: None,
            project,
            queue_status,
            revision,
            type_: None,
            uri: None,
            url,
        }
    }
}
pub mod definition_reference {
    use super::*;
    #[doc = "A value that indicates whether builds can be queued against this definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueueStatus {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "paused")]
        Paused,
        #[serde(rename = "disabled")]
        Disabled,
    }
    #[doc = "The type of the definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "xaml")]
        Xaml,
        #[serde(rename = "build")]
        Build,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefinitionResourceReference {
    #[doc = "Indicates whether the resource is authorized for use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[doc = "The id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A friendly name for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DefinitionResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefinitionResourceReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DefinitionResourceReference>,
}
impl DefinitionResourceReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a demand used by a definition or build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Demand {
    #[doc = "The name of the capability referenced by the demand."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The demanded value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Demand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a dependency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dependency {
    #[doc = "The event. The dependency is satisfied when the referenced object emits this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[doc = "The scope. This names the object referenced by the dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl Dependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the data from the build information nodes for type \"DeploymentInformation\" for xaml builds"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment information for type \"Build\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentBuild {
    #[serde(flatten)]
    pub deployment: Deployment,
    #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<i32>,
}
impl DeploymentBuild {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment information for type \"Deploy\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentDeploy {
    #[serde(flatten)]
    pub deployment: Deployment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DeploymentDeploy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment information for type \"Test\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentTest {
    #[serde(flatten)]
    pub deployment: Deployment,
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
}
impl DeploymentTest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a build process supported by the build definition designer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesignerProcess {
    #[serde(flatten)]
    pub build_process: BuildProcess,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub phases: Vec<Phase>,
    #[doc = "Represents the target for the build process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<DesignerProcessTarget>,
}
impl DesignerProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the target for the build process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesignerProcessTarget {
    #[doc = "Specification of the agent defined by the pool provider."]
    #[serde(
        rename = "agentSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_specification: Option<AgentSpecification>,
}
impl DesignerProcessTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DockerProcess {
    #[serde(flatten)]
    pub build_process: BuildProcess,
    #[doc = "Represents the target for the docker build process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<DockerProcessTarget>,
}
impl DockerProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the target for the docker build process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DockerProcessTarget {
    #[serde(flatten)]
    pub designer_process_target: DesignerProcessTarget,
}
impl DockerProcessTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a folder that contains build definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Folder {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "The date the folder was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "lastChangedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_changed_by: Option<IdentityRef>,
    #[doc = "The date the folder was last changed."]
    #[serde(
        rename = "lastChangedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_date: Option<time::OffsetDateTime>,
    #[doc = "The full path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
}
impl Folder {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FolderList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Folder>,
}
impl FolderList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the ability to build forks of the selected repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Forks {
    #[doc = "Indicates whether a build should allow a full access token or scope it down when building forks of the selected repository."]
    #[serde(
        rename = "allowFullAccessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_full_access_token: Option<bool>,
    #[doc = "Indicates whether a build should use secrets when building forks of the selected repository."]
    #[serde(
        rename = "allowSecrets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_secrets: Option<bool>,
    #[doc = "Indicates whether the trigger should queue builds for forks of the selected repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl Forks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a gated check-in trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatedCheckInTrigger {
    #[serde(flatten)]
    pub build_trigger: BuildTrigger,
    #[serde(
        rename = "pathFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub path_filters: Vec<String>,
    #[doc = "Indicates whether CI triggers should run after the gated check-in succeeds."]
    #[serde(
        rename = "runContinuousIntegration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_continuous_integration: Option<bool>,
    #[doc = "Indicates whether to take workspace mappings into account when determining whether a build should run."]
    #[serde(
        rename = "useWorkspaceMappings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_workspace_mappings: Option<bool>,
}
impl GatedCheckInTrigger {
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
#[doc = "Data representation of an information node associated with a build"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationNode {
    #[doc = "Fields of the information node"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    #[doc = "Process or person that last modified this node"]
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<String>,
    #[doc = "Date this node was last modified"]
    #[serde(
        rename = "lastModifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_date: Option<time::OffsetDateTime>,
    #[doc = "Node Id of this information node"]
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<i32>,
    #[doc = "Id of parent node (xml tree)"]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    #[doc = "The type of the information node"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl InformationNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an issue (error, warning) associated with a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Issue {
    #[doc = "The category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "A description of the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The type (error, warning) of the issue."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<issue::Type>,
}
impl Issue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod issue {
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
#[doc = "Job in pipeline. This is related to matrixing in YAML."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobReference {
    #[doc = "Attempt number of the job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Matrixing in YAML generates copies of a job with different inputs in matrix. JobName is the name of those input. Maximum supported length for name is 256 character."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}
impl JobReference {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JustInTimeProcess {
    #[serde(flatten)]
    pub build_process: BuildProcess,
}
impl JustInTimeProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Link URL"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}
impl Link {
    pub fn new(href: String) -> Self {
        Self { href }
    }
}
#[doc = "Represents an entry in a workspace mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MappingDetails {
    #[doc = "The local path."]
    #[serde(rename = "localPath", default, skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    #[doc = "The mapping type."]
    #[serde(
        rename = "mappingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mapping_type: Option<String>,
    #[doc = "The server path."]
    #[serde(
        rename = "serverPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_path: Option<String>,
}
impl MappingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MinimalRetentionLease {
    #[doc = "The pipeline definition of the run."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "User-provided string that identifies the owner of a retention lease."]
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[doc = "The pipeline run to protect."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
}
impl MinimalRetentionLease {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents options for running a phase against multiple agents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MultipleAgentExecutionOptions {
    #[serde(flatten)]
    pub agent_target_execution_options: AgentTargetExecutionOptions,
    #[doc = "Indicates whether failure on one agent should prevent the phase from running on other agents."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "The maximum number of agents to use simultaneously."]
    #[serde(
        rename = "maxConcurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrency: Option<i32>,
}
impl MultipleAgentExecutionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Required information to create a new retention lease."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewRetentionLease {
    #[doc = "The number of days to consider the lease valid. A retention lease valid for more than 100 years (36500 days) will display as retaining the build \"forever\"."]
    #[serde(rename = "daysValid", default, skip_serializing_if = "Option::is_none")]
    pub days_valid: Option<i32>,
    #[doc = "The pipeline definition of the run."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "User-provided string that identifies the owner of a retention lease."]
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[doc = "If set, this lease will also prevent the pipeline from being deleted while the lease is still valid."]
    #[serde(
        rename = "protectPipeline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_pipeline: Option<bool>,
    #[doc = "The pipeline run to protect."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
}
impl NewRetentionLease {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a phase of a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Phase {
    #[doc = "The condition that must be true for this phase to execute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependencies: Vec<Dependency>,
    #[doc = "The job authorization scope for builds queued against this definition."]
    #[serde(
        rename = "jobAuthorizationScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_authorization_scope: Option<phase::JobAuthorizationScope>,
    #[doc = "The cancellation timeout, in minutes, for builds queued against this definition."]
    #[serde(
        rename = "jobCancelTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_cancel_timeout_in_minutes: Option<i32>,
    #[doc = "The job execution timeout, in minutes, for builds queued against this definition."]
    #[serde(
        rename = "jobTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_timeout_in_minutes: Option<i32>,
    #[doc = "The name of the phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unique ref name of the phase."]
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub steps: Vec<BuildDefinitionStep>,
    #[doc = "Represents the target of a phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<PhaseTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl Phase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod phase {
    use super::*;
    #[doc = "The job authorization scope for builds queued against this definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobAuthorizationScope {
        #[serde(rename = "projectCollection")]
        ProjectCollection,
        #[serde(rename = "project")]
        Project,
    }
}
#[doc = "Phase in pipeline"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PhaseReference {
    #[doc = "Attempt number of the phase"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Name of the phase. Maximum supported length for name is 256 character."]
    #[serde(rename = "phaseName", default, skip_serializing_if = "Option::is_none")]
    pub phase_name: Option<String>,
}
impl PhaseReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the target of a phase."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PhaseTarget {
    #[doc = "The type of the target."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
}
impl PhaseTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains pipeline general settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineGeneralSettings {
    #[doc = "If enabled, audit logs will be generated whenever someone queues a pipeline run and defines variables that are not marked as \"Settable at queue time\"."]
    #[serde(
        rename = "auditEnforceSettableVar",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audit_enforce_settable_var: Option<bool>,
    #[doc = "Enable forked repositories to build pull requests."]
    #[serde(
        rename = "buildsEnabledForForks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub builds_enabled_for_forks: Option<bool>,
    #[doc = "Disable classic build pipelines creation."]
    #[serde(
        rename = "disableClassicBuildPipelineCreation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_classic_build_pipeline_creation: Option<bool>,
    #[doc = "Disable classic pipelines creation."]
    #[serde(
        rename = "disableClassicPipelineCreation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_classic_pipeline_creation: Option<bool>,
    #[doc = "Disable classic release pipelines creation."]
    #[serde(
        rename = "disableClassicReleasePipelineCreation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_classic_release_pipeline_creation: Option<bool>,
    #[doc = "Enable shell tasks args sanitizing."]
    #[serde(
        rename = "enableShellTasksArgsSanitizing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_shell_tasks_args_sanitizing: Option<bool>,
    #[doc = "Enable shell tasks args sanitizing preview."]
    #[serde(
        rename = "enableShellTasksArgsSanitizingAudit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_shell_tasks_args_sanitizing_audit: Option<bool>,
    #[doc = "If enabled, scope of access for all non-release pipelines reduces to the current project."]
    #[serde(
        rename = "enforceJobAuthScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_job_auth_scope: Option<bool>,
    #[doc = "Enforce job auth scope for builds of forked repositories."]
    #[serde(
        rename = "enforceJobAuthScopeForForks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_job_auth_scope_for_forks: Option<bool>,
    #[doc = "If enabled, scope of access for all release pipelines reduces to the current project."]
    #[serde(
        rename = "enforceJobAuthScopeForReleases",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_job_auth_scope_for_releases: Option<bool>,
    #[doc = "Enforce no access to secrets for builds of forked repositories."]
    #[serde(
        rename = "enforceNoAccessToSecretsFromForks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_no_access_to_secrets_from_forks: Option<bool>,
    #[doc = "Restricts the scope of access for all pipelines to only repositories explicitly referenced by the pipeline."]
    #[serde(
        rename = "enforceReferencedRepoScopedToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_referenced_repo_scoped_token: Option<bool>,
    #[doc = "If enabled, only those variables that are explicitly marked as \"Settable at queue time\" can be set at queue time."]
    #[serde(
        rename = "enforceSettableVar",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_settable_var: Option<bool>,
    #[doc = "Enable settings that enforce certain levels of protection for building pull requests from forks globally."]
    #[serde(
        rename = "forkProtectionEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fork_protection_enabled: Option<bool>,
    #[doc = "Make comments required to have builds in all pull requests."]
    #[serde(
        rename = "isCommentRequiredForPullRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_comment_required_for_pull_request: Option<bool>,
    #[doc = "Allows pipelines to record metadata."]
    #[serde(
        rename = "publishPipelineMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publish_pipeline_metadata: Option<bool>,
    #[doc = "Make comments required to have builds in pull requests from non-team members and non-contributors."]
    #[serde(
        rename = "requireCommentsForNonTeamMemberAndNonContributors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_comments_for_non_team_member_and_non_contributors: Option<bool>,
    #[doc = "Make comments required to have builds in pull requests from non-team members."]
    #[serde(
        rename = "requireCommentsForNonTeamMembersOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_comments_for_non_team_members_only: Option<bool>,
    #[doc = "Anonymous users can access the status badge API for all pipelines unless this option is enabled."]
    #[serde(
        rename = "statusBadgesArePrivate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_badges_are_private: Option<bool>,
}
impl PipelineGeneralSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pipeline reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineReference {
    #[doc = "Job in pipeline. This is related to matrixing in YAML."]
    #[serde(
        rename = "jobReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_reference: Option<JobReference>,
    #[doc = "Phase in pipeline"]
    #[serde(
        rename = "phaseReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phase_reference: Option<PhaseReference>,
    #[doc = "Reference of the pipeline with which this pipeline instance is related."]
    #[serde(
        rename = "pipelineId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pipeline_id: Option<i32>,
    #[doc = "Stage in pipeline"]
    #[serde(
        rename = "stageReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_reference: Option<StageReference>,
}
impl PipelineReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the effective settings applicable to individual pipeline triggers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTriggerSettings {
    #[doc = "Enable forked repositories to build pull requests."]
    #[serde(
        rename = "buildsEnabledForForks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub builds_enabled_for_forks: Option<bool>,
    #[doc = "Enforce job auth scope for builds of forked repositories."]
    #[serde(
        rename = "enforceJobAuthScopeForForks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_job_auth_scope_for_forks: Option<bool>,
    #[doc = "Enforce no access to secrets for builds of forked repositories."]
    #[serde(
        rename = "enforceNoAccessToSecretsFromForks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_no_access_to_secrets_from_forks: Option<bool>,
    #[doc = "Enable settings that enforce certain levels of protection for building pull requests from forks globally."]
    #[serde(
        rename = "forkProtectionEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fork_protection_enabled: Option<bool>,
    #[doc = "Make comments required to have builds in all pull requests."]
    #[serde(
        rename = "isCommentRequiredForPullRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_comment_required_for_pull_request: Option<bool>,
    #[doc = "Make comments required to have builds in pull requests from non-team members and non-contributors."]
    #[serde(
        rename = "requireCommentsForNonTeamMemberAndNonContributors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_comments_for_non_team_member_and_non_contributors: Option<bool>,
    #[doc = "Make comments required to have builds in pull requests from non-team members."]
    #[serde(
        rename = "requireCommentsForNonTeamMembersOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_comments_for_non_team_members_only: Option<bool>,
}
impl PipelineTriggerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessParameters {
    #[serde(
        rename = "dataSourceBindings",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_source_bindings: Vec<DataSourceBindingBase>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<TaskInputDefinitionBase>,
    #[serde(
        rename = "sourceDefinitions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_definitions: Vec<TaskSourceDefinitionBase>,
}
impl ProcessParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the settings for the retention rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectRetentionSetting {
    #[doc = "Contains the minimum, maximum, and current value for a retention setting."]
    #[serde(
        rename = "purgeArtifacts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub purge_artifacts: Option<RetentionSetting>,
    #[doc = "Contains the minimum, maximum, and current value for a retention setting."]
    #[serde(
        rename = "purgePullRequestRuns",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub purge_pull_request_runs: Option<RetentionSetting>,
    #[doc = "Contains the minimum, maximum, and current value for a retention setting."]
    #[serde(rename = "purgeRuns", default, skip_serializing_if = "Option::is_none")]
    pub purge_runs: Option<RetentionSetting>,
    #[doc = "Contains the minimum, maximum, and current value for a retention setting."]
    #[serde(
        rename = "retainRunsPerProtectedBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retain_runs_per_protected_branch: Option<RetentionSetting>,
}
impl ProjectRetentionSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertiesCollection {
    #[doc = "The count of properties in the collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<serde_json::Value>,
    #[doc = "The set of keys in the collection."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub keys: Vec<String>,
    #[doc = "The set of values in the collection."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl PropertiesCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a pull request object.  These are retrieved from Source Providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequest {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[doc = "Current state of the pull request, e.g. open, merged, closed, conflicts, etc."]
    #[serde(
        rename = "currentState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_state: Option<String>,
    #[doc = "Description for the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Returns if pull request is draft"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[doc = "Unique identifier for the pull request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the provider this pull request is associated with."]
    #[serde(
        rename = "providerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
    #[doc = "Source branch ref of this pull request"]
    #[serde(
        rename = "sourceBranchRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_branch_ref: Option<String>,
    #[doc = "Owner of the source repository of this pull request"]
    #[serde(
        rename = "sourceRepositoryOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_repository_owner: Option<String>,
    #[doc = "Target branch ref of this pull request"]
    #[serde(
        rename = "targetBranchRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_branch_ref: Option<String>,
    #[doc = "Owner of the target repository of this pull request"]
    #[serde(
        rename = "targetRepositoryOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_repository_owner: Option<String>,
    #[doc = "Title of the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl PullRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a pull request trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequestTrigger {
    #[serde(flatten)]
    pub build_trigger: BuildTrigger,
    #[doc = "Indicates if an update to a PR should delete current in-progress builds."]
    #[serde(
        rename = "autoCancel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_cancel: Option<bool>,
    #[serde(
        rename = "branchFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_filters: Vec<String>,
    #[doc = "Represents the ability to build forks of the selected repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks: Option<Forks>,
    #[serde(
        rename = "isCommentRequiredForPullRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_comment_required_for_pull_request: Option<bool>,
    #[serde(
        rename = "pathFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub path_filters: Vec<String>,
    #[doc = "Represents the effective settings applicable to individual pipeline triggers."]
    #[serde(
        rename = "pipelineTriggerSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pipeline_trigger_settings: Option<PipelineTriggerSettings>,
    #[serde(
        rename = "requireCommentsForNonTeamMemberAndNonContributors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_comments_for_non_team_member_and_non_contributors: Option<bool>,
    #[serde(
        rename = "requireCommentsForNonTeamMembersOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_comments_for_non_team_members_only: Option<bool>,
    #[serde(
        rename = "settingsSourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub settings_source_type: Option<i32>,
}
impl PullRequestTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeBuildEvent {
    #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<i32>,
}
impl RealtimeBuildEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class to represent a collection of REST reference links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLinks {
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge: Option<Link>,
    #[doc = "Link URL"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<Link>,
    #[doc = "Link URL"]
    #[serde(
        rename = "sourceVersionDisplayUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_version_display_uri: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<Link>,
}
impl ReferenceLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a release."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseReference {
    #[doc = "Number of Release Attempt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Release Creation Date(UTC)."]
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Release definition ID."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "Environment creation Date(UTC)."]
    #[serde(
        rename = "environmentCreationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub environment_creation_date: Option<time::OffsetDateTime>,
    #[doc = "Release environment definition ID."]
    #[serde(
        rename = "environmentDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_definition_id: Option<i32>,
    #[doc = "Release environment definition name."]
    #[serde(
        rename = "environmentDefinitionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_definition_name: Option<String>,
    #[doc = "Release environment ID."]
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[doc = "Release environment name."]
    #[serde(
        rename = "environmentName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_name: Option<String>,
    #[doc = "Release ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Release name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ReleaseReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a repository's webhook returned from a source provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryWebhook {
    #[doc = "The friendly name of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub types: Vec<serde_json::Value>,
    #[doc = "The URL of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl RepositoryWebhook {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryWebhookList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RepositoryWebhook>,
}
impl RepositoryWebhookList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ResourceRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceRef>,
}
impl ResourceRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "An alias to be used when referencing the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A valid retention lease prevents automated systems from deleting a pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionLease {
    #[doc = "When the lease was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The pipeline definition of the run."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "The unique identifier for this lease."]
    #[serde(rename = "leaseId", default, skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<i32>,
    #[doc = "Non-unique string that identifies the owner of a retention lease."]
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[doc = "If set, this lease will also prevent the pipeline from being deleted while the lease is still valid."]
    #[serde(
        rename = "protectPipeline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_pipeline: Option<bool>,
    #[doc = "The pipeline run protected by this lease."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
    #[doc = "The last day the lease is considered valid."]
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_until: Option<time::OffsetDateTime>,
}
impl RetentionLease {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionLeaseList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RetentionLease>,
}
impl RetentionLeaseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An update to the retention parameters of a retention lease."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionLeaseUpdate {
    #[doc = "The number of days to consider the lease valid. A retention lease valid for more than 100 years (36500 days) will display as retaining the build \"forever\"."]
    #[serde(rename = "daysValid", default, skip_serializing_if = "Option::is_none")]
    pub days_valid: Option<i32>,
    #[doc = "If set, this lease will also prevent the pipeline from being deleted while the lease is still valid."]
    #[serde(
        rename = "protectPipeline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_pipeline: Option<bool>,
}
impl RetentionLeaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a retention policy for a build definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionPolicy {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<String>,
    #[serde(
        rename = "artifactTypesToDelete",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifact_types_to_delete: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branches: Vec<String>,
    #[doc = "The number of days to keep builds."]
    #[serde(
        rename = "daysToKeep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_keep: Option<i32>,
    #[doc = "Indicates whether the build record itself should be deleted."]
    #[serde(
        rename = "deleteBuildRecord",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_build_record: Option<bool>,
    #[doc = "Indicates whether to delete test results associated with the build."]
    #[serde(
        rename = "deleteTestResults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_test_results: Option<bool>,
    #[doc = "The minimum number of builds to keep."]
    #[serde(
        rename = "minimumToKeep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_to_keep: Option<i32>,
}
impl RetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the minimum, maximum, and current value for a retention setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}
impl RetentionSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[serde(
        rename = "branchFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_filters: Vec<String>,
    #[doc = "Days for a build (flags enum for days of the week)"]
    #[serde(
        rename = "daysToBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_build: Option<schedule::DaysToBuild>,
    #[doc = "The Job Id of the Scheduled job that will queue the scheduled build. Since a single trigger can have multiple schedules and we want a single job to process a single schedule (since each schedule has a list of branches to build), the schedule itself needs to define the Job Id. This value will be filled in when a definition is added or updated.  The UI does not provide it or use it."]
    #[serde(
        rename = "scheduleJobId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_job_id: Option<String>,
    #[doc = "Flag to determine if this schedule should only build if the associated source has been changed."]
    #[serde(
        rename = "scheduleOnlyWithChanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_only_with_changes: Option<bool>,
    #[doc = "Local timezone hour to start"]
    #[serde(
        rename = "startHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_hours: Option<i32>,
    #[doc = "Local timezone minute to start"]
    #[serde(
        rename = "startMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_minutes: Option<i32>,
    #[doc = "Time zone of the build schedule (String representation of the time zone ID)"]
    #[serde(
        rename = "timeZoneId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_zone_id: Option<String>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod schedule {
    use super::*;
    #[doc = "Days for a build (flags enum for days of the week)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DaysToBuild {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "monday")]
        Monday,
        #[serde(rename = "tuesday")]
        Tuesday,
        #[serde(rename = "wednesday")]
        Wednesday,
        #[serde(rename = "thursday")]
        Thursday,
        #[serde(rename = "friday")]
        Friday,
        #[serde(rename = "saturday")]
        Saturday,
        #[serde(rename = "sunday")]
        Sunday,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "Represents a schedule trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleTrigger {
    #[serde(flatten)]
    pub build_trigger: BuildTrigger,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schedules: Vec<Schedule>,
}
impl ScheduleTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a secure file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureFileReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The ID of the secure file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SecureFileReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a phase target that runs on the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTarget {
    #[serde(flatten)]
    pub phase_target: PhaseTarget,
    #[doc = "Represents options for running a phase on the server."]
    #[serde(
        rename = "executionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_options: Option<ServerTargetExecutionOptions>,
}
impl ServerTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents options for running a phase on the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTargetExecutionOptions {
    #[doc = "The type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
}
impl ServerTargetExecutionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a referenec to a service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceEndpointReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The ID of the service endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ServiceEndpointReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceProviderAttributes {
    #[doc = "The name of the source provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The capabilities supported by this source provider."]
    #[serde(
        rename = "supportedCapabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_capabilities: Option<serde_json::Value>,
    #[doc = "The types of triggers supported by this source provider."]
    #[serde(
        rename = "supportedTriggers",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_triggers: Vec<SupportedTrigger>,
}
impl SourceProviderAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceProviderAttributesList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SourceProviderAttributes>,
}
impl SourceProviderAttributesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a work item related to some source item. These are retrieved from Source Providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRelatedWorkItem {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<IdentityRef>,
    #[doc = "Current state of the work item, e.g. Active, Resolved, Closed, etc."]
    #[serde(
        rename = "currentState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_state: Option<String>,
    #[doc = "Long description for the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Unique identifier for the work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the provider the work item is associated with."]
    #[serde(
        rename = "providerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
    #[doc = "Short name for the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Type of work item, e.g. Bug, Task, User Story, etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl SourceRelatedWorkItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of repositories returned from the source provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRepositories {
    #[doc = "A token used to continue this paged request; 'null' if the request is complete"]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "The number of repositories requested for each page"]
    #[serde(
        rename = "pageLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub page_length: Option<i32>,
    #[doc = "A list of repositories"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub repositories: Vec<SourceRepository>,
    #[doc = "The total number of pages, or '-1' if unknown"]
    #[serde(
        rename = "totalPageCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_page_count: Option<i32>,
}
impl SourceRepositories {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a repository returned from a source provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRepository {
    #[doc = "The name of the default branch."]
    #[serde(
        rename = "defaultBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_branch: Option<String>,
    #[doc = "The full name of the repository."]
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The ID of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The friendly name of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The name of the source provider the repository is from."]
    #[serde(
        rename = "sourceProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_provider_name: Option<String>,
    #[doc = "The URL of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl SourceRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an item in a repository from a source provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRepositoryItem {
    #[doc = "Whether the item is able to have sub-items (e.g., is a folder)."]
    #[serde(
        rename = "isContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_container: Option<bool>,
    #[doc = "The full path of the item, relative to the root of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The type of the item (folder, file, etc)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The URL of the item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl SourceRepositoryItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRepositoryItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SourceRepositoryItem>,
}
impl SourceRepositoryItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stage in pipeline"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageReference {
    #[doc = "Attempt number of stage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Name of the stage. Maximum supported length for name is 256 character."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}
impl StageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedTrigger {
    #[doc = "The default interval to wait between polls (only relevant when NotificationType is Polling)."]
    #[serde(
        rename = "defaultPollingInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_polling_interval: Option<i32>,
    #[doc = "How the trigger is notified of changes."]
    #[serde(
        rename = "notificationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_type: Option<String>,
    #[doc = "The capabilities supported by this trigger."]
    #[serde(
        rename = "supportedCapabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_capabilities: Option<serde_json::Value>,
    #[doc = "The type of trigger."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<supported_trigger::Type>,
}
impl SupportedTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod supported_trigger {
    use super::*;
    #[doc = "The type of trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "batchedContinuousIntegration")]
        BatchedContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "gatedCheckIn")]
        GatedCheckIn,
        #[serde(rename = "batchedGatedCheckIn")]
        BatchedGatedCheckIn,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "Represents a Subversion mapping entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SvnMappingDetails {
    #[doc = "The depth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    #[doc = "Indicates whether to ignore externals."]
    #[serde(
        rename = "ignoreExternals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_externals: Option<bool>,
    #[doc = "The local path."]
    #[serde(rename = "localPath", default, skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    #[doc = "The revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[doc = "The server path."]
    #[serde(
        rename = "serverPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_path: Option<String>,
}
impl SvnMappingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a subversion workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SvnWorkspace {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<SvnMappingDetails>,
}
impl SvnWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to an agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskAgentPoolReference {
    #[doc = "The pool ID."]
    pub id: i32,
    #[doc = "A value indicating whether or not this pool is managed by the service."]
    #[serde(rename = "isHosted", default, skip_serializing_if = "Option::is_none")]
    pub is_hosted: Option<bool>,
    #[doc = "The pool name."]
    pub name: String,
}
impl TaskAgentPoolReference {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            is_hosted: None,
            name,
        }
    }
}
#[doc = "A reference to a task definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskDefinitionReference {
    #[doc = "The type of task (task or task group)."]
    #[serde(
        rename = "definitionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_type: Option<String>,
    #[doc = "The ID of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The version of the task."]
    #[serde(
        rename = "versionSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_spec: Option<String>,
}
impl TaskDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskInputDefinitionBase {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aliases: Vec<String>,
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(
        rename = "helpMarkDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub help_mark_down: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<TaskInputValidation>,
    #[serde(
        rename = "visibleRule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visible_rule: Option<String>,
}
impl TaskInputDefinitionBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskInputValidation {
    #[doc = "Conditional expression"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "Message explaining how user can correct if validation fails"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl TaskInputValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a plan group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationPlanGroupReference {
    #[doc = "The name of the plan group."]
    #[serde(rename = "planGroup", default, skip_serializing_if = "Option::is_none")]
    pub plan_group: Option<String>,
    #[doc = "The project ID."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl TaskOrchestrationPlanGroupReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationPlanGroupsStartedEvent {
    #[serde(
        rename = "planGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plan_groups: Vec<TaskOrchestrationPlanGroupReference>,
}
impl TaskOrchestrationPlanGroupsStartedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to an orchestration plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskOrchestrationPlanReference {
    #[doc = "The type of the plan."]
    #[serde(
        rename = "orchestrationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orchestration_type: Option<i32>,
    #[doc = "The ID of the plan."]
    #[serde(rename = "planId")]
    pub plan_id: String,
}
impl TaskOrchestrationPlanReference {
    pub fn new(plan_id: String) -> Self {
        Self {
            orchestration_type: None,
            plan_id,
        }
    }
}
#[doc = "Represents a reference to a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskReference {
    #[doc = "The ID of the task definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the task definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the task definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl TaskReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskSourceDefinitionBase {
    #[serde(rename = "authKey", default, skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(
        rename = "keySelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl TaskSourceDefinitionBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a shallow reference to a TeamProject."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProjectReference {
    #[doc = "Project abbreviation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[doc = "Url to default team identity image."]
    #[serde(
        rename = "defaultTeamImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team_image_url: Option<String>,
    #[doc = "The project's description (if any)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Project identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Project last update time."]
    #[serde(
        rename = "lastUpdateTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "Project name."]
    pub name: String,
    #[doc = "Project revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[doc = "Project state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<team_project_reference::State>,
    #[doc = "Url to the full version of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Project visibility."]
    pub visibility: team_project_reference::Visibility,
}
impl TeamProjectReference {
    pub fn new(name: String, visibility: team_project_reference::Visibility) -> Self {
        Self {
            abbreviation: None,
            default_team_image_url: None,
            description: None,
            id: None,
            last_update_time: None,
            name,
            revision: None,
            state: None,
            url: None,
            visibility,
        }
    }
}
pub mod team_project_reference {
    use super::*;
    #[doc = "Project state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "wellFormed")]
        WellFormed,
        #[serde(rename = "createPending")]
        CreatePending,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "unchanged")]
        Unchanged,
        #[serde(rename = "deleted")]
        Deleted,
    }
    #[doc = "Project visibility."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Visibility {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "organization")]
        Organization,
        #[serde(rename = "unchanged")]
        Unchanged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsContext {
    #[doc = "Represents a reference to a build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildReference>,
    #[serde(
        rename = "contextType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_type: Option<test_results_context::ContextType>,
    #[doc = "Pipeline reference"]
    #[serde(
        rename = "pipelineReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pipeline_reference: Option<PipelineReference>,
    #[doc = "Reference to a release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseReference>,
}
impl TestResultsContext {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_results_context {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ContextType {
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "release")]
        Release,
        #[serde(rename = "pipeline")]
        Pipeline,
    }
}
#[doc = "Represents the timeline of a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Timeline {
    #[serde(flatten)]
    pub timeline_reference: TimelineReference,
    #[doc = "The process or person that last changed the timeline."]
    #[serde(
        rename = "lastChangedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_changed_by: Option<String>,
    #[doc = "The time the timeline was last changed."]
    #[serde(
        rename = "lastChangedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_on: Option<time::OffsetDateTime>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub records: Vec<TimelineRecord>,
}
impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineAttempt {
    #[doc = "Gets or sets the attempt of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Gets or sets the record identifier located within the specified timeline."]
    #[serde(rename = "recordId", default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[doc = "Gets or sets the timeline identifier which owns the record representing this attempt."]
    #[serde(
        rename = "timelineId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_id: Option<String>,
}
impl TimelineAttempt {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an entry in a build's timeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineRecord {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Attempt number of record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "The change ID."]
    #[serde(rename = "changeId", default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<i32>,
    #[doc = "A string that indicates the current operation."]
    #[serde(
        rename = "currentOperation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_operation: Option<String>,
    #[doc = "Represents a reference to a timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<TimelineReference>,
    #[doc = "The number of errors produced by this operation."]
    #[serde(
        rename = "errorCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_count: Option<i32>,
    #[doc = "The finish time."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The ID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "String identifier that is consistent across attempts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub issues: Vec<Issue>,
    #[doc = "The time the record was last modified."]
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "Represents a reference to a build log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<BuildLogReference>,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An ordinal value relative to other records."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "The ID of the record's parent."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The current completion percentage."]
    #[serde(
        rename = "percentComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percent_complete: Option<i32>,
    #[serde(
        rename = "previousAttempts",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub previous_attempts: Vec<TimelineAttempt>,
    #[doc = "The queue ID of the queue that the operation ran on."]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[doc = "The result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<timeline_record::Result>,
    #[doc = "The result code."]
    #[serde(
        rename = "resultCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_code: Option<String>,
    #[doc = "The start time."]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The state of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<timeline_record::State>,
    #[doc = "Represents a reference to a task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskReference>,
    #[doc = "The type of the record."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The REST URL of the timeline record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The number of warnings produced by this operation."]
    #[serde(
        rename = "warningCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_count: Option<i32>,
    #[doc = "The name of the agent running the operation."]
    #[serde(
        rename = "workerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_name: Option<String>,
}
impl TimelineRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod timeline_record {
    use super::*;
    #[doc = "The result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "succeededWithIssues")]
        SucceededWithIssues,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
    #[doc = "The state of the record."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineRecordsUpdatedEvent {
    #[serde(flatten)]
    pub realtime_build_event: RealtimeBuildEvent,
    #[serde(
        rename = "timelineRecords",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub timeline_records: Vec<TimelineRecord>,
}
impl TimelineRecordsUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a timeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimelineReference {
    #[doc = "The change ID."]
    #[serde(rename = "changeId", default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<i32>,
    #[doc = "The ID of the timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The REST URL of the timeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TimelineReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains members for updating the retention settings values. All fields are optional."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProjectRetentionSettingModel {
    #[serde(
        rename = "artifactsRetention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifacts_retention: Option<UpdateRetentionSettingModel>,
    #[serde(
        rename = "pullRequestRunRetention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_run_retention: Option<UpdateRetentionSettingModel>,
    #[serde(
        rename = "retainRunsPerProtectedBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retain_runs_per_protected_branch: Option<UpdateRetentionSettingModel>,
    #[serde(
        rename = "runRetention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_retention: Option<UpdateRetentionSettingModel>,
}
impl UpdateProjectRetentionSettingModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateRetentionSettingModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}
impl UpdateRetentionSettingModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateStageParameters {
    #[serde(
        rename = "forceRetryAllJobs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_retry_all_jobs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<update_stage_parameters::State>,
}
impl UpdateStageParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_stage_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "cancel")]
        Cancel,
        #[serde(rename = "retry")]
        Retry,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateTagParameters {
    #[serde(
        rename = "tagsToAdd",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags_to_add: Vec<String>,
    #[serde(
        rename = "tagsToRemove",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags_to_remove: Vec<String>,
}
impl UpdateTagParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a variable group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroup {
    #[serde(flatten)]
    pub variable_group_reference: VariableGroupReference,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the variable group."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl VariableGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a variable group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroupReference {
    #[doc = "The Name of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[doc = "The ID of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl VariableGroupReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents options for running a phase based on values specified by a list of variables."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableMultipliersAgentExecutionOptions {
    #[serde(flatten)]
    pub agent_target_execution_options: AgentTargetExecutionOptions,
    #[doc = "Indicates whether failure on one agent should prevent the phase from running on other agents."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "The maximum number of agents to use in parallel."]
    #[serde(
        rename = "maxConcurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrency: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub multipliers: Vec<String>,
}
impl VariableMultipliersAgentExecutionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents options for running a phase based on values specified by a list of variables."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableMultipliersServerExecutionOptions {
    #[serde(flatten)]
    pub server_target_execution_options: ServerTargetExecutionOptions,
    #[doc = "Indicates whether failure of one job should prevent the phase from running in other jobs."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "The maximum number of server jobs to run in parallel."]
    #[serde(
        rename = "maxConcurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrency: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub multipliers: Vec<String>,
}
impl VariableMultipliersServerExecutionOptions {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiConnectedServiceRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WebApiConnectedServiceRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mapping for a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceMapping {
    #[doc = "Uri of the associated definition"]
    #[serde(
        rename = "definitionUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_uri: Option<String>,
    #[doc = "Depth of this mapping"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    #[doc = "local location of the definition"]
    #[serde(rename = "localItem", default, skip_serializing_if = "Option::is_none")]
    pub local_item: Option<String>,
    #[doc = "type of workspace mapping"]
    #[serde(
        rename = "mappingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mapping_type: Option<workspace_mapping::MappingType>,
    #[doc = "Server location of the definition"]
    #[serde(
        rename = "serverItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_item: Option<String>,
    #[doc = "Id of the workspace"]
    #[serde(
        rename = "workspaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workspace_id: Option<i32>,
}
impl WorkspaceMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_mapping {
    use super::*;
    #[doc = "type of workspace mapping"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MappingType {
        #[serde(rename = "map")]
        Map,
        #[serde(rename = "cloak")]
        Cloak,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceTemplate {
    #[doc = "Uri of the associated definition"]
    #[serde(
        rename = "definitionUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_uri: Option<String>,
    #[doc = "The identity that last modified this template"]
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<String>,
    #[doc = "The last time this template was modified"]
    #[serde(
        rename = "lastModifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_date: Option<time::OffsetDateTime>,
    #[doc = "List of workspace mappings"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<WorkspaceMapping>,
    #[doc = "Id of the workspace for this template"]
    #[serde(
        rename = "workspaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workspace_id: Option<i32>,
}
impl WorkspaceTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct XamlBuildControllerReference {
    #[doc = "Id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the linked resource (definition name, controller name, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl XamlBuildControllerReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XamlBuildDefinition {
    #[serde(flatten)]
    pub definition_reference: DefinitionReference,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Batch size of the definition"]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "buildArgs", default, skip_serializing_if = "Option::is_none")]
    pub build_args: Option<String>,
    #[doc = "The continuous integration quiet period"]
    #[serde(
        rename = "continuousIntegrationQuietPeriod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuous_integration_quiet_period: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<BuildController>,
    #[doc = "The date this definition was created"]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Default drop location for builds from this definition"]
    #[serde(
        rename = "defaultDropLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_drop_location: Option<String>,
    #[doc = "Description of the definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastBuild", default, skip_serializing_if = "Option::is_none")]
    pub last_build: Option<XamlBuildReference>,
    #[doc = "Represents a repository used by a build definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<BuildRepository>,
    #[doc = "The reasons supported by the template"]
    #[serde(
        rename = "supportedReasons",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_reasons: Option<xaml_build_definition::SupportedReasons>,
    #[doc = "How builds are triggered from this definition"]
    #[serde(
        rename = "triggerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_type: Option<xaml_build_definition::TriggerType>,
}
impl XamlBuildDefinition {
    pub fn new(definition_reference: DefinitionReference) -> Self {
        Self {
            definition_reference,
            links: None,
            batch_size: None,
            build_args: None,
            continuous_integration_quiet_period: None,
            controller: None,
            created_on: None,
            default_drop_location: None,
            description: None,
            last_build: None,
            repository: None,
            supported_reasons: None,
            trigger_type: None,
        }
    }
}
pub mod xaml_build_definition {
    use super::*;
    #[doc = "The reasons supported by the template"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SupportedReasons {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "individualCI")]
        IndividualCi,
        #[serde(rename = "batchedCI")]
        BatchedCi,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "scheduleForced")]
        ScheduleForced,
        #[serde(rename = "userCreated")]
        UserCreated,
        #[serde(rename = "validateShelveset")]
        ValidateShelveset,
        #[serde(rename = "checkInShelveset")]
        CheckInShelveset,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "resourceTrigger")]
        ResourceTrigger,
        #[serde(rename = "triggered")]
        Triggered,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "How builds are triggered from this definition"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TriggerType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "batchedContinuousIntegration")]
        BatchedContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "gatedCheckIn")]
        GatedCheckIn,
        #[serde(rename = "batchedGatedCheckIn")]
        BatchedGatedCheckIn,
        #[serde(rename = "pullRequest")]
        PullRequest,
        #[serde(rename = "buildCompletion")]
        BuildCompletion,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct XamlBuildReference {
    #[doc = "Id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the linked resource (definition name, controller name, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl XamlBuildReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct XamlBuildServerReference {
    #[doc = "Id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the linked resource (definition name, controller name, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl XamlBuildServerReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct XamlDefinitionReference {
    #[doc = "Id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the linked resource (definition name, controller name, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl XamlDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a yaml build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct YamlBuild {
    #[doc = "The yaml used to define the build"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yaml: Option<String>,
}
impl YamlBuild {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a YAML process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct YamlProcess {
    #[serde(flatten)]
    pub build_process: BuildProcess,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<String>,
    #[doc = "Represents resources used by a build process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BuildProcessResources>,
    #[doc = "The YAML filename."]
    #[serde(
        rename = "yamlFilename",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub yaml_filename: Option<String>,
}
impl YamlProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
