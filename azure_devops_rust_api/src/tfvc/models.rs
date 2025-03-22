// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociatedWorkItem {
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<String>,
    #[doc = "Id of associated the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "REST Url of the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
impl AssociatedWorkItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociatedWorkItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AssociatedWorkItem>,
}
impl AssociatedWorkItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Change {
    #[doc = "The type of change that was made to the item."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<change::ChangeType>,
    #[doc = "Current version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(
        rename = "newContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_content: Option<ItemContent>,
    #[doc = "Path of the item on the server."]
    #[serde(
        rename = "sourceServerItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_server_item: Option<String>,
    #[doc = "URL to retrieve the item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Change {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod change {
    use super::*;
    #[doc = "The type of change that was made to the item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "edit")]
        Edit,
        #[serde(rename = "encoding")]
        Encoding,
        #[serde(rename = "rename")]
        Rename,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "undelete")]
        Undelete,
        #[serde(rename = "branch")]
        Branch,
        #[serde(rename = "merge")]
        Merge,
        #[serde(rename = "lock")]
        Lock,
        #[serde(rename = "rollback")]
        Rollback,
        #[serde(rename = "sourceRename")]
        SourceRename,
        #[serde(rename = "targetRename")]
        TargetRename,
        #[serde(rename = "property")]
        Property,
        #[serde(rename = "all")]
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckinNote {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CheckinNote {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileContentMetadata {
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "isBinary", default, skip_serializing_if = "Option::is_none")]
    pub is_binary: Option<bool>,
    #[serde(rename = "isImage", default, skip_serializing_if = "Option::is_none")]
    pub is_image: Option<bool>,
    #[serde(rename = "vsLink", default, skip_serializing_if = "Option::is_none")]
    pub vs_link: Option<String>,
}
impl FileContentMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRepository {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "defaultBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "True if the repository is disabled. False otherwise."]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "True if the repository was created as a fork."]
    #[serde(rename = "isFork", default, skip_serializing_if = "Option::is_none")]
    pub is_fork: Option<bool>,
    #[doc = "True if the repository is in maintenance. False otherwise."]
    #[serde(
        rename = "isInMaintenance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_in_maintenance: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "parentRepository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_repository: Option<GitRepositoryRef>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[doc = "Compressed size (bytes) of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "sshUrl", default, skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(
        rename = "validRemoteUrls",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_remote_urls: Vec<String>,
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}
impl GitRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRepositoryRef {
    #[doc = "Reference object for a TeamProjectCollection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<TeamProjectCollectionReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "True if the repository was created as a fork"]
    #[serde(rename = "isFork", default, skip_serializing_if = "Option::is_none")]
    pub is_fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[serde(rename = "sshUrl", default, skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitRepositoryRef {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ItemContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<item_content::ContentType>,
}
impl ItemContent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod item_content {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ContentType {
        #[serde(rename = "rawText")]
        RawText,
        #[serde(rename = "base64Encoded")]
        Base64Encoded,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ItemModel {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(
        rename = "contentMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_metadata: Option<FileContentMetadata>,
    #[serde(rename = "isFolder", default, skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    #[serde(rename = "isSymLink", default, skip_serializing_if = "Option::is_none")]
    pub is_sym_link: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ItemModel {
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
#[doc = "Reference object for a TeamProjectCollection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamProjectCollectionReference {
    #[doc = "Collection avatar Url."]
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[doc = "Collection Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Collection Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Collection REST Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TeamProjectCollectionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a shallow reference to a TeamProject."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProjectReference {
    #[doc = "Project abbreviation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[doc = "Url to default team identity image."]
    #[serde(
        rename = "defaultTeamImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team_image_url: Option<String>,
    #[doc = "The project's description (if any)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Project identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Project last update time."]
    #[serde(
        rename = "lastUpdateTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "Project name."]
    pub name: String,
    #[doc = "Project revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[doc = "Project state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<team_project_reference::State>,
    #[doc = "Url to the full version of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Project visibility."]
    pub visibility: team_project_reference::Visibility,
}
impl TeamProjectReference {
    pub fn new(name: String, visibility: team_project_reference::Visibility) -> Self {
        Self {
            abbreviation: None,
            default_team_image_url: None,
            description: None,
            id: None,
            last_update_time: None,
            name,
            revision: None,
            state: None,
            url: None,
            visibility,
        }
    }
}
pub mod team_project_reference {
    use super::*;
    #[doc = "Project state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "new")]
        New,
        #[serde(rename = "wellFormed")]
        WellFormed,
        #[serde(rename = "createPending")]
        CreatePending,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "unchanged")]
        Unchanged,
        #[serde(rename = "deleted")]
        Deleted,
    }
    #[doc = "Project visibility."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Visibility {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "organization")]
        Organization,
        #[serde(rename = "unchanged")]
        Unchanged,
    }
}
#[doc = "Class representing a branch object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcBranch {
    #[serde(flatten)]
    pub tfvc_branch_ref: TfvcBranchRef,
    #[doc = "List of children for the branch."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<TfvcBranch>,
    #[doc = "List of branch mappings."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<TfvcBranchMapping>,
    #[doc = "This is the shallow branchref class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<TfvcShallowBranchRef>,
    #[doc = "List of paths of the related branches."]
    #[serde(
        rename = "relatedBranches",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_branches: Vec<TfvcShallowBranchRef>,
}
impl TfvcBranch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcBranchList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TfvcBranch>,
}
impl TfvcBranchList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A branch mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcBranchMapping {
    #[doc = "Depth of the branch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<String>,
    #[doc = "Server item for the branch."]
    #[serde(
        rename = "serverItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_item: Option<String>,
    #[doc = "Type of the branch."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl TfvcBranchMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for a branchref."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcBranchRef {
    #[serde(flatten)]
    pub tfvc_shallow_branch_ref: TfvcShallowBranchRef,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Creation date of the branch."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Branch description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Is the branch deleted?"]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "URL to retrieve the item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TfvcBranchRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcBranchRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TfvcBranchRef>,
}
impl TfvcBranchRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcChange {
    #[serde(flatten)]
    pub change: Change,
    #[doc = "List of merge sources in case of rename or branch creation."]
    #[serde(
        rename = "mergeSources",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub merge_sources: Vec<TfvcMergeSource>,
    #[doc = "Version at which a (shelved) change was pended against"]
    #[serde(
        rename = "pendingVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_version: Option<i32>,
}
impl TfvcChange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcChangeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TfvcChange>,
}
impl TfvcChangeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of changes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcChangeset {
    #[serde(flatten)]
    pub tfvc_changeset_ref: TfvcChangesetRef,
    #[doc = "Changeset Account Id also known as Organization Id."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "List of associated changes."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changes: Vec<TfvcChange>,
    #[doc = "List of Checkin Notes for the changeset."]
    #[serde(
        rename = "checkinNotes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub checkin_notes: Vec<CheckinNote>,
    #[doc = "Changeset collection Id."]
    #[serde(
        rename = "collectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_id: Option<String>,
    #[doc = "True if more changes are available."]
    #[serde(
        rename = "hasMoreChanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_more_changes: Option<bool>,
    #[doc = "Information on the policy override."]
    #[serde(
        rename = "policyOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_override: Option<TfvcPolicyOverrideInfo>,
    #[doc = "Team Project Ids for the changeset."]
    #[serde(
        rename = "teamProjectIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team_project_ids: Vec<String>,
    #[doc = "List of work items associated with the changeset."]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<AssociatedWorkItem>,
}
impl TfvcChangeset {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for a changeset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcChangesetRef {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[doc = "Changeset Id."]
    #[serde(
        rename = "changesetId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub changeset_id: Option<i32>,
    #[serde(
        rename = "checkedInBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub checked_in_by: Option<IdentityRef>,
    #[doc = "Comment for the changeset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Was the Comment result truncated?"]
    #[serde(
        rename = "commentTruncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_truncated: Option<bool>,
    #[doc = "Creation date of the changeset."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "URL to retrieve the item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TfvcChangesetRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcChangesetRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TfvcChangesetRef>,
}
impl TfvcChangesetRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Criteria used in a search for change lists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcChangesetSearchCriteria {
    #[doc = "Alias or display name of user who made the changes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "Whether or not to follow renames for the given item being queried."]
    #[serde(
        rename = "followRenames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_renames: Option<bool>,
    #[doc = "If provided, only include changesets created after this date (string)."]
    #[serde(rename = "fromDate", default, skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    #[doc = "If provided, only include changesets after this changesetID."]
    #[serde(rename = "fromId", default, skip_serializing_if = "Option::is_none")]
    pub from_id: Option<i32>,
    #[doc = "Whether to include the _links field on the shallow references."]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[doc = "Path of item to search under."]
    #[serde(rename = "itemPath", default, skip_serializing_if = "Option::is_none")]
    pub item_path: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mappings: Vec<TfvcMappingFilter>,
    #[doc = "If provided, only include changesets created before this date (string)."]
    #[serde(rename = "toDate", default, skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    #[doc = "If provided, a version descriptor for the latest change list to include."]
    #[serde(rename = "toId", default, skip_serializing_if = "Option::is_none")]
    pub to_id: Option<i32>,
}
impl TfvcChangesetSearchCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for Get batched changesets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcChangesetsRequestData {
    #[doc = "List of changeset Ids."]
    #[serde(
        rename = "changesetIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changeset_ids: Vec<i32>,
    #[doc = "Max length of the comment."]
    #[serde(
        rename = "commentLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_length: Option<i32>,
    #[doc = "Whether to include the _links field on the shallow references"]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
}
impl TfvcChangesetsRequestData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for an item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcItem {
    #[serde(flatten)]
    pub item_model: ItemModel,
    #[doc = "Item changed datetime."]
    #[serde(
        rename = "changeDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub change_date: Option<time::OffsetDateTime>,
    #[doc = "Greater than 0 if item is deleted."]
    #[serde(
        rename = "deletionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_id: Option<i32>,
    #[doc = "File encoding from database, -1 represents binary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<i32>,
    #[doc = "MD5 hash as a base 64 string, applies to files only."]
    #[serde(rename = "hashValue", default, skip_serializing_if = "Option::is_none")]
    pub hash_value: Option<String>,
    #[doc = "True if item is a branch."]
    #[serde(rename = "isBranch", default, skip_serializing_if = "Option::is_none")]
    pub is_branch: Option<bool>,
    #[doc = "True if there is a change pending."]
    #[serde(
        rename = "isPendingChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_pending_change: Option<bool>,
    #[doc = "The size of the file, if applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Changeset version Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl TfvcItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Item path and Version descriptor properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcItemDescriptor {
    #[doc = "Item path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Defaults to OneLevel."]
    #[serde(
        rename = "recursionLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursion_level: Option<tfvc_item_descriptor::RecursionLevel>,
    #[doc = "Specify the desired version, can be null or empty string only if VersionType is latest or tip."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Defaults to None."]
    #[serde(
        rename = "versionOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_option: Option<tfvc_item_descriptor::VersionOption>,
    #[doc = "Defaults to Latest."]
    #[serde(
        rename = "versionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_type: Option<tfvc_item_descriptor::VersionType>,
}
impl TfvcItemDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tfvc_item_descriptor {
    use super::*;
    #[doc = "Defaults to OneLevel."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RecursionLevel {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "oneLevel")]
        OneLevel,
        #[serde(rename = "oneLevelPlusNestedEmptyFolders")]
        OneLevelPlusNestedEmptyFolders,
        #[serde(rename = "full")]
        Full,
    }
    #[doc = "Defaults to None."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionOption {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previous")]
        Previous,
        #[serde(rename = "useRename")]
        UseRename,
    }
    #[doc = "Defaults to Latest."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "changeset")]
        Changeset,
        #[serde(rename = "shelveset")]
        Shelveset,
        #[serde(rename = "change")]
        Change,
        #[serde(rename = "date")]
        Date,
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "tip")]
        Tip,
        #[serde(rename = "mergeSource")]
        MergeSource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TfvcItem>,
}
impl TfvcItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body used by Get Items Batch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcItemRequestData {
    #[doc = "If true, include metadata about the file type"]
    #[serde(
        rename = "includeContentMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_content_metadata: Option<bool>,
    #[doc = "Whether to include the _links field on the shallow references"]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[serde(
        rename = "itemDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub item_descriptors: Vec<TfvcItemDescriptor>,
}
impl TfvcItemRequestData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for a label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcLabel {
    #[serde(flatten)]
    pub tfvc_label_ref: TfvcLabelRef,
    #[doc = "List of items."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TfvcItem>,
}
impl TfvcLabel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for a Label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcLabelRef {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Label description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Label Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Label scope."]
    #[serde(
        rename = "labelScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub label_scope: Option<String>,
    #[doc = "Last modified datetime for the label."]
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "Label name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "Label Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TfvcLabelRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcLabelRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TfvcLabelRef>,
}
impl TfvcLabelRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcLabelRequestData {
    #[doc = "Whether to include the _links field on the shallow references"]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[serde(
        rename = "itemLabelFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_label_filter: Option<String>,
    #[serde(
        rename = "labelScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub label_scope: Option<String>,
    #[serde(
        rename = "maxItemCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_item_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}
impl TfvcLabelRequestData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MappingFilter can be used to include or exclude specific paths."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcMappingFilter {
    #[doc = "True if ServerPath should be excluded."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    #[doc = "Path to be included or excluded."]
    #[serde(
        rename = "serverPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_path: Option<String>,
}
impl TfvcMappingFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcMergeSource {
    #[doc = "Indicates if this a rename source. If false, it is a merge source."]
    #[serde(rename = "isRename", default, skip_serializing_if = "Option::is_none")]
    pub is_rename: Option<bool>,
    #[doc = "The server item of the merge source."]
    #[serde(
        rename = "serverItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_item: Option<String>,
    #[doc = "Start of the version range."]
    #[serde(
        rename = "versionFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_from: Option<i32>,
    #[doc = "End of the version range."]
    #[serde(rename = "versionTo", default, skip_serializing_if = "Option::is_none")]
    pub version_to: Option<i32>,
}
impl TfvcMergeSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy failure information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcPolicyFailureInfo {
    #[doc = "Policy failure message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Name of the policy that failed."]
    #[serde(
        rename = "policyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_name: Option<String>,
}
impl TfvcPolicyFailureInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information on the policy override."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcPolicyOverrideInfo {
    #[doc = "Overidden policy comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Information on the failed policy that was overridden."]
    #[serde(
        rename = "policyFailures",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_failures: Vec<TfvcPolicyFailureInfo>,
}
impl TfvcPolicyOverrideInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the shallow branchref class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcShallowBranchRef {
    #[doc = "Path for the branch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl TfvcShallowBranchRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for a shelveset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcShelveset {
    #[serde(flatten)]
    pub tfvc_shelveset_ref: TfvcShelvesetRef,
    #[doc = "List of changes."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changes: Vec<TfvcChange>,
    #[doc = "List of checkin notes."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notes: Vec<CheckinNote>,
    #[doc = "Information on the policy override."]
    #[serde(
        rename = "policyOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_override: Option<TfvcPolicyOverrideInfo>,
    #[doc = "List of associated workitems."]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<AssociatedWorkItem>,
}
impl TfvcShelveset {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for a shallow shelveset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcShelvesetRef {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Shelveset comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Shelveset comment truncated as applicable."]
    #[serde(
        rename = "commentTruncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_truncated: Option<bool>,
    #[doc = "Shelveset create date."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Shelveset Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Shelveset name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "Shelveset Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TfvcShelvesetRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcShelvesetRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TfvcShelvesetRef>,
}
impl TfvcShelvesetRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcShelvesetRequestData {
    #[doc = "Whether to include policyOverride and notes Only applies when requesting a single deep shelveset"]
    #[serde(
        rename = "includeDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_details: Option<bool>,
    #[doc = "Whether to include the _links field on the shallow references. Does not apply when requesting a single deep shelveset object. Links will always be included in the deep shelveset."]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[doc = "Whether to include workItems"]
    #[serde(
        rename = "includeWorkItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_work_items: Option<bool>,
    #[doc = "Max number of changes to include"]
    #[serde(
        rename = "maxChangeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_change_count: Option<i32>,
    #[doc = "Max length of comment"]
    #[serde(
        rename = "maxCommentLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_comment_length: Option<i32>,
    #[doc = "Shelveset name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Owner's ID. Could be a name or a guid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}
impl TfvcShelvesetRequestData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcStatistics {
    #[doc = "Id of the last changeset the stats are based on."]
    #[serde(
        rename = "changesetId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub changeset_id: Option<i32>,
    #[doc = "Count of files at the requested scope."]
    #[serde(
        rename = "fileCountTotal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_count_total: Option<i64>,
}
impl TfvcStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Version descriptor properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcVersionDescriptor {
    #[doc = "Version object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "versionOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_option: Option<tfvc_version_descriptor::VersionOption>,
    #[serde(
        rename = "versionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_type: Option<tfvc_version_descriptor::VersionType>,
}
impl TfvcVersionDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tfvc_version_descriptor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionOption {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previous")]
        Previous,
        #[serde(rename = "useRename")]
        UseRename,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "changeset")]
        Changeset,
        #[serde(rename = "shelveset")]
        Shelveset,
        #[serde(rename = "change")]
        Change,
        #[serde(rename = "date")]
        Date,
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "tip")]
        Tip,
        #[serde(rename = "mergeSource")]
        MergeSource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionControlProjectInfo {
    #[serde(
        rename = "defaultSourceControlType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_source_control_type: Option<version_control_project_info::DefaultSourceControlType>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[serde(
        rename = "supportsGit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_git: Option<bool>,
    #[serde(
        rename = "supportsTFVC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_tfvc: Option<bool>,
}
impl VersionControlProjectInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod version_control_project_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultSourceControlType {
        #[serde(rename = "tfvc")]
        Tfvc,
        #[serde(rename = "git")]
        Git,
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
