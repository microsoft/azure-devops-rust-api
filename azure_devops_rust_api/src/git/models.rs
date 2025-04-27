// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvSecEnablementStatus {
    #[doc = "Enabled by VSID"]
    #[serde(
        rename = "changedById",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub changed_by_id: Option<String>,
    #[doc = "Enabled changed on datetime"]
    #[serde(
        rename = "changedOnDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub changed_on_date: Option<time::OffsetDateTime>,
    #[doc = "Enabled status 0 disabled, 1 enabled, Null never explicitly set, always whatever project is, ya this should probably be an enum somewhere"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Enabled changed on datetime To Be Removed M223 +"]
    #[serde(
        rename = "enabledChangedOnDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub enabled_changed_on_date: Option<time::OffsetDateTime>,
    #[doc = "ProjectId"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "RepositoryId"]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl AdvSecEnablementStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvSecEnablementUpdate {
    #[doc = "New status"]
    #[serde(rename = "newStatus", default, skip_serializing_if = "Option::is_none")]
    pub new_status: Option<bool>,
    #[doc = "ProjectId"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "RepositoryId Actual RepositoryId to Modify or Magic Repository Id \"FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF\" for ALL Repositories for that project"]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl AdvSecEnablementUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
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
pub struct AsyncGitOperationNotification {
    #[serde(
        rename = "operationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_id: Option<i32>,
}
impl AsyncGitOperationNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncRefOperationCommitLevelEventNotification {
    #[serde(flatten)]
    pub async_git_operation_notification: AsyncGitOperationNotification,
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
}
impl AsyncRefOperationCommitLevelEventNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncRefOperationCompletedNotification {
    #[serde(flatten)]
    pub async_git_operation_notification: AsyncGitOperationNotification,
    #[serde(
        rename = "newRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_ref_name: Option<String>,
}
impl AsyncRefOperationCompletedNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncRefOperationConflictNotification {
    #[serde(flatten)]
    pub async_ref_operation_commit_level_event_notification:
        AsyncRefOperationCommitLevelEventNotification,
}
impl AsyncRefOperationConflictNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncRefOperationGeneralFailureNotification {
    #[serde(flatten)]
    pub async_git_operation_notification: AsyncGitOperationNotification,
}
impl AsyncRefOperationGeneralFailureNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncRefOperationProgressNotification {
    #[serde(flatten)]
    pub async_ref_operation_commit_level_event_notification:
        AsyncRefOperationCommitLevelEventNotification,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}
impl AsyncRefOperationProgressNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncRefOperationTimeoutNotification {
    #[serde(flatten)]
    pub async_git_operation_notification: AsyncGitOperationNotification,
}
impl AsyncRefOperationTimeoutNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Meta data for a file attached to an artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attachment {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[doc = "Content hash of on-disk representation of file content. Its calculated by the server by using SHA1 hash function."]
    #[serde(
        rename = "contentHash",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_hash: Option<String>,
    #[doc = "The time the attachment was uploaded."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The description of the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name of the attachment. Can't be null or empty."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Id of the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "The url to download the content of the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl Attachment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachmentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Attachment>,
}
impl AttachmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for an auto-complete update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoCompleteUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl AutoCompleteUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Used by AdvSec to return billable committers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillableCommitter {
    #[doc = "RepositoryId commit was pushed to."]
    #[serde(rename = "repoId", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    #[doc = "Visual Studio ID /Team Foundation ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vsid: Option<String>,
}
impl BillableCommitter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillableCommitterDetail {
    #[serde(flatten)]
    pub billable_committer: BillableCommitter,
    #[doc = "ID (SHA-1) of the commit."]
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[doc = "Committer email address after parsing."]
    #[serde(
        rename = "committerEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub committer_email: Option<String>,
    #[doc = "Time reported by the commit."]
    #[serde(
        rename = "commitTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub commit_time: Option<time::OffsetDateTime>,
    #[doc = "Project Id commit was pushed to."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Project name commit was pushed to."]
    #[serde(
        rename = "projectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    #[doc = "Time of the push that contained the commit."]
    #[serde(
        rename = "pushedTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub pushed_time: Option<time::OffsetDateTime>,
    #[doc = "Push Id that contained the commit."]
    #[serde(rename = "pushId", default, skip_serializing_if = "Option::is_none")]
    pub push_id: Option<i32>,
    #[doc = "Repository name commit was pushed to."]
    #[serde(rename = "repoName", default, skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
}
impl BillableCommitterDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for a source/target branch update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BranchUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
    #[doc = "If true, the source branch of the pull request was updated"]
    #[serde(
        rename = "isSourceUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_source_update: Option<bool>,
}
impl BranchUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Change {
    #[doc = "The type of change that was made to the item."]
    #[serde(rename = "changeType")]
    pub change_type: change::ChangeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<serde_json::Value>,
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
    pub fn new(change_type: change::ChangeType) -> Self {
        Self {
            change_type,
            item: None,
            new_content: None,
            source_server_item: None,
            url: None,
        }
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
        #[serde(rename = "delete, sourceRename")]
        DeleteSourceRename,
        #[serde(rename = "edit, rename")]
        EditRename,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeCountDictionary {}
impl ChangeCountDictionary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeList {
    #[serde(
        rename = "allChangesIncluded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_changes_included: Option<bool>,
    #[serde(
        rename = "changeCounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_counts: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changes: Vec<Change>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "commentTruncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_truncated: Option<bool>,
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notes: Vec<CheckinNote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(
        rename = "ownerDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_display_name: Option<String>,
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(
        rename = "sortDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub sort_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ChangeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Criteria used in a search for change lists"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeListSearchCriteria {
    #[doc = "If provided, a version descriptor to compare against base"]
    #[serde(
        rename = "compareVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compare_version: Option<String>,
    #[doc = "If true, don't include delete history entries"]
    #[serde(
        rename = "excludeDeletes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_deletes: Option<bool>,
    #[doc = "Whether or not to follow renames for the given item being queried"]
    #[serde(
        rename = "followRenames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_renames: Option<bool>,
    #[doc = "If provided, only include history entries created after this date (string)"]
    #[serde(rename = "fromDate", default, skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    #[doc = "If provided, a version descriptor for the earliest change list to include"]
    #[serde(
        rename = "fromVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_version: Option<String>,
    #[doc = "Path of item to search under. If the itemPaths memebr is used then it will take precedence over this."]
    #[serde(rename = "itemPath", default, skip_serializing_if = "Option::is_none")]
    pub item_path: Option<String>,
    #[doc = "List of item paths to search under. If this member is used then itemPath will be ignored."]
    #[serde(
        rename = "itemPaths",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub item_paths: Vec<String>,
    #[doc = "Version of the items to search"]
    #[serde(
        rename = "itemVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_version: Option<String>,
    #[doc = "Number of results to skip (used when clicking more...)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[doc = "If provided, only include history entries created before this date (string)"]
    #[serde(rename = "toDate", default, skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    #[doc = "If provided, the maximum number of history entries to return"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "If provided, a version descriptor for the latest change list to include"]
    #[serde(rename = "toVersion", default, skip_serializing_if = "Option::is_none")]
    pub to_version: Option<String>,
    #[doc = "Alias or display name of user who made the changes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl ChangeListSearchCriteria {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Represents a comment which is one of potentially many in a comment thread."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Comment {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[doc = "The comment type at the time of creation."]
    #[serde(
        rename = "commentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_type: Option<comment::CommentType>,
    #[doc = "The comment content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The comment ID. IDs start at 1 and are unique to a pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Whether or not this comment was soft-deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "The date the comment's content was last updated."]
    #[serde(
        rename = "lastContentUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_content_updated_date: Option<time::OffsetDateTime>,
    #[doc = "The date the comment was last updated."]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "The ID of the parent comment. This is used for replies."]
    #[serde(
        rename = "parentCommentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_comment_id: Option<i64>,
    #[doc = "The date the comment was first published."]
    #[serde(
        rename = "publishedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "A list of the users who have liked this comment."]
    #[serde(
        rename = "usersLiked",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users_liked: Vec<IdentityRef>,
}
impl Comment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod comment {
    use super::*;
    #[doc = "The comment type at the time of creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CommentType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "text")]
        Text,
        #[serde(rename = "codeChange")]
        CodeChange,
        #[serde(rename = "system")]
        System,
    }
}
#[doc = "Comment iteration context is used to identify which diff was being viewed when the thread was created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentIterationContext {
    #[doc = "The iteration of the file on the left side of the diff when the thread was created. If this value is equal to SecondComparingIteration, then this version is the common commit between the source and target branches of the pull request."]
    #[serde(
        rename = "firstComparingIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub first_comparing_iteration: Option<i64>,
    #[doc = "The iteration of the file on the right side of the diff when the thread was created."]
    #[serde(
        rename = "secondComparingIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub second_comparing_iteration: Option<i64>,
}
impl CommentIterationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Comment>,
}
impl CommentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentPosition {
    #[doc = "The line number of a thread's position. Starts at 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    #[doc = "The character offset of a thread's position inside of a line. Starts at 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}
impl CommentPosition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a comment thread of a pull request. A thread contains meta data about the file it was left on along with one or more comments (an initial comment and the subsequent replies)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentThread {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "A list of the comments."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub comments: Vec<Comment>,
    #[doc = "The comment thread id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Set of identities related to this thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identities: Option<serde_json::Value>,
    #[doc = "Specify if the thread is deleted which happens when all comments are deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "The time this thread was last updated."]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[doc = "The time this thread was published."]
    #[serde(
        rename = "publishedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "The status of the comment thread."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<comment_thread::Status>,
    #[serde(
        rename = "threadContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_context: Option<CommentThreadContext>,
}
impl CommentThread {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod comment_thread {
    use super::*;
    #[doc = "The status of the comment thread."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "fixed")]
        Fixed,
        #[serde(rename = "wontFix")]
        WontFix,
        #[serde(rename = "closed")]
        Closed,
        #[serde(rename = "byDesign")]
        ByDesign,
        #[serde(rename = "pending")]
        Pending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentThreadContext {
    #[doc = "File path relative to the root of the repository. It's up to the client to use any path format."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(
        rename = "leftFileEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub left_file_end: Option<CommentPosition>,
    #[serde(
        rename = "leftFileStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub left_file_start: Option<CommentPosition>,
    #[serde(
        rename = "rightFileEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub right_file_end: Option<CommentPosition>,
    #[serde(
        rename = "rightFileStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub right_file_start: Option<CommentPosition>,
}
impl CommentThreadContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Comment tracking criteria is used to identify which iteration context the thread has been tracked to (if any) along with some detail about the original position and filename."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommentTrackingCriteria {
    #[doc = "The iteration of the file on the left side of the diff that the thread will be tracked to. Threads were tracked if this is greater than 0."]
    #[serde(
        rename = "firstComparingIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub first_comparing_iteration: Option<i32>,
    #[doc = "Original filepath the thread was created on before tracking. This will be different than the current thread filepath if the file in question was renamed in a later iteration."]
    #[serde(
        rename = "origFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_file_path: Option<String>,
    #[serde(
        rename = "origLeftFileEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_left_file_end: Option<CommentPosition>,
    #[serde(
        rename = "origLeftFileStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_left_file_start: Option<CommentPosition>,
    #[serde(
        rename = "origRightFileEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_right_file_end: Option<CommentPosition>,
    #[serde(
        rename = "origRightFileStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_right_file_start: Option<CommentPosition>,
    #[doc = "The iteration of the file on the right side of the diff that the thread will be tracked to. Threads were tracked if this is greater than 0."]
    #[serde(
        rename = "secondComparingIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub second_comparing_iteration: Option<i32>,
}
impl CommentTrackingCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for a completion errors on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompletionErrorsEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
    #[doc = "The error message associated with the completion error"]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}
impl CompletionErrorsEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for a discussions update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscussionsUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl DiscussionsUpdatedEvent {
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
#[doc = "Provides properties that describe file differences"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileDiff {
    #[doc = "The collection of line diff blocks"]
    #[serde(
        rename = "lineDiffBlocks",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub line_diff_blocks: Vec<LineDiffBlock>,
    #[doc = "Original path of item if different from current path."]
    #[serde(
        rename = "originalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_path: Option<String>,
    #[doc = "Current path of item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl FileDiff {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides parameters that describe inputs for the file diff"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileDiffParams {
    #[doc = "Original path of the file"]
    #[serde(
        rename = "originalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_path: Option<String>,
    #[doc = "Current path of the file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl FileDiffParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides properties that describe inputs for the file diffs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileDiffsCriteria {
    #[doc = "Commit ID of the base version"]
    #[serde(
        rename = "baseVersionCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_version_commit: Option<String>,
    #[doc = "List of parameters for each of the files for which we need to get the file diff"]
    #[serde(
        rename = "fileDiffParams",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_diff_params: Vec<FileDiffParams>,
    #[doc = "Commit ID of the target version"]
    #[serde(
        rename = "targetVersionCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_version_commit: Option<String>,
}
impl FileDiffsCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Git annotated tag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitAnnotatedTag {
    #[doc = "The tagging Message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The name of the annotated tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The objectId (Sha1Id) of the tag."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "User info and date for Git operations."]
    #[serde(rename = "taggedBy", default, skip_serializing_if = "Option::is_none")]
    pub tagged_by: Option<GitUserDate>,
    #[doc = "Git object identifier and type information."]
    #[serde(
        rename = "taggedObject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tagged_object: Option<GitObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitAnnotatedTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitAsyncRefOperation {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Information about the progress of a cherry pick or revert operation."]
    #[serde(
        rename = "detailedStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_status: Option<GitAsyncRefOperationDetail>,
    #[doc = "Parameters that are provided in the request body when requesting to cherry pick or revert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<GitAsyncRefOperationParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<git_async_ref_operation::Status>,
    #[doc = "A URL that can be used to make further requests for status about the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitAsyncRefOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_async_ref_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
}
#[doc = "Information about the progress of a cherry pick or revert operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitAsyncRefOperationDetail {
    #[doc = "Indicates if there was a conflict generated when trying to cherry pick or revert the changes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict: Option<bool>,
    #[doc = "The current commit from the list of commits that are being cherry picked or reverted."]
    #[serde(
        rename = "currentCommitId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_commit_id: Option<String>,
    #[doc = "Detailed information about why the cherry pick or revert failed to complete."]
    #[serde(
        rename = "failureMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_message: Option<String>,
    #[doc = "A number between 0 and 1 indicating the percent complete of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[doc = "Provides a status code that indicates the reason the cherry pick or revert failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<git_async_ref_operation_detail::Status>,
    #[doc = "Indicates if the operation went beyond the maximum time allowed for a cherry pick or revert operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timedout: Option<bool>,
}
impl GitAsyncRefOperationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_async_ref_operation_detail {
    use super::*;
    #[doc = "Provides a status code that indicates the reason the cherry pick or revert failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "invalidRefName")]
        InvalidRefName,
        #[serde(rename = "refNameConflict")]
        RefNameConflict,
        #[serde(rename = "createBranchPermissionRequired")]
        CreateBranchPermissionRequired,
        #[serde(rename = "writePermissionRequired")]
        WritePermissionRequired,
        #[serde(rename = "targetBranchDeleted")]
        TargetBranchDeleted,
        #[serde(rename = "gitObjectTooLarge")]
        GitObjectTooLarge,
        #[serde(rename = "operationIndentityNotFound")]
        OperationIndentityNotFound,
        #[serde(rename = "asyncOperationNotFound")]
        AsyncOperationNotFound,
        #[serde(rename = "other")]
        Other,
        #[serde(rename = "emptyCommitterSignature")]
        EmptyCommitterSignature,
    }
}
#[doc = "Parameters that are provided in the request body when requesting to cherry pick or revert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitAsyncRefOperationParameters {
    #[doc = "Proposed target branch name for the cherry pick or revert operation."]
    #[serde(
        rename = "generatedRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generated_ref_name: Option<String>,
    #[doc = "The target branch for the cherry pick or revert operation."]
    #[serde(
        rename = "ontoRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub onto_ref_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<GitRepository>,
    #[doc = "GitAsyncRefOperationSource specifies the pull request or list of commits to use when making a cherry pick and revert operation request. Only one should be provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GitAsyncRefOperationSource>,
}
impl GitAsyncRefOperationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitAsyncRefOperationSource specifies the pull request or list of commits to use when making a cherry pick and revert operation request. Only one should be provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitAsyncRefOperationSource {
    #[doc = "A list of commits to cherry pick or revert"]
    #[serde(
        rename = "commitList",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commit_list: Vec<GitCommitRef>,
    #[doc = "Id of the pull request to cherry pick or revert"]
    #[serde(
        rename = "pullRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_id: Option<i32>,
}
impl GitAsyncRefOperationSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitBaseVersionDescriptor {
    #[serde(flatten)]
    pub git_version_descriptor: GitVersionDescriptor,
    #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
    #[serde(
        rename = "baseVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_version: Option<String>,
    #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
    #[serde(
        rename = "baseVersionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_version_options: Option<git_base_version_descriptor::BaseVersionOptions>,
    #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
    #[serde(
        rename = "baseVersionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_version_type: Option<git_base_version_descriptor::BaseVersionType>,
}
impl GitBaseVersionDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_base_version_descriptor {
    use super::*;
    #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BaseVersionOptions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previousChange")]
        PreviousChange,
        #[serde(rename = "firstParent")]
        FirstParent,
    }
    #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BaseVersionType {
        #[serde(rename = "branch")]
        Branch,
        #[serde(rename = "tag")]
        Tag,
        #[serde(rename = "commit")]
        Commit,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitBlobRef {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "SHA1 hash of git object"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Size of blob content (in bytes)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitBlobRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ahead and behind counts for a particular ref."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitBranchStats {
    #[doc = "Number of commits ahead."]
    #[serde(
        rename = "aheadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ahead_count: Option<i32>,
    #[doc = "Number of commits behind."]
    #[serde(
        rename = "behindCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub behind_count: Option<i32>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<GitCommitRef>,
    #[doc = "True if this is the result for the base version."]
    #[serde(
        rename = "isBaseVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_base_version: Option<bool>,
    #[doc = "Name of the ref."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl GitBranchStats {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitBranchStatsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitBranchStats>,
}
impl GitBranchStatsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitChange {
    #[serde(flatten)]
    pub change: Change,
    #[doc = "ID of the change within the group of changes."]
    #[serde(rename = "changeId", default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<i32>,
    #[serde(
        rename = "newContentTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_content_template: Option<GitTemplate>,
    #[doc = "Original path of item if different from current path."]
    #[serde(
        rename = "originalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_path: Option<String>,
}
impl GitChange {
    pub fn new(change: Change) -> Self {
        Self {
            change,
            change_id: None,
            new_content_template: None,
            original_path: None,
        }
    }
}
#[doc = "This object is returned from Cherry Pick operations and provides the id and status of the operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCherryPick {
    #[serde(flatten)]
    pub git_async_ref_operation: GitAsyncRefOperation,
    #[serde(
        rename = "cherryPickId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cherry_pick_id: Option<i32>,
}
impl GitCherryPick {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCommit {
    #[serde(flatten)]
    pub git_commit_ref: GitCommitRef,
    #[serde(rename = "treeId", default, skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}
impl GitCommit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCommitChanges {
    #[serde(
        rename = "changeCounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_counts: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changes: Vec<GitChange>,
}
impl GitCommitChanges {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCommitDiffs {
    #[serde(
        rename = "aheadCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ahead_count: Option<i32>,
    #[serde(
        rename = "allChangesIncluded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_changes_included: Option<bool>,
    #[serde(
        rename = "baseCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_commit: Option<String>,
    #[serde(
        rename = "behindCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub behind_count: Option<i32>,
    #[serde(
        rename = "changeCounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_counts: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changes: Vec<GitChange>,
    #[serde(
        rename = "commonCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub common_commit: Option<String>,
    #[serde(
        rename = "targetCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_commit: Option<String>,
}
impl GitCommitDiffs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides properties that describe a Git commit and associated metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCommitRef {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "User info and date for Git operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<GitUserDate>,
    #[serde(
        rename = "changeCounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_counts: Option<serde_json::Value>,
    #[doc = "An enumeration of the changes included with the commit."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changes: Vec<GitChange>,
    #[doc = "Comment or message of the commit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Indicates if the comment is truncated from the full Git commit comment message."]
    #[serde(
        rename = "commentTruncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_truncated: Option<bool>,
    #[doc = "ID (SHA-1) of the commit."]
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[doc = "User info and date for Git operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<GitUserDate>,
    #[doc = "Indicates that commit contains too many changes to be displayed"]
    #[serde(
        rename = "commitTooManyChanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commit_too_many_changes: Option<bool>,
    #[doc = "An enumeration of the parent commit IDs for this commit."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parents: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<GitPushRef>,
    #[doc = "Remote URL path to the commit."]
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[doc = "A list of status metadata from services and extensions that may associate additional information to the commit."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub statuses: Vec<GitStatus>,
    #[doc = "REST URL for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "A list of workitems associated with this commit."]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<ResourceRef>,
}
impl GitCommitRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCommitRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitCommitRef>,
}
impl GitCommitRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCommitToCreate {
    #[serde(rename = "baseRef", default, skip_serializing_if = "Option::is_none")]
    pub base_ref: Option<GitRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "pathActions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub path_actions: Vec<GitPathAction>,
}
impl GitCommitToCreate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflict {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        rename = "conflictId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conflict_id: Option<i32>,
    #[serde(
        rename = "conflictPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conflict_path: Option<String>,
    #[serde(
        rename = "conflictType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conflict_type: Option<git_conflict::ConflictType>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "mergeBaseCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_base_commit: Option<GitCommitRef>,
    #[serde(
        rename = "mergeOrigin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_origin: Option<GitMergeOriginRef>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "mergeSourceCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_source_commit: Option<GitCommitRef>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "mergeTargetCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_target_commit: Option<GitCommitRef>,
    #[serde(
        rename = "resolutionError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_error: Option<git_conflict::ResolutionError>,
    #[serde(
        rename = "resolutionStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_status: Option<git_conflict::ResolutionStatus>,
    #[serde(
        rename = "resolvedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolved_by: Option<IdentityRef>,
    #[serde(
        rename = "resolvedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub resolved_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitConflict {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_conflict {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConflictType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "addAdd")]
        AddAdd,
        #[serde(rename = "addRename")]
        AddRename,
        #[serde(rename = "deleteEdit")]
        DeleteEdit,
        #[serde(rename = "deleteRename")]
        DeleteRename,
        #[serde(rename = "directoryFile")]
        DirectoryFile,
        #[serde(rename = "directoryChild")]
        DirectoryChild,
        #[serde(rename = "editDelete")]
        EditDelete,
        #[serde(rename = "editEdit")]
        EditEdit,
        #[serde(rename = "fileDirectory")]
        FileDirectory,
        #[serde(rename = "rename1to2")]
        Rename1to2,
        #[serde(rename = "rename2to1")]
        Rename2to1,
        #[serde(rename = "renameAdd")]
        RenameAdd,
        #[serde(rename = "renameDelete")]
        RenameDelete,
        #[serde(rename = "renameRename")]
        RenameRename,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResolutionError {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "mergeContentNotFound")]
        MergeContentNotFound,
        #[serde(rename = "pathInUse")]
        PathInUse,
        #[serde(rename = "invalidPath")]
        InvalidPath,
        #[serde(rename = "unknownAction")]
        UnknownAction,
        #[serde(rename = "unknownMergeType")]
        UnknownMergeType,
        #[serde(rename = "otherError")]
        OtherError,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResolutionStatus {
        #[serde(rename = "unresolved")]
        Unresolved,
        #[serde(rename = "partiallyResolved")]
        PartiallyResolved,
        #[serde(rename = "resolved")]
        Resolved,
    }
}
#[doc = "Data object for AddAdd conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictAddAdd {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionMergeContent>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
}
impl GitConflictAddAdd {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for RenameAdd conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictAddRename {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPathConflict>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetOriginalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_original_path: Option<String>,
}
impl GitConflictAddRename {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for EditDelete conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictDeleteEdit {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPickOneAction>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
}
impl GitConflictDeleteEdit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for RenameDelete conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictDeleteRename {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPickOneAction>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetNewPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_new_path: Option<String>,
}
impl GitConflictDeleteRename {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for FileDirectory conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictDirectoryFile {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPathConflict>,
    #[serde(
        rename = "sourceTree",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_tree: Option<GitTreeRef>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
}
impl GitConflictDirectoryFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for DeleteEdit conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictEditDelete {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPickOneAction>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
}
impl GitConflictEditDelete {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for EditEdit conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictEditEdit {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionMergeContent>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
}
impl GitConflictEditEdit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for DirectoryFile conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictFileDirectory {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPathConflict>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetTree",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_tree: Option<GitTreeRef>,
}
impl GitConflictFileDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for Rename1to2 conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictRename1to2 {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionRename1to2>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "sourceNewPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_new_path: Option<String>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetNewPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_new_path: Option<String>,
}
impl GitConflictRename1to2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for Rename2to1 conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictRename2to1 {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPathConflict>,
    #[serde(
        rename = "sourceNewBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_new_blob: Option<GitBlobRef>,
    #[serde(
        rename = "sourceOriginalBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_original_blob: Option<GitBlobRef>,
    #[serde(
        rename = "sourceOriginalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_original_path: Option<String>,
    #[serde(
        rename = "targetNewBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_new_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetOriginalBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_original_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetOriginalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_original_path: Option<String>,
}
impl GitConflictRename2to1 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for AddRename conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictRenameAdd {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPathConflict>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "sourceOriginalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_original_path: Option<String>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
}
impl GitConflictRenameAdd {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for DeleteRename conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictRenameDelete {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionPickOneAction>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "sourceNewPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_new_path: Option<String>,
}
impl GitConflictRenameDelete {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data object for RenameRename conflict"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictRenameRename {
    #[serde(flatten)]
    pub git_conflict: GitConflict,
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<GitBlobRef>,
    #[serde(
        rename = "originalPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GitResolutionMergeContent>,
    #[serde(
        rename = "sourceBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_blob: Option<GitBlobRef>,
    #[serde(
        rename = "targetBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_blob: Option<GitBlobRef>,
}
impl GitConflictRenameRename {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitConflictUpdateResult {
    #[doc = "Conflict ID that was provided by input"]
    #[serde(
        rename = "conflictId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conflict_id: Option<i32>,
    #[doc = "Reason for failing"]
    #[serde(
        rename = "customMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_message: Option<String>,
    #[serde(
        rename = "updatedConflict",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_conflict: Option<GitConflict>,
    #[doc = "Status of the update on the server"]
    #[serde(
        rename = "updateStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_status: Option<git_conflict_update_result::UpdateStatus>,
}
impl GitConflictUpdateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_conflict_update_result {
    use super::*;
    #[doc = "Status of the update on the server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateStatus {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "badRequest")]
        BadRequest,
        #[serde(rename = "invalidResolution")]
        InvalidResolution,
        #[serde(rename = "unsupportedConflictType")]
        UnsupportedConflictType,
        #[serde(rename = "notFound")]
        NotFound,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitDeletedRepository {
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[serde(rename = "deletedBy", default, skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<IdentityRef>,
    #[serde(
        rename = "deletedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub deleted_date: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
}
impl GitDeletedRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitDeletedRepositoryList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitDeletedRepository>,
}
impl GitDeletedRepositoryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitFilePathsCollection {
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitFilePathsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status information about a requested fork operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitForkOperationStatusDetail {
    #[doc = "All valid steps for the forking process"]
    #[serde(
        rename = "allSteps",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_steps: Vec<String>,
    #[doc = "Index into AllSteps for the current step"]
    #[serde(
        rename = "currentStep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_step: Option<i32>,
    #[doc = "Error message if the operation failed."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}
impl GitForkOperationStatusDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a fork ref."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitForkRef {
    #[serde(flatten)]
    pub git_ref: GitRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<GitRepository>,
}
impl GitForkRef {
    pub fn new(git_ref: GitRef) -> Self {
        Self {
            git_ref,
            repository: None,
        }
    }
}
#[doc = "Request to sync data between two forks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitForkSyncRequest {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Status information about a requested fork operation."]
    #[serde(
        rename = "detailedStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_status: Option<GitForkOperationStatusDetail>,
    #[doc = "Unique identifier for the operation."]
    #[serde(
        rename = "operationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_id: Option<i32>,
    #[doc = "Globally unique key for a repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GlobalGitRepositoryKey>,
    #[doc = "If supplied, the set of ref mappings to use when performing a \"sync\" or create. If missing, all refs will be synchronized."]
    #[serde(
        rename = "sourceToTargetRefs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_to_target_refs: Vec<SourceToTargetRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<git_fork_sync_request::Status>,
}
impl GitForkSyncRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_fork_sync_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitForkSyncRequestList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitForkSyncRequest>,
}
impl GitForkSyncRequestList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating a fork request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitForkSyncRequestParameters {
    #[doc = "Globally unique key for a repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GlobalGitRepositoryKey>,
    #[doc = "If supplied, the set of ref mappings to use when performing a \"sync\" or create. If missing, all refs will be synchronized."]
    #[serde(
        rename = "sourceToTargetRefs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_to_target_refs: Vec<SourceToTargetRef>,
}
impl GitForkSyncRequestParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitForkTeamProjectReference {
    #[serde(flatten)]
    pub team_project_reference: TeamProjectReference,
}
impl GitForkTeamProjectReference {
    pub fn new(team_project_reference: TeamProjectReference) -> Self {
        Self {
            team_project_reference,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportFailedEvent {
    #[serde(
        rename = "sourceRepositoryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_repository_name: Option<String>,
    #[serde(
        rename = "targetRepository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_repository: Option<GitRepository>,
}
impl GitImportFailedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter for creating a git import request when source is Git version control"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportGitSource {
    #[doc = "Tells if this is a sync request or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[doc = "Url for the source repo"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitImportGitSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request to import data from a remote source control system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportRequest {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Additional status information about an import request."]
    #[serde(
        rename = "detailedStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_status: Option<GitImportStatusDetail>,
    #[doc = "The unique identifier for this import request."]
    #[serde(
        rename = "importRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_request_id: Option<i32>,
    #[doc = "Parameters for creating an import request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<GitImportRequestParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<GitRepository>,
    #[doc = "Current status of the import."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<git_import_request::Status>,
    #[doc = "A link back to this import request resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitImportRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_import_request {
    use super::*;
    #[doc = "Current status of the import."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportRequestList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitImportRequest>,
}
impl GitImportRequestList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating an import request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportRequestParameters {
    #[doc = "Option to delete service endpoint when import is done"]
    #[serde(
        rename = "deleteServiceEndpointAfterImportIsDone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_service_endpoint_after_import_is_done: Option<bool>,
    #[doc = "Parameter for creating a git import request when source is Git version control"]
    #[serde(rename = "gitSource", default, skip_serializing_if = "Option::is_none")]
    pub git_source: Option<GitImportGitSource>,
    #[doc = "Service Endpoint for connection to external endpoint"]
    #[serde(
        rename = "serviceEndpointId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoint_id: Option<String>,
    #[doc = "Parameter for creating a git import request when source is tfvc version control"]
    #[serde(
        rename = "tfvcSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tfvc_source: Option<GitImportTfvcSource>,
}
impl GitImportRequestParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional status information about an import request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportStatusDetail {
    #[doc = "All valid steps for the import process"]
    #[serde(
        rename = "allSteps",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_steps: Vec<String>,
    #[doc = "Index into AllSteps for the current step"]
    #[serde(
        rename = "currentStep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_step: Option<i32>,
    #[doc = "Error message if the operation failed."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}
impl GitImportStatusDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportSucceededEvent {
    #[serde(
        rename = "sourceRepositoryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_repository_name: Option<String>,
    #[serde(
        rename = "targetRepository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_repository: Option<GitRepository>,
}
impl GitImportSucceededEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter for creating a git import request when source is tfvc version control"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitImportTfvcSource {
    #[doc = "Set true to import History, false otherwise"]
    #[serde(
        rename = "importHistory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_history: Option<bool>,
    #[doc = "Get history for last n days (max allowed value is 180 days)"]
    #[serde(
        rename = "importHistoryDurationInDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_history_duration_in_days: Option<i32>,
    #[doc = "Path which we want to import (this can be copied from Path Control in Explorer)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl GitImportTfvcSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitItem {
    #[serde(flatten)]
    pub item_model: ItemModel,
    #[doc = "SHA1 of commit item was fetched at"]
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[doc = "Type of object (Commit, Tree, Blob, Tag, ...)"]
    #[serde(
        rename = "gitObjectType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub git_object_type: Option<git_item::GitObjectType>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "latestProcessedChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_processed_change: Option<GitCommitRef>,
    #[doc = "Git object id"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Git object id"]
    #[serde(
        rename = "originalObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_object_id: Option<String>,
}
impl GitItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_item {
    use super::*;
    #[doc = "Type of object (Commit, Tree, Blob, Tag, ...)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GitObjectType {
        #[serde(rename = "bad")]
        Bad,
        #[serde(rename = "commit")]
        Commit,
        #[serde(rename = "tree")]
        Tree,
        #[serde(rename = "blob")]
        Blob,
        #[serde(rename = "tag")]
        Tag,
        #[serde(rename = "ext2")]
        Ext2,
        #[serde(rename = "ofsDelta")]
        OfsDelta,
        #[serde(rename = "refDelta")]
        RefDelta,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitItemDescriptor {
    #[doc = "Path to item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Specifies whether to include children (OneLevel), all descendants (Full), or None"]
    #[serde(
        rename = "recursionLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursion_level: Option<git_item_descriptor::RecursionLevel>,
    #[doc = "Version string (interpretation based on VersionType defined in subclass"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Version modifiers (e.g. previous)"]
    #[serde(
        rename = "versionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_options: Option<git_item_descriptor::VersionOptions>,
    #[doc = "How to interpret version (branch,tag,commit)"]
    #[serde(
        rename = "versionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_type: Option<git_item_descriptor::VersionType>,
}
impl GitItemDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_item_descriptor {
    use super::*;
    #[doc = "Specifies whether to include children (OneLevel), all descendants (Full), or None"]
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
    #[doc = "Version modifiers (e.g. previous)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum VersionOptions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previousChange")]
        PreviousChange,
        #[serde(rename = "firstParent")]
        FirstParent,
    }
    #[doc = "How to interpret version (branch,tag,commit)"]
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
pub struct GitItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitItem>,
}
impl GitItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitItemRequestData {
    #[doc = "Whether to include metadata for all items"]
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
    #[doc = "Collection of items to fetch, including path, version, and recursion level"]
    #[serde(
        rename = "itemDescriptors",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub item_descriptors: Vec<GitItemDescriptor>,
    #[doc = "Whether to include shallow ref to commit that last changed each item"]
    #[serde(
        rename = "latestProcessedChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_processed_change: Option<bool>,
}
impl GitItemRequestData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type GitItems = Vec<GitItem>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitItemsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitItems>,
}
impl GitItemsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLastChangeItem {
    #[doc = "Gets or sets the commit Id this item was modified most recently for the provided version."]
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[doc = "Gets or sets the path of the item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl GitLastChangeItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLastChangeTreeItems {
    #[doc = "The list of commits referenced by Items, if they were requested."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commits: Vec<GitCommitRef>,
    #[doc = "The last change of items."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GitLastChangeItem>,
    #[doc = "The last explored time, in case the result is not comprehensive. Null otherwise."]
    #[serde(
        rename = "lastExploredTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_explored_time: Option<time::OffsetDateTime>,
}
impl GitLastChangeTreeItems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitMerge {
    #[serde(flatten)]
    pub git_merge_parameters: GitMergeParameters,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Status information about a requested merge operation."]
    #[serde(
        rename = "detailedStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed_status: Option<GitMergeOperationStatusDetail>,
    #[doc = "Unique identifier for the merge operation."]
    #[serde(
        rename = "mergeOperationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_operation_id: Option<i32>,
    #[doc = "Status of the merge operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<git_merge::Status>,
}
impl GitMerge {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_merge {
    use super::*;
    #[doc = "Status of the merge operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "abandoned")]
        Abandoned,
    }
}
#[doc = "Status information about a requested merge operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitMergeOperationStatusDetail {
    #[doc = "Error message if the operation failed."]
    #[serde(
        rename = "failureMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_message: Option<String>,
    #[doc = "The commitId of the resultant merge commit."]
    #[serde(
        rename = "mergeCommitId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_commit_id: Option<String>,
}
impl GitMergeOperationStatusDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitMergeOriginRef {
    #[serde(
        rename = "cherryPickId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cherry_pick_id: Option<i32>,
    #[serde(
        rename = "pullRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_id: Option<i32>,
    #[serde(rename = "revertId", default, skip_serializing_if = "Option::is_none")]
    pub revert_id: Option<i32>,
}
impl GitMergeOriginRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters required for performing git merge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitMergeParameters {
    #[doc = "Comment or message of the commit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "An enumeration of the parent commit IDs for the merge  commit."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parents: Vec<String>,
}
impl GitMergeParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Git object identifier and type information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitObject {
    #[doc = "Object Id (Sha1Id)."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Type of object (Commit, Tree, Blob, Tag)"]
    #[serde(
        rename = "objectType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub object_type: Option<git_object::ObjectType>,
}
impl GitObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_object {
    use super::*;
    #[doc = "Type of object (Commit, Tree, Blob, Tag)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ObjectType {
        #[serde(rename = "bad")]
        Bad,
        #[serde(rename = "commit")]
        Commit,
        #[serde(rename = "tree")]
        Tree,
        #[serde(rename = "blob")]
        Blob,
        #[serde(rename = "tag")]
        Tag,
        #[serde(rename = "ext2")]
        Ext2,
        #[serde(rename = "ofsDelta")]
        OfsDelta,
        #[serde(rename = "refDelta")]
        RefDelta,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPathAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<git_path_action::Action>,
    #[serde(
        rename = "base64Content",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base64_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "rawTextContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub raw_text_content: Option<String>,
    #[serde(
        rename = "targetPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_path: Option<String>,
}
impl GitPathAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_path_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "edit")]
        Edit,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "rename")]
        Rename,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPathToItemsCollection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<serde_json::Value>,
}
impl GitPathToItemsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPolicyConfigurationResponse {
    #[doc = "The HTTP client methods find the continuation token header in the response and populate this field."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[serde(
        rename = "policyConfigurations",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_configurations: Vec<PolicyConfiguration>,
}
impl GitPolicyConfigurationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents all the data associated with a pull request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitPullRequest {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "A string which uniquely identifies this pull request. To generate an artifact ID for a pull request, use this template: ```vstfs:///Git/PullRequestId/{projectId}/{repositoryId}/{pullRequestId}```"]
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_id: Option<String>,
    #[serde(
        rename = "autoCompleteSetBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_complete_set_by: Option<IdentityRef>,
    #[serde(rename = "closedBy", default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<IdentityRef>,
    #[doc = "The date when the pull request was closed (completed, abandoned, or merged externally)."]
    #[serde(
        rename = "closedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub closed_date: Option<time::OffsetDateTime>,
    #[doc = "The code review ID of the pull request. Used internally."]
    #[serde(
        rename = "codeReviewId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code_review_id: Option<i32>,
    #[doc = "The commits contained in the pull request."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commits: Vec<GitCommitRef>,
    #[doc = "Preferences about how the pull request should be completed."]
    #[serde(
        rename = "completionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_options: Option<GitPullRequestCompletionOptions>,
    #[doc = "The most recent date at which the pull request entered the queue to be completed. Used internally."]
    #[serde(
        rename = "completionQueueTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completion_queue_time: Option<time::OffsetDateTime>,
    #[serde(rename = "createdBy")]
    pub created_by: IdentityRef,
    #[doc = "The date when the pull request was created."]
    #[serde(rename = "creationDate", with = "crate::date_time::rfc3339")]
    pub creation_date: time::OffsetDateTime,
    #[doc = "The description of the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Information about a fork ref."]
    #[serde(
        rename = "forkSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fork_source: Option<GitForkRef>,
    #[doc = "Multiple mergebases warning"]
    #[serde(
        rename = "hasMultipleMergeBases",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_multiple_merge_bases: Option<bool>,
    #[doc = "Draft / WIP pull request."]
    #[serde(rename = "isDraft")]
    pub is_draft: bool,
    #[doc = "The labels associated with the pull request."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<WebApiTagDefinition>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "lastMergeCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_merge_commit: Option<GitCommitRef>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "lastMergeSourceCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_merge_source_commit: Option<GitCommitRef>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "lastMergeTargetCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_merge_target_commit: Option<GitCommitRef>,
    #[doc = "If set, pull request merge failed for this reason."]
    #[serde(
        rename = "mergeFailureMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_failure_message: Option<String>,
    #[doc = "The type of failure (if any) of the pull request merge."]
    #[serde(
        rename = "mergeFailureType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_failure_type: Option<git_pull_request::MergeFailureType>,
    #[doc = "The ID of the job used to run the pull request merge. Used internally."]
    #[serde(rename = "mergeId", default, skip_serializing_if = "Option::is_none")]
    pub merge_id: Option<String>,
    #[doc = "The options which are used when a pull request merge is created."]
    #[serde(
        rename = "mergeOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_options: Option<GitPullRequestMergeOptions>,
    #[doc = "The current status of the pull request merge."]
    #[serde(
        rename = "mergeStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_status: Option<git_pull_request::MergeStatus>,
    #[doc = "The ID of the pull request."]
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: i32,
    #[doc = "Used internally."]
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    pub repository: GitRepository,
    #[doc = "A list of reviewers on the pull request along with the state of their votes."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reviewers: Vec<IdentityRefWithVote>,
    #[doc = "The name of the source branch of the pull request."]
    #[serde(rename = "sourceRefName")]
    pub source_ref_name: String,
    #[doc = "The status of the pull request."]
    pub status: git_pull_request::Status,
    #[doc = "If true, this pull request supports multiple iterations. Iteration support means individual pushes to the source branch of the pull request can be reviewed and comments left in one iteration will be tracked across future iterations."]
    #[serde(
        rename = "supportsIterations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_iterations: Option<bool>,
    #[doc = "The name of the target branch of the pull request."]
    #[serde(rename = "targetRefName")]
    pub target_ref_name: String,
    #[doc = "The title of the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Used internally."]
    pub url: String,
    #[doc = "Any work item references associated with this pull request."]
    #[serde(
        rename = "workItemRefs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_refs: Vec<ResourceRef>,
}
impl GitPullRequest {
    pub fn new(
        created_by: IdentityRef,
        creation_date: time::OffsetDateTime,
        is_draft: bool,
        pull_request_id: i32,
        repository: GitRepository,
        source_ref_name: String,
        status: git_pull_request::Status,
        target_ref_name: String,
        url: String,
    ) -> Self {
        Self {
            links: None,
            artifact_id: None,
            auto_complete_set_by: None,
            closed_by: None,
            closed_date: None,
            code_review_id: None,
            commits: Vec::new(),
            completion_options: None,
            completion_queue_time: None,
            created_by,
            creation_date,
            description: None,
            fork_source: None,
            has_multiple_merge_bases: None,
            is_draft,
            labels: Vec::new(),
            last_merge_commit: None,
            last_merge_source_commit: None,
            last_merge_target_commit: None,
            merge_failure_message: None,
            merge_failure_type: None,
            merge_id: None,
            merge_options: None,
            merge_status: None,
            pull_request_id,
            remote_url: None,
            repository,
            reviewers: Vec::new(),
            source_ref_name,
            status,
            supports_iterations: None,
            target_ref_name,
            title: None,
            url,
            work_item_refs: Vec::new(),
        }
    }
}
pub mod git_pull_request {
    use super::*;
    #[doc = "The type of failure (if any) of the pull request merge."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MergeFailureType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "caseSensitive")]
        CaseSensitive,
        #[serde(rename = "objectTooLarge")]
        ObjectTooLarge,
    }
    #[doc = "The current status of the pull request merge."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MergeStatus {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "conflicts")]
        Conflicts,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "rejectedByPolicy")]
        RejectedByPolicy,
        #[serde(rename = "failure")]
        Failure,
    }
    #[doc = "The status of the pull request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "abandoned")]
        Abandoned,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "Change made in a pull request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitPullRequestChange {
    #[serde(flatten)]
    pub git_change: GitChange,
    #[doc = "ID used to track files through multiple changes."]
    #[serde(
        rename = "changeTrackingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_tracking_id: Option<i32>,
}
impl GitPullRequestChange {
    pub fn new(git_change: GitChange) -> Self {
        Self {
            git_change,
            change_tracking_id: None,
        }
    }
}
#[doc = "Represents a comment thread of a pull request. A thread contains meta data about the file it was left on (if any) along with one or more comments (an initial comment and the subsequent replies)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestCommentThread {
    #[serde(flatten)]
    pub comment_thread: CommentThread,
    #[doc = "Comment thread context contains details about what diffs were being viewed at the time of thread creation and whether or not the thread has been tracked from that original diff."]
    #[serde(
        rename = "pullRequestThreadContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_thread_context: Option<GitPullRequestCommentThreadContext>,
}
impl GitPullRequestCommentThread {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Comment thread context contains details about what diffs were being viewed at the time of thread creation and whether or not the thread has been tracked from that original diff."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestCommentThreadContext {
    #[doc = "Used to track a comment across iterations. This value can be found by looking at the iteration's changes list. Must be set for pull requests with iteration support. Otherwise, it's not required for 'legacy' pull requests."]
    #[serde(
        rename = "changeTrackingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_tracking_id: Option<i32>,
    #[doc = "Comment iteration context is used to identify which diff was being viewed when the thread was created."]
    #[serde(
        rename = "iterationContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_context: Option<CommentIterationContext>,
    #[doc = "Comment tracking criteria is used to identify which iteration context the thread has been tracked to (if any) along with some detail about the original position and filename."]
    #[serde(
        rename = "trackingCriteria",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tracking_criteria: Option<CommentTrackingCriteria>,
}
impl GitPullRequestCommentThreadContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestCommentThreadList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitPullRequestCommentThread>,
}
impl GitPullRequestCommentThreadList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Preferences about how the pull request should be completed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestCompletionOptions {
    #[doc = "List of any policy configuration Id's which auto-complete should not wait for. Only applies to optional policies (isBlocking == false). Auto-complete always waits for required policies (isBlocking == true)."]
    #[serde(
        rename = "autoCompleteIgnoreConfigIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub auto_complete_ignore_config_ids: Vec<i32>,
    #[doc = "If true, policies will be explicitly bypassed while the pull request is completed."]
    #[serde(
        rename = "bypassPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_policy: Option<bool>,
    #[doc = "If policies are bypassed, this reason is stored as to why bypass was used."]
    #[serde(
        rename = "bypassReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_reason: Option<String>,
    #[doc = "If true, the source branch of the pull request will be deleted after completion."]
    #[serde(
        rename = "deleteSourceBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_source_branch: Option<bool>,
    #[doc = "If set, this will be used as the commit message of the merge commit."]
    #[serde(
        rename = "mergeCommitMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_commit_message: Option<String>,
    #[doc = "Specify the strategy used to merge the pull request during completion. If MergeStrategy is not set to any value, a no-FF merge will be created if SquashMerge == false. If MergeStrategy is not set to any value, the pull request commits will be squashed if SquashMerge == true. The SquashMerge property is deprecated. It is recommended that you explicitly set MergeStrategy in all cases. If an explicit value is provided for MergeStrategy, the SquashMerge property will be ignored."]
    #[serde(
        rename = "mergeStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_strategy: Option<git_pull_request_completion_options::MergeStrategy>,
    #[doc = "SquashMerge is deprecated. You should explicitly set the value of MergeStrategy. If MergeStrategy is set to any value, the SquashMerge value will be ignored. If MergeStrategy is not set, the merge strategy will be no-fast-forward if this flag is false, or squash if true."]
    #[serde(
        rename = "squashMerge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub squash_merge: Option<bool>,
    #[doc = "If true, we will attempt to transition any work items linked to the pull request into the next logical state (i.e. Active -> Resolved)"]
    #[serde(
        rename = "transitionWorkItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transition_work_items: Option<bool>,
    #[doc = "If true, the current completion attempt was triggered via auto-complete. Used internally."]
    #[serde(
        rename = "triggeredByAutoComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub triggered_by_auto_complete: Option<bool>,
}
impl GitPullRequestCompletionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_pull_request_completion_options {
    use super::*;
    #[doc = "Specify the strategy used to merge the pull request during completion. If MergeStrategy is not set to any value, a no-FF merge will be created if SquashMerge == false. If MergeStrategy is not set to any value, the pull request commits will be squashed if SquashMerge == true. The SquashMerge property is deprecated. It is recommended that you explicitly set MergeStrategy in all cases. If an explicit value is provided for MergeStrategy, the SquashMerge property will be ignored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MergeStrategy {
        #[serde(rename = "noFastForward")]
        NoFastForward,
        #[serde(rename = "squash")]
        Squash,
        #[serde(rename = "rebase")]
        Rebase,
        #[serde(rename = "rebaseMerge")]
        RebaseMerge,
    }
}
#[doc = "Pull Request create options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitPullRequestCreateOptions {
    #[doc = "The description of the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Draft / WIP pull request."]
    #[serde(rename = "isDraft", default, skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    #[doc = "The labels associated with the pull request."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<WebApiCreateTagRequestData>,
    #[doc = "The name of the source branch of the pull request."]
    #[serde(rename = "sourceRefName")]
    pub source_ref_name: String,
    #[doc = "The name of the target branch of the pull request."]
    #[serde(rename = "targetRefName")]
    pub target_ref_name: String,
    #[doc = "The title of the pull request."]
    pub title: String,
    #[doc = "The options which are used when a pull request merge is created."]
    #[serde(
        rename = "mergeOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_options: Option<GitPullRequestMergeOptions>,
    #[doc = "Preferences about how the pull request should be completed."]
    #[serde(
        rename = "completionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_options: Option<GitPullRequestCompletionOptions>,
    #[doc = "Any work item references associated with this pull request."]
    #[serde(
        rename = "workItemRefs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_refs: Vec<ResourceRef>,
    #[doc = "A list of reviewers on the pull request."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reviewers: Vec<IdentityId>,
}
impl GitPullRequestCreateOptions {
    pub fn new(source_ref_name: String, target_ref_name: String, title: String) -> Self {
        Self {
            description: None,
            is_draft: None,
            labels: Vec::new(),
            source_ref_name,
            target_ref_name,
            title,
            merge_options: None,
            completion_options: None,
            work_item_refs: Vec::new(),
            reviewers: Vec::new(),
        }
    }
}
#[doc = "Provides properties that describe a Git pull request iteration. Iterations are created as a result of creating and pushing updates to a pull request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestIteration {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
    #[doc = "Changes included with the pull request iteration."]
    #[serde(
        rename = "changeList",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub change_list: Vec<GitPullRequestChange>,
    #[doc = "The commits included with the pull request iteration."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commits: Vec<GitCommitRef>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "commonRefCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub common_ref_commit: Option<GitCommitRef>,
    #[doc = "The creation date of the pull request iteration."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Description of the pull request iteration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicates if the Commits property contains a truncated list of commits in this pull request iteration."]
    #[serde(
        rename = "hasMoreCommits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_more_commits: Option<bool>,
    #[doc = "ID of the pull request iteration. Iterations are created as a result of creating and pushing updates to a pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "If the iteration reason is Retarget, this is the refName of the new target"]
    #[serde(
        rename = "newTargetRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_target_ref_name: Option<String>,
    #[doc = "If the iteration reason is Retarget, this is the original target refName"]
    #[serde(
        rename = "oldTargetRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub old_target_ref_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<GitPushRef>,
    #[doc = "The reason for which the pull request iteration was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<git_pull_request_iteration::Reason>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "sourceRefCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_ref_commit: Option<GitCommitRef>,
    #[doc = "Provides properties that describe a Git commit and associated metadata."]
    #[serde(
        rename = "targetRefCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_ref_commit: Option<GitCommitRef>,
    #[doc = "The updated date of the pull request iteration."]
    #[serde(
        rename = "updatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub updated_date: Option<time::OffsetDateTime>,
}
impl GitPullRequestIteration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_pull_request_iteration {
    use super::*;
    #[doc = "The reason for which the pull request iteration was created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        #[serde(rename = "push")]
        Push,
        #[serde(rename = "forcePush")]
        ForcePush,
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "rebase")]
        Rebase,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "retarget")]
        Retarget,
        #[serde(rename = "resolveConflicts")]
        ResolveConflicts,
    }
}
#[doc = "Collection of changes made in a pull request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestIterationChanges {
    #[doc = "Changes made in the iteration."]
    #[serde(
        rename = "changeEntries",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub change_entries: Vec<GitPullRequestChange>,
    #[doc = "Value to specify as skip to get the next page of changes.  This will be zero if there are no more changes."]
    #[serde(rename = "nextSkip", default, skip_serializing_if = "Option::is_none")]
    pub next_skip: Option<i32>,
    #[doc = "Value to specify as top to get the next page of changes.  This will be zero if there are no more changes."]
    #[serde(rename = "nextTop", default, skip_serializing_if = "Option::is_none")]
    pub next_top: Option<i32>,
}
impl GitPullRequestIterationChanges {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestIterationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitPullRequestIteration>,
}
impl GitPullRequestIterationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitPullRequest>,
}
impl GitPullRequestList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The options which are used when a pull request merge is created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestMergeOptions {
    #[doc = "If true, conflict resolutions applied during the merge will be put in separate commits to preserve authorship info for git blame, etc."]
    #[serde(
        rename = "conflictAuthorshipCommits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conflict_authorship_commits: Option<bool>,
    #[serde(
        rename = "detectRenameFalsePositives",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detect_rename_false_positives: Option<bool>,
    #[doc = "If true, rename detection will not be performed during the merge."]
    #[serde(
        rename = "disableRenames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_renames: Option<bool>,
}
impl GitPullRequestMergeOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of pull request queries and their results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestQuery {
    #[doc = "The queries to perform."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub queries: Vec<GitPullRequestQueryInput>,
    #[doc = "The results of the queries. This matches the QueryInputs list so Results\\[n\\] are the results of QueryInputs\\[n\\]. Each entry in the list is a dictionary of commit->pull requests."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<serde_json::Value>,
}
impl GitPullRequestQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pull request query input parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestQueryInput {
    #[doc = "The list of commit IDs to search for."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
    #[doc = "The type of query to perform."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<git_pull_request_query_input::Type>,
}
impl GitPullRequestQueryInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_pull_request_query_input {
    use super::*;
    #[doc = "The type of query to perform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "lastMergeCommit")]
        LastMergeCommit,
        #[serde(rename = "commit")]
        Commit,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestReviewFileContentInfo {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "The file change path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Content hash of on-disk representation of file content. Its calculated by the client by using SHA1 hash function. Ensure that uploaded file has same encoding as in source control."]
    #[serde(rename = "shA1Hash", default, skip_serializing_if = "Option::is_none")]
    pub sh_a1_hash: Option<String>,
}
impl GitPullRequestReviewFileContentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pull requests can be searched for matching this criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestSearchCriteria {
    #[doc = "If set, search for pull requests that were created by this identity."]
    #[serde(rename = "creatorId", default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[doc = "Whether to include the _links field on the shallow references"]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[doc = "If specified, filters pull requests that created/closed before this date based on the queryTimeRangeType specified."]
    #[serde(
        rename = "maxTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub max_time: Option<time::OffsetDateTime>,
    #[doc = "If specified, filters pull requests that created/closed after this date based on the queryTimeRangeType specified."]
    #[serde(
        rename = "minTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub min_time: Option<time::OffsetDateTime>,
    #[doc = "The type of time range which should be used for minTime and maxTime. Defaults to Created if unset."]
    #[serde(
        rename = "queryTimeRangeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_time_range_type: Option<git_pull_request_search_criteria::QueryTimeRangeType>,
    #[doc = "If set, search for pull requests whose target branch is in this repository."]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
    #[doc = "If set, search for pull requests that have this identity as a reviewer."]
    #[serde(
        rename = "reviewerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reviewer_id: Option<String>,
    #[doc = "If set, search for pull requests from this branch."]
    #[serde(
        rename = "sourceRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_ref_name: Option<String>,
    #[doc = "If set, search for pull requests whose source branch is in this repository."]
    #[serde(
        rename = "sourceRepositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_repository_id: Option<String>,
    #[doc = "If set, search for pull requests that are in this state. Defaults to Active if unset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<git_pull_request_search_criteria::Status>,
    #[doc = "If set, search for pull requests into this branch."]
    #[serde(
        rename = "targetRefName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_ref_name: Option<String>,
}
impl GitPullRequestSearchCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_pull_request_search_criteria {
    use super::*;
    #[doc = "The type of time range which should be used for minTime and maxTime. Defaults to Created if unset."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryTimeRangeType {
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "closed")]
        Closed,
    }
    #[doc = "If set, search for pull requests that are in this state. Defaults to Active if unset."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "abandoned")]
        Abandoned,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "all")]
        All,
    }
}
#[doc = "This class contains the metadata of a service/extension posting pull request status. Status can be associated with a pull request or an iteration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestStatus {
    #[serde(flatten)]
    pub git_status: GitStatus,
    #[doc = "ID of the iteration to associate status with. Minimum value is 1."]
    #[serde(
        rename = "iterationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_id: Option<i32>,
    #[doc = "The class represents a property bag as a collection of key-value pairs. Values of all primitive types (any type with a `TypeCode != TypeCode.Object`) except for `DBNull` are accepted. Values of type Byte[], Int32, Double, DateType and String preserve their type, other primitives are retuned as a String. Byte[] expected as base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
}
impl GitPullRequestStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestStatusList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitPullRequestStatus>,
}
impl GitPullRequestStatusList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pull Request update options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPullRequestUpdateOptions {
    #[doc = "The description of the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The title of the pull request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "A list of reviewers on the pull request."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reviewers: Vec<IdentityId>,
    #[doc = "The options which are used when a pull request merge is created."]
    #[serde(
        rename = "mergeStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_status: Option<GitPullRequestMergeOptions>,
    #[doc = "Pull request status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PullRequestStatus>,
    #[serde(
        rename = "autoCompleteSetBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_complete_set_by: Option<IdentityRef>,
    #[doc = "Preferences about how the pull request should be completed."]
    #[serde(
        rename = "completionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_options: Option<GitPullRequestCompletionOptions>,
}
impl GitPullRequestUpdateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPush {
    #[serde(flatten)]
    pub git_push_ref: GitPushRef,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commits: Vec<GitCommitRef>,
    #[serde(
        rename = "refUpdates",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ref_updates: Vec<GitRefUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<GitRepository>,
}
impl GitPush {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPushEventData {
    #[serde(rename = "afterId", default, skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
    #[serde(rename = "beforeId", default, skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commits: Vec<GitCommit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<GitRepository>,
}
impl GitPushEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPushList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitPush>,
}
impl GitPushList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPushRef {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date: Option<time::OffsetDateTime>,
    #[serde(rename = "pushedBy", default, skip_serializing_if = "Option::is_none")]
    pub pushed_by: Option<IdentityRef>,
    #[serde(rename = "pushId", default, skip_serializing_if = "Option::is_none")]
    pub push_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitPushRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitPushSearchCriteria {
    #[serde(
        rename = "fromDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub from_date: Option<time::OffsetDateTime>,
    #[doc = "Whether to include the _links field on the shallow references"]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[serde(
        rename = "includeRefUpdates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_ref_updates: Option<bool>,
    #[serde(rename = "pusherId", default, skip_serializing_if = "Option::is_none")]
    pub pusher_id: Option<String>,
    #[serde(rename = "refName", default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[serde(
        rename = "toDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub to_date: Option<time::OffsetDateTime>,
}
impl GitPushSearchCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitQueryBranchStatsCriteria {
    #[serde(
        rename = "baseCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_commit: Option<GitVersionDescriptor>,
    #[serde(
        rename = "targetCommits",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_commits: Vec<GitVersionDescriptor>,
}
impl GitQueryBranchStatsCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitQueryCommitsCriteria {
    #[doc = "Number of entries to skip"]
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[doc = "Maximum number of entries to retrieve"]
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "Alias or display name of the author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(
        rename = "compareVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compare_version: Option<GitVersionDescriptor>,
    #[doc = "Only applies when an itemPath is specified. This determines whether to exclude delete entries of the specified path."]
    #[serde(
        rename = "excludeDeletes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_deletes: Option<bool>,
    #[doc = "If provided, a lower bound for filtering commits alphabetically"]
    #[serde(
        rename = "fromCommitId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_commit_id: Option<String>,
    #[doc = "If provided, only include history entries created after this date (string)"]
    #[serde(rename = "fromDate", default, skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    #[doc = "What Git history mode should be used. This only applies to the search criteria when Ids = null and an itemPath is specified."]
    #[serde(
        rename = "historyMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub history_mode: Option<git_query_commits_criteria::HistoryMode>,
    #[doc = "If provided, specifies the exact commit ids of the commits to fetch. May not be combined with other parameters."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<String>,
    #[doc = "Whether to include the _links field on the shallow references"]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[doc = "Whether to include the push information"]
    #[serde(
        rename = "includePushData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_push_data: Option<bool>,
    #[doc = "Whether to include the image Url for committers and authors"]
    #[serde(
        rename = "includeUserImageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_user_image_url: Option<bool>,
    #[doc = "Whether to include linked work items"]
    #[serde(
        rename = "includeWorkItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_work_items: Option<bool>,
    #[doc = "Path of item to search under"]
    #[serde(rename = "itemPath", default, skip_serializing_if = "Option::is_none")]
    pub item_path: Option<String>,
    #[serde(
        rename = "itemVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_version: Option<GitVersionDescriptor>,
    #[doc = "If enabled, this option will ignore the itemVersion and compareVersion parameters"]
    #[serde(
        rename = "showOldestCommitsFirst",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_oldest_commits_first: Option<bool>,
    #[doc = "If provided, an upper bound for filtering commits alphabetically"]
    #[serde(
        rename = "toCommitId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub to_commit_id: Option<String>,
    #[doc = "If provided, only include history entries created before this date (string)"]
    #[serde(rename = "toDate", default, skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    #[doc = "Alias or display name of the committer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl GitQueryCommitsCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_query_commits_criteria {
    use super::*;
    #[doc = "What Git history mode should be used. This only applies to the search criteria when Ids = null and an itemPath is specified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HistoryMode {
        #[serde(rename = "simplifiedHistory")]
        SimplifiedHistory,
        #[serde(rename = "firstParent")]
        FirstParent,
        #[serde(rename = "fullHistory")]
        FullHistory,
        #[serde(rename = "fullHistorySimplifyMerges")]
        FullHistorySimplifyMerges,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitQueryRefsCriteria {
    #[doc = "List of commit Ids to be searched"]
    #[serde(
        rename = "commitIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commit_ids: Vec<String>,
    #[doc = "List of complete or partial names for refs to be searched"]
    #[serde(
        rename = "refNames",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ref_names: Vec<String>,
    #[doc = "Type of search on refNames, if provided"]
    #[serde(
        rename = "searchType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_type: Option<git_query_refs_criteria::SearchType>,
}
impl GitQueryRefsCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_query_refs_criteria {
    use super::*;
    #[doc = "Type of search on refNames, if provided"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SearchType {
        #[serde(rename = "exact")]
        Exact,
        #[serde(rename = "startsWith")]
        StartsWith,
        #[serde(rename = "contains")]
        Contains,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRecycleBinRepositoryDetails {
    #[doc = "Setting to false will undo earlier deletion and restore the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
impl GitRecycleBinRepositoryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRef {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IdentityRef>,
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(
        rename = "isLockedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_locked_by: Option<IdentityRef>,
    pub name: String,
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(
        rename = "peeledObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peeled_object_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub statuses: Vec<GitStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitRef {
    pub fn new(name: String, object_id: String) -> Self {
        Self {
            links: None,
            creator: None,
            is_locked: None,
            is_locked_by: None,
            name,
            object_id,
            peeled_object_id: None,
            statuses: Vec::new(),
            url: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRefFavorite {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(
        rename = "identityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<git_ref_favorite::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitRefFavorite {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_ref_favorite {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "invalid")]
        Invalid,
        #[serde(rename = "folder")]
        Folder,
        #[serde(rename = "ref")]
        Ref,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRefFavoriteList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitRefFavorite>,
}
impl GitRefFavoriteList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitRef>,
}
impl GitRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRefUpdate {
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "newObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_object_id: Option<String>,
    #[serde(
        rename = "oldObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub old_object_id: Option<String>,
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl GitRefUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRefUpdateResult {
    #[doc = "Custom message for the result object For instance, Reason for failing."]
    #[serde(
        rename = "customMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_message: Option<String>,
    #[doc = "Whether the ref is locked or not"]
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[doc = "Ref name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "New object ID"]
    #[serde(
        rename = "newObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_object_id: Option<String>,
    #[doc = "Old object ID"]
    #[serde(
        rename = "oldObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub old_object_id: Option<String>,
    #[doc = "Name of the plugin that rejected the updated."]
    #[serde(
        rename = "rejectedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rejected_by: Option<String>,
    #[doc = "Repository ID"]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
    #[doc = "True if the ref update succeeded, false otherwise"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[doc = "Status of the update from the TFS server."]
    #[serde(
        rename = "updateStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_status: Option<git_ref_update_result::UpdateStatus>,
}
impl GitRefUpdateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_ref_update_result {
    use super::*;
    #[doc = "Status of the update from the TFS server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateStatus {
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "forcePushRequired")]
        ForcePushRequired,
        #[serde(rename = "staleOldObjectId")]
        StaleOldObjectId,
        #[serde(rename = "invalidRefName")]
        InvalidRefName,
        #[serde(rename = "unprocessed")]
        Unprocessed,
        #[serde(rename = "unresolvableToCommit")]
        UnresolvableToCommit,
        #[serde(rename = "writePermissionRequired")]
        WritePermissionRequired,
        #[serde(rename = "manageNotePermissionRequired")]
        ManageNotePermissionRequired,
        #[serde(rename = "createBranchPermissionRequired")]
        CreateBranchPermissionRequired,
        #[serde(rename = "createTagPermissionRequired")]
        CreateTagPermissionRequired,
        #[serde(rename = "rejectedByPlugin")]
        RejectedByPlugin,
        #[serde(rename = "locked")]
        Locked,
        #[serde(rename = "refNameConflict")]
        RefNameConflict,
        #[serde(rename = "rejectedByPolicy")]
        RejectedByPolicy,
        #[serde(rename = "succeededNonExistentRef")]
        SucceededNonExistentRef,
        #[serde(rename = "succeededCorruptRef")]
        SucceededCorruptRef,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRefUpdateResultList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitRefUpdateResult>,
}
impl GitRefUpdateResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub id: String,
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
    pub name: String,
    #[serde(
        rename = "parentRepository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_repository: Option<GitRepositoryRef>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    pub project: TeamProjectReference,
    #[serde(rename = "remoteUrl", default, skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[doc = "Compressed size (bytes) of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "sshUrl", default, skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    pub url: String,
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
    pub fn new(id: String, name: String, project: TeamProjectReference, url: String) -> Self {
        Self {
            links: None,
            default_branch: None,
            id,
            is_disabled: None,
            is_fork: None,
            is_in_maintenance: None,
            name,
            parent_repository: None,
            project,
            remote_url: None,
            size: None,
            ssh_url: None,
            url,
            valid_remote_urls: Vec::new(),
            web_url: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRepositoryCreateOptions {
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
}
impl GitRepositoryCreateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRepositoryList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitRepository>,
}
impl GitRepositoryList {
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
pub struct GitRepositoryRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitRepositoryRef>,
}
impl GitRepositoryRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRepositoryStats {
    #[serde(
        rename = "activePullRequestsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_pull_requests_count: Option<i32>,
    #[serde(
        rename = "branchesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub branches_count: Option<i32>,
    #[serde(
        rename = "commitsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commits_count: Option<i32>,
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl GitRepositoryStats {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitResolution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentityRef>,
}
impl GitResolution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitResolutionMergeContent {
    #[serde(flatten)]
    pub git_resolution: GitResolution,
    #[serde(rename = "mergeType", default, skip_serializing_if = "Option::is_none")]
    pub merge_type: Option<git_resolution_merge_content::MergeType>,
    #[serde(
        rename = "userMergedBlob",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_merged_blob: Option<GitBlobRef>,
    #[serde(
        rename = "userMergedContent",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_merged_content: Vec<String>,
}
impl GitResolutionMergeContent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_resolution_merge_content {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MergeType {
        #[serde(rename = "undecided")]
        Undecided,
        #[serde(rename = "takeSourceContent")]
        TakeSourceContent,
        #[serde(rename = "takeTargetContent")]
        TakeTargetContent,
        #[serde(rename = "autoMerged")]
        AutoMerged,
        #[serde(rename = "userMerged")]
        UserMerged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitResolutionPathConflict {
    #[serde(flatten)]
    pub git_resolution: GitResolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<git_resolution_path_conflict::Action>,
    #[serde(
        rename = "renamePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rename_path: Option<String>,
}
impl GitResolutionPathConflict {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_resolution_path_conflict {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        #[serde(rename = "undecided")]
        Undecided,
        #[serde(rename = "keepSourceRenameTarget")]
        KeepSourceRenameTarget,
        #[serde(rename = "keepSourceDeleteTarget")]
        KeepSourceDeleteTarget,
        #[serde(rename = "keepTargetRenameSource")]
        KeepTargetRenameSource,
        #[serde(rename = "keepTargetDeleteSource")]
        KeepTargetDeleteSource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitResolutionPickOneAction {
    #[serde(flatten)]
    pub git_resolution: GitResolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<git_resolution_pick_one_action::Action>,
}
impl GitResolutionPickOneAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_resolution_pick_one_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        #[serde(rename = "undecided")]
        Undecided,
        #[serde(rename = "pickSourceAction")]
        PickSourceAction,
        #[serde(rename = "pickTargetAction")]
        PickTargetAction,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitResolutionRename1to2 {
    #[serde(flatten)]
    pub git_resolution_merge_content: GitResolutionMergeContent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<git_resolution_rename1to2::Action>,
}
impl GitResolutionRename1to2 {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_resolution_rename1to2 {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        #[serde(rename = "undecided")]
        Undecided,
        #[serde(rename = "keepSourcePath")]
        KeepSourcePath,
        #[serde(rename = "keepTargetPath")]
        KeepTargetPath,
        #[serde(rename = "keepBothFiles")]
        KeepBothFiles,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitRevert {
    #[serde(flatten)]
    pub git_async_ref_operation: GitAsyncRefOperation,
    #[serde(rename = "revertId", default, skip_serializing_if = "Option::is_none")]
    pub revert_id: Option<i32>,
}
impl GitRevert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class contains the metadata of a service/extension posting a status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitStatus {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Status context that uniquely identifies the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<GitStatusContext>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "Creation date and time of the status."]
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Status description. Typically describes current state of the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Status identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "State of the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<git_status::State>,
    #[doc = "URL with status details."]
    #[serde(rename = "targetUrl", default, skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[doc = "Last update date and time of the status."]
    #[serde(
        rename = "updatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub updated_date: Option<time::OffsetDateTime>,
}
impl GitStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_status {
    use super::*;
    #[doc = "State of the status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "notSet")]
        NotSet,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "notApplicable")]
        NotApplicable,
    }
}
#[doc = "Status context that uniquely identifies the status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitStatusContext {
    #[doc = "Genre of the status. Typically name of the service/tool generating the status, can be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[doc = "Name identifier of the status, cannot be null or empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl GitStatusContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitStatusList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitStatus>,
}
impl GitStatusList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object describing the git suggestion.  Git suggestions are currently limited to suggested pull requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitSuggestion {
    #[doc = "Specific properties describing the suggestion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The type of suggestion (e.g. pull request)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl GitSuggestion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitSuggestionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitSuggestion>,
}
impl GitSuggestionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitTargetVersionDescriptor {
    #[serde(flatten)]
    pub git_version_descriptor: GitVersionDescriptor,
    #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
    #[serde(
        rename = "targetVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_version: Option<String>,
    #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
    #[serde(
        rename = "targetVersionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_version_options: Option<git_target_version_descriptor::TargetVersionOptions>,
    #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
    #[serde(
        rename = "targetVersionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_version_type: Option<git_target_version_descriptor::TargetVersionType>,
}
impl GitTargetVersionDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_target_version_descriptor {
    use super::*;
    #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetVersionOptions {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "previousChange")]
        PreviousChange,
        #[serde(rename = "firstParent")]
        FirstParent,
    }
    #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetVersionType {
        #[serde(rename = "branch")]
        Branch,
        #[serde(rename = "tag")]
        Tag,
        #[serde(rename = "commit")]
        Commit,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitTemplate {
    #[doc = "Name of the Template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the Template"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl GitTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitTreeDiff {
    #[doc = "ObjectId of the base tree of this diff."]
    #[serde(
        rename = "baseTreeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_tree_id: Option<String>,
    #[doc = "List of tree entries that differ between the base and target tree.  Renames and object type changes are returned as a delete for the old object and add for the new object.  If a continuation token is returned in the response header, some tree entries are yet to be processed and may yield more diff entries. If the continuation token is not returned all the diff entries have been included in this response."]
    #[serde(
        rename = "diffEntries",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub diff_entries: Vec<GitTreeDiffEntry>,
    #[doc = "ObjectId of the target tree of this diff."]
    #[serde(
        rename = "targetTreeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_tree_id: Option<String>,
    #[doc = "REST Url to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitTreeDiff {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitTreeDiffEntry {
    #[doc = "SHA1 hash of the object in the base tree, if it exists. Will be null in case of adds."]
    #[serde(
        rename = "baseObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_object_id: Option<String>,
    #[doc = "Type of change that affected this entry."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<git_tree_diff_entry::ChangeType>,
    #[doc = "Object type of the tree entry. Blob, Tree or Commit(\"submodule\")"]
    #[serde(
        rename = "objectType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub object_type: Option<git_tree_diff_entry::ObjectType>,
    #[doc = "Relative path in base and target trees."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "SHA1 hash of the object in the target tree, if it exists. Will be null in case of deletes."]
    #[serde(
        rename = "targetObjectId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_object_id: Option<String>,
}
impl GitTreeDiffEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_tree_diff_entry {
    use super::*;
    #[doc = "Type of change that affected this entry."]
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
    #[doc = "Object type of the tree entry. Blob, Tree or Commit(\"submodule\")"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ObjectType {
        #[serde(rename = "bad")]
        Bad,
        #[serde(rename = "commit")]
        Commit,
        #[serde(rename = "tree")]
        Tree,
        #[serde(rename = "blob")]
        Blob,
        #[serde(rename = "tag")]
        Tag,
        #[serde(rename = "ext2")]
        Ext2,
        #[serde(rename = "ofsDelta")]
        OfsDelta,
        #[serde(rename = "refDelta")]
        RefDelta,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitTreeDiffResponse {
    #[doc = "The HTTP client methods find the continuation token header in the response and populate this field."]
    #[serde(
        rename = "continuationToken",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub continuation_token: Vec<String>,
    #[serde(rename = "treeDiff", default, skip_serializing_if = "Option::is_none")]
    pub tree_diff: Option<GitTreeDiff>,
}
impl GitTreeDiffResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitTreeEntryRef {
    #[doc = "Blob or tree"]
    #[serde(
        rename = "gitObjectType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub git_object_type: Option<git_tree_entry_ref::GitObjectType>,
    #[doc = "Mode represented as octal string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[doc = "SHA1 hash of git object"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Path relative to parent tree object"]
    #[serde(
        rename = "relativePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_path: Option<String>,
    #[doc = "Size of content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "url to retrieve tree or blob"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitTreeEntryRef {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_tree_entry_ref {
    use super::*;
    #[doc = "Blob or tree"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GitObjectType {
        #[serde(rename = "bad")]
        Bad,
        #[serde(rename = "commit")]
        Commit,
        #[serde(rename = "tree")]
        Tree,
        #[serde(rename = "blob")]
        Blob,
        #[serde(rename = "tag")]
        Tag,
        #[serde(rename = "ext2")]
        Ext2,
        #[serde(rename = "ofsDelta")]
        OfsDelta,
        #[serde(rename = "refDelta")]
        RefDelta,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitTreeRef {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "SHA1 hash of git object"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Sum of sizes of all children"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Blobs and trees under this tree"]
    #[serde(
        rename = "treeEntries",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tree_entries: Vec<GitTreeEntryRef>,
    #[doc = "Url to tree"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GitTreeRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User info and date for Git operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitUserDate {
    #[doc = "Date of the Git operation."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "Email address of the user performing the Git operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Url for the user's avatar."]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[doc = "Name of the user performing the Git operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl GitUserDate {
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
#[doc = "Globally unique key for a repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalGitRepositoryKey {
    #[doc = "Team Project Collection ID of the collection for the repository."]
    #[serde(
        rename = "collectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_id: Option<String>,
    #[doc = "Team Project ID of the project for the repository."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "ID of the repository."]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl GlobalGitRepositoryKey {
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
pub struct HistoryEntry {
    #[serde(
        rename = "changeList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_list: Option<ChangeList>,
    #[doc = "The change made to the item from this change list (only relevant for File history, not folders)"]
    #[serde(
        rename = "itemChangeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_change_type: Option<history_entry::ItemChangeType>,
    #[doc = "The path of the item at this point in history (only relevant for File history, not folders)"]
    #[serde(
        rename = "serverItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_item: Option<String>,
}
impl HistoryEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod history_entry {
    use super::*;
    #[doc = "The change made to the item from this change list (only relevant for File history, not folders)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ItemChangeType {
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
#[doc = "Identity id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityId {
    #[doc = "The user identity"]
    pub id: String,
}
impl IdentityId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub id: String,
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
    pub fn new(id: String) -> Self {
        Self {
            graph_subject_base: GraphSubjectBase::default(),
            directory_alias: None,
            id,
            image_url: None,
            inactive: None,
            is_aad_identity: None,
            is_container: None,
            is_deleted_in_origin: None,
            profile_url: None,
            unique_name: None,
        }
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
#[doc = "Identity information including a vote on a pull request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityRefWithVote {
    #[serde(flatten)]
    pub identity_ref: IdentityRef,
    #[doc = "Indicates if this reviewer has declined to review this pull request."]
    #[serde(
        rename = "hasDeclined",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_declined: Option<bool>,
    #[doc = "Indicates if this reviewer is flagged for attention on this pull request."]
    #[serde(rename = "isFlagged", default, skip_serializing_if = "Option::is_none")]
    pub is_flagged: Option<bool>,
    #[doc = "Indicates if this approve vote should still be handled even though vote didn't change."]
    #[serde(
        rename = "isReapprove",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_reapprove: Option<bool>,
    #[doc = "Indicates if this is a required reviewer for this pull request. <br /> Branches can have policies that require particular reviewers are required for pull requests."]
    #[serde(
        rename = "isRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
    #[doc = "URL to retrieve information about this identity"]
    #[serde(
        rename = "reviewerUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reviewer_url: Option<String>,
    #[doc = "Vote on a pull request:<br /> 10 - approved 5 - approved with suggestions 0 - no vote -5 - waiting for author -10 - rejected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vote: Option<i64>,
    #[doc = "Groups or teams that this reviewer contributed to. <br /> Groups and teams can be reviewers on pull requests but can not vote directly.  When a member of the group or team votes, that vote is rolled up into the group or team vote.  VotedFor is a list of such votes."]
    #[serde(
        rename = "votedFor",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub voted_for: Vec<IdentityRefWithVote>,
}
impl IdentityRefWithVote {
    pub fn new(identity_ref: IdentityRef) -> Self {
        Self {
            identity_ref,
            has_declined: None,
            is_flagged: None,
            is_reapprove: None,
            is_required: None,
            reviewer_url: None,
            vote: None,
            voted_for: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityRefWithVoteList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IdentityRefWithVote>,
}
impl IdentityRefWithVoteList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportRepositoryValidation {
    #[doc = "Parameter for creating a git import request when source is Git version control"]
    #[serde(rename = "gitSource", default, skip_serializing_if = "Option::is_none")]
    pub git_source: Option<GitImportGitSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Parameter for creating a git import request when source is tfvc version control"]
    #[serde(
        rename = "tfvcSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tfvc_source: Option<GitImportTfvcSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl ImportRepositoryValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncludedGitCommit {
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(
        rename = "commitTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub commit_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "parentCommitIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parent_commit_ids: Vec<String>,
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl IncludedGitCommit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for IsDraft update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IsDraftUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl IsDraftUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemContent {
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: item_content::ContentType,
}
impl ItemContent {
    pub fn new(content: String, content_type: item_content::ContentType) -> Self {
        Self {
            content,
            content_type,
        }
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
#[doc = "Optional details to include when returning an item model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ItemDetailsOptions {
    #[doc = "If true, include metadata about the file type"]
    #[serde(
        rename = "includeContentMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_content_metadata: Option<bool>,
    #[doc = "Specifies whether to include children (OneLevel), all descendants (Full) or None for folder items"]
    #[serde(
        rename = "recursionLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursion_level: Option<item_details_options::RecursionLevel>,
}
impl ItemDetailsOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod item_details_options {
    use super::*;
    #[doc = "Specifies whether to include children (OneLevel), all descendants (Full) or None for folder items"]
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
#[doc = "Real time event (SignalR) for updated labels on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelsUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl LabelsUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class to represent the line diff block"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LineDiffBlock {
    #[doc = "Type of change that was made to the block."]
    #[serde(
        rename = "changeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<line_diff_block::ChangeType>,
    #[doc = "Line number where this block starts in modified file."]
    #[serde(
        rename = "modifiedLineNumberStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_line_number_start: Option<i32>,
    #[doc = "Count of lines in this block in modified file."]
    #[serde(
        rename = "modifiedLinesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_lines_count: Option<i32>,
    #[doc = "Line number where this block starts in original file."]
    #[serde(
        rename = "originalLineNumberStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_line_number_start: Option<i32>,
    #[doc = "Count of lines in this block in original file."]
    #[serde(
        rename = "originalLinesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_lines_count: Option<i32>,
}
impl LineDiffBlock {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod line_diff_block {
    use super::*;
    #[doc = "Type of change that was made to the block."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "edit")]
        Edit,
    }
}
#[doc = "Link URL"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}
impl Link {
    pub fn new(href: String) -> Self {
        Self { href }
    }
}
#[doc = "Real time event (SignalR) for a merge completed on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeCompletedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl MergeCompletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The full policy configuration with settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyConfiguration {
    #[serde(flatten)]
    pub versioned_policy_configuration_ref: VersionedPolicyConfigurationRef,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentityRef>,
    #[doc = "The date and time when the policy was created."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether the policy is blocking."]
    #[serde(
        rename = "isBlocking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_blocking: Option<bool>,
    #[doc = "Indicates whether the policy has been (soft) deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Indicates whether the policy is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "If set, this policy requires \"Manage Enterprise Policies\" permission to create, edit, or delete."]
    #[serde(
        rename = "isEnterpriseManaged",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_enterprise_managed: Option<bool>,
    #[doc = "The policy configuration settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
impl PolicyConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyConfigurationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyConfiguration>,
}
impl PolicyConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy configuration reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyConfigurationRef {
    #[doc = "The policy configuration ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Policy type reference."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<PolicyTypeRef>,
    #[doc = "The URL where the policy configuration can be retrieved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PolicyConfigurationRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for a policy evaluation update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvaluationUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl PolicyEvaluationUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy type reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyTypeRef {
    #[doc = "Display name of the policy type."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "The policy type ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The URL where the policy type can be retrieved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PolicyTypeRef {
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
#[doc = "Real time event (SignalR) for pull request creation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequestCreatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl PullRequestCreatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pull request status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PullRequestStatus {
    #[serde(rename = "notSet")]
    NotSet,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "abandoned")]
    Abandoned,
    #[serde(rename = "completed")]
    Completed,
}
#[doc = "Initial config contract sent to extensions creating tabs on the pull request page"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PullRequestTabExtensionConfig {
    #[serde(
        rename = "pullRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_id: Option<i32>,
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
}
impl PullRequestTabExtensionConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base contract for a real time pull request event (SignalR)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RealTimePullRequestEvent {
    #[doc = "The id of this event. Can be used to track send/receive state between client and server."]
    #[serde(rename = "eventId", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[doc = "The id of the pull request this event was generated for."]
    #[serde(
        rename = "pullRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_id: Option<i32>,
}
impl RealTimePullRequestEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class to represent a collection of REST reference links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLinks {
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<Link>,
    #[doc = "Link URL"]
    #[serde(
        rename = "pullRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_requests: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushes: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refs: Option<Link>,
    #[doc = "Link URL"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh: Option<Link>,
    #[doc = "Link URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<Link>,
}
impl ReferenceLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ResourceRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRefList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceRef>,
}
impl ResourceRefList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for when the target branch of a pull request is changed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetargetEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl RetargetEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for a reviewer vote update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReviewerVoteUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl ReviewerVoteUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for an update to reviewers on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReviewersUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl ReviewersUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for reviewer votes being reset on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReviewersVotesResetEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl ReviewersVotesResetEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Context used while sharing a pull request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareNotificationContext {
    #[doc = "Optional user note or message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Identities of users who will receive a share notification."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub receivers: Vec<IdentityRef>,
}
impl ShareNotificationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceToTargetRef {
    #[doc = "The source ref to copy. For example, refs/heads/master."]
    #[serde(rename = "sourceRef", default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[doc = "The target ref to update. For example, refs/heads/master."]
    #[serde(rename = "targetRef", default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
}
impl SourceToTargetRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for an added status on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusAddedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl StatusAddedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for a status update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl StatusUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Real time event (SignalR) for deleted statuses on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusesDeletedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl StatusesDeletedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Supported IDE entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedIde {
    #[doc = "The download URL for the IDE."]
    #[serde(
        rename = "downloadUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_url: Option<String>,
    #[doc = "The type of the IDE."]
    #[serde(rename = "ideType", default, skip_serializing_if = "Option::is_none")]
    pub ide_type: Option<supported_ide::IdeType>,
    #[doc = "The name of the IDE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URL to open the protocol handler for the IDE."]
    #[serde(
        rename = "protocolHandlerUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_handler_url: Option<String>,
    #[doc = "A list of SupportedPlatforms."]
    #[serde(
        rename = "supportedPlatforms",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_platforms: Vec<String>,
}
impl SupportedIde {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod supported_ide {
    use super::*;
    #[doc = "The type of the IDE."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IdeType {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "androidStudio")]
        AndroidStudio,
        #[serde(rename = "appCode")]
        AppCode,
        #[serde(rename = "cLion")]
        CLion,
        #[serde(rename = "dataGrip")]
        DataGrip,
        #[serde(rename = "eclipse")]
        Eclipse,
        #[serde(rename = "intelliJ")]
        IntelliJ,
        #[serde(rename = "mps")]
        Mps,
        #[serde(rename = "phpStorm")]
        PhpStorm,
        #[serde(rename = "pyCharm")]
        PyCharm,
        #[serde(rename = "rubyMine")]
        RubyMine,
        #[serde(rename = "tower")]
        Tower,
        #[serde(rename = "visualStudio")]
        VisualStudio,
        #[serde(rename = "vsCode")]
        VsCode,
        #[serde(rename = "webStorm")]
        WebStorm,
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
#[doc = "A change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub fn new(change: Change) -> Self {
        Self {
            change,
            merge_sources: Vec::new(),
            pending_version: None,
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcCheckinEventData {
    #[doc = "A collection of changes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changeset: Option<TfvcChangeset>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
}
impl TfvcCheckinEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcHistoryEntry {
    #[serde(flatten)]
    pub history_entry: HistoryEntry,
    #[doc = "The encoding of the item at this point in history (only relevant for File history, not folders)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<i32>,
    #[doc = "The file id of the item at this point in history (only relevant for File history, not folders)"]
    #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i32>,
}
impl TfvcHistoryEntry {
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
#[doc = "Metadata for an item including the previous hash value for files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcItemPreviousHash {
    #[serde(flatten)]
    pub tfvc_item: TfvcItem,
    #[doc = "MD5 hash as a base 64 string, applies to files only."]
    #[serde(
        rename = "previousHashValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_hash_value: Option<String>,
}
impl TfvcItemPreviousHash {
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
#[doc = "Real time event (SignalR) for a title/description update on a pull request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TitleDescriptionUpdatedEvent {
    #[serde(flatten)]
    pub real_time_pull_request_event: RealTimePullRequestEvent,
}
impl TitleDescriptionUpdatedEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateRefsRequest {
    #[serde(
        rename = "refUpdateRequests",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ref_update_requests: Vec<GitRefUpdate>,
    #[serde(
        rename = "updateMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_mode: Option<update_refs_request::UpdateMode>,
}
impl UpdateRefsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_refs_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateMode {
        #[serde(rename = "bestEffort")]
        BestEffort,
        #[serde(rename = "allOrNone")]
        AllOrNone,
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
#[doc = "A particular revision for a policy configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionedPolicyConfigurationRef {
    #[serde(flatten)]
    pub policy_configuration_ref: PolicyConfigurationRef,
    #[doc = "The policy configuration revision ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl VersionedPolicyConfigurationRef {
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
#[doc = "The representation of data needed to create a tag definition which is sent across the wire."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebApiCreateTagRequestData {
    #[doc = "Name of the tag definition that will be created."]
    pub name: String,
}
impl WebApiCreateTagRequestData {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "The representation of a tag definition which is sent across the wire."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiTagDefinition {
    #[doc = "Whether or not the tag definition is active."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "ID of the tag definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the tag definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource URL for the Tag Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WebApiTagDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApiTagDefinitionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WebApiTagDefinition>,
}
impl WebApiTagDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
