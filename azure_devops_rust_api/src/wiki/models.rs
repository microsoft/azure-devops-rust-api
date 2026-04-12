// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Comment on an artifact like Work Item or Wiki, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Comment {
    #[serde(flatten)]
    pub comment_resource_reference: CommentResourceReference,
    #[doc = "The id of the artifact this comment belongs to"]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "The creation date of the comment."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The id assigned to the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Indicates if the comment has been deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "The mentions of the comment."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mentions: Vec<CommentMention>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "The last modification date of the comment."]
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "The comment id of the parent comment, if any"]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    #[doc = "The reactions on the comment."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reactions: Vec<CommentReaction>,
    #[doc = "The rendered text of the comment"]
    #[serde(
        rename = "renderedText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rendered_text: Option<String>,
    #[doc = "Represents a list of comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<CommentList>,
    #[doc = "Indicates the current state of the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<comment::State>,
    #[doc = "The plaintext/markdown version of the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The current version of the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl Comment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod comment {
    use super::*;
    #[doc = "Indicates the current state of the comment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "resolved")]
        Resolved,
        #[serde(rename = "closed")]
        Closed,
    }
}
#[doc = "Represents an attachment to a comment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentAttachment {
    #[serde(flatten)]
    pub comment_resource_reference: CommentResourceReference,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "The creation date of the attachment."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Unique Id of the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl CommentAttachment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a request to create a work item comment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentCreateParameters {
    #[doc = "Optional CommentId of the parent in order to add a reply for an existing comment"]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl CommentCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a list of comments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentList {
    #[serde(flatten)]
    pub comment_resource_reference: CommentResourceReference,
    #[doc = "List of comments in the current batch."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub comments: Vec<Comment>,
    #[doc = "A string token that can be used to retrieving next page of comments if available. Otherwise null."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "The count of comments in the current batch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Uri to the next page of comments if it is available. Otherwise null."]
    #[serde(rename = "nextPage", default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[doc = "Total count of comments on a work item."]
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i32>,
}
impl CommentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about various artifacts mentioned in the comment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentMention {
    #[serde(flatten)]
    pub comment_resource_reference: CommentResourceReference,
    #[doc = "Id of the artifact this mention belongs to"]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[doc = "Id of the comment associated with this mention. Nullable to support legacy mentions which can potentially have null commentId"]
    #[serde(rename = "commentId", default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i32>,
    #[doc = "Value of the mentioned artifact. Expected Value varies by CommentMentionType: Person:         VSID associated with the identity Work Item:      ID of the work item Pull Request:   ID of the Pull Request"]
    #[serde(
        rename = "mentionedArtifact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mentioned_artifact: Option<String>,
    #[doc = "The context which represent where this mentioned was parsed from"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<comment_mention::Type>,
}
impl CommentMention {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod comment_mention {
    use super::*;
    #[doc = "The context which represent where this mentioned was parsed from"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "person")]
        Person,
        #[serde(rename = "workItem")]
        WorkItem,
        #[serde(rename = "pullRequest")]
        PullRequest,
    }
}
#[doc = "Contains information about comment reaction for a particular reaction type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentReaction {
    #[serde(flatten)]
    pub comment_resource_reference: CommentResourceReference,
    #[doc = "The id of the comment this reaction belongs to."]
    #[serde(rename = "commentId", default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i32>,
    #[doc = "Total number of reactions for the CommentReactionType."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Flag to indicate if the current user has engaged on this particular EngagementType (e.g. if they liked the associated comment)."]
    #[serde(
        rename = "isCurrentUserEngaged",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_current_user_engaged: Option<bool>,
    #[doc = "Type of the reaction."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<comment_reaction::Type>,
}
impl CommentReaction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod comment_reaction {
    use super::*;
    #[doc = "Type of the reaction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "like")]
        Like,
        #[serde(rename = "dislike")]
        Dislike,
        #[serde(rename = "heart")]
        Heart,
        #[serde(rename = "hooray")]
        Hooray,
        #[serde(rename = "smile")]
        Smile,
        #[serde(rename = "confused")]
        Confused,
    }
}
#[doc = "Base class for comment resource references"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentResourceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CommentResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a request to update a comment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentUpdateParameters {
    #[doc = "Set the current state of the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<comment_update_parameters::State>,
    #[doc = "The updated text of the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl CommentUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod comment_update_parameters {
    use super::*;
    #[doc = "Set the current state of the comment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "resolved")]
        Resolved,
        #[serde(rename = "closed")]
        Closed,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<TeamProjectCollectionReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "True if the repository was created as a fork"]
    #[serde(rename = "isFork", default, skip_serializing_if = "Option::is_none")]
    pub is_fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct GitVersionDescriptor {
    #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
    #[serde(
        rename = "versionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_options: Option<git_version_descriptor::VersionOptions>,
    #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
    #[serde(
        rename = "versionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_type: Option<git_version_descriptor::VersionType>,
}
impl GitVersionDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_version_descriptor {
    use super::*;
    #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionOptions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previousChange")]
        PreviousChange,
        #[serde(rename = "firstParent")]
        FirstParent,
    }
    #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionType {
        #[serde(rename = "branch")]
        Branch,
        #[serde(rename = "tag")]
        Tag,
        #[serde(rename = "commit")]
        Commit,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamProjectCollectionReference {
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TeamProjectCollectionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamProjectReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(
        rename = "defaultTeamImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team_image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "lastUpdateTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_update_time: Option<time::OffsetDateTime>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<team_project_reference::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
#[doc = "Defines a wiki repository which encapsulates the git repository backing the wiki."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Wiki {
    #[serde(flatten)]
    pub wiki_create_parameters: WikiCreateParameters,
    #[doc = "The head commit associated with the git repository backing up the wiki."]
    #[serde(
        rename = "headCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub head_commit: Option<String>,
    #[doc = "The ID of the wiki which is same as the ID of the Git repository that it is backed by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<GitRepository>,
}
impl Wiki {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines properties for wiki attachment file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiAttachment {
    #[doc = "Name of the wiki attachment file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Path of the wiki attachment file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl WikiAttachment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response contract for the Wiki Attachments API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiAttachmentResponse {
    #[doc = "Defines properties for wiki attachment file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<WikiAttachment>,
    #[doc = "Contains the list of ETag values from the response header of the attachments API call. The first item in the list contains the version of the wiki attachment."]
    #[serde(
        rename = "eTag",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub e_tag: Vec<String>,
}
impl WikiAttachmentResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base wiki creation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiCreateBaseParameters {
    #[doc = "Folder path inside repository which is shown as Wiki. Not required for ProjectWiki type."]
    #[serde(
        rename = "mappedPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mapped_path: Option<String>,
    #[doc = "Wiki name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the project in which the wiki is to be created."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "ID of the git repository that backs up the wiki. Not required for ProjectWiki type."]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
    #[doc = "Type of the wiki."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<wiki_create_base_parameters::Type>,
}
impl WikiCreateBaseParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod wiki_create_base_parameters {
    use super::*;
    #[doc = "Type of the wiki."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "projectWiki")]
        ProjectWiki,
        #[serde(rename = "codeWiki")]
        CodeWiki,
    }
}
#[doc = "Wiki creations parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiCreateParameters {
    #[doc = "Wiki name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the project in which the wiki is to be created."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl WikiCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wiki creation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiCreateParametersV2 {
    #[serde(flatten)]
    pub wiki_create_base_parameters: WikiCreateBaseParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<GitVersionDescriptor>,
}
impl WikiCreateParametersV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a page in a wiki."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPage {
    #[serde(flatten)]
    pub wiki_page_create_or_update_parameters: WikiPageCreateOrUpdateParameters,
    #[doc = "Path of the git item corresponding to the wiki page stored in the backing Git repository."]
    #[serde(
        rename = "gitItemPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub git_item_path: Option<String>,
    #[doc = "When present, permanent identifier for the wiki page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "True if a page is non-conforming, i.e. 1) if the name doesn't match page naming standards. 2) if the page does not have a valid entry in the appropriate order file."]
    #[serde(
        rename = "isNonConformant",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_non_conformant: Option<bool>,
    #[doc = "True if this page has subpages under its path."]
    #[serde(
        rename = "isParentPage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_parent_page: Option<bool>,
    #[doc = "Order of the wiki page, relative to other pages in the same hierarchy level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Path of the wiki page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Remote web url to the wiki page."]
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[doc = "List of subpages of the current page."]
    #[serde(
        rename = "subPages",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_pages: Vec<WikiPage>,
    #[doc = "REST url for this wiki page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WikiPage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contract encapsulating parameters for the page create or update operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageCreateOrUpdateParameters {
    #[doc = "Content of the wiki page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl WikiPageCreateOrUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a page with its metedata in a wiki."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageDetail {
    #[doc = "When present, permanent identifier for the wiki page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Path of the wiki page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Path of the wiki page."]
    #[serde(
        rename = "viewStats",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub view_stats: Vec<WikiPageStat>,
}
impl WikiPageDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageDetailList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WikiPageDetail>,
}
impl WikiPageDetailList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request contract for Wiki Page Move."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageMove {
    #[serde(flatten)]
    pub wiki_page_move_parameters: WikiPageMoveParameters,
    #[doc = "Defines a page in a wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<WikiPage>,
}
impl WikiPageMove {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contract encapsulating parameters for the page move operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageMoveParameters {
    #[doc = "New order of the wiki page."]
    #[serde(rename = "newOrder", default, skip_serializing_if = "Option::is_none")]
    pub new_order: Option<i32>,
    #[doc = "New path of the wiki page."]
    #[serde(rename = "newPath", default, skip_serializing_if = "Option::is_none")]
    pub new_path: Option<String>,
    #[doc = "Current path of the wiki page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl WikiPageMoveParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response contract for the Wiki Page Move API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageMoveResponse {
    #[doc = "Contains the list of ETag values from the response header of the page move API call. The first item in the list contains the version of the wiki page subject to page move."]
    #[serde(
        rename = "eTag",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub e_tag: Vec<String>,
    #[doc = "Request contract for Wiki Page Move."]
    #[serde(rename = "pageMove", default, skip_serializing_if = "Option::is_none")]
    pub page_move: Option<WikiPageMove>,
}
impl WikiPageMoveResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response contract for the Wiki Pages PUT, PATCH and DELETE APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageResponse {
    #[doc = "Contains the list of ETag values from the response header of the pages API call. The first item in the list contains the version of the wiki page."]
    #[serde(
        rename = "eTag",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub e_tag: Vec<String>,
    #[doc = "Defines a page in a wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<WikiPage>,
}
impl WikiPageResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines properties for wiki page stat."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageStat {
    #[doc = "the count of the stat for the Day"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Day of the stat"]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub day: Option<time::OffsetDateTime>,
}
impl WikiPageStat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines properties for wiki page view stats."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPageViewStats {
    #[doc = "Wiki page view count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Wiki page last viewed time."]
    #[serde(
        rename = "lastViewedTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_viewed_time: Option<time::OffsetDateTime>,
    #[doc = "Wiki page path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl WikiPageViewStats {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contract encapsulating parameters for the pages batch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiPagesBatchRequest {
    #[doc = "If the list of page data returned is not complete, a continuation token to query next batch of pages is included in the response header as \"x-ms-continuationtoken\". Omit this parameter to get the first batch of Wiki Page Data."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "last N days from the current day for which page views is to be returned. It's inclusive of current day."]
    #[serde(
        rename = "pageViewsForDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub page_views_for_days: Option<i32>,
    #[doc = "Total count of pages on a wiki to return."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
impl WikiPagesBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wiki update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiUpdateParameters {
    #[doc = "Name for wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Versions of the wiki."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub versions: Vec<GitVersionDescriptor>,
}
impl WikiUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiUpdatedNotificationMessage {
    #[doc = "Collection host Id for which the wikis are updated."]
    #[serde(
        rename = "collectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_id: Option<String>,
    #[doc = "Project Id for which the wikis are updated."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Repository Id associated with the particular wiki which is added, updated or deleted."]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl WikiUpdatedNotificationMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a wiki resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiV2 {
    #[serde(flatten)]
    pub wiki_create_base_parameters: WikiCreateBaseParameters,
    #[doc = "ID of the wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Is wiki repository disabled"]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Properties of the wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Remote web url to the wiki."]
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[doc = "REST url for this wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Versions of the wiki."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub versions: Vec<GitVersionDescriptor>,
}
impl WikiV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiV2List {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WikiV2>,
}
impl WikiV2List {
    pub fn new() -> Self {
        Self::default()
    }
}
