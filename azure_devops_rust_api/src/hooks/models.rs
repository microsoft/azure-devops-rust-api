// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Defines the data contract of a consumer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Consumer {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets this consumer's actions."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<ConsumerAction>,
    #[doc = "Gets or sets this consumer's authentication type."]
    #[serde(
        rename = "authenticationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_type: Option<consumer::AuthenticationType>,
    #[doc = "Gets or sets this consumer's localized description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Describes how to configure a subscription that is managed externally."]
    #[serde(
        rename = "externalConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_configuration: Option<ExternalConfigurationDescriptor>,
    #[doc = "Gets or sets this consumer's identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets this consumer's image URL, if any."]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[doc = "Gets or sets this consumer's information URL, if any."]
    #[serde(
        rename = "informationUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub information_url: Option<String>,
    #[doc = "Gets or sets this consumer's input descriptors."]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "Gets or sets this consumer's localized name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The url for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Consumer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod consumer {
    use super::*;
    #[doc = "Gets or sets this consumer's authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthenticationType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "oAuth")]
        OAuth,
        #[serde(rename = "external")]
        External,
    }
}
#[doc = "Defines the data contract of a consumer action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerAction {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets or sets the flag indicating if resource version can be overridden when creating or editing a subscription."]
    #[serde(
        rename = "allowResourceVersionOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_resource_version_override: Option<bool>,
    #[doc = "Gets or sets the identifier of the consumer to which this action belongs."]
    #[serde(
        rename = "consumerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_id: Option<String>,
    #[doc = "Gets or sets this action's localized description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets this action's identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets this action's input descriptors."]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "Gets or sets this action's localized name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets this action's supported event identifiers."]
    #[serde(
        rename = "supportedEventTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_event_types: Vec<String>,
    #[doc = "Gets or sets this action's supported resource versions."]
    #[serde(
        rename = "supportedResourceVersions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_resource_versions: Option<serde_json::Value>,
    #[doc = "The url for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ConsumerAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerActionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConsumerAction>,
}
impl ConsumerActionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Consumer>,
}
impl ConsumerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encapsulates the properties of an event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[doc = "Gets or sets the UTC-based date and time that this event was created."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Provides different formats of an event message"]
    #[serde(
        rename = "detailedMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_message: Option<FormattedEventMessage>,
    #[doc = "Gets or sets the type of this event."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "Gets or sets the unique identifier of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Provides different formats of an event message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<FormattedEventMessage>,
    #[doc = "Gets or sets the identifier of the publisher that raised this event."]
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[doc = "Gets or sets the data associated with this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[doc = "Gets or sets the resource containers."]
    #[serde(
        rename = "resourceContainers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_containers: Option<serde_json::Value>,
    #[doc = "Gets or sets the version of the data associated with this event."]
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<String>,
    #[doc = "Represents a session token to be attached in Events for Consumer actions that need it."]
    #[serde(
        rename = "sessionToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_token: Option<SessionToken>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a type of event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventTypeDescriptor {
    #[doc = "A localized description of the event type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A unique id for the event type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Event-specific inputs"]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "A localized friendly name for the event type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A unique id for the publisher of this event type"]
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[doc = "Supported versions for the event's resource payloads."]
    #[serde(
        rename = "supportedResourceVersions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_resource_versions: Vec<String>,
    #[doc = "The url for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl EventTypeDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventTypeDescriptorList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EventTypeDescriptor>,
}
impl EventTypeDescriptorList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes how to configure a subscription that is managed externally."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalConfigurationDescriptor {
    #[doc = "Url of the site to create this type of subscription."]
    #[serde(
        rename = "createSubscriptionUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_subscription_url: Option<String>,
    #[doc = "The name of an input property that contains the URL to edit a subscription."]
    #[serde(
        rename = "editSubscriptionPropertyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edit_subscription_property_name: Option<String>,
    #[doc = "True if the external configuration applies only to hosted."]
    #[serde(
        rename = "hostedOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hosted_only: Option<bool>,
}
impl ExternalConfigurationDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides different formats of an event message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormattedEventMessage {
    #[doc = "Gets or sets the html format of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[doc = "Gets or sets the markdown format of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    #[doc = "Gets or sets the raw text of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl FormattedEventMessage {
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
#[doc = "Defines a filter for subscription inputs. The filter matches a set of inputs if any (one or more) of the groups evaluates to true."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputFilter {
    #[doc = "Groups of input filter expressions. This filter matches a set of inputs if any (one or more) of the groups evaluates to true."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<InputFilterCondition>,
}
impl InputFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An expression which can be applied to filter a list of subscription inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputFilterCondition {
    #[doc = "Whether or not to do a case sensitive match"]
    #[serde(
        rename = "caseSensitive",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub case_sensitive: Option<bool>,
    #[doc = "The Id of the input to filter on"]
    #[serde(rename = "inputId", default, skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[doc = "The \"expected\" input value to compare with the actual input value"]
    #[serde(
        rename = "inputValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub input_value: Option<String>,
    #[doc = "The operator applied between the expected and actual input value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<input_filter_condition::Operator>,
}
impl InputFilterCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod input_filter_condition {
    use super::*;
    #[doc = "The operator applied between the expected and actual input value"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        #[serde(rename = "equals")]
        Equals,
        #[serde(rename = "notEquals")]
        NotEquals,
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
    pub max_value: Option<f32>,
    #[doc = "Gets or sets the minimum length of this descriptor."]
    #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[doc = "Gets or sets the minimum value for this descriptor."]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f32>,
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
#[doc = "Defines the data contract of the result of processing an event for a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Notification {
    #[doc = "Gets or sets date and time that this result was created."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Defines the data contract of notification details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<NotificationDetails>,
    #[doc = "The event id associated with this notification"]
    #[serde(rename = "eventId", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[doc = "The notification id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets or sets date and time that this result was last modified."]
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "Result of the notification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<notification::Result>,
    #[doc = "Status of the notification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<notification::Status>,
    #[doc = "The subscriber Id  associated with this notification. This is the last identity who touched in the subscription. In case of test notifications it can be the tester if the subscription is not created yet."]
    #[serde(
        rename = "subscriberId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscriber_id: Option<String>,
    #[doc = "The subscription id associated with this notification"]
    #[serde(
        rename = "subscriptionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_id: Option<String>,
}
impl Notification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod notification {
    use super::*;
    #[doc = "Result of the notification"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "filtered")]
        Filtered,
    }
    #[doc = "Status of the notification"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "processing")]
        Processing,
        #[serde(rename = "requestInProgress")]
        RequestInProgress,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[doc = "Defines the data contract of notification details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationDetails {
    #[doc = "Gets or sets the time that this notification was completed (response received from the consumer)"]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets this notification detail's consumer action identifier."]
    #[serde(
        rename = "consumerActionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_action_id: Option<String>,
    #[doc = "Gets or sets this notification detail's consumer identifier."]
    #[serde(
        rename = "consumerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_id: Option<String>,
    #[doc = "Gets or sets this notification detail's consumer inputs."]
    #[serde(
        rename = "consumerInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_inputs: Option<serde_json::Value>,
    #[doc = "Gets or sets the time that this notification was dequeued for processing"]
    #[serde(
        rename = "dequeuedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub dequeued_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets this notification detail's error detail."]
    #[serde(
        rename = "errorDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_detail: Option<String>,
    #[doc = "Gets or sets this notification detail's error message."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Encapsulates the properties of an event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    #[doc = "Gets or sets this notification detail's event type."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "Gets or sets the time that this notification was finished processing (just before the request is sent to the consumer)"]
    #[serde(
        rename = "processedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub processed_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets this notification detail's publisher identifier."]
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[doc = "Gets or sets this notification detail's publisher inputs."]
    #[serde(
        rename = "publisherInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_inputs: Option<serde_json::Value>,
    #[doc = "Gets or sets the time that this notification was queued (created)"]
    #[serde(
        rename = "queuedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub queued_date: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets this notification detail's request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[doc = "Number of requests attempted to be sent to the consumer"]
    #[serde(
        rename = "requestAttempts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_attempts: Option<i32>,
    #[doc = "Duration of the request to the consumer in seconds"]
    #[serde(
        rename = "requestDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_duration: Option<f64>,
    #[doc = "Gets or sets this notification detail's response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
}
impl NotificationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Notification>,
}
impl NotificationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of a particular result and count."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationResultsSummaryDetail {
    #[doc = "Count of notification sent out with a matching result."]
    #[serde(
        rename = "notificationCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_count: Option<i32>,
    #[doc = "Result of the notification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<notification_results_summary_detail::Result>,
}
impl NotificationResultsSummaryDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod notification_results_summary_detail {
    use super::*;
    #[doc = "Result of the notification"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "filtered")]
        Filtered,
    }
}
#[doc = "Summary of the notifications for a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSummary {
    #[doc = "The notification results for this particular subscription."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<NotificationResultsSummaryDetail>,
    #[doc = "The subscription id associated with this notification"]
    #[serde(
        rename = "subscriptionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_id: Option<String>,
}
impl NotificationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a query for service hook notifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationsQuery {
    #[doc = "The subscriptions associated with the notifications returned from the query"]
    #[serde(
        rename = "associatedSubscriptions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub associated_subscriptions: Vec<Subscription>,
    #[doc = "If true, we will return all notification history for the query provided; otherwise, the summary is returned."]
    #[serde(
        rename = "includeDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_details: Option<bool>,
    #[doc = "Optional maximum date at which the notification was created"]
    #[serde(
        rename = "maxCreatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub max_created_date: Option<time::OffsetDateTime>,
    #[doc = "Optional maximum number of overall results to include"]
    #[serde(
        rename = "maxResults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_results: Option<i32>,
    #[doc = "Optional maximum number of results for each subscription. Only takes effect when a list of subscription ids is supplied in the query."]
    #[serde(
        rename = "maxResultsPerSubscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_results_per_subscription: Option<i32>,
    #[doc = "Optional minimum date at which the notification was created"]
    #[serde(
        rename = "minCreatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub min_created_date: Option<time::OffsetDateTime>,
    #[doc = "Optional publisher id to restrict the results to"]
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[doc = "Results from the query"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<Notification>,
    #[doc = "Optional notification result type to filter results to"]
    #[serde(
        rename = "resultType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_type: Option<notifications_query::ResultType>,
    #[doc = "Optional notification status to filter results to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<notifications_query::Status>,
    #[doc = "Optional list of subscription ids to restrict the results to"]
    #[serde(
        rename = "subscriptionIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscription_ids: Vec<String>,
    #[doc = "Summary of notifications - the count of each result type (success, fail, ..)."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub summary: Vec<NotificationSummary>,
}
impl NotificationsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod notifications_query {
    use super::*;
    #[doc = "Optional notification result type to filter results to"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultType {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "filtered")]
        Filtered,
    }
    #[doc = "Optional notification status to filter results to"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "processing")]
        Processing,
        #[serde(rename = "requestInProgress")]
        RequestInProgress,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[doc = "Defines the data contract of an event publisher."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Publisher {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Gets this publisher's localized description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets this publisher's identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Publisher-specific inputs"]
    #[serde(
        rename = "inputDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_descriptors: Vec<InputDescriptor>,
    #[doc = "Gets this publisher's localized name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The service instance type of the first party publisher."]
    #[serde(
        rename = "serviceInstanceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_instance_type: Option<String>,
    #[doc = "Gets this publisher's supported event types."]
    #[serde(
        rename = "supportedEvents",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_events: Vec<EventTypeDescriptor>,
    #[doc = "The url for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Publisher {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wrapper around an event which is being published"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublisherEvent {
    #[doc = "Add key/value pairs which will be stored with a published notification in the SH service DB.  This key/value pairs are for diagnostic purposes only and will have not effect on the delivery of a notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<serde_json::Value>,
    #[doc = "Encapsulates the properties of an event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    #[doc = "Gets or sets flag for filtered events"]
    #[serde(
        rename = "isFilteredEvent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_filtered_event: Option<bool>,
    #[doc = "Additional data that needs to be sent as part of notification to complement the Resource data in the Event"]
    #[serde(
        rename = "notificationData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_data: Option<serde_json::Value>,
    #[doc = "Gets or sets the array of older supported resource versions."]
    #[serde(
        rename = "otherResourceVersions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_resource_versions: Vec<VersionedResource>,
    #[doc = "Optional publisher-input filters which restricts the set of subscriptions which are triggered by the event"]
    #[serde(
        rename = "publisherInputFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub publisher_input_filters: Vec<InputFilter>,
    #[doc = "Encapsulates an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}
impl PublisherEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublisherList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Publisher>,
}
impl PublisherList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a query for service hook publishers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishersQuery {
    #[doc = "Optional list of publisher ids to restrict the results to"]
    #[serde(
        rename = "publisherIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub publisher_ids: Vec<String>,
    #[doc = "Filter for publisher inputs"]
    #[serde(
        rename = "publisherInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_inputs: Option<serde_json::Value>,
    #[doc = "Results from the query"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<Publisher>,
}
impl PublishersQuery {
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
#[doc = "The base class for all resource containers, i.e. Account, Collection, Project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceContainer {
    #[doc = "Gets or sets the container's base URL, i.e. the URL of the host (collection, application, or deployment) containing the container resource."]
    #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[doc = "Gets or sets the container's specific Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the container's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the container's REST API URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ResourceContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a session token to be attached in Events for Consumer actions that need it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionToken {
    #[doc = "The error message in case of error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The access token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "The expiration date in UTC"]
    #[serde(
        rename = "validTo",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub valid_to: Option<time::OffsetDateTime>,
}
impl SessionToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encapsulates an event subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subscription {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(
        rename = "actionDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_description: Option<String>,
    #[serde(
        rename = "consumerActionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_action_id: Option<String>,
    #[serde(
        rename = "consumerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_id: Option<String>,
    #[doc = "Consumer input values"]
    #[serde(
        rename = "consumerInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_inputs: Option<serde_json::Value>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "eventDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_description: Option<String>,
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "lastProbationRetryDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_probation_retry_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "probationRetries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub probation_retries: Option<i32>,
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[doc = "Publisher input values"]
    #[serde(
        rename = "publisherInputs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_inputs: Option<serde_json::Value>,
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<subscription::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber: Option<IdentityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Subscription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "onProbation")]
        OnProbation,
        #[serde(rename = "disabledByUser")]
        DisabledByUser,
        #[serde(rename = "disabledBySystem")]
        DisabledBySystem,
        #[serde(rename = "disabledByInactiveIdentity")]
        DisabledByInactiveIdentity,
    }
}
#[doc = "Contains all the diagnostics settings for a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionDiagnostics {
    #[doc = "Data controlling a single diagnostic setting for a subscription."]
    #[serde(
        rename = "deliveryResults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_results: Option<SubscriptionTracing>,
    #[doc = "Data controlling a single diagnostic setting for a subscription."]
    #[serde(
        rename = "deliveryTracing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_tracing: Option<SubscriptionTracing>,
    #[doc = "Data controlling a single diagnostic setting for a subscription."]
    #[serde(
        rename = "evaluationTracing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evaluation_tracing: Option<SubscriptionTracing>,
}
impl SubscriptionDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query for obtaining information about the possible/allowed values for one or more subscription inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionInputValuesQuery {
    #[doc = "The input values to return on input, and the result from the consumer on output."]
    #[serde(
        rename = "inputValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_values: Vec<InputValues>,
    #[doc = "The scope at which the properties to query belong"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<subscription_input_values_query::Scope>,
    #[doc = "Encapsulates an event subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}
impl SubscriptionInputValuesQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_input_values_query {
    use super::*;
    #[doc = "The scope at which the properties to query belong"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        #[serde(rename = "publisher")]
        Publisher,
        #[serde(rename = "consumer")]
        Consumer,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Subscription>,
}
impl SubscriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data controlling a single diagnostic setting for a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionTracing {
    #[doc = "Indicates whether the diagnostic tracing is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Trace until the specified end date."]
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "The maximum number of result details to trace."]
    #[serde(
        rename = "maxTracedEntries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_traced_entries: Option<i32>,
    #[doc = "The date and time tracing started."]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Trace until remaining count reaches 0."]
    #[serde(
        rename = "tracedEntries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traced_entries: Option<i32>,
}
impl SubscriptionTracing {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a query for service hook subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionsQuery {
    #[doc = "Optional consumer action id to restrict the results to (null for any)"]
    #[serde(
        rename = "consumerActionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_action_id: Option<String>,
    #[doc = "Optional consumer id to restrict the results to (null for any)"]
    #[serde(
        rename = "consumerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_id: Option<String>,
    #[doc = "Filter for subscription consumer inputs"]
    #[serde(
        rename = "consumerInputFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub consumer_input_filters: Vec<InputFilter>,
    #[doc = "Optional event type id to restrict the results to (null for any)"]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "Optional publisher id to restrict the results to (null for any)"]
    #[serde(
        rename = "publisherId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_id: Option<String>,
    #[doc = "Filter for subscription publisher inputs"]
    #[serde(
        rename = "publisherInputFilters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub publisher_input_filters: Vec<InputFilter>,
    #[doc = "Results from the query"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<Subscription>,
    #[doc = "Optional subscriber filter."]
    #[serde(
        rename = "subscriberId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscriber_id: Option<String>,
}
impl SubscriptionsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to update diagnostics settings for a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSubscripitonDiagnosticsParameters {
    #[doc = "Parameters to update a specific diagnostic setting."]
    #[serde(
        rename = "deliveryResults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_results: Option<UpdateSubscripitonTracingParameters>,
    #[doc = "Parameters to update a specific diagnostic setting."]
    #[serde(
        rename = "deliveryTracing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_tracing: Option<UpdateSubscripitonTracingParameters>,
    #[doc = "Parameters to update a specific diagnostic setting."]
    #[serde(
        rename = "evaluationTracing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evaluation_tracing: Option<UpdateSubscripitonTracingParameters>,
}
impl UpdateSubscripitonDiagnosticsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to update a specific diagnostic setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSubscripitonTracingParameters {
    #[doc = "Indicates whether to enable to disable the diagnostic tracing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl UpdateSubscripitonTracingParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encapsulates the resource version and its data or reference to the compatible version. Only one of the two last fields should be not null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionedResource {
    #[doc = "Gets or sets the reference to the compatible version."]
    #[serde(
        rename = "compatibleWith",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatible_with: Option<String>,
    #[doc = "Gets or sets the resource data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[doc = "Gets or sets the version of the resource data."]
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<String>,
}
impl VersionedResource {
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
