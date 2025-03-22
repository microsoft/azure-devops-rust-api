// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountMyWorkResult {
    #[doc = "True, when length of WorkItemDetails is same as the limit"]
    #[serde(
        rename = "querySizeLimitExceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_size_limit_exceeded: Option<bool>,
    #[doc = "WorkItem Details"]
    #[serde(
        rename = "workItemDetails",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_details: Vec<AccountWorkWorkItemModel>,
}
impl AccountMyWorkResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Work Item Recent Activity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountRecentActivityWorkItemModel {
    #[serde(flatten)]
    pub account_recent_activity_work_item_model_base: AccountRecentActivityWorkItemModelBase,
    #[doc = "Assigned To"]
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<String>,
}
impl AccountRecentActivityWorkItemModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Work Item Recent Activity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountRecentActivityWorkItemModel2 {
    #[serde(flatten)]
    pub account_recent_activity_work_item_model_base: AccountRecentActivityWorkItemModelBase,
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<IdentityRef>,
}
impl AccountRecentActivityWorkItemModel2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountRecentActivityWorkItemModel2List {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccountRecentActivityWorkItemModel2>,
}
impl AccountRecentActivityWorkItemModel2List {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Work Item Recent Activity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountRecentActivityWorkItemModelBase {
    #[doc = "Date of the last Activity by the user"]
    #[serde(
        rename = "activityDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub activity_date: Option<time::OffsetDateTime>,
    #[doc = "Type of the activity"]
    #[serde(
        rename = "activityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_type: Option<account_recent_activity_work_item_model_base::ActivityType>,
    #[doc = "Last changed date of the work item"]
    #[serde(
        rename = "changedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub changed_date: Option<time::OffsetDateTime>,
    #[doc = "Work Item Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "TeamFoundationId of the user this activity belongs to"]
    #[serde(
        rename = "identityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_id: Option<String>,
    #[doc = "State of the work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Team project the work item belongs to"]
    #[serde(
        rename = "teamProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_project: Option<String>,
    #[doc = "Title of the work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Type of Work Item"]
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
impl AccountRecentActivityWorkItemModelBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_recent_activity_work_item_model_base {
    use super::*;
    #[doc = "Type of the activity"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActivityType {
        #[serde(rename = "visited")]
        Visited,
        #[serde(rename = "edited")]
        Edited,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "restored")]
        Restored,
    }
}
#[doc = "Represents Recent Mention Work Item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountRecentMentionWorkItemModel {
    #[doc = "Assigned To"]
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<String>,
    #[doc = "Work Item Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Latest date that the user were mentioned"]
    #[serde(
        rename = "mentionedDateField",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub mentioned_date_field: Option<time::OffsetDateTime>,
    #[doc = "State of the work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Team project the work item belongs to"]
    #[serde(
        rename = "teamProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_project: Option<String>,
    #[doc = "Title of the work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Type of Work Item"]
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
impl AccountRecentMentionWorkItemModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountWorkWorkItemModel {
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<String>,
    #[serde(
        rename = "changedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub changed_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(
        rename = "teamProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_project: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(
        rename = "workItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type: Option<String>,
}
impl AccountWorkWorkItemModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains criteria for querying work items based on artifact URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactUriQuery {
    #[doc = "List of artifact URIs to use for querying work items."]
    #[serde(
        rename = "artifactUris",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifact_uris: Vec<String>,
}
impl ArtifactUriQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines result of artifact URI query on work items. Contains mapping of work item IDs to artifact URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactUriQueryResult {
    #[doc = "A Dictionary that maps a list of work item references to the given list of artifact URI."]
    #[serde(
        rename = "artifactUrisQueryResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_uris_query_result: Option<serde_json::Value>,
}
impl ArtifactUriQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachmentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl AttachmentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Comment on a Work Item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
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
    #[doc = "Effective Date/time value for adding the comment. Can be optionally different from CreatedDate."]
    #[serde(
        rename = "createdOnBehalfDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on_behalf_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "createdOnBehalfOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_on_behalf_of: Option<IdentityRef>,
    #[doc = "Represents the possible types for the comment format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<comment::Format>,
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
    #[doc = "The reactions of the comment."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reactions: Vec<CommentReaction>,
    #[doc = "The text of the comment in HTML format."]
    #[serde(
        rename = "renderedText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rendered_text: Option<String>,
    #[doc = "The text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The current version of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "The id of the work item this comment belongs to."]
    #[serde(
        rename = "workItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_id: Option<i32>,
}
impl Comment {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            created_by: None,
            created_date: None,
            created_on_behalf_date: None,
            created_on_behalf_of: None,
            format: None,
            id: None,
            is_deleted: None,
            mentions: Vec::new(),
            modified_by: None,
            modified_date: None,
            reactions: Vec::new(),
            rendered_text: None,
            text: None,
            version: None,
            work_item_id: None,
        }
    }
}
pub mod comment {
    use super::*;
    #[doc = "Represents the possible types for the comment format."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        #[serde(rename = "markdown")]
        Markdown,
        #[serde(rename = "html")]
        Html,
    }
}
#[doc = "Represents a request to create a work item comment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentCreate {
    #[doc = "The text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl CommentCreate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a list of work item comments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentList {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
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
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            comments: Vec::new(),
            continuation_token: None,
            count: None,
            next_page: None,
            total_count: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentMention {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "The artifact portion of the parsed text. (i.e. the work item's id)"]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[doc = "The type the parser assigned to the mention. (i.e. person, work item, etc)"]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[doc = "The comment id of the mention."]
    #[serde(rename = "commentId", default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i32>,
    #[doc = "The resolved target of the mention. An example of this could be a user's tfid"]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}
