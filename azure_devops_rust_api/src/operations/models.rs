// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Contains information about the progress or result of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(flatten)]
    pub operation_reference: OperationReference,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Detailed messaged about the status of an operation."]
    #[serde(
        rename = "detailedMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_message: Option<String>,
    #[doc = "Result message for an operation."]
    #[serde(
        rename = "resultMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_message: Option<String>,
    #[serde(rename = "resultUrl", default, skip_serializing_if = "Option::is_none")]
    pub result_url: Option<OperationResultReference>,
}
impl Operation {
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
pub struct OperationResultReference {
    #[doc = "URL to the operation result."]
    #[serde(rename = "resultUrl", default, skip_serializing_if = "Option::is_none")]
    pub result_url: Option<String>,
}
impl OperationResultReference {
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
