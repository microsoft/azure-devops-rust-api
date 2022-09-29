// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Identifies an attribute with a name and a container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeDescriptor {
    #[doc = "The name of the attribute."]
    #[serde(
        rename = "attributeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attribute_name: Option<String>,
    #[doc = "The container the attribute resides in."]
    #[serde(
        rename = "containerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_name: Option<String>,
}
impl AttributeDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stores a set of named profile attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributesContainer {
    #[doc = "The attributes stored by the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "The name of the container."]
    #[serde(
        rename = "containerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_name: Option<String>,
    #[doc = "The maximum revision number of any attribute within the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl AttributesContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Avatar {
    #[serde(
        rename = "isAutoGenerated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_auto_generated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<avatar::Size>,
    #[serde(
        rename = "timeStamp",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub time_stamp: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl Avatar {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod avatar {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        #[serde(rename = "small")]
        Small,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "large")]
        Large,
    }
}
#[doc = "A profile attribute which always has a value for each profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CoreProfileAttribute {
    #[serde(flatten)]
    pub profile_attribute_base: ProfileAttributeBase,
}
impl CoreProfileAttribute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProfileContext {
    #[serde(rename = "ciData", default, skip_serializing_if = "Option::is_none")]
    pub ci_data: Option<serde_json::Value>,
    #[serde(
        rename = "contactWithOffers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_with_offers: Option<bool>,
    #[serde(
        rename = "countryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_name: Option<String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        rename = "emailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_address: Option<String>,
    #[serde(
        rename = "hasAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_account: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(
        rename = "phoneNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number: Option<String>,
    #[doc = "The current state of the profile."]
    #[serde(
        rename = "profileState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_state: Option<create_profile_context::ProfileState>,
}
impl CreateProfileContext {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod create_profile_context {
    use super::*;
    #[doc = "The current state of the profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProfileState {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "customReadOnly")]
        CustomReadOnly,
        #[serde(rename = "readOnly")]
        ReadOnly,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeoRegion {
    #[serde(
        rename = "regionCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub region_code: Option<String>,
}
impl GeoRegion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a JSON object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[doc = "Gets the node type for this JToken."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl JObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A user profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Profile {
    #[doc = "Stores a set of named profile attributes."]
    #[serde(
        rename = "applicationContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_container: Option<AttributesContainer>,
    #[doc = "The core attributes of this profile."]
    #[serde(
        rename = "coreAttributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_attributes: Option<serde_json::Value>,
    #[doc = "The maximum revision number of any attribute."]
    #[serde(
        rename = "coreRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_revision: Option<i32>,
    #[doc = "The unique identifier of the profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The current state of the profile."]
    #[serde(
        rename = "profileState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_state: Option<profile::ProfileState>,
    #[doc = "The maximum revision number of any attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "The time at which this profile was last changed."]
    #[serde(
        rename = "timeStamp",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub time_stamp: Option<time::OffsetDateTime>,
}
impl Profile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod profile {
    use super::*;
    #[doc = "The current state of the profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProfileState {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "customReadOnly")]
        CustomReadOnly,
        #[serde(rename = "readOnly")]
        ReadOnly,
    }
}
#[doc = "A named object associated with a profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileAttribute {
    #[serde(flatten)]
    pub profile_attribute_base: ProfileAttributeBase,
}
impl ProfileAttribute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileAttributeBase {
    #[doc = "Identifies an attribute with a name and a container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<AttributeDescriptor>,
    #[doc = "The revision number of the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "The time the attribute was last changed."]
    #[serde(
        rename = "timeStamp",
        default,
        with = "crate::date_time::rfc3339::option"
    )]
    pub time_stamp: Option<time::OffsetDateTime>,
    #[doc = "The value of the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ProfileAttributeBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Country/region information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileRegion {
    #[doc = "The two-letter code defined in ISO 3166 for the country/region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Localized country/region name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ProfileRegion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container of country/region information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileRegions {
    #[doc = "List of country/region code with contact consent requirement type of notice"]
    #[serde(
        rename = "noticeContactConsentRequirementRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notice_contact_consent_requirement_regions: Vec<String>,
    #[doc = "List of country/region code with contact consent requirement type of opt-out"]
    #[serde(
        rename = "optOutContactConsentRequirementRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub opt_out_contact_consent_requirement_regions: Vec<String>,
    #[doc = "List of country/regions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<ProfileRegion>,
}
impl ProfileRegions {
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