impl CommentMention {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            artifact_id: None,
            artifact_type: None,
            comment_id: None,
            target_id: None,
        }
    }
}
#[doc = "Contains information about work item comment reaction for a particular reaction type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentReaction {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
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
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            comment_id: None,
            count: None,
            is_current_user_engaged: None,
            type_: None,
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentReactionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CommentReaction>,
}
impl CommentReactionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a request to update a work item comment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentUpdate {
    #[doc = "The updated text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl CommentUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a specific version of a comment on a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentVersion {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
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
    #[doc = "Effective Date/time value for adding the comment. Can be optionally different from CreatedDate."]
    #[serde(
        rename = "createdOnBehalfDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_on_behalf_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "createdOnBehalfOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_on_behalf_of: Option<IdentityRef>,
    #[doc = "The id assigned to the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Indicates if the comment has been deleted at this version."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<IdentityRef>,
    #[doc = "The modification date of the comment for this version."]
    #[serde(
        rename = "modifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "The rendered content of the comment at this version."]
    #[serde(
        rename = "renderedText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rendered_text: Option<String>,
    #[doc = "The text of the comment at this version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The version number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl CommentVersion {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            created_by: None,
            created_date: None,
            created_on_behalf_date: None,
            created_on_behalf_of: None,
            id: None,
            is_deleted: None,
            modified_by: None,
            modified_date: None,
            rendered_text: None,
            text: None,
            version: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentVersionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CommentVersion>,
}
impl CommentVersionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailRecipients {
    #[doc = "Plaintext email addresses."]
    #[serde(
        rename = "emailAddresses",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_addresses: Vec<String>,
    #[doc = "TfIds"]
    #[serde(
        rename = "tfIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tf_ids: Vec<String>,
    #[doc = "Unresolved entity ids"]
    #[serde(
        rename = "unresolvedEntityIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub unresolved_entity_ids: Vec<String>,
}
impl EmailRecipients {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalDeployment {
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ExternalEnvironment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<ExternalPipeline>,
    #[serde(
        rename = "relatedWorkItemIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_work_item_ids: Vec<i32>,
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
    #[serde(
        rename = "sequenceNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence_number: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        rename = "statusDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub status_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ExternalDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalEnvironment {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ExternalEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalPipeline {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ExternalPipeline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a list of dependent fields for a rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldDependentRule {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "The dependent fields."]
    #[serde(
        rename = "dependentFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependent_fields: Vec<WorkItemFieldReference>,
}
impl FieldDependentRule {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            dependent_fields: Vec::new(),
        }
    }
}
#[doc = "Describes an update request for a work item field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldUpdate {
    #[doc = "Indicates whether the user wants to restore the field."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Indicates whether the user wants to lock the field."]
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
}
impl FieldUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Github connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubConnectionModel {
    #[doc = "Github connection authorization type (f. e. PAT, OAuth)"]
    #[serde(
        rename = "authorizationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_type: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Github connection id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Whether current Github connection is valid or not"]
    #[serde(
        rename = "isConnectionValid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_connection_valid: Option<bool>,
    #[doc = "Github connection name (should contain organization/user name)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl GitHubConnectionModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Github connection's repo."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubConnectionRepoModel {
    #[doc = "Error message"]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Repository web url"]
    #[serde(
        rename = "gitHubRepositoryUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub git_hub_repository_url: Option<String>,
}
impl GitHubConnectionRepoModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Github connection's repo bulk request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubConnectionReposBatchRequest {
    #[doc = "Requested repos urls"]
    #[serde(
        rename = "gitHubRepositoryUrls",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub git_hub_repository_urls: Vec<GitHubConnectionRepoModel>,
    #[doc = "Operation type (f. e. add, remove)"]
    #[serde(
        rename = "operationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_type: Option<String>,
}
impl GitHubConnectionReposBatchRequest {
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
pub struct IdentityRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IdentityRef>,
}
impl IdentityRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityReference {
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
impl IdentityReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON model for JSON Patch Operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPatchDocument {}
impl JsonPatchDocument {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON model for a JSON Patch operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPatchOperation {
    #[doc = "The path to copy from for the Move/Copy operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "The patch operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<json_patch_operation::Op>,
    #[doc = "The path for the operation. In the case of an array, a zero based index can be used to specify the position in the array (e.g. /biscuits/0/name). The \"-\" character can be used instead of an index to insert at the end of the array (e.g. /biscuits/-)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The value for the operation. This is either a primitive or a JToken."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl JsonPatchOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod json_patch_operation {
    use super::*;
    #[doc = "The patch operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "replace")]
        Replace,
        #[serde(rename = "move")]
        Move,
        #[serde(rename = "copy")]
        Copy,
        #[serde(rename = "test")]
        Test,
    }
}
#[doc = "Link description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    #[doc = "Collection of link attributes."]
    pub attributes: serde_json::Value,
    #[doc = "Relation type."]
    pub rel: String,
    #[doc = "Link url."]
    pub url: String,
}
impl Link {
    pub fn new(attributes: serde_json::Value, rel: String, url: String) -> Self {
        Self {
            attributes,
            rel,
            url,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MailMessage {
    #[doc = "The mail body in HTML format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<EmailRecipients>,
    #[doc = "The in-reply-to header value"]
    #[serde(rename = "inReplyTo", default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    #[doc = "The Message Id value"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "replyTo", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EmailRecipients>,
    #[doc = "The mail subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<EmailRecipients>,
}
impl MailMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stores process ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessIdModel {
    #[doc = "The ID of the process."]
    #[serde(rename = "typeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
}
impl ProcessIdModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stores project ID and its process ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessMigrationResultModel {
    #[doc = "The ID of the process."]
    #[serde(rename = "processId", default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[doc = "The ID of the project."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}
impl ProcessMigrationResultModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Project work item type state colors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectWorkItemStateColors {
    #[doc = "Project name"]
    #[serde(
        rename = "projectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    #[doc = "State colors for all work item type in a project"]
    #[serde(
        rename = "workItemTypeStateColors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_type_state_colors: Vec<WorkItemTypeStateColors>,
}
impl ProjectWorkItemStateColors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of an update work item type XML update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisioningResult {
    #[doc = "Details about of the provisioning import events."]
    #[serde(
        rename = "provisioningImportEvents",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub provisioning_import_events: Vec<String>,
}
impl ProvisioningResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a request to get a list of queries"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryBatchGetRequest {
    #[doc = "The expand parameters for queries. Possible options are { None, Wiql, Clauses, All, Minimal }"]
    #[serde(rename = "$expand", default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<query_batch_get_request::Expand>,
    #[doc = "The flag to control error policy in a query batch request. Possible options are { Fail, Omit }."]
    #[serde(
        rename = "errorPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_policy: Option<query_batch_get_request::ErrorPolicy>,
    #[doc = "The requested query ids"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<String>,
}
impl QueryBatchGetRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_batch_get_request {
    use super::*;
    #[doc = "The expand parameters for queries. Possible options are { None, Wiql, Clauses, All, Minimal }"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Expand {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "wiql")]
        Wiql,
        #[serde(rename = "clauses")]
        Clauses,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "minimal")]
        Minimal,
    }
    #[doc = "The flag to control error policy in a query batch request. Possible options are { Fail, Omit }."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ErrorPolicy {
        #[serde(rename = "fail")]
        Fail,
        #[serde(rename = "omit")]
        Omit,
    }
}
#[doc = "Represents an item in the work item query hierarchy. This can be either a query or a folder."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryHierarchyItem {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "The child query items inside a query folder."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<QueryHierarchyItem>,
    #[doc = "Represents a clause in a work item query. This shows the structure of a work item query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clauses: Option<WorkItemQueryClause>,
    #[doc = "The columns of the query."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<WorkItemFieldReference>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityReference>,
    #[doc = "When the query item was created."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The link query mode."]
    #[serde(
        rename = "filterOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_options: Option<query_hierarchy_item::FilterOptions>,
    #[doc = "If this is a query folder, indicates if it contains any children."]
    #[serde(
        rename = "hasChildren",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_children: Option<bool>,
    #[doc = "The id of the query item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates if this query item is deleted. Setting this to false on a deleted query item will undelete it. Undeleting a query or folder will not bring back the permission changes that were previously applied to it."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Indicates if this is a query folder or a query."]
    #[serde(rename = "isFolder", default, skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    #[doc = "Indicates if the WIQL of this query is invalid. This could be due to invalid syntax or a no longer valid area/iteration path."]
    #[serde(
        rename = "isInvalidSyntax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_invalid_syntax: Option<bool>,
    #[doc = "Indicates if this query item is public or private."]
    #[serde(rename = "isPublic", default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(
        rename = "lastExecutedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_executed_by: Option<IdentityReference>,
    #[doc = "When the query was last run."]
    #[serde(
        rename = "lastExecutedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_executed_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<IdentityReference>,
    #[doc = "When the query item was last modified."]
    #[serde(
        rename = "lastModifiedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_modified_date: Option<time::OffsetDateTime>,
    #[doc = "Represents a clause in a work item query. This shows the structure of a work item query."]
    #[serde(
        rename = "linkClauses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub link_clauses: Option<WorkItemQueryClause>,
    #[doc = "The name of the query item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The path of the query item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The recursion option for use in a tree query."]
    #[serde(
        rename = "queryRecursionOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_recursion_option: Option<query_hierarchy_item::QueryRecursionOption>,
    #[doc = "The type of query."]
    #[serde(rename = "queryType", default, skip_serializing_if = "Option::is_none")]
    pub query_type: Option<query_hierarchy_item::QueryType>,
    #[doc = "The sort columns of the query."]
    #[serde(
        rename = "sortColumns",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sort_columns: Vec<WorkItemQuerySortColumn>,
    #[doc = "Represents a clause in a work item query. This shows the structure of a work item query."]
    #[serde(
        rename = "sourceClauses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_clauses: Option<WorkItemQueryClause>,
    #[doc = "Represents a clause in a work item query. This shows the structure of a work item query."]
    #[serde(
        rename = "targetClauses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_clauses: Option<WorkItemQueryClause>,
    #[doc = "The WIQL text of the query"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiql: Option<String>,
}
impl QueryHierarchyItem {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            children: Vec::new(),
            clauses: None,
            columns: Vec::new(),
            created_by: None,
            created_date: None,
            filter_options: None,
            has_children: None,
            id: None,
            is_deleted: None,
            is_folder: None,
            is_invalid_syntax: None,
            is_public: None,
            last_executed_by: None,
            last_executed_date: None,
            last_modified_by: None,
            last_modified_date: None,
            link_clauses: None,
            name: None,
            path: None,
            query_recursion_option: None,
            query_type: None,
            sort_columns: Vec::new(),
            source_clauses: None,
            target_clauses: None,
            wiql: None,
        }
    }
}
pub mod query_hierarchy_item {
    use super::*;
    #[doc = "The link query mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FilterOptions {
        #[serde(rename = "workItems")]
        WorkItems,
        #[serde(rename = "linksOneHopMustContain")]
        LinksOneHopMustContain,
        #[serde(rename = "linksOneHopMayContain")]
        LinksOneHopMayContain,
        #[serde(rename = "linksOneHopDoesNotContain")]
        LinksOneHopDoesNotContain,
        #[serde(rename = "linksRecursiveMustContain")]
        LinksRecursiveMustContain,
        #[serde(rename = "linksRecursiveMayContain")]
        LinksRecursiveMayContain,
        #[serde(rename = "linksRecursiveDoesNotContain")]
        LinksRecursiveDoesNotContain,
    }
    #[doc = "The recursion option for use in a tree query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryRecursionOption {
        #[serde(rename = "parentFirst")]
        ParentFirst,
        #[serde(rename = "childFirst")]
        ChildFirst,
    }
    #[doc = "The type of query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryType {
        #[serde(rename = "flat")]
        Flat,
        #[serde(rename = "tree")]
        Tree,
        #[serde(rename = "oneHop")]
        OneHop,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryHierarchyItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<QueryHierarchyItem>,
}
impl QueryHierarchyItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryHierarchyItemsResult {
    #[doc = "The count of items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Indicates if the max return limit was hit but there are still more items"]
    #[serde(rename = "hasMore", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[doc = "The list of items"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<QueryHierarchyItem>,
}
impl QueryHierarchyItemsResult {
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
pub struct ReportingWorkItemLinksBatch {
    #[serde(flatten)]
    pub streamed_batch: StreamedBatch,
}
impl ReportingWorkItemLinksBatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportingWorkItemRevisionsBatch {
    #[serde(flatten)]
    pub streamed_batch: StreamedBatch,
}
impl ReportingWorkItemRevisionsBatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class represents the reporting work item revision filer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportingWorkItemRevisionsFilter {
    #[doc = "A list of fields to return in work item revisions. Omit this parameter to get all reportable fields."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<String>,
    #[doc = "Include deleted work item in the result."]
    #[serde(
        rename = "includeDeleted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_deleted: Option<bool>,
    #[doc = "Return an identity reference instead of a string value for identity fields."]
    #[serde(
        rename = "includeIdentityRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_identity_ref: Option<bool>,
    #[doc = "Include only the latest version of a work item, skipping over all previous revisions of the work item."]
    #[serde(
        rename = "includeLatestOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_latest_only: Option<bool>,
    #[doc = "Include tag reference instead of string value for System.Tags field"]
    #[serde(
        rename = "includeTagRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_tag_ref: Option<bool>,
    #[doc = "A list of types to filter the results to specific work item types. Omit this parameter to get work item revisions of all work item types."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub types: Vec<String>,
}
impl ReportingWorkItemRevisionsFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SendMailBody {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<MailMessage>,
    #[serde(
        rename = "persistenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub persistence_id: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "sortFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sort_fields: Vec<String>,
    #[serde(
        rename = "tempQueryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub temp_query_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiql: Option<String>,
}
impl SendMailBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class describes reporting work item revision batch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamedBatch {
    #[doc = "ContinuationToken acts as a waterMark. Used while querying large results."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "Returns 'true' if it's last batch, 'false' otherwise."]
    #[serde(
        rename = "isLastBatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_last_batch: Option<bool>,
    #[doc = "The next link for the work item."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Values such as rel, sourceId, TargetId, ChangedDate, isActive."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl StreamedBatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Team Context for an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamContext {
    #[doc = "The team project Id or name.  Ignored if ProjectId is set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[doc = "The Team Project ID.  Required if Project is not set."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The Team Id or name.  Ignored if TeamId is set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[doc = "The Team Id"]
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}
impl TeamContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a request to create a temporary query"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemporaryQueryRequestModel {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "The WIQL text of the temporary query"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiql: Option<String>,
}
impl TemporaryQueryRequestModel {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            wiql: None,
        }
    }
}
#[doc = "The result of a temporary query creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemporaryQueryResponseModel {
    #[doc = "The id of the temporary query item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl TemporaryQueryResponseModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an update request for a work item field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateWorkItemField {
    #[doc = "Indicates whether the user wants to restore the field."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}
