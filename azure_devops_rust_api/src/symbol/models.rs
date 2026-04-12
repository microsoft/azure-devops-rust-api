// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A dual-purpose data object, the debug entry is used by the client to publish the symbol file (with file's blob identifier, which can be calculated from VSTS hashing algorithm) or query the file (with a client key). Since the symbol server tries to return a matched symbol file with the richest information level, it may not always point to the same symbol file for different queries with same client key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugEntry {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[doc = "BlobIdentifier with block hashes formatted to be deserialzied for symbol service."]
    #[serde(
        rename = "blobDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_details: Option<JsonBlobIdentifierWithBlocks>,
    #[serde(
        rename = "blobIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_identifier: Option<JsonBlobIdentifier>,
    #[doc = "The URI to get the symbol file. Provided by the server, the URI contains authentication information and is readily accessible by plain HTTP GET request. The client is recommended to retrieve the file as soon as it can since the URI will expire in a short period."]
    #[serde(rename = "blobUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_uri: Option<String>,
    #[doc = "A key the client (debugger, for example) uses to find the debug entry. Note it is not unique for each different symbol file as it does not distinguish between those which only differ by information level."]
    #[serde(rename = "clientKey", default, skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    #[serde(rename = "domainId", default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<IDomainId>,
    #[doc = "The information level this debug entry contains."]
    #[serde(
        rename = "informationLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub information_level: Option<debug_entry::InformationLevel>,
    #[doc = "The identifier of symbol request to which this debug entry belongs."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The size for the debug entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The status of debug entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<debug_entry::Status>,
}
impl DebugEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod debug_entry {
    use super::*;
    #[doc = "The information level this debug entry contains."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InformationLevel {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "binary")]
        Binary,
        #[serde(rename = "publics")]
        Publics,
        #[serde(rename = "traceFormatPresent")]
        TraceFormatPresent,
        #[serde(rename = "typeInfo")]
        TypeInfo,
        #[serde(rename = "lineNumbers")]
        LineNumbers,
        #[serde(rename = "globalSymbols")]
        GlobalSymbols,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "sourceIndexed")]
        SourceIndexed,
    }
    #[doc = "The status of debug entry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "blobMissing")]
        BlobMissing,
    }
}
#[doc = "A batch of debug entry to create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugEntryCreateBatch {
    #[doc = "Defines what to do when a debug entry in the batch already exists."]
    #[serde(
        rename = "createBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_behavior: Option<debug_entry_create_batch::CreateBehavior>,
    #[doc = "The debug entries."]
    #[serde(
        rename = "debugEntries",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub debug_entries: Vec<DebugEntry>,
    #[doc = "Serialized Proof nodes, used to verify uploads on server side for Chunk Dedup DebugEntry"]
    #[serde(
        rename = "proofNodes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub proof_nodes: Vec<String>,
}
impl DebugEntryCreateBatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod debug_entry_create_batch {
    use super::*;
    #[doc = "Defines what to do when a debug entry in the batch already exists."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateBehavior {
        #[serde(rename = "throwIfExists")]
        ThrowIfExists,
        #[serde(rename = "skipIfExists")]
        SkipIfExists,
        #[serde(rename = "overwriteIfExists")]
        OverwriteIfExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugEntryList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DebugEntry>,
}
impl DebugEntryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IDomainId {}
impl IDomainId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BlobBlock hash formatted to be deserialized for symbol service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonBlobBlockHash {
    #[doc = "Array of hash bytes."]
    #[serde(
        rename = "hashBytes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hash_bytes: Vec<String>,
}
impl JsonBlobBlockHash {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonBlobIdentifier {
    #[serde(
        rename = "identifierValue",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identifier_value: Vec<String>,
}
impl JsonBlobIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BlobIdentifier with block hashes formatted to be deserialzied for symbol service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonBlobIdentifierWithBlocks {
    #[doc = "List of blob block hashes."]
    #[serde(
        rename = "blockHashes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub block_hashes: Vec<JsonBlobBlockHash>,
    #[doc = "Array of blobId bytes."]
    #[serde(
        rename = "identifierValue",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identifier_value: Vec<String>,
}
impl JsonBlobIdentifierWithBlocks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Symbol request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Request {
    #[serde(flatten)]
    pub resource_base: ResourceBase,
    #[doc = "An optional human-facing description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainId", default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<IDomainId>,
    #[doc = "An optional expiration date for the request. The request will become inaccessible and get deleted after the date, regardless of its status.  On an HTTP POST, if expiration date is null/missing, the server will assign a default expiration data (30 days unless overwridden in the registry at the account level). On PATCH, if expiration date is null/missing, the behavior is to not change whatever the request's current expiration date is."]
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Indicates if request should be chunk dedup"]
    #[serde(rename = "isChunked", default, skip_serializing_if = "Option::is_none")]
    pub is_chunked: Option<bool>,
    #[doc = "A human-facing name for the request. Required on POST, ignored on PATCH."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The total Size for this request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The status for this request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<request::Status>,
}
impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod request {
    use super::*;
    #[doc = "The status for this request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "sealed")]
        Sealed,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceBase {
    #[doc = "The ID of user who created this item. Optional."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date time when this item is created. Optional."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "An identifier for this item. Optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "An opaque ETag used to synchronize with the version stored at server end. Optional."]
    #[serde(
        rename = "storageETag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_e_tag: Option<String>,
    #[doc = "A URI which can be used to retrieve this item in its raw format. Optional. Note this is distinguished from other URIs that are present in a derived resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ResourceBase {
    pub fn new() -> Self {
        Self::default()
    }
}
