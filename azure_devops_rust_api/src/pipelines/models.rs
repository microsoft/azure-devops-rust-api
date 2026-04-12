// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Artifacts are collections of files produced by a pipeline. Use artifacts to share files between stages in a pipeline or between different pipelines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Artifact {
    #[doc = "The name of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A signed url allowing limited-time anonymous access to private resources."]
    #[serde(
        rename = "signedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_content: Option<SignedUrl>,
    #[doc = "Self-referential url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Artifact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildResourceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl BuildResourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Container {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(
        rename = "mapDockerSocket",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_docker_socket: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ports: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volumes: Vec<String>,
}
impl Container {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
}
impl ContainerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerResourceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ContainerResourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration parameters of the pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatePipelineConfigurationParameters {
    #[doc = "Type of configuration."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<create_pipeline_configuration_parameters::Type>,
}
impl CreatePipelineConfigurationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod create_pipeline_configuration_parameters {
    use super::*;
    #[doc = "Type of configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "yaml")]
        Yaml,
        #[serde(rename = "designerJson")]
        DesignerJson,
        #[serde(rename = "justInTime")]
        JustInTime,
        #[serde(rename = "designerHyphenJson")]
        DesignerHyphenJson,
    }
}
#[doc = "Parameters to create a pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatePipelineParameters {
    #[doc = "Configuration parameters of the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CreatePipelineConfigurationParameters>,
    #[doc = "Folder of the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[doc = "Name of the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CreatePipelineParameters {
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
#[doc = "Log for a pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Log {
    #[doc = "The date and time the log was created."]
    #[serde(
        rename = "createdOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The ID of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
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
    #[doc = "A signed url allowing limited-time anonymous access to private resources."]
    #[serde(
        rename = "signedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_content: Option<SignedUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Log {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogCollection {
    #[doc = "The list of logs."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logs: Vec<Log>,
    #[doc = "A signed url allowing limited-time anonymous access to private resources."]
    #[serde(
        rename = "signedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_content: Option<SignedUrl>,
    #[doc = "URL of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl LogCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageResourceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PackageResourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links")]
    pub links: ReferenceLinks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PipelineConfiguration>,
    #[doc = "URL of the pipeline"]
    pub url: String,
    #[doc = "Pipeline folder"]
    pub folder: String,
    #[doc = "Pipeline ID"]
    pub id: i32,
    #[doc = "Pipeline name"]
    pub name: String,
    #[doc = "Revision number"]
    pub revision: i32,
}
impl Pipeline {
    pub fn new(
        links: ReferenceLinks,
        url: String,
        folder: String,
        id: i32,
        name: String,
        revision: i32,
    ) -> Self {
        Self {
            links,
            configuration: None,
            url,
            folder,
            id,
            name,
            revision,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineBase {
    #[doc = "Pipeline folder"]
    pub folder: String,
    #[doc = "Pipeline ID"]
    pub id: i32,
    #[doc = "Pipeline name"]
    pub name: String,
    #[doc = "Revision number"]
    pub revision: i32,
}
impl PipelineBase {
    pub fn new(folder: String, id: i32, name: String, revision: i32) -> Self {
        Self {
            folder,
            id,
            name,
            revision,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfiguration {
    #[serde(rename = "type")]
    pub type_: pipeline_configuration::Type,
    pub path: String,
    pub repository: pipeline_configuration::Repository,
}
impl PipelineConfiguration {
    pub fn new(
        type_: pipeline_configuration::Type,
        path: String,
        repository: pipeline_configuration::Repository,
    ) -> Self {
        Self {
            type_,
            path,
            repository,
        }
    }
}
pub mod pipeline_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "yaml")]
        Yaml,
        #[serde(rename = "designerJson")]
        DesignerJson,
        #[serde(rename = "justInTime")]
        JustInTime,
        #[serde(rename = "designerHyphenJson")]
        DesignerHyphenJson,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Repository {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Repository {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Pipeline>,
}
impl PipelineList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reference to a Pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineReference {
    #[serde(flatten)]
    pub pipeline_base: PipelineBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PipelineReference {
    pub fn new(pipeline_base: PipelineBase) -> Self {
        Self {
            pipeline_base,
            url: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineResource {
    #[doc = "A reference to a Pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PipelineResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineResourceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PipelineResourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviewRun {
    #[serde(rename = "finalYaml", default, skip_serializing_if = "Option::is_none")]
    pub final_yaml: Option<String>,
}
impl PreviewRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class to represent a collection of REST reference links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLinks {
    #[doc = "Link URL"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<Link>,
}
impl ReferenceLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Repository {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<repository::Type>,
}
impl Repository {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod repository {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "gitHub")]
        GitHub,
        #[serde(rename = "azureReposGit")]
        AzureReposGit,
        #[serde(rename = "gitHubEnterprise")]
        GitHubEnterprise,
        #[serde(rename = "azureReposGitHyphenated")]
        AzureReposGitHyphenated,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryResource {
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl RepositoryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryResourceParameters {
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[doc = "This is the security token to use when connecting to the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Optional. This is the type of the token given. If not provided, a type of \"Bearer\" is assumed. Note: Use \"Basic\" for a PAT token."]
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl RepositoryResourceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Run {
    #[serde(flatten)]
    pub run_reference: RunReference,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links")]
    pub links: ReferenceLinks,
    #[serde(rename = "createdDate", with = "crate::date_time::rfc3339")]
    pub created_date: time::OffsetDateTime,
    #[serde(rename = "finalYaml", default, skip_serializing_if = "Option::is_none")]
    pub final_yaml: Option<String>,
    #[serde(
        rename = "finishedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub finished_date: Option<time::OffsetDateTime>,
    #[doc = "A reference to a Pipeline."]
    pub pipeline: PipelineReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<RunResources>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<run::Result>,
    pub state: run::State,
    #[serde(
        rename = "templateParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_parameters: Option<serde_json::Value>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
impl Run {
    pub fn new(
        run_reference: RunReference,
        links: ReferenceLinks,
        created_date: time::OffsetDateTime,
        pipeline: PipelineReference,
        state: run::State,
        url: String,
    ) -> Self {
        Self {
            run_reference,
            links,
            created_date,
            final_yaml: None,
            finished_date: None,
            pipeline,
            resources: None,
            result: None,
            state,
            template_parameters: None,
            url,
            variables: None,
        }
    }
}
pub mod run {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "canceling")]
        Canceling,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Run>,
}
impl RunList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings which influence pipeline runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunPipelineParameters {
    #[doc = "If true, don't actually create a new run. Instead, return the final YAML document after parsing templates."]
    #[serde(
        rename = "previewRun",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<RunResourcesParameters>,
    #[serde(
        rename = "stagesToSkip",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stages_to_skip: Vec<String>,
    #[serde(
        rename = "templateParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
    #[doc = "If you use the preview run option, you may optionally supply different YAML. This allows you to preview the final YAML document without committing a changed file."]
    #[serde(
        rename = "yamlOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub yaml_override: Option<String>,
}
impl RunPipelineParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunReference {
    pub id: i32,
    pub name: String,
}
impl RunReference {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<serde_json::Value>,
}
impl RunResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunResourcesParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builds: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<serde_json::Value>,
}
impl RunResourcesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRConnection {
    #[doc = "A signed url allowing limited-time anonymous access to private resources."]
    #[serde(
        rename = "signedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_content: Option<SignedUrl>,
}
impl SignalRConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A signed url allowing limited-time anonymous access to private resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignedUrl {
    #[doc = "Timestamp when access expires."]
    #[serde(
        rename = "signatureExpires",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub signature_expires: Option<time::OffsetDateTime>,
    #[doc = "The URL to allow access to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl SignedUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Variable {
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Variable {
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