impl UpdateWorkItemField {
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
#[doc = "A WIQL query"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Wiql {
    #[doc = "The text of the WIQL query"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl Wiql {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A work artifact link describes an outbound artifact link type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkArtifactLink {
    #[doc = "Target artifact type."]
    #[serde(
        rename = "artifactType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    #[doc = "Outbound link type."]
    #[serde(rename = "linkType", default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    #[doc = "Target tool type."]
    #[serde(rename = "toolType", default, skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<String>,
}
impl WorkArtifactLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkArtifactLinkList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkArtifactLink>,
}
impl WorkArtifactLinkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItem {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "Represents the reference to a specific version of a comment on a Work Item."]
    #[serde(
        rename = "commentVersionRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_version_ref: Option<WorkItemCommentVersionRef>,
    #[doc = "Map of field and values for the work item."]
    pub fields: serde_json::Value,
    #[doc = "The work item ID."]
    pub id: i32,
    #[doc = "Relations of the work item."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relations: Vec<WorkItemRelation>,
    #[doc = "Revision number of the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev: Option<i32>,
}
impl WorkItem {
    pub fn new(
        work_item_tracking_resource: WorkItemTrackingResource,
        fields: serde_json::Value,
        id: i32,
    ) -> Self {
        Self {
            work_item_tracking_resource,
            comment_version_ref: None,
            fields,
            id,
            relations: Vec::new(),
            rev: None,
        }
    }
}
#[doc = "Describes a request to get a set of work items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemBatchGetRequest {
    #[doc = "The expand parameters for work item attributes. Possible options are { None, Relations, Fields, Links, All }"]
    #[serde(rename = "$expand", default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<work_item_batch_get_request::Expand>,
    #[doc = "AsOf UTC date time string"]
    #[serde(
        rename = "asOf",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub as_of: Option<time::OffsetDateTime>,
    #[doc = "The flag to control error policy in a bulk get work items request. Possible options are {Fail, Omit}."]
    #[serde(
        rename = "errorPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_policy: Option<work_item_batch_get_request::ErrorPolicy>,
    #[doc = "The requested fields"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<String>,
    #[doc = "The requested work item ids"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<i32>,
}
impl WorkItemBatchGetRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod work_item_batch_get_request {
    use super::*;
    #[doc = "The expand parameters for work item attributes. Possible options are { None, Relations, Fields, Links, All }"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Expand {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "relations")]
        Relations,
        #[serde(rename = "fields")]
        Fields,
        #[serde(rename = "links")]
        Links,
        #[serde(rename = "all")]
        All,
    }
    #[doc = "The flag to control error policy in a bulk get work items request. Possible options are {Fail, Omit}."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ErrorPolicy {
        #[serde(rename = "fail")]
        Fail,
        #[serde(rename = "omit")]
        Omit,
    }
}
#[doc = "Defines a classification node for work item tracking."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemClassificationNode {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "Dictionary that has node attributes like start/finish date for iteration nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "List of child nodes fetched."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<WorkItemClassificationNode>,
    #[doc = "Flag that indicates if the classification node has any child nodes."]
    #[serde(
        rename = "hasChildren",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_children: Option<bool>,
    #[doc = "Integer ID of the classification node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "GUID ID of the classification node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "Name of the classification node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Path of the classification node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Node structure type."]
    #[serde(
        rename = "structureType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub structure_type: Option<work_item_classification_node::StructureType>,
}
impl WorkItemClassificationNode {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            attributes: None,
            children: Vec::new(),
            has_children: None,
            id: None,
            identifier: None,
            name: None,
            path: None,
            structure_type: None,
        }
    }
}
pub mod work_item_classification_node {
    use super::*;
    #[doc = "Node structure type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StructureType {
        #[serde(rename = "area")]
        Area,
        #[serde(rename = "iteration")]
        Iteration,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemClassificationNodeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemClassificationNode>,
}
impl WorkItemClassificationNodeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Comment on Work Item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemComment {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "Represents the possible types for the comment format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<work_item_comment::Format>,
    #[doc = "The text of the comment in HTML format."]
    #[serde(
        rename = "renderedText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rendered_text: Option<String>,
    #[serde(rename = "revisedBy", default, skip_serializing_if = "Option::is_none")]
    pub revised_by: Option<IdentityReference>,
    #[doc = "The date of comment."]
    #[serde(
        rename = "revisedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub revised_date: Option<time::OffsetDateTime>,
    #[doc = "The work item revision number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "The text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl WorkItemComment {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            format: None,
            rendered_text: None,
            revised_by: None,
            revised_date: None,
            revision: None,
            text: None,
        }
    }
}
pub mod work_item_comment {
    use super::*;
    #[doc = "Represents the possible types for the comment format."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        #[serde(rename = "markdown")]
        Markdown,
        #[serde(rename = "html")]
        Html,
    }
}
#[doc = "Represents the reference to a specific version of a comment on a Work Item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemCommentVersionRef {
    #[serde(flatten)]
    pub work_item_tracking_resource_reference: WorkItemTrackingResourceReference,
    #[doc = "The id assigned to the comment."]
    #[serde(rename = "commentId", default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i32>,
    #[doc = "\\[Internal\\] The work item revision where this comment was originally added."]
    #[serde(
        rename = "createdInRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_in_revision: Option<i32>,
    #[doc = "\\[Internal\\] Specifies whether comment was deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "\\[Internal\\] The text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The version number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl WorkItemCommentVersionRef {
    pub fn new(work_item_tracking_resource_reference: WorkItemTrackingResourceReference) -> Self {
        Self {
            work_item_tracking_resource_reference,
            comment_id: None,
            created_in_revision: None,
            is_deleted: None,
            text: None,
            version: None,
        }
    }
}
#[doc = "Collection of comments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemComments {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "Comments collection."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub comments: Vec<WorkItemComment>,
    #[doc = "The count of comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Count of comments from the revision."]
    #[serde(
        rename = "fromRevisionCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_revision_count: Option<i32>,
    #[doc = "Total count of comments."]
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i32>,
}
impl WorkItemComments {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            comments: Vec::new(),
            count: None,
            from_revision_count: None,
            total_count: None,
        }
    }
}
#[doc = "Full deleted work item object. Includes the work item itself."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDelete {
    #[serde(flatten)]
    pub work_item_delete_reference: WorkItemDeleteReference,
    #[doc = "Describes a work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<WorkItem>,
}
impl WorkItemDelete {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes response to delete a set of work items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDeleteBatch {
    #[doc = "List of results for each work item"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<WorkItemDelete>,
}
impl WorkItemDeleteBatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a request to delete a set of work items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDeleteBatchRequest {
    #[doc = "Optional parameter, if set to true, the work item is deleted permanently. Please note: the destroy action is PERMANENT and cannot be undone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destroy: Option<bool>,
    #[doc = "The requested work item ids"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<i32>,
    #[doc = "Optional parameter, if set to true, notifications will be disabled."]
    #[serde(
        rename = "skipNotifications",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_notifications: Option<bool>,
}
impl WorkItemDeleteBatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a deleted work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDeleteReference {
    #[doc = "The HTTP status code for work item operation in a batch request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[doc = "The user who deleted the work item type."]
    #[serde(rename = "deletedBy", default, skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    #[doc = "The work item deletion date."]
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_date: Option<String>,
    #[doc = "Work item ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The exception message for work item operation in a batch request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Name or title of the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Parent project of the deleted work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[doc = "Type of work item."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "REST API URL of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemDeleteReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDeleteReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemDeleteReference>,
}
impl WorkItemDeleteReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Shallow Reference to a deleted work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDeleteShallowReference {
    #[doc = "Work item ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "REST API URL of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemDeleteShallowReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDeleteShallowReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemDeleteShallowReference>,
}
impl WorkItemDeleteShallowReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an update request for a deleted work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDeleteUpdate {
    #[doc = "Sets a value indicating whether this work item is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}
impl WorkItemDeleteUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a field on a work item and it's properties specific to that work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemField {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "Indicates whether the field is sortable in server queries."]
    #[serde(rename = "canSortBy", default, skip_serializing_if = "Option::is_none")]
    pub can_sort_by: Option<bool>,
    #[doc = "The description of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicates whether this field is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Indicates whether this field is an identity field."]
    #[serde(
        rename = "isIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_identity: Option<bool>,
    #[doc = "Indicates whether this instance is picklist."]
    #[serde(
        rename = "isPicklist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_picklist: Option<bool>,
    #[doc = "Indicates whether this instance is a suggested picklist ."]
    #[serde(
        rename = "isPicklistSuggested",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_picklist_suggested: Option<bool>,
    #[doc = "Indicates whether the field can be queried in the server."]
    #[serde(
        rename = "isQueryable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_queryable: Option<bool>,
    #[doc = "The name of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If this field is picklist, the identifier of the picklist associated, otherwise null"]
    #[serde(
        rename = "picklistId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub picklist_id: Option<String>,
    #[doc = "Indicates whether the field is [read only]."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "The reference name of the field."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "The supported operations on this field."]
    #[serde(
        rename = "supportedOperations",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_operations: Vec<WorkItemFieldOperation>,
    #[doc = "The type of the field."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<work_item_field::Type>,
    #[doc = "The usage of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<work_item_field::Usage>,
}
impl WorkItemField {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            can_sort_by: None,
            description: None,
            is_deleted: None,
            is_identity: None,
            is_picklist: None,
            is_picklist_suggested: None,
            is_queryable: None,
            name: None,
            picklist_id: None,
            read_only: None,
            reference_name: None,
            supported_operations: Vec::new(),
            type_: None,
            usage: None,
        }
    }
}
pub mod work_item_field {
    use super::*;
    #[doc = "The type of the field."]
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
        #[serde(rename = "picklistString")]
        PicklistString,
        #[serde(rename = "picklistInteger")]
        PicklistInteger,
        #[serde(rename = "picklistDouble")]
        PicklistDouble,
    }
    #[doc = "The usage of the field."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Usage {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "workItem")]
        WorkItem,
        #[serde(rename = "workItemLink")]
        WorkItemLink,
        #[serde(rename = "tree")]
        Tree,
        #[serde(rename = "workItemTypeExtension")]
        WorkItemTypeExtension,
    }
}
#[doc = "Describes a field on a work item and it's properties specific to that work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemField2 {
    #[serde(flatten)]
    pub work_item_field: WorkItemField,
    #[doc = "Indicates whether this field is marked as locked for editing."]
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
}
impl WorkItemField2 {
    pub fn new(work_item_field: WorkItemField) -> Self {
        Self {
            work_item_field,
            is_locked: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemField2List {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemField2>,
}
impl WorkItemField2List {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the list of allowed values of the field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemFieldAllowedValues {
    #[doc = "The list of field allowed values."]
    #[serde(
        rename = "allowedValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_values: Vec<String>,
    #[doc = "Name of the field."]
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
}
impl WorkItemFieldAllowedValues {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a work item field operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemFieldOperation {
    #[doc = "Friendly name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Reference name of the operation."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
}
impl WorkItemFieldOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a field in a work item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemFieldReference {
    #[doc = "The friendly name of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reference name of the field."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "The REST URL of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemFieldReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an update to a work item field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemFieldUpdate {
    #[doc = "The new value of the field."]
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<serde_json::Value>,
    #[doc = "The old value of the field."]
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<serde_json::Value>,
}
impl WorkItemFieldUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemHistory {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev: Option<i32>,
    #[serde(rename = "revisedBy", default, skip_serializing_if = "Option::is_none")]
    pub revised_by: Option<IdentityReference>,
    #[serde(
        rename = "revisedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub revised_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl WorkItemHistory {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            rev: None,
            revised_by: None,
            revised_date: None,
            value: None,
        }
    }
}
#[doc = "Reference to a work item icon."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemIcon {
    #[doc = "The identifier of the icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The REST URL of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemIcon {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemIconList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemIcon>,
}
impl WorkItemIconList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A link between two work items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemLink {
    #[doc = "The type of link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[doc = "Contains reference to a work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<WorkItemReference>,
    #[doc = "Contains reference to a work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<WorkItemReference>,
}
impl WorkItemLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItem>,
}
impl WorkItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the next state for a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemNextStateOnTransition {
    #[doc = "Error code if there is no next state transition possible."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Work item ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Error message if there is no next state transition possible."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Name of the next state on transition."]
    #[serde(
        rename = "stateOnTransition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_on_transition: Option<String>,
}
impl WorkItemNextStateOnTransition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemNextStateOnTransitionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemNextStateOnTransition>,
}
impl WorkItemNextStateOnTransitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a clause in a work item query. This shows the structure of a work item query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemQueryClause {
    #[doc = "Child clauses if the current clause is a logical operator"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub clauses: Vec<WorkItemQueryClause>,
    #[doc = "Reference to a field in a work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<WorkItemFieldReference>,
    #[doc = "Reference to a field in a work item"]
    #[serde(
        rename = "fieldValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_value: Option<WorkItemFieldReference>,
    #[doc = "Determines if this is a field to field comparison"]
    #[serde(
        rename = "isFieldValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_field_value: Option<bool>,
    #[doc = "Logical operator separating the condition clause"]
    #[serde(
        rename = "logicalOperator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logical_operator: Option<work_item_query_clause::LogicalOperator>,
    #[doc = "Describes a work item field operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<WorkItemFieldOperation>,
    #[doc = "Right side of the condition when a field to value comparison"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl WorkItemQueryClause {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod work_item_query_clause {
    use super::*;
    #[doc = "Logical operator separating the condition clause"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LogicalOperator {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "and")]
        And,
        #[serde(rename = "or")]
        Or,
    }
}
#[doc = "The result of a work item query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemQueryResult {
    #[doc = "The date the query was run in the context of."]
    #[serde(
        rename = "asOf",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub as_of: Option<time::OffsetDateTime>,
    #[doc = "The columns of the query."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<WorkItemFieldReference>,
    #[doc = "The result type"]
    #[serde(
        rename = "queryResultType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_result_type: Option<work_item_query_result::QueryResultType>,
    #[doc = "The type of the query"]
    #[serde(rename = "queryType", default, skip_serializing_if = "Option::is_none")]
    pub query_type: Option<work_item_query_result::QueryType>,
    #[doc = "The sort columns of the query."]
    #[serde(
        rename = "sortColumns",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sort_columns: Vec<WorkItemQuerySortColumn>,
    #[doc = "The work item links returned by the query."]
    #[serde(
        rename = "workItemRelations",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_relations: Vec<WorkItemLink>,
    #[doc = "The work items returned by the query."]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<WorkItemReference>,
}
impl WorkItemQueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod work_item_query_result {
    use super::*;
    #[doc = "The result type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryResultType {
        #[serde(rename = "workItem")]
        WorkItem,
        #[serde(rename = "workItemLink")]
        WorkItemLink,
    }
    #[doc = "The type of the query"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryType {
        #[serde(rename = "flat")]
        Flat,
        #[serde(rename = "tree")]
        Tree,
        #[serde(rename = "oneHop")]
        OneHop,
    }
}
#[doc = "A sort column."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemQuerySortColumn {
    #[doc = "The direction to sort by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descending: Option<bool>,
    #[doc = "Reference to a field in a work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<WorkItemFieldReference>,
}
impl WorkItemQuerySortColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains reference to a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemReference {
    #[doc = "Work item ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "REST API URL of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemRelation {
    #[serde(flatten)]
    pub link: Link,
}
impl WorkItemRelation {
    pub fn new(link: Link) -> Self {
        Self { link }
    }
}
#[doc = "Represents the work item type relation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemRelationType {
    #[serde(flatten)]
    pub work_item_tracking_reference: WorkItemTrackingReference,
    #[doc = "The collection of relation type attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}
