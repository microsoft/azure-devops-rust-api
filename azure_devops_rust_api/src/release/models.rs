// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentArtifactDefinition {
    #[doc = "Gets or sets the artifact definition alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[doc = "Gets or sets the artifact type."]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<agent_artifact_definition::ArtifactType>,
    #[doc = "Gets or sets the artifact definition details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "Gets or sets the name of artifact definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the version of artifact definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl AgentArtifactDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_artifact_definition {
    use super::*;
    #[doc = "Gets or sets the artifact type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ArtifactType {
        #[serde(rename = "xamlBuild")]
        XamlBuild,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "jenkins")]
        Jenkins,
        #[serde(rename = "fileShare")]
        FileShare,
        #[serde(rename = "nuget")]
        Nuget,
        #[serde(rename = "tfsOnPrem")]
        TfsOnPrem,
        #[serde(rename = "gitHub")]
        GitHub,
        #[serde(rename = "tfGit")]
        TfGit,
        #[serde(rename = "externalTfsBuild")]
        ExternalTfsBuild,
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "tfvc")]
        Tfvc,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentBasedDeployPhase {
    #[serde(flatten)]
    pub deploy_phase: DeployPhase,
    #[serde(
        rename = "deploymentInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_input: Option<AgentDeploymentInput>,
}
impl AgentBasedDeployPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentDeploymentInput {
    #[serde(flatten)]
    pub deployment_input: DeploymentInput,
    #[doc = "Specification of the agent defined by the pool provider."]
    #[serde(
        rename = "agentSpecification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_specification: Option<AgentSpecification>,
    #[doc = "Gets or sets the image ID."]
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<i32>,
    #[serde(
        rename = "parallelExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parallel_execution: Option<ExecutionInput>,
}
impl AgentDeploymentInput {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalOptions {
    #[doc = "Specify whether the approval can be skipped if the same approver approved the previous stage."]
    #[serde(
        rename = "autoTriggeredAndPreviousEnvironmentApprovedCanBeSkipped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_triggered_and_previous_environment_approved_can_be_skipped: Option<bool>,
    #[doc = "Specify whether revalidate identity of approver before completing the approval."]
    #[serde(
        rename = "enforceIdentityRevalidation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_identity_revalidation: Option<bool>,
    #[doc = "Approvals execution order."]
    #[serde(
        rename = "executionOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_order: Option<approval_options::ExecutionOrder>,
    #[doc = "Specify whether the user requesting a release or deployment should allow to approver."]
    #[serde(
        rename = "releaseCreatorCanBeApprover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_creator_can_be_approver: Option<bool>,
    #[doc = "The number of approvals required to move release forward. '0' means all approvals required."]
    #[serde(
        rename = "requiredApproverCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_approver_count: Option<i32>,
    #[doc = "Approval timeout. Approval default timeout is 30 days. Maximum allowed timeout is 365 days. '0' means default timeout i.e 30 days."]
    #[serde(
        rename = "timeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_in_minutes: Option<i32>,
}
impl ApprovalOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval_options {
    use super::*;
    #[doc = "Approvals execution order."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionOrder {
        #[serde(rename = "beforeGates")]
        BeforeGates,
        #[serde(rename = "afterSuccessfulGates")]
        AfterSuccessfulGates,
        #[serde(rename = "afterGatesAlways")]
        AfterGatesAlways,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Artifact {
    #[doc = "Gets or sets alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[doc = "Gets or sets definition reference. e.g. {\"project\":{\"id\":\"fed755ea-49c5-4399-acea-fd5b5aa90a6c\",\"name\":\"myProject\"},\"definition\":{\"id\":\"1\",\"name\":\"mybuildDefinition\"},\"connection\":{\"id\":\"1\",\"name\":\"myConnection\"}}."]
    #[serde(
        rename = "definitionReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_reference: Option<serde_json::Value>,
    #[doc = "Indicates whether artifact is primary or not."]
    #[serde(rename = "isPrimary", default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[doc = "Indicates whether artifact is retained by release or not."]
    #[serde(
        rename = "isRetained",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_retained: Option<bool>,
    #[doc = "Gets or sets type. It can have value as 'Build', 'Jenkins', 'GitHub', 'Nuget', 'Team Build (external)', 'ExternalTFSBuild', 'Git', 'TFVC', 'ExternalTfsXamlBuild'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Artifact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactContributionDefinition {
    #[serde(
        rename = "artifactTriggerConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_trigger_configuration: Option<ArtifactTriggerConfiguration>,
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[serde(
        rename = "artifactTypeStreamMapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type_stream_mapping: Option<serde_json::Value>,
    #[serde(
        rename = "browsableArtifactTypeMapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub browsable_artifact_type_mapping: Option<serde_json::Value>,
    #[serde(
        rename = "dataSourceBindings",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_source_bindings: Vec<DataSourceBinding>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        rename = "downloadTaskId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_task_id: Option<String>,
    #[serde(
        rename = "endpointTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_type_id: Option<String>,
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[serde(
        rename = "isCommitsTraceabilitySupported",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_commits_traceability_supported: Option<bool>,
    #[serde(
        rename = "isWorkitemsTraceabilitySupported",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_workitems_traceability_supported: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "taskInputMapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_input_mapping: Option<serde_json::Value>,
    #[serde(
        rename = "uniqueSourceIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_source_identifier: Option<String>,
}
impl ArtifactContributionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactDownloadInputBase {
    #[doc = "Gets or sets the alias of artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[doc = "Gets or sets the name of artifact definition. Valid values are 'Skip', 'Selective', 'All'."]
    #[serde(
        rename = "artifactDownloadMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_download_mode: Option<String>,
    #[doc = "Gets or sets the artifact items of the input."]
    #[serde(
        rename = "artifactItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifact_items: Vec<String>,
    #[doc = "Gets or sets the type of artifact."]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
}
impl ArtifactDownloadInputBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactFilter {
    #[doc = "Gets or sets whether a release should be created on build tagging."]
    #[serde(
        rename = "createReleaseOnBuildTagging",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_release_on_build_tagging: Option<bool>,
    #[doc = "Gets or sets the branch for the filter."]
    #[serde(
        rename = "sourceBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_branch: Option<String>,
    #[serde(rename = "tagFilter", default, skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    #[doc = "Gets or sets the list of tags for the filter."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Gets or sets whether filter should default to build definition branch."]
    #[serde(
        rename = "useBuildDefinitionBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_build_definition_branch: Option<bool>,
}
impl ArtifactFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactInstanceData {
    #[serde(
        rename = "accountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    #[serde(
        rename = "authenticationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_token: Option<String>,
    #[serde(rename = "tfsUrl", default, skip_serializing_if = "Option::is_none")]
    pub tfs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ArtifactInstanceData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactMetadata {
    #[doc = "Sets alias of artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(
        rename = "instanceReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_reference: Option<BuildVersion>,
}
impl ArtifactMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactProvider {
    #[doc = "Gets or sets the id of artifact provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets the name of artifact provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the link of artifact provider."]
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[doc = "Gets or sets the version of artifact provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ArtifactProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceId {
    #[doc = "Gets or sets the artifact type of artifact source."]
    #[serde(
        rename = "artifactTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type_id: Option<String>,
    #[doc = "Gets or sets the list of sourceIdInput of artifact source."]
    #[serde(
        rename = "sourceIdInputs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_id_inputs: Vec<SourceIdInput>,
}
impl ArtifactSourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceIdsQueryResult {
    #[doc = "Gets or sets the list of artifactsourceIds."]
    #[serde(
        rename = "artifactSourceIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifact_source_ids: Vec<ArtifactSourceId>,
}
impl ArtifactSourceIdsQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceReference {
    #[doc = "ID of the artifact source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the artifact source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ArtifactSourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceTrigger {
    #[serde(flatten)]
    pub release_trigger_base: ReleaseTriggerBase,
    #[doc = "Artifact source alias for Artifact Source trigger type"]
    #[serde(
        rename = "artifactAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_alias: Option<String>,
    #[serde(
        rename = "triggerConditions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trigger_conditions: Vec<ArtifactFilter>,
}
impl ArtifactSourceTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactTriggerConfiguration {
    #[doc = "Gets or sets the whether trigger is supported or not."]
    #[serde(
        rename = "isTriggerSupported",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_trigger_supported: Option<bool>,
    #[doc = "Gets or sets the whether trigger is supported only on hosted environment."]
    #[serde(
        rename = "isTriggerSupportedOnlyInHosted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_trigger_supported_only_in_hosted: Option<bool>,
    #[doc = "Gets or sets the whether webhook is supported at server level."]
    #[serde(
        rename = "isWebhookSupportedAtServerLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_webhook_supported_at_server_level: Option<bool>,
    #[doc = "Gets or sets the payload hash header name for the artifact trigger configuration."]
    #[serde(
        rename = "payloadHashHeaderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub payload_hash_header_name: Option<String>,
    #[doc = "Gets or sets the resources for artifact trigger configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<serde_json::Value>,
    #[doc = "Gets or sets the webhook payload mapping for artifact trigger configuration."]
    #[serde(
        rename = "webhookPayloadMapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_payload_mapping: Option<serde_json::Value>,
}
impl ArtifactTriggerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactTypeDefinition {
    #[serde(
        rename = "artifactTriggerConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_trigger_configuration: Option<ArtifactTriggerConfiguration>,
    #[doc = "Gets or sets the artifact type of artifact type definition. Valid values are 'Build', 'Package', 'Source' or 'ContainerImage'."]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[doc = "Gets or sets the display name of artifact type definition."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the endpoint type id of artifact type definition."]
    #[serde(
        rename = "endpointTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_type_id: Option<String>,
    #[doc = "Gets or sets the input descriptors of artifact type definition."]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "Gets or sets the is commits tracebility supported value of artifact type defintion."]
    #[serde(
        rename = "isCommitsTraceabilitySupported",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_commits_traceability_supported: Option<bool>,
    #[doc = "Gets or sets the is workitems tracebility supported value of artifact type defintion."]
    #[serde(
        rename = "isWorkitemsTraceabilitySupported",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_workitems_traceability_supported: Option<bool>,
    #[doc = "Gets or sets the name of artifact type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the unique source identifier of artifact type definition."]
    #[serde(
        rename = "uniqueSourceIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_source_identifier: Option<String>,
}
impl ArtifactTypeDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactVersion {
    #[doc = "Gets or sets the alias of artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(
        rename = "defaultVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_version: Option<BuildVersion>,
    #[doc = "Gets or sets the error message encountered during querying of versions for artifact."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Gets or sets the list of build versions of artifact."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub versions: Vec<BuildVersion>,
}
impl ArtifactVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactVersionQueryResult {
    #[doc = "Gets or sets the list for artifact versions of artifact version query result."]
    #[serde(
        rename = "artifactVersions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifact_versions: Vec<ArtifactVersion>,
}
impl ArtifactVersionQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactsDownloadInput {
    #[serde(
        rename = "downloadInputs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub download_inputs: Vec<ArtifactDownloadInputBase>,
}
impl ArtifactsDownloadInput {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoTriggerIssue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<Issue>,
    #[serde(
        rename = "issueSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_source: Option<auto_trigger_issue::IssueSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(
        rename = "releaseDefinitionReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition_reference: Option<ReleaseDefinitionShallowReference>,
    #[serde(
        rename = "releaseTriggerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_trigger_type: Option<auto_trigger_issue::ReleaseTriggerType>,
}
impl AutoTriggerIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_trigger_issue {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IssueSource {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReleaseTriggerType {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "artifactSource")]
        ArtifactSource,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "sourceRepo")]
        SourceRepo,
        #[serde(rename = "containerImage")]
        ContainerImage,
        #[serde(rename = "package")]
        Package,
        #[serde(rename = "pullRequest")]
        PullRequest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultVariableGroupProviderData {
    #[doc = "Gets or sets last refreshed time."]
    #[serde(
        rename = "lastRefreshedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_refreshed_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the service endpoint ID."]
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
    #[doc = "Gets or sets the vault name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<String>,
}
impl AzureKeyVaultVariableGroupProviderData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureKeyVaultVariableValue {
    #[serde(flatten)]
    pub variable_value: VariableValue,
    #[doc = "Gets or sets the content type of key vault variable value."]
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[doc = "Indicates the vault variable value enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets or sets the expire time of key vault variable value."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub expires: Option<time::OffsetDateTime>,
}
impl AzureKeyVaultVariableValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseDeploymentInput {
    #[doc = "Gets or sets the job condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Gets or sets the job cancel timeout in minutes for deployment which are cancelled by user for this release environment."]
    #[serde(
        rename = "jobCancelTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_cancel_timeout_in_minutes: Option<i32>,
    #[doc = "Gets or sets the override inputs."]
    #[serde(
        rename = "overrideInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub override_inputs: Option<serde_json::Value>,
    #[doc = "Gets or sets the job execution timeout in minutes for deployment which are queued against this release environment."]
    #[serde(
        rename = "timeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_in_minutes: Option<i32>,
}
impl BaseDeploymentInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildArtifactDownloadInput {
    #[serde(flatten)]
    pub artifact_download_input_base: ArtifactDownloadInputBase,
}
impl BuildArtifactDownloadInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildVersion {
    #[doc = "Gets or sets the commit message for the artifact."]
    #[serde(
        rename = "commitMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commit_message: Option<String>,
    #[doc = "Gets or sets the definition id."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<String>,
    #[doc = "Gets or sets the definition name."]
    #[serde(
        rename = "definitionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_name: Option<String>,
    #[doc = "Gets or sets the build id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets if the artifact supports multiple definitions."]
    #[serde(
        rename = "isMultiDefinitionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_multi_definition_type: Option<bool>,
    #[doc = "Gets or sets the build number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the source branch for the artifact."]
    #[serde(
        rename = "sourceBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_branch: Option<String>,
    #[serde(
        rename = "sourcePullRequestVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_pull_request_version: Option<SourcePullRequestVersion>,
    #[doc = "Gets or sets the repository id for the artifact."]
    #[serde(
        rename = "sourceRepositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_repository_id: Option<String>,
    #[doc = "Gets or sets the repository type for the artifact."]
    #[serde(
        rename = "sourceRepositoryType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_repository_type: Option<String>,
    #[doc = "Gets or sets the source version for the artifact."]
    #[serde(
        rename = "sourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_version: Option<String>,
}
impl BuildVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a change associated with a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Change {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[doc = "The type of source. \"TfsVersionControl\", \"TfsGit\", etc."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<String>,
    #[doc = "The location of a user-friendly representation of the resource."]
    #[serde(
        rename = "displayUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_uri: Option<String>,
    #[doc = "Something that identifies the change. For a commit, this would be the SHA1. For a TFVC changeset, this would be the changeset id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The location of the full representation of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "A description of the change. This might be a commit message or changeset description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "pushedBy", default, skip_serializing_if = "Option::is_none")]
    pub pushed_by: Option<IdentityRef>,
    #[doc = "A timestamp for the change."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl Change {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeRepositoryReference {
    #[doc = "Gets and sets the repository references."]
    #[serde(
        rename = "repositoryReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_reference: Option<serde_json::Value>,
    #[doc = "It can have value as ‘GitHub’, ‘Vsts’."]
    #[serde(
        rename = "systemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_type: Option<code_repository_reference::SystemType>,
}
impl CodeRepositoryReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod code_repository_reference {
    use super::*;
    #[doc = "It can have value as ‘GitHub’, ‘Vsts’."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SystemType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "tfsGit")]
        TfsGit,
        #[serde(rename = "gitHub")]
        GitHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceSettings {
    #[doc = "Scan the release definition for secrets"]
    #[serde(
        rename = "checkForCredentialsAndOtherSecrets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_for_credentials_and_other_secrets: Option<bool>,
}
impl ComplianceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    #[doc = "Gets or sets the condition type."]
    #[serde(
        rename = "conditionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub condition_type: Option<condition::ConditionType>,
    #[doc = "Gets or sets the name of the condition. e.g. 'ReleaseStarted'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or set value of the condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Condition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod condition {
    use super::*;
    #[doc = "Gets or sets the condition type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConditionType {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "event")]
        Event,
        #[serde(rename = "environmentState")]
        EnvironmentState,
        #[serde(rename = "artifact")]
        Artifact,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationVariableValue {
    #[doc = "Gets and sets if a variable can be overridden at deployment time or not."]
    #[serde(
        rename = "allowOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_override: Option<bool>,
    #[doc = "Gets or sets as variable is secret or not."]
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[doc = "Gets and sets value of the configuration variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ConfigurationVariableValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Consumer {
    #[doc = "ID of the consumer."]
    #[serde(
        rename = "consumerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_id: Option<i32>,
    #[doc = "Name of the consumer."]
    #[serde(
        rename = "consumerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_name: Option<String>,
}
impl Consumer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerImageTrigger {
    #[serde(flatten)]
    pub release_trigger_base: ReleaseTriggerBase,
    #[doc = "Alias of the trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[doc = "List tag filters applied while trigger."]
    #[serde(
        rename = "tagFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tag_filters: Vec<TagFilter>,
}
impl ContainerImageTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinuousDeploymentTriggerIssue {
    #[serde(flatten)]
    pub auto_trigger_issue: AutoTriggerIssue,
    #[doc = "Artifact type."]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[doc = "ArtifactVersion ID."]
    #[serde(
        rename = "artifactVersionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_version_id: Option<String>,
    #[doc = "Artifact source ID."]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}
impl ContinuousDeploymentTriggerIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlOptions {
    #[doc = "Always run the job."]
    #[serde(rename = "alwaysRun", default, skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    #[doc = "Indicates whether to continue job on error or not."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "Indicates the job enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ControlOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomArtifactDownloadInput {
    #[serde(flatten)]
    pub artifact_download_input_base: ArtifactDownloadInputBase,
}
impl CustomArtifactDownloadInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceBinding {
    #[doc = "Pagination format supported by this data source(ContinuationToken/SkipTop)."]
    #[serde(
        rename = "callbackContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_context_template: Option<String>,
    #[doc = "Subsequent calls needed?"]
    #[serde(
        rename = "callBackRequiredTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub call_back_required_template: Option<String>,
    #[doc = "Name of the datasource."]
    #[serde(
        rename = "dataSourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_name: Option<String>,
    #[doc = "Endpoint ID of the datasource."]
    #[serde(
        rename = "endpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<String>,
    #[doc = "Endpoint URL of the datasource."]
    #[serde(
        rename = "endpointUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_url: Option<String>,
    #[doc = "Defines the initial value of the query params"]
    #[serde(
        rename = "initialContextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_context_template: Option<String>,
    #[doc = "Parameters of the datasource."]
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
    #[doc = "Result selector applied on output of datasource result, for example jsonpath:$.value[?(@.properties.isEnabled == true)]."]
    #[serde(
        rename = "resultSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_selector: Option<String>,
    #[doc = "Format of the return results, for example. { \"Value\" : \"{{{id}}}\", \"DisplayValue\" : \"{{{name}}}\" }."]
    #[serde(
        rename = "resultTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_template: Option<String>,
    #[doc = "Target of the datasource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl DataSourceBinding {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefinitionEnvironmentReference {
    #[doc = "Definition environment ID."]
    #[serde(
        rename = "definitionEnvironmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_environment_id: Option<i32>,
    #[doc = "Definition environment name."]
    #[serde(
        rename = "definitionEnvironmentName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_environment_name: Option<String>,
    #[doc = "ReleaseDefinition ID."]
    #[serde(
        rename = "releaseDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition_id: Option<i32>,
    #[doc = "ReleaseDefinition name."]
    #[serde(
        rename = "releaseDefinitionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition_name: Option<String>,
}
impl DefinitionEnvironmentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Demand {
    #[doc = "Gets and sets the name of demand."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets and sets the value of demand."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Demand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployPhase {
    #[doc = "Gets and sets the name of deploy phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates the deploy phase type."]
    #[serde(rename = "phaseType", default, skip_serializing_if = "Option::is_none")]
    pub phase_type: Option<deploy_phase::PhaseType>,
    #[doc = "Gets and sets the rank of deploy phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[doc = "Gets and sets the reference name of deploy phase."]
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[doc = "Gets and sets the workflow tasks for the deploy phase."]
    #[serde(
        rename = "workflowTasks",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub workflow_tasks: Vec<WorkflowTask>,
}
impl DeployPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deploy_phase {
    use super::*;
    #[doc = "Indicates the deploy phase type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PhaseType {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "agentBasedDeployment")]
        AgentBasedDeployment,
        #[serde(rename = "runOnServer")]
        RunOnServer,
        #[serde(rename = "machineGroupBasedDeployment")]
        MachineGroupBasedDeployment,
        #[serde(rename = "deploymentGates")]
        DeploymentGates,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[doc = "Gets attempt number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Gets the date on which deployment is complete."]
    #[serde(
        rename = "completedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_on: Option<time::OffsetDateTime>,
    #[doc = "Gets the list of condition associated with deployment."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<Condition>,
    #[doc = "Gets release definition environment id."]
    #[serde(
        rename = "definitionEnvironmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_environment_id: Option<i32>,
    #[doc = "Gets status of the deployment."]
    #[serde(
        rename = "deploymentStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_status: Option<deployment::DeploymentStatus>,
    #[doc = "Gets the unique identifier for deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<IdentityRef>,
    #[doc = "Gets the date on which deployment is last modified."]
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets operation status of deployment."]
    #[serde(
        rename = "operationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_status: Option<deployment::OperationStatus>,
    #[doc = "Gets list of PostDeployApprovals."]
    #[serde(
        rename = "postDeployApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub post_deploy_approvals: Vec<ReleaseApproval>,
    #[doc = "Gets list of PreDeployApprovals."]
    #[serde(
        rename = "preDeployApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pre_deploy_approvals: Vec<ReleaseApproval>,
    #[serde(
        rename = "projectReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_reference: Option<ProjectReference>,
    #[doc = "Gets the date on which deployment is queued."]
    #[serde(
        rename = "queuedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queued_on: Option<time::OffsetDateTime>,
    #[doc = "Gets reason of deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<deployment::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseReference>,
    #[serde(
        rename = "releaseDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition: Option<ReleaseDefinitionShallowReference>,
    #[serde(
        rename = "releaseEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_environment: Option<ReleaseEnvironmentShallowReference>,
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
    #[doc = "Gets the date on which deployment is scheduled."]
    #[serde(
        rename = "scheduledDeploymentTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub scheduled_deployment_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the date on which deployment is started."]
    #[serde(
        rename = "startedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_on: Option<time::OffsetDateTime>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment {
    use super::*;
    #[doc = "Gets status of the deployment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeploymentStatus {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notDeployed")]
        NotDeployed,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "Gets operation status of deployment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationStatus {
        Undefined,
        Queued,
        Scheduled,
        Pending,
        Approved,
        Rejected,
        Deferred,
        QueuedForAgent,
        PhaseInProgress,
        PhaseSucceeded,
        PhasePartiallySucceeded,
        PhaseFailed,
        Canceled,
        PhaseCanceled,
        ManualInterventionPending,
        QueuedForPipeline,
        Cancelling,
        EvaluatingGates,
        GateFailed,
        All,
    }
    #[doc = "Gets reason of deployment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "automated")]
        Automated,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "redeployTrigger")]
        RedeployTrigger,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentApprovalCompletedEvent {
    #[serde(flatten)]
    pub deployment_event: DeploymentEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<ReleaseApproval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
}
impl DeploymentApprovalCompletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentApprovalPendingEvent {
    #[serde(flatten)]
    pub deployment_event: DeploymentEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<ReleaseApproval>,
    #[serde(
        rename = "approvalOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_options: Option<ApprovalOptions>,
    #[serde(
        rename = "completedApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub completed_approvals: Vec<ReleaseApproval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[serde(
        rename = "isMultipleRankApproval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_multiple_rank_approval: Option<bool>,
    #[serde(
        rename = "pendingApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pending_approvals: Vec<ReleaseApproval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
}
impl DeploymentApprovalPendingEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentAttempt {
    #[doc = "Deployment attempt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "ID of the deployment."]
    #[serde(
        rename = "deploymentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_id: Option<i32>,
    #[doc = "Specifies whether deployment has started or not."]
    #[serde(
        rename = "hasStarted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_started: Option<bool>,
    #[doc = "ID of deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "All the issues related to the deployment."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub issues: Vec<Issue>,
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<IdentityRef>,
    #[doc = "Time when this deployment last modified."]
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[doc = "Deployment operation status."]
    #[serde(
        rename = "operationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_status: Option<deployment_attempt::OperationStatus>,
    #[serde(
        rename = "postDeploymentGates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_deployment_gates: Option<ReleaseGates>,
    #[serde(
        rename = "preDeploymentGates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_deployment_gates: Option<ReleaseGates>,
    #[doc = "When this deployment queued on."]
    #[serde(
        rename = "queuedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queued_on: Option<time::OffsetDateTime>,
    #[doc = "Reason for the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<deployment_attempt::Reason>,
    #[doc = "List of release deployphases executed in this deployment."]
    #[serde(
        rename = "releaseDeployPhases",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub release_deploy_phases: Vec<ReleaseDeployPhase>,
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
    #[doc = "status of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<deployment_attempt::Status>,
}
impl DeploymentAttempt {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_attempt {
    use super::*;
    #[doc = "Deployment operation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationStatus {
        Undefined,
        Queued,
        Scheduled,
        Pending,
        Approved,
        Rejected,
        Deferred,
        QueuedForAgent,
        PhaseInProgress,
        PhaseSucceeded,
        PhasePartiallySucceeded,
        PhaseFailed,
        Canceled,
        PhaseCanceled,
        ManualInterventionPending,
        QueuedForPipeline,
        Cancelling,
        EvaluatingGates,
        GateFailed,
        All,
    }
    #[doc = "Reason for the deployment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "automated")]
        Automated,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "redeployTrigger")]
        RedeployTrigger,
    }
    #[doc = "status of the deployment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notDeployed")]
        NotDeployed,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentAuthorizationInfo {
    #[doc = "Authorization header type, typically either RevalidateApproverIdentity or OnBehalfOf."]
    #[serde(
        rename = "authorizationHeaderFor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_header_for: Option<deployment_authorization_info::AuthorizationHeaderFor>,
    #[doc = "List of resources."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<String>,
    #[doc = "ID of the tenant."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Access token key."]
    #[serde(
        rename = "vstsAccessTokenKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vsts_access_token_key: Option<String>,
}
impl DeploymentAuthorizationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_authorization_info {
    use super::*;
    #[doc = "Authorization header type, typically either RevalidateApproverIdentity or OnBehalfOf."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthorizationHeaderFor {
        #[serde(rename = "revalidateApproverIdentity")]
        RevalidateApproverIdentity,
        #[serde(rename = "onBehalfOf")]
        OnBehalfOf,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentCompletedEvent {
    #[serde(flatten)]
    pub deployment_event: DeploymentEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ReleaseEnvironment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
}
impl DeploymentCompletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentEvent {
    #[serde(flatten)]
    pub release_event: ReleaseEvent,
    #[serde(rename = "attemptId", default, skip_serializing_if = "Option::is_none")]
    pub attempt_id: Option<i32>,
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}
impl DeploymentEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentInput {
    #[serde(flatten)]
    pub base_deployment_input: BaseDeploymentInput,
    #[serde(
        rename = "artifactsDownloadInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifacts_download_input: Option<ArtifactsDownloadInput>,
    #[doc = "List demands that needs to meet to execute the job."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[doc = "Indicates whether to include access token in deployment job or not."]
    #[serde(
        rename = "enableAccessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_access_token: Option<bool>,
    #[doc = "Id of the pool on which job get executed."]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[doc = "Indicates whether artifacts downloaded while job execution or not."]
    #[serde(
        rename = "skipArtifactsDownload",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_artifacts_download: Option<bool>,
}
impl DeploymentInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<ReleaseTask>,
    #[doc = "List of  executed tasks with in job."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<ReleaseTask>,
}
impl DeploymentJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Deployment>,
}
impl DeploymentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentManualInterventionPendingEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<ReleaseApproval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[serde(
        rename = "emailRecipients",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_recipients: Vec<String>,
    #[serde(
        rename = "environmentOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_owner: Option<IdentityRef>,
    #[serde(
        rename = "manualIntervention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manual_intervention: Option<ManualIntervention>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
}
impl DeploymentManualInterventionPendingEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentQueryParameters {
    #[doc = "Query deployments based specified artifact source id."]
    #[serde(
        rename = "artifactSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_source_id: Option<String>,
    #[doc = "Query deployments based specified artifact type id."]
    #[serde(
        rename = "artifactTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type_id: Option<String>,
    #[doc = "Query deployments based specified artifact versions."]
    #[serde(
        rename = "artifactVersions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifact_versions: Vec<String>,
    #[doc = "Query deployments number of deployments per environment."]
    #[serde(
        rename = "deploymentsPerEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployments_per_environment: Option<i32>,
    #[doc = "Query deployment based on deployment status."]
    #[serde(
        rename = "deploymentStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_status: Option<deployment_query_parameters::DeploymentStatus>,
    #[doc = "Query deployments of specified environments."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environments: Vec<DefinitionEnvironmentReference>,
    #[doc = "Query deployments based specified expands."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<deployment_query_parameters::Expands>,
    #[doc = "Specify deleted deployments should return or not."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Query deployment based on deployment operation status."]
    #[serde(
        rename = "operationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_status: Option<deployment_query_parameters::OperationStatus>,
    #[doc = "Query deployments based query type."]
    #[serde(rename = "queryType", default, skip_serializing_if = "Option::is_none")]
    pub query_type: Option<deployment_query_parameters::QueryType>,
    #[doc = "Query deployments based specified source branch."]
    #[serde(
        rename = "sourceBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_branch: Option<String>,
}
impl DeploymentQueryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_query_parameters {
    use super::*;
    #[doc = "Query deployment based on deployment status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeploymentStatus {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notDeployed")]
        NotDeployed,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "Query deployments based specified expands."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Expands {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "deploymentOnly")]
        DeploymentOnly,
        #[serde(rename = "approvals")]
        Approvals,
        #[serde(rename = "artifacts")]
        Artifacts,
    }
    #[doc = "Query deployment based on deployment operation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationStatus {
        Undefined,
        Queued,
        Scheduled,
        Pending,
        Approved,
        Rejected,
        Deferred,
        QueuedForAgent,
        PhaseInProgress,
        PhaseSucceeded,
        PhasePartiallySucceeded,
        PhaseFailed,
        Canceled,
        PhaseCanceled,
        ManualInterventionPending,
        QueuedForPipeline,
        Cancelling,
        EvaluatingGates,
        GateFailed,
        All,
    }
    #[doc = "Query deployments based query type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryType {
        #[serde(rename = "regular")]
        Regular,
        #[serde(rename = "failingSince")]
        FailingSince,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentStartedEvent {
    #[serde(flatten)]
    pub deployment_event: DeploymentEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ReleaseEnvironment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
}
impl DeploymentStartedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailRecipients {
    #[doc = "List of email addresses."]
    #[serde(
        rename = "emailAddresses",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_addresses: Vec<String>,
    #[doc = "List of TFS IDs guids."]
    #[serde(
        rename = "tfsIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tfs_ids: Vec<String>,
}
impl EmailRecipients {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines policy on environment queuing at Release Management side queue. We will send to Environment Runner [creating pre-deploy and other steps] only when the policies mentioned are satisfied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentExecutionPolicy {
    #[doc = "This policy decides, how many environments would be with Environment Runner."]
    #[serde(
        rename = "concurrencyCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub concurrency_count: Option<i32>,
    #[doc = "Queue depth in the EnvironmentQueue table, this table keeps the environment entries till Environment Runner is free [as per it's policy] to take another environment for running."]
    #[serde(
        rename = "queueDepthCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_depth_count: Option<i32>,
}
impl EnvironmentExecutionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentOptions {
    #[doc = "Gets and sets as the auto link workitems or not."]
    #[serde(
        rename = "autoLinkWorkItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_link_work_items: Option<bool>,
    #[doc = "Gets and sets as the badge enabled or not."]
    #[serde(
        rename = "badgeEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub badge_enabled: Option<bool>,
    #[doc = "Gets and sets as the publish deployment status or not."]
    #[serde(
        rename = "publishDeploymentStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publish_deployment_status: Option<bool>,
    #[doc = "Gets and sets as the.pull request deployment enabled or not."]
    #[serde(
        rename = "pullRequestDeploymentEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_deployment_enabled: Option<bool>,
}
impl EnvironmentOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentRetentionPolicy {
    #[doc = "Gets and sets the number of days to keep environment."]
    #[serde(
        rename = "daysToKeep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_keep: Option<i32>,
    #[doc = "Gets and sets the number of releases to keep."]
    #[serde(
        rename = "releasesToKeep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub releases_to_keep: Option<i32>,
    #[doc = "Gets and sets as the build to be retained or not."]
    #[serde(
        rename = "retainBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retain_build: Option<bool>,
}
impl EnvironmentRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTrigger {
    #[doc = "Definition environment ID on which this trigger applicable."]
    #[serde(
        rename = "definitionEnvironmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_environment_id: Option<i32>,
    #[doc = "ReleaseDefinition ID on which this trigger applicable."]
    #[serde(
        rename = "releaseDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition_id: Option<i32>,
    #[doc = "Gets or sets the trigger content."]
    #[serde(
        rename = "triggerContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_content: Option<String>,
    #[doc = "Gets or sets the trigger type."]
    #[serde(
        rename = "triggerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_type: Option<environment_trigger::TriggerType>,
}
impl EnvironmentTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod environment_trigger {
    use super::*;
    #[doc = "Gets or sets the trigger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TriggerType {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "deploymentGroupRedeploy")]
        DeploymentGroupRedeploy,
        #[serde(rename = "rollbackRedeploy")]
        RollbackRedeploy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTriggerContent {
    #[doc = "Gets or sets action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Gets or sets list of event types."]
    #[serde(
        rename = "eventTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub event_types: Vec<String>,
}
impl EnvironmentTriggerContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExecutionInput {
    #[doc = "Parallel execution type, for example MultiConfiguration or MultiMachine."]
    #[serde(
        rename = "parallelExecutionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parallel_execution_type: Option<execution_input::ParallelExecutionType>,
}
impl ExecutionInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod execution_input {
    use super::*;
    #[doc = "Parallel execution type, for example MultiConfiguration or MultiMachine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ParallelExecutionType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "multiConfiguration")]
        MultiConfiguration,
        #[serde(rename = "multiMachine")]
        MultiMachine,
    }
}
#[doc = "Class to represent favorite entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteItem {
    #[doc = "Application specific data for the entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "Unique Id of the entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display text for favorite entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Application specific favorite entry type. Empty or Null represents that Favorite item is a Folder."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl FavoriteItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Folder {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Time when this folder created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Description of the folder."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "lastChangedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_changed_by: Option<IdentityRef>,
    #[doc = "Time when this folder last changed."]
    #[serde(
        rename = "lastChangedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_changed_date: Option<time::OffsetDateTime>,
    #[doc = "path of the folder."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GateUpdateMetadata {
    #[doc = "Comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Name of gate to be ignored."]
    #[serde(
        rename = "gatesToIgnore",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub gates_to_ignore: Vec<String>,
}
impl GateUpdateMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatesDeployPhase {
    #[serde(flatten)]
    pub deploy_phase: DeployPhase,
    #[serde(
        rename = "deploymentInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_input: Option<GatesDeploymentInput>,
}
impl GatesDeployPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatesDeploymentInput {
    #[serde(flatten)]
    pub base_deployment_input: BaseDeploymentInput,
    #[doc = "Gates minimum success duration."]
    #[serde(
        rename = "minimumSuccessDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_success_duration: Option<i32>,
    #[doc = "Gates sampling interval."]
    #[serde(
        rename = "samplingInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sampling_interval: Option<i32>,
    #[doc = "Gates stabilization time."]
    #[serde(
        rename = "stabilizationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stabilization_time: Option<i32>,
}
impl GatesDeploymentInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitArtifactDownloadInput {
    #[serde(flatten)]
    pub artifact_download_input_base: ArtifactDownloadInputBase,
}
impl GitArtifactDownloadInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubArtifactDownloadInput {
    #[serde(flatten)]
    pub artifact_download_input_base: ArtifactDownloadInputBase,
}
impl GitHubArtifactDownloadInput {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IgnoredGate {
    #[doc = "Gets the date on which gate is last ignored."]
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[doc = "Name of gate ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl IgnoredGate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an input for subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputDescriptor {
    #[doc = "The ids of all inputs that the value of this input is dependent on."]
    #[serde(
        rename = "dependencyInputIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependency_input_ids: Vec<String>,
    #[doc = "Description of what this input is used for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The group localized name to which this input belongs and can be shown as a header for the container that will include all the inputs in the group."]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[doc = "If true, the value information for this input is dynamic and should be fetched when the value of dependency inputs change."]
    #[serde(
        rename = "hasDynamicValueInformation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_dynamic_value_information: Option<bool>,
    #[doc = "Identifier for the subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Mode in which the value of this input should be entered"]
    #[serde(rename = "inputMode", default, skip_serializing_if = "Option::is_none")]
    pub input_mode: Option<input_descriptor::InputMode>,
    #[doc = "Gets whether this input is confidential, such as for a password or application key"]
    #[serde(
        rename = "isConfidential",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_confidential: Option<bool>,
    #[doc = "Localized name which can be shown as a label for the subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Custom properties for the input which can be used by the service provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Underlying data type for the input value. When this value is specified, InputMode, Validation and Values are optional."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets whether this input is included in the default generated action description."]
    #[serde(
        rename = "useInDefaultDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_in_default_description: Option<bool>,
    #[doc = "Describes what values are valid for a subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<InputValidation>,
    #[doc = "A hint for input value. It can be used in the UI as the input placeholder."]
    #[serde(rename = "valueHint", default, skip_serializing_if = "Option::is_none")]
    pub value_hint: Option<String>,
    #[doc = "Information about the possible/allowed values for a given subscription input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<InputValues>,
}
impl InputDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod input_descriptor {
    use super::*;
    #[doc = "Mode in which the value of this input should be entered"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InputMode {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "textBox")]
        TextBox,
        #[serde(rename = "passwordBox")]
        PasswordBox,
        #[serde(rename = "combo")]
        Combo,
        #[serde(rename = "radioButtons")]
        RadioButtons,
        #[serde(rename = "checkBox")]
        CheckBox,
        #[serde(rename = "textArea")]
        TextArea,
    }
}
#[doc = "Describes what values are valid for a subscription input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValidation {
    #[doc = "Gets or sets the data type to validate."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<input_validation::DataType>,
    #[doc = "Gets or sets if this is a required field."]
    #[serde(
        rename = "isRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
    #[doc = "Gets or sets the maximum length of this descriptor."]
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[doc = "Gets or sets the minimum value for this descriptor."]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[doc = "Gets or sets the minimum length of this descriptor."]
    #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[doc = "Gets or sets the minimum value for this descriptor."]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    #[doc = "Gets or sets the pattern to validate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Gets or sets the error on pattern mismatch."]
    #[serde(
        rename = "patternMismatchErrorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pattern_mismatch_error_message: Option<String>,
}
impl InputValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod input_validation {
    use super::*;
    #[doc = "Gets or sets the data type to validate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "string")]
        String,
        #[serde(rename = "number")]
        Number,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(rename = "guid")]
        Guid,
        #[serde(rename = "uri")]
        Uri,
    }
}
#[doc = "Information about a single value for an input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValue {
    #[doc = "Any other data about this input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "The text to show for the display of this value"]
    #[serde(
        rename = "displayValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_value: Option<String>,
    #[doc = "The value to store for this input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl InputValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the possible/allowed values for a given subscription input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValues {
    #[doc = "The default value to use for this input"]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
    #[doc = "Error information related to a subscription input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<InputValuesError>,
    #[doc = "The id of the input"]
    #[serde(rename = "inputId", default, skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[doc = "Should this input be disabled"]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Should the value be restricted to one of the values in the PossibleValues (True) or are the values in PossibleValues just a suggestion (False)"]
    #[serde(
        rename = "isLimitedToPossibleValues",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_limited_to_possible_values: Option<bool>,
    #[doc = "Should this input be made read-only"]
    #[serde(
        rename = "isReadOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_read_only: Option<bool>,
    #[doc = "Possible values that this input can take"]
    #[serde(
        rename = "possibleValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub possible_values: Vec<InputValue>,
}
impl InputValues {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error information related to a subscription input value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValuesError {
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl InputValuesError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValuesQuery {
    #[serde(
        rename = "currentValues",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_values: Option<serde_json::Value>,
    #[doc = "The input values to return on input, and the result from the consumer on output."]
    #[serde(
        rename = "inputValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_values: Vec<InputValues>,
    #[doc = "Subscription containing information about the publisher/consumer and the current input values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
}
impl InputValuesQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Issue {
    #[doc = "Issue data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Issue type, for example error, warning or info."]
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    #[doc = "Issue message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Issue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JenkinsArtifactDownloadInput {
    #[serde(flatten)]
    pub artifact_download_input_base: ArtifactDownloadInputBase,
}
impl JenkinsArtifactDownloadInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineGroupBasedDeployPhase {
    #[serde(flatten)]
    pub deploy_phase: DeployPhase,
    #[serde(
        rename = "deploymentInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_input: Option<MachineGroupDeploymentInput>,
}
impl MachineGroupBasedDeployPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineGroupDeploymentInput {
    #[serde(flatten)]
    pub deployment_input: DeploymentInput,
    #[doc = "Deployment group health option."]
    #[serde(
        rename = "deploymentHealthOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_health_option: Option<String>,
    #[doc = "Minimum percentage of the targets guaranteed to be healthy."]
    #[serde(
        rename = "healthPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub health_percent: Option<i32>,
    #[doc = "Deployment target tag filter."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl MachineGroupDeploymentInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MailMessage {
    #[doc = "Body of mail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<EmailRecipients>,
    #[doc = "Reply to."]
    #[serde(rename = "inReplyTo", default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    #[doc = "Message ID of the mail."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Data when should be replied to mail."]
    #[serde(
        rename = "replyBy",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub reply_by: Option<time::OffsetDateTime>,
    #[serde(rename = "replyTo", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EmailRecipients>,
    #[doc = "List of mail section types."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sections: Vec<serde_json::Value>,
    #[doc = "Mail sender type."]
    #[serde(
        rename = "senderType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sender_type: Option<mail_message::SenderType>,
    #[doc = "Subject of the mail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<EmailRecipients>,
}
impl MailMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mail_message {
    use super::*;
    #[doc = "Mail sender type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SenderType {
        #[serde(rename = "serviceAccount")]
        ServiceAccount,
        #[serde(rename = "requestingUser")]
        RequestingUser,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManualIntervention {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<IdentityRef>,
    #[doc = "Gets or sets comments for approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Gets date on which it got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets the unique identifier for manual intervention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets instructions for approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[doc = "Gets date on which it got modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseShallowReference>,
    #[serde(
        rename = "releaseDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition: Option<ReleaseDefinitionShallowReference>,
    #[serde(
        rename = "releaseEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_environment: Option<ReleaseEnvironmentShallowReference>,
    #[doc = "Gets or sets the status of the manual intervention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<manual_intervention::Status>,
    #[doc = "Get task instance identifier."]
    #[serde(
        rename = "taskInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_instance_id: Option<String>,
    #[doc = "Gets url to access the manual intervention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ManualIntervention {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod manual_intervention {
    use super::*;
    #[doc = "Gets or sets the status of the manual intervention."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "canceled")]
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManualInterventionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManualIntervention>,
}
impl ManualInterventionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManualInterventionUpdateMetadata {
    #[doc = "Sets the comment for manual intervention update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Sets the status of the manual intervention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<manual_intervention_update_metadata::Status>,
}
impl ManualInterventionUpdateMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod manual_intervention_update_metadata {
    use super::*;
    #[doc = "Sets the status of the manual intervention."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "canceled")]
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MappingDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<serde_json::Value>,
}
impl MappingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metric {
    #[doc = "Name of the Metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the Metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}
impl Metric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MultiConfigInput {
    #[serde(flatten)]
    pub parallel_execution_input_base: ParallelExecutionInputBase,
    #[doc = "Multipliers for parallel execution of deployment, for example x86,x64."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multipliers: Option<String>,
}
impl MultiConfigInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MultiMachineInput {
    #[serde(flatten)]
    pub parallel_execution_input_base: ParallelExecutionInputBase,
}
impl MultiMachineInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrgPipelineReleaseSettings {
    #[doc = "Defines whether user can manage pipeline settings."]
    #[serde(
        rename = "hasManagePipelinePoliciesPermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_manage_pipeline_policies_permission: Option<bool>,
    #[doc = "EnforceJobAuthScope setting at organisaion level. If enabled, scope of access for all release pipelines in the organisation reduces to the current project."]
    #[serde(
        rename = "orgEnforceJobAuthScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub org_enforce_job_auth_scope: Option<bool>,
}
impl OrgPipelineReleaseSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrgPipelineReleaseSettingsUpdateParameters {
    #[doc = "EnforceJobAuthScope setting at organisaion level. If enabled, scope of access for all release pipelines in the organisation reduces to the current project."]
    #[serde(
        rename = "orgEnforceJobAuthScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub org_enforce_job_auth_scope: Option<bool>,
}
impl OrgPipelineReleaseSettingsUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageTrigger {
    #[serde(flatten)]
    pub release_trigger_base: ReleaseTriggerBase,
    #[doc = "Package trigger alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}
impl PackageTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParallelExecutionInputBase {
    #[serde(flatten)]
    pub execution_input: ExecutionInput,
    #[doc = "Indicate whether continue execution of deployment on error or not."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "Maximum number of agents used while parallel execution."]
    #[serde(
        rename = "maxNumberOfAgents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_number_of_agents: Option<i32>,
}
impl ParallelExecutionInputBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineProcess {
    #[doc = "Pipeline process type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<pipeline_process::Type>,
}
impl PipelineProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pipeline_process {
    use super::*;
    #[doc = "Pipeline process type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "designer")]
        Designer,
        #[serde(rename = "yaml")]
        Yaml,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectPipelineReleaseSettings {
    #[doc = "EnforceJobAuthScope setting at project level. If enabled, scope of access for all release pipelines reduces to the current project."]
    #[serde(
        rename = "enforceJobAuthScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_job_auth_scope: Option<bool>,
    #[doc = "Defines whether user can manage pipeline settings."]
    #[serde(
        rename = "hasManageSettingsPermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_manage_settings_permission: Option<bool>,
    #[doc = "EnforceJobAuthScope setting at organisaion level. If enabled, scope of access for all release pipelines in the organisation reduces to the current project."]
    #[serde(
        rename = "orgEnforceJobAuthScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub org_enforce_job_auth_scope: Option<bool>,
    #[doc = "Defines whether project is public."]
    #[serde(
        rename = "publicProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_project: Option<bool>,
}
impl ProjectPipelineReleaseSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectPipelineReleaseSettingsUpdateParameters {
    #[doc = "EnforceJobAuthScope setting at project level. If enabled, scope of access for all release pipelines reduces to the current project."]
    #[serde(
        rename = "enforceJobAuthScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_job_auth_scope: Option<bool>,
}
impl ProjectPipelineReleaseSettingsUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectReference {
    #[doc = "Gets the unique identifier of this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets name of project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProjectReference {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertySelector {
    #[doc = "List of properties."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub properties: Vec<String>,
    #[doc = "Property selector type."]
    #[serde(
        rename = "selectorType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selector_type: Option<property_selector::SelectorType>,
}
impl PropertySelector {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod property_selector {
    use super::*;
    #[doc = "Property selector type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SelectorType {
        #[serde(rename = "inclusion")]
        Inclusion,
        #[serde(rename = "exclusion")]
        Exclusion,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequestConfiguration {
    #[serde(
        rename = "codeRepositoryReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code_repository_reference: Option<CodeRepositoryReference>,
    #[doc = "In case of Source based artifacts, Code reference will be present in Artifact details."]
    #[serde(
        rename = "useArtifactReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_artifact_reference: Option<bool>,
}
impl PullRequestConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequestFilter {
    #[doc = "List of tags."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Target branch of pull request."]
    #[serde(
        rename = "targetBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_branch: Option<String>,
}
impl PullRequestFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequestTrigger {
    #[serde(flatten)]
    pub release_trigger_base: ReleaseTriggerBase,
    #[doc = "Artifact alias trigger is linked to."]
    #[serde(
        rename = "artifactAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_alias: Option<String>,
    #[serde(
        rename = "pullRequestConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_configuration: Option<PullRequestConfiguration>,
    #[doc = "Policy name using which status will be published to pull request."]
    #[serde(
        rename = "statusPolicyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_policy_name: Option<String>,
    #[doc = "List of filters applied while trigger."]
    #[serde(
        rename = "triggerConditions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trigger_conditions: Vec<PullRequestFilter>,
}
impl PullRequestTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueuedReleaseData {
    #[doc = "Project ID of the release."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Release queue position."]
    #[serde(
        rename = "queuePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_position: Option<i32>,
    #[doc = "Queued release ID."]
    #[serde(rename = "releaseId", default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
}
impl QueuedReleaseData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeReleaseDefinitionEvent {
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl RealtimeReleaseDefinitionEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeReleaseEvent {
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "releaseId", default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
}
impl RealtimeReleaseEvent {
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
pub struct Release {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets or sets the list of artifacts."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<Artifact>,
    #[doc = "Gets or sets comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[serde(
        rename = "createdFor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_for: Option<IdentityRef>,
    #[doc = "Gets date on which it got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets revision number of definition snapshot."]
    #[serde(
        rename = "definitionSnapshotRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_snapshot_revision: Option<i32>,
    #[doc = "Gets or sets description of release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets list of environments."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environments: Vec<ReleaseEnvironment>,
    #[doc = "Gets the unique identifier of this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Whether to exclude the release from retention policies."]
    #[serde(
        rename = "keepForever",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_forever: Option<bool>,
    #[doc = "Gets logs container url."]
    #[serde(
        rename = "logsContainerUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logs_container_url: Option<String>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Gets date on which it got modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets pool name."]
    #[serde(rename = "poolName", default, skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    #[serde(
        rename = "projectReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_reference: Option<ProjectReference>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Gets reason of release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<release::Reason>,
    #[serde(
        rename = "releaseDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition: Option<ReleaseDefinitionShallowReference>,
    #[doc = "Gets or sets the release definition revision."]
    #[serde(
        rename = "releaseDefinitionRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition_revision: Option<i32>,
    #[doc = "Gets release name format."]
    #[serde(
        rename = "releaseNameFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_name_format: Option<String>,
    #[doc = "Gets status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release::Status>,
    #[doc = "Gets or sets list of tags."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[serde(
        rename = "triggeringArtifactAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub triggering_artifact_alias: Option<String>,
    #[doc = "Gets the list of variable groups."]
    #[serde(
        rename = "variableGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_groups: Vec<VariableGroup>,
    #[doc = "Gets or sets the dictionary of variables."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl Release {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release {
    use super::*;
    #[doc = "Gets reason of release."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "pullRequest")]
        PullRequest,
    }
    #[doc = "Gets status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "draft")]
        Draft,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseAbandonedEvent {
    #[serde(flatten)]
    pub release_event: ReleaseEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
}
impl ReleaseAbandonedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseApproval {
    #[doc = "Gets or sets the type of approval."]
    #[serde(
        rename = "approvalType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_type: Option<release_approval::ApprovalType>,
    #[serde(
        rename = "approvedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approved_by: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<IdentityRef>,
    #[doc = "Gets or sets attempt which specifies as which deployment attempt it belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Gets or sets comments for approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Gets date on which it got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets history which specifies all approvals associated with this approval."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub history: Vec<ReleaseApprovalHistory>,
    #[doc = "Gets the unique identifier of this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets as approval is automated or not."]
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets date on which it got modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets rank which specifies the order of the approval. e.g. Same rank denotes parallel approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseShallowReference>,
    #[serde(
        rename = "releaseDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition: Option<ReleaseDefinitionShallowReference>,
    #[serde(
        rename = "releaseEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_environment: Option<ReleaseEnvironmentShallowReference>,
    #[doc = "Gets the revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "Gets or sets the status of the approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release_approval::Status>,
    #[doc = "Gets url to access the approval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ReleaseApproval {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_approval {
    use super::*;
    #[doc = "Gets or sets the type of approval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApprovalType {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "preDeploy")]
        PreDeploy,
        #[serde(rename = "postDeploy")]
        PostDeploy,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "Gets or sets the status of the approval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "reassigned")]
        Reassigned,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "skipped")]
        Skipped,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseApprovalHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<IdentityRef>,
    #[serde(rename = "changedBy", default, skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<IdentityRef>,
    #[doc = "Approval history comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Time when this approval created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Time when this approval modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Approval history revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl ReleaseApprovalHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseApprovalList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReleaseApproval>,
}
impl ReleaseApprovalList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseApprovalPendingEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<ReleaseApproval>,
    #[serde(
        rename = "approvalOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_options: Option<ApprovalOptions>,
    #[serde(
        rename = "completedApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub completed_approvals: Vec<ReleaseApproval>,
    #[serde(
        rename = "definitionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[serde(
        rename = "environmentName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environments: Vec<ReleaseEnvironment>,
    #[serde(
        rename = "isMultipleRankApproval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_multiple_rank_approval: Option<bool>,
    #[serde(
        rename = "pendingApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pending_approvals: Vec<ReleaseApproval>,
    #[serde(
        rename = "releaseCreator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_creator: Option<String>,
    #[serde(
        rename = "releaseName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(
        rename = "webAccessUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_access_uri: Option<String>,
}
impl ReleaseApprovalPendingEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseArtifact {
    #[serde(
        rename = "artifactProvider",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_provider: Option<ArtifactProvider>,
    #[doc = "Gets or sets the artifact type of ReleaseArtifact."]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[doc = "Gets or sets the definition json of ReleaseArtifact."]
    #[serde(
        rename = "definitionData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_data: Option<String>,
    #[doc = "Gets or sets the definition id of ReleaseArtifact."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "Gets or sets the description of ReleaseArtifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the id of ReleaseArtifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets the name of ReleaseArtifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the release id."]
    #[serde(rename = "releaseId", default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
}
impl ReleaseArtifact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseCondition {
    #[serde(flatten)]
    pub condition: Condition,
    #[doc = "The release condition result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}
impl ReleaseCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseCreatedEvent {
    #[serde(flatten)]
    pub release_event: ReleaseEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
}
impl ReleaseCreatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinition {
    #[serde(flatten)]
    pub release_definition_shallow_reference: ReleaseDefinitionShallowReference,
    #[doc = "Gets or sets the list of artifacts."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<Artifact>,
    #[doc = "Gets or sets comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets date on which it got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the list of environments."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environments: Vec<ReleaseDefinitionEnvironment>,
    #[doc = "Whether release definition is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(
        rename = "lastRelease",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_release: Option<ReleaseReference>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Gets date on which it got modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Gets or sets the release name format."]
    #[serde(
        rename = "releaseNameFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_name_format: Option<String>,
    #[doc = "Gets the revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "Gets or sets source of release definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<release_definition::Source>,
    #[doc = "Gets or sets list of tags."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Gets or sets the list of triggers."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub triggers: Vec<ReleaseTriggerBase>,
    #[doc = "Gets or sets the list of variable groups."]
    #[serde(
        rename = "variableGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_groups: Vec<i32>,
    #[doc = "Gets or sets the dictionary of variables."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl ReleaseDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_definition {
    use super::*;
    #[doc = "Gets or sets source of release definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "restApi")]
        RestApi,
        #[serde(rename = "userInterface")]
        UserInterface,
        #[serde(rename = "ibiza")]
        Ibiza,
        #[serde(rename = "portalExtensionApi")]
        PortalExtensionApi,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionApprovalStep {
    #[serde(flatten)]
    pub release_definition_environment_step: ReleaseDefinitionEnvironmentStep,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<IdentityRef>,
    #[doc = "Indicates whether the approval automated."]
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Indicates whether the approval notification set."]
    #[serde(
        rename = "isNotificationOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_notification_on: Option<bool>,
    #[doc = "Gets or sets the rank of approval step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}
impl ReleaseDefinitionApprovalStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionApprovals {
    #[serde(
        rename = "approvalOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_options: Option<ApprovalOptions>,
    #[doc = "Gets or sets the approvals."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub approvals: Vec<ReleaseDefinitionApprovalStep>,
}
impl ReleaseDefinitionApprovals {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionDeployStep {
    #[serde(flatten)]
    pub release_definition_environment_step: ReleaseDefinitionEnvironmentStep,
    #[doc = "The list of steps for this definition."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<WorkflowTask>,
}
impl ReleaseDefinitionDeployStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionEnvironment {
    #[doc = "Gets or sets the BadgeUrl. BadgeUrl will be used when Badge will be enabled in Release Definition Environment."]
    #[serde(rename = "badgeUrl", default, skip_serializing_if = "Option::is_none")]
    pub badge_url: Option<String>,
    #[doc = "Gets or sets the environment conditions."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<Condition>,
    #[serde(
        rename = "currentRelease",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_release: Option<ReleaseShallowReference>,
    #[doc = "Gets or sets the demands."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub demands: Vec<Demand>,
    #[doc = "Gets or sets the deploy phases of environment."]
    #[serde(
        rename = "deployPhases",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deploy_phases: Vec<DeployPhase>,
    #[serde(
        rename = "deployStep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deploy_step: Option<ReleaseDefinitionDeployStep>,
    #[serde(
        rename = "environmentOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_options: Option<EnvironmentOptions>,
    #[doc = "Gets or sets the triggers on environment."]
    #[serde(
        rename = "environmentTriggers",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environment_triggers: Vec<EnvironmentTrigger>,
    #[doc = "Defines policy on environment queuing at Release Management side queue. We will send to Environment Runner [creating pre-deploy and other steps] only when the policies mentioned are satisfied."]
    #[serde(
        rename = "executionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_policy: Option<EnvironmentExecutionPolicy>,
    #[doc = "Gets and sets the ID of the ReleaseDefinitionEnvironment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets and sets the name of the ReleaseDefinitionEnvironment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[serde(
        rename = "postDeployApprovals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_deploy_approvals: Option<ReleaseDefinitionApprovals>,
    #[serde(
        rename = "postDeploymentGates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_deployment_gates: Option<ReleaseDefinitionGatesStep>,
    #[serde(
        rename = "preDeployApprovals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_deploy_approvals: Option<ReleaseDefinitionApprovals>,
    #[serde(
        rename = "preDeploymentGates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_deployment_gates: Option<ReleaseDefinitionGatesStep>,
    #[serde(
        rename = "processParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub process_parameters: Option<ProcessParameters>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Gets or sets the queue ID."]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[doc = "Gets and sets the rank of the ReleaseDefinitionEnvironment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(
        rename = "retentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_policy: Option<EnvironmentRetentionPolicy>,
    #[doc = "Gets or sets the schedules"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schedules: Vec<ReleaseSchedule>,
    #[doc = "Gets or sets the variable groups."]
    #[serde(
        rename = "variableGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_groups: Vec<i32>,
    #[doc = "Gets and sets the variables."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl ReleaseDefinitionEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionEnvironmentStep {
    #[doc = "ID of the approval or deploy step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl ReleaseDefinitionEnvironmentStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionEnvironmentSummary {
    #[doc = "ID of ReleaseDefinition environment summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "List of release shallow reference deployed using this ReleaseDefinition."]
    #[serde(
        rename = "lastReleases",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub last_releases: Vec<ReleaseShallowReference>,
    #[doc = "Name of ReleaseDefinition environment summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ReleaseDefinitionEnvironmentSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionEnvironmentTemplate {
    #[doc = "Indicates whether template can be deleted or not."]
    #[serde(rename = "canDelete", default, skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[doc = "Category of the ReleaseDefinition environment template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Description of the ReleaseDefinition environment template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ReleaseDefinitionEnvironment>,
    #[doc = "ID of the task which used to display icon used for this template."]
    #[serde(
        rename = "iconTaskId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_task_id: Option<String>,
    #[doc = "Icon uri of the template."]
    #[serde(rename = "iconUri", default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[doc = "ID of the ReleaseDefinition environment template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates whether template deleted or not."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Name of the ReleaseDefinition environment template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ReleaseDefinitionEnvironmentTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionGate {
    #[doc = "Gets or sets the gates workflow."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<WorkflowTask>,
}
impl ReleaseDefinitionGate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionGatesOptions {
    #[doc = "Gets or sets as the gates enabled or not."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Gets or sets the minimum duration for steady results after a successful gates evaluation."]
    #[serde(
        rename = "minimumSuccessDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_success_duration: Option<i32>,
    #[doc = "Gets or sets the time between re-evaluation of gates."]
    #[serde(
        rename = "samplingInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sampling_interval: Option<i32>,
    #[doc = "Gets or sets the delay before evaluation."]
    #[serde(
        rename = "stabilizationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stabilization_time: Option<i32>,
    #[doc = "Gets or sets the timeout after which gates fail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}
impl ReleaseDefinitionGatesOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionGatesStep {
    #[doc = "Gets or sets the gates."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub gates: Vec<ReleaseDefinitionGate>,
    #[serde(
        rename = "gatesOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gates_options: Option<ReleaseDefinitionGatesOptions>,
    #[doc = "ID of the ReleaseDefinitionGateStep."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl ReleaseDefinitionGatesStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReleaseDefinition>,
}
impl ReleaseDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionRevision {
    #[doc = "Gets api-version for revision object."]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[serde(rename = "changedBy", default, skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<IdentityRef>,
    #[doc = "Gets date on which ReleaseDefinition changed."]
    #[serde(
        rename = "changedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub changed_date: Option<time::OffsetDateTime>,
    #[doc = "Gets type of change."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<release_definition_revision::ChangeType>,
    #[doc = "Gets comments for revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Get id of the definition."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "Gets definition URL."]
    #[serde(
        rename = "definitionUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_url: Option<String>,
    #[doc = "Get revision number of the definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl ReleaseDefinitionRevision {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_definition_revision {
    use super::*;
    #[doc = "Gets type of change."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "undelete")]
        Undelete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionRevisionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReleaseDefinitionRevision>,
}
impl ReleaseDefinitionRevisionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionShallowReference {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets the unique identifier of release definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets the name of the release definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the path of the release definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "projectReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_reference: Option<ProjectReference>,
    #[doc = "Gets the REST API url to access the release definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ReleaseDefinitionShallowReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionSummary {
    #[doc = "List of Release Definition environment summary."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environments: Vec<ReleaseDefinitionEnvironmentSummary>,
    #[serde(
        rename = "releaseDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition: Option<ReleaseDefinitionShallowReference>,
    #[doc = "List of releases deployed using this Release Definition."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub releases: Vec<Release>,
}
impl ReleaseDefinitionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDefinitionUndeleteParameter {
    #[doc = "Gets or sets comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
impl ReleaseDefinitionUndeleteParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseDeployPhase {
    #[doc = "Deployment jobs of the phase."]
    #[serde(
        rename = "deploymentJobs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployment_jobs: Vec<DeploymentJob>,
    #[doc = "Phase execution error logs."]
    #[serde(rename = "errorLog", default, skip_serializing_if = "Option::is_none")]
    pub error_log: Option<String>,
    #[doc = "List of manual intervention tasks execution information in phase."]
    #[serde(
        rename = "manualInterventions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub manual_interventions: Vec<ManualIntervention>,
    #[doc = "Name of the phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the phase."]
    #[serde(rename = "phaseId", default, skip_serializing_if = "Option::is_none")]
    pub phase_id: Option<String>,
    #[doc = "Type of the phase."]
    #[serde(rename = "phaseType", default, skip_serializing_if = "Option::is_none")]
    pub phase_type: Option<release_deploy_phase::PhaseType>,
    #[doc = "Rank of the phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[doc = "Run Plan ID of the phase."]
    #[serde(rename = "runPlanId", default, skip_serializing_if = "Option::is_none")]
    pub run_plan_id: Option<String>,
    #[doc = "Phase start time."]
    #[serde(
        rename = "startedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Status of the phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release_deploy_phase::Status>,
}
impl ReleaseDeployPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_deploy_phase {
    use super::*;
    #[doc = "Type of the phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PhaseType {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "agentBasedDeployment")]
        AgentBasedDeployment,
        #[serde(rename = "runOnServer")]
        RunOnServer,
        #[serde(rename = "machineGroupBasedDeployment")]
        MachineGroupBasedDeployment,
        #[serde(rename = "deploymentGates")]
        DeploymentGates,
    }
    #[doc = "Status of the phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "cancelling")]
        Cancelling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseEnvironment {
    #[doc = "Gets list of conditions."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<ReleaseCondition>,
    #[doc = "Gets date on which it got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets definition environment id."]
    #[serde(
        rename = "definitionEnvironmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_environment_id: Option<i32>,
    #[doc = "Gets list of deploy phases snapshot."]
    #[serde(
        rename = "deployPhasesSnapshot",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deploy_phases_snapshot: Vec<DeployPhase>,
    #[doc = "Gets deploy steps."]
    #[serde(
        rename = "deploySteps",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deploy_steps: Vec<DeploymentAttempt>,
    #[serde(
        rename = "environmentOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_options: Option<EnvironmentOptions>,
    #[doc = "Gets the unique identifier of this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets date on which it got modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets next scheduled UTC time."]
    #[serde(
        rename = "nextScheduledUtcTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub next_scheduled_utc_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[serde(
        rename = "postApprovalsSnapshot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_approvals_snapshot: Option<ReleaseDefinitionApprovals>,
    #[doc = "Gets list of post deploy approvals."]
    #[serde(
        rename = "postDeployApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub post_deploy_approvals: Vec<ReleaseApproval>,
    #[serde(
        rename = "postDeploymentGatesSnapshot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_deployment_gates_snapshot: Option<ReleaseDefinitionGatesStep>,
    #[serde(
        rename = "preApprovalsSnapshot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_approvals_snapshot: Option<ReleaseDefinitionApprovals>,
    #[doc = "Gets list of pre deploy approvals."]
    #[serde(
        rename = "preDeployApprovals",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pre_deploy_approvals: Vec<ReleaseApproval>,
    #[serde(
        rename = "preDeploymentGatesSnapshot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_deployment_gates_snapshot: Option<ReleaseDefinitionGatesStep>,
    #[serde(
        rename = "processParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub process_parameters: Option<ProcessParameters>,
    #[doc = "Gets rank."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseShallowReference>,
    #[serde(
        rename = "releaseCreatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_created_by: Option<IdentityRef>,
    #[serde(
        rename = "releaseDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition: Option<ReleaseDefinitionShallowReference>,
    #[doc = "Gets release id."]
    #[serde(rename = "releaseId", default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
    #[doc = "Gets schedule deployment time of release environment."]
    #[serde(
        rename = "scheduledDeploymentTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub scheduled_deployment_time: Option<time::OffsetDateTime>,
    #[doc = "Gets list of schedules."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schedules: Vec<ReleaseSchedule>,
    #[doc = "Gets environment status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release_environment::Status>,
    #[doc = "Gets time to deploy."]
    #[serde(
        rename = "timeToDeploy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_to_deploy: Option<f64>,
    #[doc = "Gets trigger reason."]
    #[serde(
        rename = "triggerReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_reason: Option<String>,
    #[doc = "Gets the list of variable groups."]
    #[serde(
        rename = "variableGroups",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_groups: Vec<VariableGroup>,
    #[doc = "Gets the dictionary of variables."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl ReleaseEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_environment {
    use super::*;
    #[doc = "Gets environment status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseEnvironmentCompletedEvent {
    #[serde(
        rename = "createdByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_name: Option<String>,
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[serde(
        rename = "definitionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ReleaseEnvironment>,
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[serde(
        rename = "projectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<release_environment_completed_event::Reason>,
    #[serde(
        rename = "releaseCreatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_created_by: Option<IdentityRef>,
    #[serde(
        rename = "releaseLogsUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_logs_uri: Option<String>,
    #[serde(
        rename = "releaseName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(
        rename = "webAccessUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_access_uri: Option<String>,
}
impl ReleaseEnvironmentCompletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_environment_completed_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "automated")]
        Automated,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "redeployTrigger")]
        RedeployTrigger,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseEnvironmentShallowReference {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets the unique identifier of release environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets the name of the release environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the REST API url to access the release environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ReleaseEnvironmentShallowReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseEnvironmentStatusUpdatedEvent {
    #[serde(flatten)]
    pub realtime_release_definition_event: RealtimeReleaseDefinitionEvent,
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[serde(
        rename = "environmentStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_status: Option<release_environment_status_updated_event::EnvironmentStatus>,
    #[serde(
        rename = "latestDeploymentOperationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_deployment_operation_status:
        Option<release_environment_status_updated_event::LatestDeploymentOperationStatus>,
    #[serde(
        rename = "latestDeploymentStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_deployment_status:
        Option<release_environment_status_updated_event::LatestDeploymentStatus>,
    #[serde(rename = "releaseId", default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
}
impl ReleaseEnvironmentStatusUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_environment_status_updated_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EnvironmentStatus {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LatestDeploymentOperationStatus {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "approved")]
        Approved,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "deferred")]
        Deferred,
        #[serde(rename = "queuedForAgent")]
        QueuedForAgent,
        #[serde(rename = "phaseInProgress")]
        PhaseInProgress,
        #[serde(rename = "phaseSucceeded")]
        PhaseSucceeded,
        #[serde(rename = "phasePartiallySucceeded")]
        PhasePartiallySucceeded,
        #[serde(rename = "phaseFailed")]
        PhaseFailed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "phaseCanceled")]
        PhaseCanceled,
        #[serde(rename = "manualInterventionPending")]
        ManualInterventionPending,
        #[serde(rename = "queuedForPipeline")]
        QueuedForPipeline,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "evaluatingGates")]
        EvaluatingGates,
        #[serde(rename = "gateFailed")]
        GateFailed,
        #[serde(rename = "all")]
        All,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LatestDeploymentStatus {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notDeployed")]
        NotDeployed,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseEnvironmentUpdateMetadata {
    #[doc = "Gets or sets comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Gets or sets scheduled deployment time."]
    #[serde(
        rename = "scheduledDeploymentTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub scheduled_deployment_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets status of environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release_environment_update_metadata::Status>,
    #[doc = "Sets list of environment variables to be overridden at deployment time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl ReleaseEnvironmentUpdateMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_environment_update_metadata {
    use super::*;
    #[doc = "Gets or sets status of environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ReleaseEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseGates {
    #[doc = "Contains the gates job details of each evaluation."]
    #[serde(
        rename = "deploymentJobs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployment_jobs: Vec<DeploymentJob>,
    #[doc = "ID of release gates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "List of ignored gates."]
    #[serde(
        rename = "ignoredGates",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ignored_gates: Vec<IgnoredGate>,
    #[doc = "Gates last modified time."]
    #[serde(
        rename = "lastModifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_on: Option<time::OffsetDateTime>,
    #[doc = "Run plan ID of the gates."]
    #[serde(rename = "runPlanId", default, skip_serializing_if = "Option::is_none")]
    pub run_plan_id: Option<String>,
    #[doc = "Gates stabilization completed date and time."]
    #[serde(
        rename = "stabilizationCompletedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub stabilization_completed_on: Option<time::OffsetDateTime>,
    #[doc = "Gates evaluation started time."]
    #[serde(
        rename = "startedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Status of release gates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release_gates::Status>,
    #[doc = "Date and time at which all gates executed successfully."]
    #[serde(
        rename = "succeedingSince",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub succeeding_since: Option<time::OffsetDateTime>,
}
impl ReleaseGates {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_gates {
    use super::*;
    #[doc = "Status of release gates."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseGatesPhase {
    #[serde(flatten)]
    pub release_deploy_phase: ReleaseDeployPhase,
    #[doc = "List of ignored gates."]
    #[serde(
        rename = "ignoredGates",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ignored_gates: Vec<IgnoredGate>,
    #[doc = "Date and time at which stabilization of gates completed."]
    #[serde(
        rename = "stabilizationCompletedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub stabilization_completed_on: Option<time::OffsetDateTime>,
    #[doc = "Date and time at which all gates executed successfully."]
    #[serde(
        rename = "succeedingSince",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub succeeding_since: Option<time::OffsetDateTime>,
}
impl ReleaseGatesPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Release>,
}
impl ReleaseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseManagementInputValue {
    #[doc = "The text to show for the display of this value."]
    #[serde(
        rename = "displayValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_value: Option<String>,
    #[doc = "The value to store for this input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ReleaseManagementInputValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseNotCreatedEvent {
    #[serde(
        rename = "definitionReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_reference: Option<ReleaseDefinitionShallowReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(
        rename = "releaseReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_reason: Option<release_not_created_event::ReleaseReason>,
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<IdentityRef>,
}
impl ReleaseNotCreatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_not_created_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReleaseReason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "pullRequest")]
        PullRequest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseReference {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets list of artifacts."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<Artifact>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets date on when this release created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "ID of the Release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Gets name of release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets reason for release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<release_reference::Reason>,
    #[serde(
        rename = "releaseDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_definition: Option<ReleaseDefinitionShallowReference>,
}
impl ReleaseReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_reference {
    use super::*;
    #[doc = "Gets reason for release."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "pullRequest")]
        PullRequest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseRevision {
    #[serde(rename = "changedBy", default, skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<IdentityRef>,
    #[doc = "Change date of the revision."]
    #[serde(
        rename = "changedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub changed_date: Option<time::OffsetDateTime>,
    #[doc = "Change details of the revision."]
    #[serde(
        rename = "changeDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_details: Option<String>,
    #[doc = "Change details of the revision. Typically ChangeDetails values are Add and Update."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<String>,
    #[doc = "Comment of the revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Release ID of which this revision belongs."]
    #[serde(
        rename = "definitionSnapshotRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_snapshot_revision: Option<i32>,
    #[doc = "Gets or sets the release ID of which this revision belongs."]
    #[serde(rename = "releaseId", default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
}
impl ReleaseRevision {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseSchedule {
    #[doc = "Days of the week to release."]
    #[serde(
        rename = "daysToRelease",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_release: Option<release_schedule::DaysToRelease>,
    #[doc = "Team Foundation Job Definition Job Id."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Flag to determine if this schedule should only release if the associated artifact has been changed or release definition changed."]
    #[serde(
        rename = "scheduleOnlyWithChanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_only_with_changes: Option<bool>,
    #[doc = "Local time zone hour to start."]
    #[serde(
        rename = "startHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_hours: Option<i32>,
    #[doc = "Local time zone minute to start."]
    #[serde(
        rename = "startMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_minutes: Option<i32>,
    #[doc = "Time zone Id of release schedule, such as 'UTC'."]
    #[serde(
        rename = "timeZoneId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_zone_id: Option<String>,
}
impl ReleaseSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_schedule {
    use super::*;
    #[doc = "Days of the week to release."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DaysToRelease {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseSettings {
    #[serde(
        rename = "complianceSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compliance_settings: Option<ComplianceSettings>,
    #[serde(
        rename = "retentionSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_settings: Option<RetentionSettings>,
}
impl ReleaseSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseShallowReference {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets the unique identifier of release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets the name of the release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the REST API url to access the release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ReleaseShallowReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseStartEnvironmentMetadata {
    #[doc = "Sets release definition environment id."]
    #[serde(
        rename = "definitionEnvironmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_environment_id: Option<i32>,
    #[doc = "Sets list of environments variables to be overridden at deployment time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl ReleaseStartEnvironmentMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseStartMetadata {
    #[doc = "Sets list of artifact to create a release."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<ArtifactMetadata>,
    #[doc = "Sets definition Id to create a release."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "Sets description to create a release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Sets list of environments meta data."]
    #[serde(
        rename = "environmentsMetadata",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environments_metadata: Vec<ReleaseStartEnvironmentMetadata>,
    #[doc = "Sets 'true' to create release in draft mode, 'false' otherwise."]
    #[serde(rename = "isDraft", default, skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    #[doc = "Sets list of environments to manual as condition."]
    #[serde(
        rename = "manualEnvironments",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub manual_environments: Vec<String>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Sets reason to create a release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<release_start_metadata::Reason>,
    #[doc = "Sets list of release variables to be overridden at deployment time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl ReleaseStartMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_start_metadata {
    use super::*;
    #[doc = "Sets reason to create a release."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "continuousIntegration")]
        ContinuousIntegration,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "pullRequest")]
        PullRequest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseTask {
    #[doc = "Agent name on which task executed."]
    #[serde(rename = "agentName", default, skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[doc = "Finish time of the release task."]
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "ID of the release task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "List of issues occurred while execution of task."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub issues: Vec<Issue>,
    #[doc = "Number of lines log release task has."]
    #[serde(rename = "lineCount", default, skip_serializing_if = "Option::is_none")]
    pub line_count: Option<i64>,
    #[doc = "Log URL of the task."]
    #[serde(rename = "logUrl", default, skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    #[doc = "Name of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Task execution complete precent."]
    #[serde(
        rename = "percentComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percent_complete: Option<i32>,
    #[doc = "Rank of the release task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[doc = "Result code of the task."]
    #[serde(
        rename = "resultCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_code: Option<String>,
    #[doc = "ID of the release task."]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Status of release task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release_task::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<WorkflowTaskReference>,
    #[doc = "Timeline record ID of the release task."]
    #[serde(
        rename = "timelineRecordId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_record_id: Option<String>,
}
impl ReleaseTask {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_task {
    use super::*;
    #[doc = "Status of release task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "failure")]
        Failure,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "skipped")]
        Skipped,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "partiallySucceeded")]
        PartiallySucceeded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseTaskAttachment {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Data and time when it created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Data and time when modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Name of the task attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Record ID of the task."]
    #[serde(rename = "recordId", default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[doc = "Timeline ID of the task."]
    #[serde(
        rename = "timelineId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_id: Option<String>,
    #[doc = "Type of task attachment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ReleaseTaskAttachment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseTaskAttachmentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReleaseTaskAttachment>,
}
impl ReleaseTaskAttachmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseTaskLogUpdatedEvent {
    #[serde(flatten)]
    pub realtime_release_event: RealtimeReleaseEvent,
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
        rename = "timelineRecordId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_record_id: Option<String>,
}
impl ReleaseTaskLogUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseTasksUpdatedEvent {
    #[serde(flatten)]
    pub realtime_release_event: RealtimeReleaseEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<ReleaseTask>,
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(
        rename = "releaseStepId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_step_id: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<ReleaseTask>,
}
impl ReleaseTasksUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseTriggerBase {
    #[doc = "Type of release trigger."]
    #[serde(
        rename = "triggerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_type: Option<release_trigger_base::TriggerType>,
}
impl ReleaseTriggerBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_trigger_base {
    use super::*;
    #[doc = "Type of release trigger."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TriggerType {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "artifactSource")]
        ArtifactSource,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "sourceRepo")]
        SourceRepo,
        #[serde(rename = "containerImage")]
        ContainerImage,
        #[serde(rename = "package")]
        Package,
        #[serde(rename = "pullRequest")]
        PullRequest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseUpdateMetadata {
    #[doc = "Sets comment for release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Set 'true' to exclude the release from retention policies."]
    #[serde(
        rename = "keepForever",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_forever: Option<bool>,
    #[doc = "Sets list of manual environments."]
    #[serde(
        rename = "manualEnvironments",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub manual_environments: Vec<String>,
    #[doc = "Sets name of the release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Sets status of the release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<release_update_metadata::Status>,
}
impl ReleaseUpdateMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod release_update_metadata {
    use super::*;
    #[doc = "Sets status of the release."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "draft")]
        Draft,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseUpdatedEvent {
    #[serde(flatten)]
    pub realtime_release_event: RealtimeReleaseEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
}
impl ReleaseUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseWorkItemRef {
    #[doc = "Gets or sets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Gets or sets the state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Gets or sets the title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Gets or sets the type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the workitem url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ReleaseWorkItemRef {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionPolicy {
    #[doc = "Indicates the number of days to keep deployment."]
    #[serde(
        rename = "daysToKeep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_keep: Option<i32>,
}
impl RetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionSettings {
    #[doc = "Number of days to keep deleted releases."]
    #[serde(
        rename = "daysToKeepDeletedReleases",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_to_keep_deleted_releases: Option<i32>,
    #[serde(
        rename = "defaultEnvironmentRetentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_environment_retention_policy: Option<EnvironmentRetentionPolicy>,
    #[serde(
        rename = "maximumEnvironmentRetentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_environment_retention_policy: Option<EnvironmentRetentionPolicy>,
}
impl RetentionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunOnServerDeployPhase {
    #[serde(flatten)]
    pub deploy_phase: DeployPhase,
    #[serde(
        rename = "deploymentInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_input: Option<ServerDeploymentInput>,
}
impl RunOnServerDeployPhase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledReleaseTrigger {
    #[serde(flatten)]
    pub release_trigger_base: ReleaseTriggerBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ReleaseSchedule>,
}
impl ScheduledReleaseTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDeploymentInput {
    #[serde(flatten)]
    pub base_deployment_input: BaseDeploymentInput,
    #[serde(
        rename = "parallelExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parallel_execution: Option<ExecutionInput>,
}
impl ServerDeploymentInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a reference to a service endpoint."]
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
pub struct SourceIdInput {
    #[doc = "ID of source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SourceIdInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourcePullRequestVersion {
    #[doc = "Pull Request Iteration Id for which the release will publish status."]
    #[serde(
        rename = "iterationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_id: Option<String>,
    #[doc = "Pull Request Id for which the release will publish status."]
    #[serde(
        rename = "pullRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_id: Option<String>,
    #[doc = "Date and time of the pull request merge creation. It is required to keep timeline record of Releases created by pull request."]
    #[serde(
        rename = "pullRequestMergedAt",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub pull_request_merged_at: Option<time::OffsetDateTime>,
    #[doc = "Source branch of the Pull Request."]
    #[serde(
        rename = "sourceBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_branch: Option<String>,
    #[doc = "Source branch commit Id of the Pull Request for which the release will publish status."]
    #[serde(
        rename = "sourceBranchCommitId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_branch_commit_id: Option<String>,
    #[doc = "Target branch of the Pull Request."]
    #[serde(
        rename = "targetBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_branch: Option<String>,
}
impl SourcePullRequestVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRepoTrigger {
    #[serde(flatten)]
    pub release_trigger_base: ReleaseTriggerBase,
    #[doc = "Alias of the source repo trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(
        rename = "branchFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_filters: Vec<String>,
}
impl SourceRepoTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryMailSection {
    #[doc = "Html content of summary mail."]
    #[serde(
        rename = "htmlContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub html_content: Option<String>,
    #[doc = "Rank of the summary mail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[doc = "Summary mail section type. MailSectionType has section types."]
    #[serde(
        rename = "sectionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub section_type: Option<summary_mail_section::SectionType>,
    #[doc = "Title of the summary mail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl SummaryMailSection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod summary_mail_section {
    use super::*;
    #[doc = "Summary mail section type. MailSectionType has section types."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SectionType {
        #[serde(rename = "details")]
        Details,
        #[serde(rename = "environments")]
        Environments,
        #[serde(rename = "issues")]
        Issues,
        #[serde(rename = "testResults")]
        TestResults,
        #[serde(rename = "workItems")]
        WorkItems,
        #[serde(rename = "releaseInfo")]
        ReleaseInfo,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagFilter {
    #[doc = "Gets or sets the tag filter pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}
impl TagFilter {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskOrchestrationPlanGroupReference {
    #[doc = "Gets or sets the plan group."]
    #[serde(rename = "planGroup", default, skip_serializing_if = "Option::is_none")]
    pub plan_group: Option<String>,
    #[doc = "ID of the Project."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcArtifactDownloadInput {
    #[serde(flatten)]
    pub artifact_download_input_base: ArtifactDownloadInputBase,
}
impl TfvcArtifactDownloadInput {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeZoneList {
    #[serde(
        rename = "utcTimeZone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub utc_time_zone: Option<TimeZone>,
    #[doc = "List of valid timezones."]
    #[serde(
        rename = "validTimeZones",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_time_zones: Vec<TimeZone>,
}
impl TimeZoneList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroup {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Gets date on which it got created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets the unique identifier of this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Denotes if a variable group is shared with other project or not."]
    #[serde(rename = "isShared", default, skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "Gets date on which it got modified."]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "providerData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_data: Option<VariableGroupProviderData>,
    #[doc = "Gets or sets type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "all project references where the variable group is shared with other projects."]
    #[serde(
        rename = "variableGroupProjectReferences",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variable_group_project_references: Vec<VariableGroupProjectReference>,
    #[doc = "Gets and sets the dictionary of variables."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl VariableGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A variable group reference is a shallow reference to variable group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroupProjectReference {
    #[doc = "Gets or sets description of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets name of the variable group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "projectReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_reference: Option<ProjectReference>,
}
impl VariableGroupProjectReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableGroupProviderData {}
impl VariableGroupProviderData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableValue {
    #[doc = "Gets or sets if the variable is read only or not."]
    #[serde(
        rename = "isReadOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_read_only: Option<bool>,
    #[doc = "Gets or sets as the variable is secret or not."]
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[doc = "Gets or sets the value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VariableValue {
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
pub struct WorkflowTask {
    #[doc = "Gets or sets as the task always run or not."]
    #[serde(rename = "alwaysRun", default, skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    #[doc = "Gets or sets the task condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Gets or sets as the task continue run on error or not."]
    #[serde(
        rename = "continueOnError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on_error: Option<bool>,
    #[doc = "Gets or sets the task definition type. Example:- 'Agent', DeploymentGroup', 'Server' or 'ServerGate'."]
    #[serde(
        rename = "definitionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_type: Option<String>,
    #[doc = "Gets or sets as the task enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets or sets the task environment variables."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    #[doc = "Gets or sets the task inputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "Gets or sets the name of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the task override inputs."]
    #[serde(
        rename = "overrideInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub override_inputs: Option<serde_json::Value>,
    #[doc = "Gets or sets the reference name of the task."]
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[doc = "Gets or sets the task retryCount."]
    #[serde(
        rename = "retryCountOnTaskFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_count_on_task_failure: Option<i32>,
    #[doc = "Gets or sets the ID of the task."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "Gets or sets the task timeout."]
    #[serde(
        rename = "timeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_in_minutes: Option<i32>,
    #[doc = "Gets or sets the version of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl WorkflowTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTaskReference {
    #[doc = "Task identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl WorkflowTaskReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct YamlFileSource {
    #[doc = "Gets or sets definition reference. e.g. {\"project\":{\"id\":\"fed755ea-49c5-4399-acea-fd5b5aa90a6c\",\"name\":\"myProject\"},\"definition\":{\"id\":\"1\",\"name\":\"mybuildDefinition\"},\"connection\":{\"id\":\"1\",\"name\":\"myConnection\"}}"]
    #[serde(
        rename = "sourceReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_reference: Option<serde_json::Value>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<yaml_file_source::Type>,
}
impl YamlFileSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod yaml_file_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "tfsGit")]
        TfsGit,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct YamlPipelineProcess {
    #[serde(flatten)]
    pub pipeline_process: PipelineProcess,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(
        rename = "fileSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_source: Option<YamlFileSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<YamlPipelineProcessResources>,
}
impl YamlPipelineProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct YamlPipelineProcessResources {
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
    pub queues: Vec<AgentPoolQueueReference>,
}
impl YamlPipelineProcessResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct YamlSourceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl YamlSourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
