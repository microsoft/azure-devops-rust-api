// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Account {
    #[doc = "Identifier for an Account"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Name for an account"]
    #[serde(
        rename = "accountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    #[doc = "Owner of account"]
    #[serde(
        rename = "accountOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_owner: Option<String>,
    #[doc = "Current account status"]
    #[serde(
        rename = "accountStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_status: Option<account::AccountStatus>,
    #[doc = "Type of account: Personal, Organization"]
    #[serde(
        rename = "accountType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_type: Option<account::AccountType>,
    #[doc = "Uri for an account"]
    #[serde(
        rename = "accountUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_uri: Option<String>,
    #[doc = "Who created the account"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Date account was created"]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[serde(rename = "hasMoved", default, skip_serializing_if = "Option::is_none")]
    pub has_moved: Option<bool>,
    #[doc = "Identity of last person to update the account"]
    #[serde(
        rename = "lastUpdatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_by: Option<String>,
    #[doc = "Date account was last updated"]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "Namespace for an account"]
    #[serde(
        rename = "namespaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_id: Option<String>,
    #[serde(
        rename = "newCollectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_collection_id: Option<String>,
    #[doc = "Organization that created the account"]
    #[serde(
        rename = "organizationName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_name: Option<String>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "Reason for current status"]
    #[serde(
        rename = "statusReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_reason: Option<String>,
}
impl Account {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account {
    use super::*;
    #[doc = "Current account status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountStatus {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "moved")]
        Moved,
    }
    #[doc = "Type of account: Personal, Organization"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        #[serde(rename = "personal")]
        Personal,
        #[serde(rename = "organization")]
        Organization,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountCreateInfoInternal {
    #[serde(
        rename = "accountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<AccountPreferencesInternal>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[serde(
        rename = "serviceDefinitions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_definitions: Vec<serde_json::Value>,
}
impl AccountCreateInfoInternal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Account>,
}
impl AccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountPreferencesInternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub culture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
impl AccountPreferencesInternal {
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