impl WorkItemRelationType {
    pub fn new(work_item_tracking_reference: WorkItemTrackingReference) -> Self {
        Self {
            work_item_tracking_reference,
            attributes: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemRelationTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemRelationType>,
}
impl WorkItemRelationTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes updates to a work item's relations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemRelationUpdates {
    #[doc = "List of newly added relations."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub added: Vec<WorkItemRelation>,
    #[doc = "List of removed relations."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub removed: Vec<WorkItemRelation>,
    #[doc = "List of updated relations."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub updated: Vec<WorkItemRelation>,
}
impl WorkItemRelationUpdates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Work item type state name, color and state category"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemStateColor {
    #[doc = "Category of state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Color value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "Work item type state name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WorkItemStateColor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemStateColorList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemStateColor>,
}
impl WorkItemStateColorList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a state transition in a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemStateTransition {
    #[doc = "Gets a list of actions needed to transition to that state."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<String>,
    #[doc = "Name of the next state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
impl WorkItemStateTransition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTagDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemTagDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTagDefinitionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemTagDefinition>,
}
impl WorkItemTagDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a work item template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemTemplate {
    #[serde(flatten)]
    pub work_item_template_reference: WorkItemTemplateReference,
    #[doc = "Mapping of field and its templated value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}
impl WorkItemTemplate {
    pub fn new(work_item_template_reference: WorkItemTemplateReference) -> Self {
        Self {
            work_item_template_reference,
            fields: None,
        }
    }
}
#[doc = "Describes a shallow reference to a work item template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemTemplateReference {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "The description of the work item template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The identifier of the work item template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the work item template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the work item type."]
    #[serde(
        rename = "workItemTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type_name: Option<String>,
}
impl WorkItemTemplateReference {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            description: None,
            id: None,
            name: None,
            work_item_type_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTemplateReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemTemplateReference>,
}
impl WorkItemTemplateReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemTrackingReference {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reference name."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
}
impl WorkItemTrackingReference {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            name: None,
            reference_name: None,
        }
    }
}
#[doc = "Base class for WIT REST resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemTrackingResource {
    #[serde(flatten)]
    pub work_item_tracking_resource_reference: WorkItemTrackingResourceReference,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
