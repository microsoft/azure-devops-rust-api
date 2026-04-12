// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Class that describes a request to add a field in a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddProcessWorkItemTypeFieldRequest {
    #[doc = "The list of field allowed values."]
    #[serde(
        rename = "allowedValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_values: Vec<String>,
    #[doc = "Allow setting field value to a group identity. Only applies to identity fields."]
    #[serde(
        rename = "allowGroups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_groups: Option<bool>,
    #[doc = "The default value of the field."]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<serde_json::Value>,
    #[doc = "If true the field cannot be edited."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Reference name of the field."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "If true the field cannot be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
impl AddProcessWorkItemTypeFieldRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent a control in the form."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Control {
    #[doc = "Properties of a work item form contribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contribution: Option<WitContribution>,
    #[doc = "Type of the control."]
    #[serde(
        rename = "controlType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub control_type: Option<String>,
    #[doc = "Height of the control, for html controls."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[doc = "The id for the layout node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A value indicating whether this layout node has been inherited. from a parent layout.  This is expected to only be only set by the combiner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[doc = "A value indicating if the layout node is contribution or not."]
    #[serde(
        rename = "isContribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_contribution: Option<bool>,
    #[doc = "Label for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Inner text of the control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[doc = "Order in which the control should appear in its group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "A value indicating whether this layout node has been overridden . by a child layout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
    #[doc = "A value indicating if the control is readonly."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "A value indicating if the control should be hidden or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[doc = "Watermark text for the textbox."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,
}
impl Control {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControlList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Control>,
}
impl ControlList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a process being created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProcessModel {
    #[doc = "Description of the process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the parent process"]
    #[serde(
        rename = "parentProcessTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_process_type_id: Option<String>,
    #[doc = "Reference name of process being created. If not specified, server will assign a unique reference name"]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
}
impl CreateProcessModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request object/class for creating a rule on a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProcessRuleRequest {
    #[doc = "List of actions to take when the rule is triggered."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<RuleAction>,
    #[doc = "List of conditions when the rule should be triggered."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<RuleCondition>,
    #[doc = "Indicates if the rule is disabled."]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Name for the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CreateProcessRuleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for create work item type request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProcessWorkItemTypeRequest {
    #[doc = "Color hexadecimal code to represent the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Description of the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Icon to represent the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Parent work item type for work item type"]
    #[serde(
        rename = "inheritsFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inherits_from: Option<String>,
    #[doc = "True if the work item type need to be disabled"]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Name of work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CreateProcessWorkItemTypeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the extensions part of the layout"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Extension {
    #[doc = "Id of the extension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl Extension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_identity: Option<bool>,
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<field_model::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl FieldModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod field_model {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "integer")]
        Integer,
        #[serde(rename = "dateTime")]
        DateTime,
        #[serde(rename = "plainText")]
        PlainText,
        #[serde(rename = "html")]
        Html,
        #[serde(rename = "treePath")]
        TreePath,
        #[serde(rename = "history")]
        History,
        #[serde(rename = "double")]
        Double,
        #[serde(rename = "guid")]
        Guid,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(rename = "identity")]
        Identity,
        #[serde(rename = "picklistInteger")]
        PicklistInteger,
        #[serde(rename = "picklistString")]
        PicklistString,
        #[serde(rename = "picklistDouble")]
        PicklistDouble,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldRuleModel {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<RuleActionModel>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<RuleConditionModel>,
    #[serde(
        rename = "friendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[serde(rename = "isSystem", default, skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,
}
impl FieldRuleModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the layout of a work item type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormLayout {
    #[doc = "Gets and sets extensions list."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extensions: Vec<Extension>,
    #[doc = "Top level tabs of the layout."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pages: Vec<Page>,
    #[doc = "Headers controls of the layout."]
    #[serde(
        rename = "systemControls",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub system_controls: Vec<Control>,
}
impl FormLayout {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent a group in the form that holds controls in it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Group {
    #[doc = "Properties of a work item form contribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contribution: Option<WitContribution>,
    #[doc = "Controls to be put in the group."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub controls: Vec<Control>,
    #[doc = "The height for the contribution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[doc = "The id for the layout node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A value indicating whether this layout node has been inherited from a parent layout.  This is expected to only be only set by the combiner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[doc = "A value indicating if the layout node is contribution are not."]
    #[serde(
        rename = "isContribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_contribution: Option<bool>,
    #[doc = "Label for the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Order in which the group should appear in the section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "A value indicating whether this layout node has been overridden by a child layout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
    #[doc = "A value indicating if the group should be hidden or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
impl Group {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class that describes the work item state is hidden."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HideStateModel {
    #[doc = "Returns 'true', if workitem state is hidden, 'false' otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}
impl HideStateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a page in the work item form layout"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Page {
    #[doc = "Properties of a work item form contribution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contribution: Option<WitContribution>,
    #[doc = "The id for the layout node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A value indicating whether this layout node has been inherited from a parent layout.  This is expected to only be only set by the combiner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[doc = "A value indicating if the layout node is contribution are not."]
    #[serde(
        rename = "isContribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_contribution: Option<bool>,
    #[doc = "The label for the page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "A value indicating whether any user operations are permitted on this page and the contents of this page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[doc = "Order in which the page should appear in the layout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "A value indicating whether this layout node has been overridden by a child layout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
    #[doc = "The icon for the page."]
    #[serde(rename = "pageType", default, skip_serializing_if = "Option::is_none")]
    pub page_type: Option<page::PageType>,
    #[doc = "The sections of the page."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sections: Vec<Section>,
    #[doc = "A value indicating if the page should be hidden or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
impl Page {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod page {
    use super::*;
    #[doc = "The icon for the page."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PageType {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "history")]
        History,
        #[serde(rename = "links")]
        Links,
        #[serde(rename = "attachments")]
        Attachments,
    }
}
#[doc = "Picklist."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PickList {
    #[serde(flatten)]
    pub pick_list_metadata: PickListMetadata,
    #[doc = "A list of PicklistItemModel."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl PickList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for picklist."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PickListMetadata {
    #[doc = "ID of the picklist"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates whether items outside of suggested list are allowed"]
    #[serde(
        rename = "isSuggested",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_suggested: Option<bool>,
    #[doc = "Name of the picklist"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "DataType of picklist"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Url of the picklist"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PickListMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PickListMetadataList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PickListMetadata>,
}
impl PickListMetadataList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Process Behavior Model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessBehavior {
    #[doc = "Color."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Indicates the type of customization on this work item. System behaviors are inherited from parent process but not modified. Inherited behaviors are modified behaviors that were inherited from parent process. Custom behaviors are behaviors created by user in current process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization: Option<process_behavior::Customization>,
    #[doc = ". Description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Process Behavior Fields."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<ProcessBehaviorField>,
    #[doc = "Process behavior Reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<ProcessBehaviorReference>,
    #[doc = "Behavior Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Rank of the behavior"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[doc = "Behavior Id"]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "Url of the behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessBehavior {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_behavior {
    use super::*;
    #[doc = "Indicates the type of customization on this work item. System behaviors are inherited from parent process but not modified. Inherited behaviors are modified behaviors that were inherited from parent process. Custom behaviors are behaviors created by user in current process."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Customization {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "inherited")]
        Inherited,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[doc = "Process Behavior Create Payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessBehaviorCreateRequest {
    #[doc = "Color."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Parent behavior id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<String>,
    #[doc = "Name of the behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ReferenceName is optional, if not specified will be auto-generated."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
}
impl ProcessBehaviorCreateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Process Behavior Field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessBehaviorField {
    #[doc = "Name of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Reference name of the field."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "Url to field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessBehaviorField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessBehaviorList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProcessBehavior>,
}
impl ProcessBehaviorList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Process behavior Reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessBehaviorReference {
    #[doc = "Id of a Behavior."]
    #[serde(
        rename = "behaviorRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_ref_name: Option<String>,
    #[doc = "Url to behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessBehaviorReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Process Behavior Replace Payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessBehaviorUpdateRequest {
    #[doc = "Color."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Behavior Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProcessBehaviorUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessInfo {
    #[doc = "Indicates the type of customization on this process. System Process is default process. Inherited Process is modified process that was System process before."]
    #[serde(
        rename = "customizationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customization_type: Option<process_info::CustomizationType>,
    #[doc = "Description of the process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Is the process default."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Is the process enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Name of the process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the parent process."]
    #[serde(
        rename = "parentProcessTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_process_type_id: Option<String>,
    #[doc = "Projects in this process to which the user is subscribed to."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub projects: Vec<ProjectReference>,
    #[doc = "Reference name of the process."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "The ID of the process."]
    #[serde(rename = "typeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
}
impl ProcessInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_info {
    use super::*;
    #[doc = "Indicates the type of customization on this process. System Process is default process. Inherited Process is modified process that was System process before."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomizationType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "inherited")]
        Inherited,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessInfoList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProcessInfo>,
}
impl ProcessInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessModel {
    #[doc = "Description of the process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Projects in this process"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub projects: Vec<ProjectReference>,
    #[doc = "Properties of the process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProcessProperties>,
    #[doc = "Reference name of the process"]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "The ID of the process"]
    #[serde(rename = "typeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
}
impl ProcessModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessProperties {
    #[doc = "Class of the process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<process_properties::Class>,
    #[doc = "Is the process default process."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Is the process enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "ID of the parent process."]
    #[serde(
        rename = "parentProcessTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_process_type_id: Option<String>,
    #[doc = "Version of the process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ProcessProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_properties {
    use super::*;
    #[doc = "Class of the process."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Class {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "derived")]
        Derived,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[doc = "Process Rule Response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessRule {
    #[serde(flatten)]
    pub create_process_rule_request: CreateProcessRuleRequest,
    #[doc = "Indicates if the rule is system generated or created by user."]
    #[serde(
        rename = "customizationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customization_type: Option<process_rule::CustomizationType>,
    #[doc = "Id to uniquely identify the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_rule {
    use super::*;
    #[doc = "Indicates if the rule is system generated or created by user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomizationType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "inherited")]
        Inherited,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessRuleList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProcessRule>,
}
impl ProcessRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class that describes a work item type object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessWorkItemType {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub behaviors: Vec<WorkItemTypeBehavior>,
    #[doc = "Color hexadecimal code to represent the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Indicates the type of customization on this work item System work item types are inherited from parent process but not modified Inherited work item types are modified work item that were inherited from parent process Custom work item types are work item types that were created in the current process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization: Option<process_work_item_type::Customization>,
    #[doc = "Description of the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Icon to represent the work item typ"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Reference name of the parent work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<String>,
    #[doc = "Indicates if a work item type is disabled"]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Describes the layout of a work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<FormLayout>,
    #[doc = "Name of the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Reference name of work item type"]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub states: Vec<WorkItemStateResultModel>,
    #[doc = "Url of the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessWorkItemType {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_work_item_type {
    use super::*;
    #[doc = "Indicates the type of customization on this work item System work item types are inherited from parent process but not modified Inherited work item types are modified work item that were inherited from parent process Custom work item types are work item types that were created in the current process"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Customization {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "inherited")]
        Inherited,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[doc = "Class that describes a field in a work item type and its properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessWorkItemTypeField {
    #[doc = "The list of field allowed values."]
    #[serde(
        rename = "allowedValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_values: Vec<serde_json::Value>,
    #[doc = "Allow setting field value to a group identity. Only applies to identity fields."]
    #[serde(
        rename = "allowGroups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_groups: Option<bool>,
    #[doc = "Indicates the type of customization on this work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization: Option<process_work_item_type_field::Customization>,
    #[doc = "The default value of the field."]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<serde_json::Value>,
    #[doc = "Description of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Information about field definition being locked for editing"]
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[doc = "Name of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If true the field cannot be edited."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Reference name of the field."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "If true the field cannot be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "Type of the field."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<process_work_item_type_field::Type>,
    #[doc = "Resource URL of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProcessWorkItemTypeField {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_work_item_type_field {
    use super::*;
    #[doc = "Indicates the type of customization on this work item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Customization {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "inherited")]
        Inherited,
        #[serde(rename = "custom")]
        Custom,
    }
    #[doc = "Type of the field."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "integer")]
        Integer,
        #[serde(rename = "dateTime")]
        DateTime,
        #[serde(rename = "plainText")]
        PlainText,
        #[serde(rename = "html")]
        Html,
        #[serde(rename = "treePath")]
        TreePath,
        #[serde(rename = "history")]
        History,
        #[serde(rename = "double")]
        Double,
        #[serde(rename = "guid")]
        Guid,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(rename = "identity")]
        Identity,
        #[serde(rename = "picklistInteger")]
        PicklistInteger,
        #[serde(rename = "picklistString")]
        PicklistString,
        #[serde(rename = "picklistDouble")]
        PicklistDouble,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessWorkItemTypeFieldList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProcessWorkItemTypeField>,
}
impl ProcessWorkItemTypeFieldList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessWorkItemTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProcessWorkItemType>,
}
impl ProcessWorkItemTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the project reference class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectReference {
    #[doc = "Description of the project"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ID of the project"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the project"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Url of the project"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ProjectReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action to take when the rule is triggered."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleAction {
    #[doc = "Type of action to take when the rule is triggered."]
    #[serde(
        rename = "actionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_type: Option<rule_action::ActionType>,
    #[doc = "Field on which the action should be taken."]
    #[serde(
        rename = "targetField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_field: Option<String>,
    #[doc = "Value to apply on target field, once the action is taken."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl RuleAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rule_action {
    use super::*;
    #[doc = "Type of action to take when the rule is triggered."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        #[serde(rename = "makeRequired")]
        MakeRequired,
        #[serde(rename = "makeReadOnly")]
        MakeReadOnly,
        #[serde(rename = "setDefaultValue")]
        SetDefaultValue,
        #[serde(rename = "setDefaultFromClock")]
        SetDefaultFromClock,
        #[serde(rename = "setDefaultFromCurrentUser")]
        SetDefaultFromCurrentUser,
        #[serde(rename = "setDefaultFromField")]
        SetDefaultFromField,
        #[serde(rename = "copyValue")]
        CopyValue,
        #[serde(rename = "copyFromClock")]
        CopyFromClock,
        #[serde(rename = "copyFromCurrentUser")]
        CopyFromCurrentUser,
        #[serde(rename = "copyFromField")]
        CopyFromField,
        #[serde(rename = "setValueToEmpty")]
        SetValueToEmpty,
        #[serde(rename = "copyFromServerClock")]
        CopyFromServerClock,
        #[serde(rename = "copyFromServerCurrentUser")]
        CopyFromServerCurrentUser,
        #[serde(rename = "hideTargetField")]
        HideTargetField,
        #[serde(rename = "disallowValue")]
        DisallowValue,
    }
}
#[doc = "Action to take when the rule is triggered."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleActionModel {
    #[serde(
        rename = "actionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_type: Option<String>,
    #[serde(
        rename = "targetField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl RuleActionModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a condition on a field when the rule should be triggered."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleCondition {
    #[doc = "Type of condition. $When. This condition limits the execution of its children to cases when another field has a particular value, i.e. when the Is value of the referenced field is equal to the given literal value. $WhenNot.This condition limits the execution of its children to cases when another field does not have a particular value, i.e.when the Is value of the referenced field is not equal to the given literal value. $WhenChanged.This condition limits the execution of its children to cases when another field has changed, i.e.when the Is value of the referenced field is not equal to the Was value of that field. $WhenNotChanged.This condition limits the execution of its children to cases when another field has not changed, i.e.when the Is value of the referenced field is equal to the Was value of that field."]
    #[serde(
        rename = "conditionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub condition_type: Option<rule_condition::ConditionType>,
    #[doc = "Field that defines condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[doc = "Value of field to define the condition for rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl RuleCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rule_condition {
    use super::*;
    #[doc = "Type of condition. $When. This condition limits the execution of its children to cases when another field has a particular value, i.e. when the Is value of the referenced field is equal to the given literal value. $WhenNot.This condition limits the execution of its children to cases when another field does not have a particular value, i.e.when the Is value of the referenced field is not equal to the given literal value. $WhenChanged.This condition limits the execution of its children to cases when another field has changed, i.e.when the Is value of the referenced field is not equal to the Was value of that field. $WhenNotChanged.This condition limits the execution of its children to cases when another field has not changed, i.e.when the Is value of the referenced field is equal to the Was value of that field."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConditionType {
        #[serde(rename = "when")]
        When,
        #[serde(rename = "whenNot")]
        WhenNot,
        #[serde(rename = "whenChanged")]
        WhenChanged,
        #[serde(rename = "whenNotChanged")]
        WhenNotChanged,
        #[serde(rename = "whenWas")]
        WhenWas,
        #[serde(rename = "whenStateChangedTo")]
        WhenStateChangedTo,
        #[serde(rename = "whenStateChangedFromAndTo")]
        WhenStateChangedFromAndTo,
        #[serde(rename = "whenWorkItemIsCreated")]
        WhenWorkItemIsCreated,
        #[serde(rename = "whenValueIsDefined")]
        WhenValueIsDefined,
        #[serde(rename = "whenValueIsNotDefined")]
        WhenValueIsNotDefined,
        #[serde(rename = "whenCurrentUserIsMemberOfGroup")]
        WhenCurrentUserIsMemberOfGroup,
        #[serde(rename = "whenCurrentUserIsNotMemberOfGroup")]
        WhenCurrentUserIsNotMemberOfGroup,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleConditionModel {
    #[serde(
        rename = "conditionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub condition_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl RuleConditionModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a section of the work item form layout"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Section {
    #[doc = "List of child groups in this section"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<Group>,
    #[doc = "The id for the layout node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A value indicating whether this layout node has been overridden by a child layout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
}
impl Section {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a request to update a process"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProcessModel {
    #[doc = "New description of the process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "If true new projects will use this process by default"]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "If false the process will be disabled and cannot be used to create projects"]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "New name of the process"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl UpdateProcessModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request class/object to update the rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProcessRuleRequest {
    #[serde(flatten)]
    pub create_process_rule_request: CreateProcessRuleRequest,
    #[doc = "Id to uniquely identify the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl UpdateProcessRuleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to describe a request that updates a field's properties in a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProcessWorkItemTypeFieldRequest {
    #[doc = "The list of field allowed values."]
    #[serde(
        rename = "allowedValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_values: Vec<String>,
    #[doc = "Allow setting field value to a group identity. Only applies to identity fields."]
    #[serde(
        rename = "allowGroups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_groups: Option<bool>,
    #[doc = "The default value of the field."]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<serde_json::Value>,
    #[doc = "If true the field cannot be edited."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "The default value of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
impl UpdateProcessWorkItemTypeFieldRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for update request on a work item type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProcessWorkItemTypeRequest {
    #[doc = "Color of the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Description of the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Icon of the work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "If set will disable the work item type"]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
}
impl UpdateProcessWorkItemTypeRequest {
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
#[doc = "Properties of a work item form contribution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WitContribution {
    #[doc = "The id for the contribution."]
    #[serde(
        rename = "contributionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contribution_id: Option<String>,
    #[doc = "The height for the contribution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[doc = "A dictionary holding key value pairs for contribution inputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "A value indicating if the contribution should be show on deleted workItem."]
    #[serde(
        rename = "showOnDeletedWorkItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_on_deleted_work_item: Option<bool>,
}
impl WitContribution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemBehavior {
    #[serde(rename = "abstract", default, skip_serializing_if = "Option::is_none")]
    pub abstract_: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<WorkItemBehaviorField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Reference to the behavior of a work item type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<WorkItemBehaviorReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overriden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemBehavior {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemBehaviorField {
    #[serde(
        rename = "behaviorFieldId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemBehaviorField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to the behavior of a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemBehaviorReference {
    #[doc = "The ID of the reference behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The url of the reference behavior."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemBehaviorReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class That represents a work item state input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemStateInputModel {
    #[doc = "Color of the state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Name of the state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Order in which state should appear"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Category of the state"]
    #[serde(
        rename = "stateCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_category: Option<String>,
}
impl WorkItemStateInputModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class that represents a work item state result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemStateResultModel {
    #[doc = "Work item state color."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Work item state customization type."]
    #[serde(
        rename = "customizationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customization_type: Option<work_item_state_result_model::CustomizationType>,
    #[doc = "If the Work item state is hidden."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[doc = "Id of the Workitemstate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Work item state name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Work item state order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Work item state statecategory."]
    #[serde(
        rename = "stateCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_category: Option<String>,
    #[doc = "Work item state url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemStateResultModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod work_item_state_result_model {
    use super::*;
    #[doc = "Work item state customization type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomizationType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "inherited")]
        Inherited,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemStateResultModelList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemStateResultModel>,
}
impl WorkItemStateResultModelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Association between a work item type and it's behavior"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeBehavior {
    #[doc = "Reference to the behavior of a work item type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<WorkItemBehaviorReference>,
    #[doc = "If true the work item type is the default work item type in the behavior"]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "If true the work item type is the default work item type in the parent behavior"]
    #[serde(
        rename = "isLegacyDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_legacy_default: Option<bool>,
    #[doc = "URL of the work item type behavior"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemTypeBehavior {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeBehaviorList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemTypeBehavior>,
}
impl WorkItemTypeBehaviorList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeModel {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub behaviors: Vec<WorkItemTypeBehavior>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<work_item_type_model::Class>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Parent WIT Id/Internal ReferenceName that it inherits from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<String>,
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Describes the layout of a work item type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<FormLayout>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub states: Vec<WorkItemStateResultModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemTypeModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod work_item_type_model {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Class {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "derived")]
        Derived,
        #[serde(rename = "custom")]
        Custom,
    }
}
