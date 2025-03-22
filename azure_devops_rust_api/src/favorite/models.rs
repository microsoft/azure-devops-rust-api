// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactProperties {}
impl ArtifactProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the scope a favorited Artifact resides in. e.g. A team project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactScope {
    #[doc = "The identifier of the scope the artifact resides in. For a TFS Project, this refers to the Project GUID string. For a Collection, marked this property with an empty string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the artifact scope (e.g. Project Name)  Note: This property is a read-only extension over the stored favorite model. This value cannot be overridden on writes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of scope the favorite artifact resides in. Known scopes include \"Project\" or \"Collection\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ArtifactScope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Implementation of Favorite contract following modern storage"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Favorite {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "ID of the favorited artifact, unique in context of this artifact type."]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[doc = "Indicates if the artifact described by this favorite could not be located."]
    #[serde(
        rename = "artifactIsDeleted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_is_deleted: Option<bool>,
    #[doc = "Last known name of the artifact."]
    #[serde(
        rename = "artifactName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_name: Option<String>,
    #[serde(
        rename = "artifactProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_properties: Option<ArtifactProperties>,
    #[doc = "Describes the scope a favorited Artifact resides in. e.g. A team project."]
    #[serde(
        rename = "artifactScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_scope: Option<ArtifactScope>,
    #[doc = "Type of artifact."]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[doc = "Date and time this Favorite was created on server."]
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Unique Id of the favorite item, defined by server at creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "Fully-Qualified link to this Resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Favorite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteCreateParameters {
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(
        rename = "artifactName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_name: Option<String>,
    #[serde(
        rename = "artifactProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_properties: Option<ArtifactProperties>,
    #[doc = "Describes the scope a favorited Artifact resides in. e.g. A team project."]
    #[serde(
        rename = "artifactScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_scope: Option<ArtifactScope>,
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
}
impl FavoriteCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Favorite>,
}
impl FavoriteList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exposes a provider of favorites."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteProvider {
    #[serde(flatten)]
    pub favorites_secured_object: FavoritesSecuredObject,
    #[doc = "Favorite artifact type"]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[doc = "URI for retrieving favorite artifacts"]
    #[serde(
        rename = "artifactUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_uri: Option<String>,
    #[doc = "contributed client side service that is available for this provider to provide dynamic associated data."]
    #[serde(
        rename = "clientServiceIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_service_identifier: Option<String>,
    #[doc = "Contribution Id"]
    #[serde(
        rename = "contributionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contribution_id: Option<String>,
    #[doc = "Css class to be applied to the icon for the artifact."]
    #[serde(rename = "iconClass", default, skip_serializing_if = "Option::is_none")]
    pub icon_class: Option<String>,
    #[doc = "Name of the fabric icon to be applied for the artifact"]
    #[serde(rename = "iconName", default, skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,
    #[doc = "Group of favorites will be rendered in this order, 0 is top If 2 types share order, they will be coalesced into a single group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Name used for rendering the title of each group of favorites"]
    #[serde(
        rename = "pluralName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub plural_name: Option<String>,
    #[doc = "Service identifier of the service."]
    #[serde(
        rename = "serviceIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_identifier: Option<String>,
    #[doc = "Base URI of the service"]
    #[serde(
        rename = "serviceUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_uri: Option<String>,
}
impl FavoriteProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoritesSecuredObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl FavoritesSecuredObject {
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