impl WorkItemTrackingResource {
    pub fn new(work_item_tracking_resource_reference: WorkItemTrackingResourceReference) -> Self {
        Self {
            work_item_tracking_resource_reference,
            links: None,
        }
    }
}
#[doc = "Base class for work item tracking resource references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemTrackingResourceReference {
    pub url: String,
}
impl WorkItemTrackingResourceReference {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
#[doc = "Describes a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemType {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "The color."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "The description of the work item type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The fields that exist on the work item type."]
    #[serde(
        rename = "fieldInstances",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_instances: Vec<WorkItemTypeFieldInstance>,
    #[doc = "The fields that exist on the work item type."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<WorkItemTypeFieldInstance>,
    #[doc = "Reference to a work item icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<WorkItemIcon>,
    #[doc = "True if work item type is disabled"]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "Gets the name of the work item type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reference name of the work item type."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "Gets state information for the work item type."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub states: Vec<WorkItemStateColor>,
    #[doc = "Gets the various state transition mappings in the work item type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<serde_json::Value>,
    #[doc = "The XML form."]
    #[serde(rename = "xmlForm", default, skip_serializing_if = "Option::is_none")]
    pub xml_form: Option<String>,
}
impl WorkItemType {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            color: None,
            description: None,
            field_instances: Vec::new(),
            fields: Vec::new(),
            icon: None,
            is_disabled: None,
            name: None,
            reference_name: None,
            states: Vec::new(),
            transitions: None,
            xml_form: None,
        }
    }
}
#[doc = "Describes a work item type category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemTypeCategory {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "Reference to a work item type."]
    #[serde(
        rename = "defaultWorkItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_work_item_type: Option<WorkItemTypeReference>,
    #[doc = "The name of the category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reference name of the category."]
    #[serde(
        rename = "referenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_name: Option<String>,
    #[doc = "The work item types that belong to the category."]
    #[serde(
        rename = "workItemTypes",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_types: Vec<WorkItemTypeReference>,
}
impl WorkItemTypeCategory {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            default_work_item_type: None,
            name: None,
            reference_name: None,
            work_item_types: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeCategoryList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemTypeCategory>,
}
impl WorkItemTypeCategoryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a work item type's colors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeColor {
    #[doc = "Gets or sets the color of the primary."]
    #[serde(
        rename = "primaryColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_color: Option<String>,
    #[doc = "Gets or sets the color of the secondary."]
    #[serde(
        rename = "secondaryColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_color: Option<String>,
    #[doc = "The name of the work item type."]
    #[serde(
        rename = "workItemTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type_name: Option<String>,
}
impl WorkItemTypeColor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes work item type name, its icon and color."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeColorAndIcon {
    #[doc = "The color of the work item type in hex format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[doc = "The work item type icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Indicates if the work item is disabled in the process."]
    #[serde(
        rename = "isDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[doc = "The name of the work item type."]
    #[serde(
        rename = "workItemTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type_name: Option<String>,
}
impl WorkItemTypeColorAndIcon {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Field instance of a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeFieldInstance {
    #[serde(flatten)]
    pub work_item_type_field_instance_base: WorkItemTypeFieldInstanceBase,
    #[doc = "The list of field allowed values."]
    #[serde(
        rename = "allowedValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_values: Vec<String>,
    #[doc = "Represents the default value of the field."]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<String>,
}
impl WorkItemTypeFieldInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base field instance for workItemType fields."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeFieldInstanceBase {
    #[serde(flatten)]
    pub work_item_field_reference: WorkItemFieldReference,
    #[doc = "Indicates whether field value is always required."]
    #[serde(
        rename = "alwaysRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_required: Option<bool>,
    #[doc = "The list of dependent fields."]
    #[serde(
        rename = "dependentFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dependent_fields: Vec<WorkItemFieldReference>,
    #[doc = "Gets the help text for the field."]
    #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
}
impl WorkItemTypeFieldInstanceBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Field Instance of a workItemype with detailed references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeFieldWithReferences {
    #[serde(flatten)]
    pub work_item_type_field_instance_base: WorkItemTypeFieldInstanceBase,
    #[doc = "The list of field allowed values."]
    #[serde(
        rename = "allowedValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_values: Vec<serde_json::Value>,
    #[doc = "Represents the default value of the field."]
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<serde_json::Value>,
}
impl WorkItemTypeFieldWithReferences {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeFieldWithReferencesList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemTypeFieldWithReferences>,
}
impl WorkItemTypeFieldWithReferencesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemType>,
}
impl WorkItemTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a work item type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemTypeReference {
    #[serde(flatten)]
    pub work_item_tracking_resource_reference: WorkItemTrackingResourceReference,
    #[doc = "Name of the work item type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WorkItemTypeReference {
    pub fn new(work_item_tracking_resource_reference: WorkItemTrackingResourceReference) -> Self {
        Self {
            work_item_tracking_resource_reference,
            name: None,
        }
    }
}
#[doc = "State colors for a work item type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeStateColors {
    #[doc = "Work item type state colors"]
    #[serde(
        rename = "stateColors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub state_colors: Vec<WorkItemStateColor>,
    #[doc = "Work item type name"]
    #[serde(
        rename = "workItemTypeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_type_name: Option<String>,
}
impl WorkItemTypeStateColors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a work item type template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeTemplate {
    #[doc = "XML template in string format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}
impl WorkItemTypeTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a update work item type template request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemTypeTemplateUpdateModel {
    #[doc = "Describes the type of the action for the update request."]
    #[serde(
        rename = "actionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_type: Option<work_item_type_template_update_model::ActionType>,
    #[doc = "Methodology to which the template belongs, eg. Agile, Scrum, CMMI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub methodology: Option<String>,
    #[doc = "String representation of the work item type template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "The type of the template described in the request body."]
    #[serde(
        rename = "templateType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_type: Option<work_item_type_template_update_model::TemplateType>,
}
impl WorkItemTypeTemplateUpdateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod work_item_type_template_update_model {
    use super::*;
    #[doc = "Describes the type of the action for the update request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        #[serde(rename = "import")]
        Import,
        #[serde(rename = "validate")]
        Validate,
    }
    #[doc = "The type of the template described in the request body."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TemplateType {
        #[serde(rename = "workItemType")]
        WorkItemType,
        #[serde(rename = "globalWorkflow")]
        GlobalWorkflow,
    }
}
#[doc = "Describes an update to a work item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemUpdate {
    #[serde(flatten)]
    pub work_item_tracking_resource: WorkItemTrackingResource,
    #[doc = "List of updates to fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    #[doc = "ID of update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Describes updates to a work item's relations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<WorkItemRelationUpdates>,
    #[doc = "The revision number of work item update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev: Option<i32>,
    #[serde(rename = "revisedBy", default, skip_serializing_if = "Option::is_none")]
    pub revised_by: Option<IdentityReference>,
    #[doc = "The work item updates revision date."]
    #[serde(
        rename = "revisedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub revised_date: Option<time::OffsetDateTime>,
    #[doc = "The work item ID."]
    #[serde(
        rename = "workItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_item_id: Option<i32>,
}
impl WorkItemUpdate {
    pub fn new(work_item_tracking_resource: WorkItemTrackingResource) -> Self {
        Self {
            work_item_tracking_resource,
            fields: None,
            id: None,
            relations: None,
            rev: None,
            revised_by: None,
            revised_date: None,
            work_item_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemUpdateList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemUpdate>,
}
impl WorkItemUpdateList {
    pub fn new() -> Self {
        Self::default()
    }
}
