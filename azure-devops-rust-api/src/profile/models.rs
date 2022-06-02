#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeDescriptor {
    #[serde(
        rename = "attributeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attribute_name: Option<String>,
    #[serde(
        rename = "containerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributesContainer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(
        rename = "containerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Avatar {
    #[serde(
        rename = "isAutoGenerated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_auto_generated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<avatar::Size>,
    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreProfileAttribute {
    #[serde(flatten)]
    pub profile_attribute_base: ProfileAttributeBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[serde(
        rename = "profileState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_state: Option<create_profile_context::ProfileState>,
}
pub mod create_profile_context {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoRegion {
    #[serde(
        rename = "regionCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub region_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(
        rename = "applicationContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_container: Option<AttributesContainer>,
    #[serde(
        rename = "coreAttributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_attributes: Option<serde_json::Value>,
    #[serde(
        rename = "coreRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_revision: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "profileState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_state: Option<profile::ProfileState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}
pub mod profile {
    use super::*;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileAttribute {
    #[serde(flatten)]
    pub profile_attribute_base: ProfileAttributeBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileAttributeBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<AttributeDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileRegion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileRegions {
    #[serde(
        rename = "noticeContactConsentRequirementRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notice_contact_consent_requirement_regions: Vec<String>,
    #[serde(
        rename = "optOutContactConsentRequirementRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub opt_out_contact_consent_requirement_regions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<ProfileRegion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapper {
    #[serde(flatten)]
    pub vss_json_collection_wrapper_base: VssJsonCollectionWrapperBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapperBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
