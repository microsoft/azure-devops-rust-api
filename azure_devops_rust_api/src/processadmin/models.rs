// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Describes an admin behavior for a process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminBehavior {
    #[doc = "Is the behavior abstract (i.e. can not be associated with any work item type)."]
    #[serde(rename = "abstract", default, skip_serializing_if = "Option::is_none")]
    pub abstract_: Option<bool>,
    #[doc = "The color associated with the behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Indicates if the behavior is custom."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[doc = "The description of the behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of behavior fields."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<AdminBehaviorField>,
    #[doc = "Behavior ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Parent behavior reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<String>,
    #[doc = "The behavior name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Is the behavior overrides a behavior from system process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overriden: Option<bool>,
    #[doc = "The rank."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}
impl AdminBehavior {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an admin behavior field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminBehaviorField {
    #[doc = "The behavior field identifier."]
    #[serde(
        rename = "behaviorFieldId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_field_id: Option<String>,
    #[doc = "The behavior ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The behavior name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AdminBehaviorField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminBehaviorList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AdminBehavior>,
}
impl AdminBehaviorList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes result of a check template existence request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckTemplateExistenceResult {
    #[doc = "Indicates whether a template exists."]
    #[serde(
        rename = "doesTemplateExist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub does_template_exist: Option<bool>,
    #[doc = "The name of the existing template."]
    #[serde(
        rename = "existingTemplateName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_template_name: Option<String>,
    #[doc = "The existing template type identifier."]
    #[serde(
        rename = "existingTemplateTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_template_type_id: Option<String>,
    #[doc = "The name of the requested template."]
    #[serde(
        rename = "requestedTemplateName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_template_name: Option<String>,
}
impl CheckTemplateExistenceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the result of a Process Import request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessImportResult {
    #[doc = "Describes result of a check template existence request."]
    #[serde(
        rename = "checkExistenceResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_existence_result: Option<CheckTemplateExistenceResult>,
    #[doc = "Help URL."]
    #[serde(rename = "helpUrl", default, skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
    #[doc = "ID of the import operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Whether this imported process is new."]
    #[serde(rename = "isNew", default, skip_serializing_if = "Option::is_none")]
    pub is_new: Option<bool>,
    #[doc = "The promote job identifier."]
    #[serde(
        rename = "promoteJobId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub promote_job_id: Option<String>,
    #[doc = "The list of validation results."]
    #[serde(
        rename = "validationResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_results: Vec<ValidationIssue>,
}
impl ProcessImportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes result of process operation promote."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessPromoteStatus {
    #[doc = "Number of projects for which promote is complete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete: Option<i32>,
    #[doc = "ID of the promote operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The error message associated with the promote operation. The string will be empty if there are no errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Number of projects for which promote is pending."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<i32>,
    #[doc = "The remaining retries."]
    #[serde(
        rename = "remainingRetries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_retries: Option<i32>,
    #[doc = "True if promote finished all the projects successfully. False if still in progress or any project promote failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
}
impl ProcessPromoteStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationIssue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<String>,
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<validation_issue::IssueType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
}
impl ValidationIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod validation_issue {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IssueType {
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
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
