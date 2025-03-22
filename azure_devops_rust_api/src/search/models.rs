// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Defines the Board result that matched a Board search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardResult {
    #[doc = "Board Type of the board document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boardtype: Option<String>,
    #[doc = "Defines the details of the collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
    #[doc = "Defines the details of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
    #[doc = "Defines the details of the team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
}
impl BoardResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a Board search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardSearchRequest {
    #[serde(flatten)]
    pub entity_search_request: EntitySearchRequest,
}
impl BoardSearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a Board search response item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardSearchResponse {
    #[serde(flatten)]
    pub entity_search_response: EntitySearchResponse,
    #[doc = "Total number of matched Board documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "List of top matched Board documents."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<BoardResult>,
}
impl BoardSearchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the configured branch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BranchInfo {
    #[doc = "Name of the indexed branch"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl BranchInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the code result containing information of the searched files and its metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeResult {
    #[doc = "Defines the details of the collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
    #[doc = "ContentId of the result file."]
    #[serde(rename = "contentId", default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[doc = "Name of the result file."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "Dictionary of field to hit offsets in the result file. Key identifies the area in which hits were found, for ex: file content/file name etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<serde_json::Value>,
    #[doc = "Path at which result file is present."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Defines the details of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
    #[doc = "Defines the details of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[doc = "Versions of the result file."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub versions: Vec<Version>,
}
impl CodeResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a code search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeSearchRequest {
    #[serde(flatten)]
    pub entity_search_request: EntitySearchRequest,
    #[doc = "Flag to opt for including matched code snippet in the result. Default behavior is false."]
    #[serde(
        rename = "includeSnippet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_snippet: Option<bool>,
}
impl CodeSearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a code search response item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeSearchResponse {
    #[serde(flatten)]
    pub entity_search_response: EntitySearchResponse,
    #[doc = "Total number of matched files."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "List of matched files."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<CodeResult>,
}
impl CodeSearchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the details of the collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Collection {
    #[doc = "Name of the collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Collection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRepositoryBranchStatusResponse {
    #[serde(
        rename = "lastIndexedChangeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_indexed_change_id: Option<i64>,
    #[serde(
        rename = "lastIndexedChangeIdChangeTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_indexed_change_id_change_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "latestChangeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_change_id: Option<i64>,
    #[serde(
        rename = "latestChangeIdChangeTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub latest_change_id_change_time: Option<time::OffsetDateTime>,
}
impl CustomRepositoryBranchStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the custom repository status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRepositoryStatusResponse {
    #[doc = "Repository Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of indexed top level folders info."]
    #[serde(
        rename = "indexedTopLevelFolders",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub indexed_top_level_folders: Vec<DepotInfo>,
    #[doc = "Repository Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CustomRepositoryStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the custom repository indexing freshness for configured branches and depots."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DepotInfo {
    #[doc = "Name of the indexed top level folder (depot)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DepotInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base contract for search request types without scroll support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntitySearchRequest {
    #[serde(flatten)]
    pub entity_search_request_base: EntitySearchRequestBase,
    #[doc = "Options for sorting search results. If set to null, the results will be returned sorted by relevance. If more than one sort option is provided, the results are sorted in the order specified in the OrderBy."]
    #[serde(
        rename = "$orderBy",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub order_by: Vec<SortOption>,
    #[doc = "Number of results to be skipped."]
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[doc = "Number of results to be returned."]
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "Flag to opt for faceting in the result. Default behavior is false."]
    #[serde(
        rename = "includeFacets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_facets: Option<bool>,
}
impl EntitySearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for search request types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntitySearchRequestBase {
    #[doc = "Filters to be applied. Set it to null if there are no filters to be applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<serde_json::Value>,
    #[doc = "The search text."]
    #[serde(
        rename = "searchText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_text: Option<String>,
}
impl EntitySearchRequestBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the base contract for search response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntitySearchResponse {
    #[doc = "A dictionary storing an array of <code>Filter</code> object against each facet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facets: Option<serde_json::Value>,
    #[doc = "Numeric code indicating any additional information: 0 - Ok, 1 - Account is being reindexed, 2 - Account indexing has not started, 3 - Invalid Request, 4 - Prefix wildcard query not supported, 5 - MultiWords with code facet not supported, 6 - Account is being onboarded, 7 - Account is being onboarded or reindexed, 8 - Top value trimmed to maxresult allowed 9 - Branches are being indexed, 10 - Faceting not enabled, 11 - Work items not accessible, 19 - Phrase queries with code type filters not supported, 20 - Wildcard queries with code type filters not supported. Any other info code is used for internal purpose."]
    #[serde(rename = "infoCode", default, skip_serializing_if = "Option::is_none")]
    pub info_code: Option<i32>,
}
impl EntitySearchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the details of a feed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeedInfo {
    #[doc = "Id of the collection."]
    #[serde(
        rename = "collectionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_id: Option<String>,
    #[doc = "Name of the collection."]
    #[serde(
        rename = "collectionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_name: Option<String>,
    #[doc = "Id of the feed."]
    #[serde(rename = "feedId", default, skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<String>,
    #[doc = "Name of the feed."]
    #[serde(rename = "feedName", default, skip_serializing_if = "Option::is_none")]
    pub feed_name: Option<String>,
    #[doc = "Latest matched version of package in this Feed."]
    #[serde(
        rename = "latestMatchedVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_matched_version: Option<String>,
    #[doc = "Latest version of package in this Feed."]
    #[serde(
        rename = "latestVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_version: Option<String>,
    #[doc = "Url of package in this Feed."]
    #[serde(
        rename = "packageUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_url: Option<String>,
    #[doc = "List of views which contain the matched package."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub views: Vec<String>,
}
impl FeedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a filter bucket item representing the total matches of search result, name and id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filter {
    #[doc = "Id of the filter bucket."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the filter bucket."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Count of matches in the filter bucket."]
    #[serde(
        rename = "resultCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_count: Option<i32>,
}
impl Filter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the position of a piece of text in a document."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Hit {
    #[doc = "Gets or sets the start character offset of a piece of text."]
    #[serde(
        rename = "charOffset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub char_offset: Option<i32>,
    #[doc = "Gets or sets an extract of code where the match appears. Usually it is the line where there is the match."]
    #[serde(
        rename = "codeSnippet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code_snippet: Option<String>,
    #[doc = "Gets or sets the column number where the match appears in the line."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    #[doc = "Gets or sets the length of a piece of text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[doc = "Gets or sets the line number where the match appears in the file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    #[doc = "Gets or sets the name of type of a piece of text."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Hit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the matched terms in the field of the package result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageHit {
    #[doc = "Reference name of the highlighted field."]
    #[serde(
        rename = "fieldReferenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_reference_name: Option<String>,
    #[doc = "Matched/highlighted snippets of the field."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub highlights: Vec<String>,
}
impl PackageHit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the package result that matched a package search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageResult {
    #[doc = "Description of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of feeds which contain the matching package."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub feeds: Vec<FeedInfo>,
    #[doc = "List of highlighted fields for the match."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hits: Vec<PackageHit>,
    #[doc = "Id of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the package."]
    #[serde(
        rename = "protocolType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_type: Option<String>,
}
impl PackageResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a package search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageSearchRequest {
    #[serde(flatten)]
    pub entity_search_request: EntitySearchRequest,
}
impl PackageSearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageSearchResponse {
    #[serde(
        rename = "activityId",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub activity_id: Vec<String>,
    #[doc = "Defines a response item that is returned for a package search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<PackageSearchResponseContent>,
}
impl PackageSearchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a response item that is returned for a package search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageSearchResponseContent {
    #[serde(flatten)]
    pub entity_search_response: EntitySearchResponse,
    #[doc = "Total number of matched packages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "List of matched packages."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<PackageResult>,
}
impl PackageSearchResponseContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the details of the project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Project {
    #[doc = "Id of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Project {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the details of the project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectReference {
    #[doc = "ID of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Visibility of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}
impl ProjectReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the details of the repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Repository {
    #[doc = "Id of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version control type of the result file."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<repository::Type>,
}
impl Repository {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod repository {
    use super::*;
    #[doc = "Version control type of the result file."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "git")]
        Git,
        #[serde(rename = "tfvc")]
        Tfvc,
        #[serde(rename = "custom")]
        Custom,
    }
}
#[doc = "Defines the repository status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepositoryStatusResponse {
    #[doc = "Repository Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of Indexed branches info."]
    #[serde(
        rename = "indexedBranches",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub indexed_branches: Vec<BranchInfo>,
    #[doc = "Repository Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RepositoryStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a scroll code search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScrollSearchRequest {
    #[serde(flatten)]
    pub entity_search_request_base: EntitySearchRequestBase,
    #[doc = "Scroll Id for scroll search query."]
    #[serde(rename = "$scrollId", default, skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<String>,
    #[doc = "Size of data to return for scroll search query. Min value is 201."]
    #[serde(
        rename = "$scrollSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_size: Option<i32>,
}
impl ScrollSearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the setting result that matched a setting search request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingResult {
    #[doc = "Description of the settings page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Icon name of the settings page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Contribution url route id of the corresponding settings page"]
    #[serde(rename = "routeId", default, skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[doc = "Contribution url route parameter of the corresponding settings page"]
    #[serde(
        rename = "routeParameterMapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub route_parameter_mapping: Option<serde_json::Value>,
    #[doc = "Scope of the settings page, either organization, project or user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<setting_result::Scope>,
    #[doc = "Title of the settings page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl SettingResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod setting_result {
    use super::*;
    #[doc = "Scope of the settings page, either organization, project or user"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "organization")]
        Organization,
        #[serde(rename = "project")]
        Project,
        #[serde(rename = "user")]
        User,
    }
}
#[doc = "Defines a setting search request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingSearchRequest {
    #[serde(flatten)]
    pub entity_search_request: EntitySearchRequest,
}
impl SettingSearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a setting search response item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingSearchResponse {
    #[serde(flatten)]
    pub entity_search_response: EntitySearchResponse,
    #[doc = "Total number of matched setting documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "List of top matched setting documents."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<SettingResult>,
}
impl SettingSearchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines how to sort the result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SortOption {
    #[doc = "Field name on which sorting should be done."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[doc = "Order (ASC/DESC) in which the results should be sorted."]
    #[serde(rename = "sortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}
impl SortOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the details of the team."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Team {
    #[doc = "Id of the team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Team {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the TFVC repository status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TfvcRepositoryStatusResponse {
    #[doc = "Repository Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of Indexing Information for TFVC repository"]
    #[serde(
        rename = "indexingInformation",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub indexing_information: Vec<BranchInfo>,
    #[doc = "Repository Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TfvcRepositoryStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the details pertaining to a version of the result file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Version {
    #[doc = "Name of the branch."]
    #[serde(
        rename = "branchName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_name: Option<String>,
    #[doc = "ChangeId in the given branch associated with this match."]
    #[serde(rename = "changeId", default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,
}
impl Version {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the details of wiki."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Wiki {
    #[doc = "Id of the wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Mapped path for the wiki."]
    #[serde(
        rename = "mappedPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mapped_path: Option<String>,
    #[doc = "Name of the wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version for wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Wiki {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the matched terms in the field of the wiki result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiHit {
    #[doc = "Reference name of the highlighted field."]
    #[serde(
        rename = "fieldReferenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_reference_name: Option<String>,
    #[doc = "Matched/highlighted snippets of the field."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub highlights: Vec<String>,
}
impl WikiHit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the wiki result that matched a wiki search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiResult {
    #[doc = "Defines the details of the collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
    #[doc = "ContentId of the result file."]
    #[serde(rename = "contentId", default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[doc = "Name of the result file."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "Highlighted snippets of fields that match the search request. The list is sorted by relevance of the snippets."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hits: Vec<WikiHit>,
    #[doc = "Path at which result file is present."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Defines the details of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectReference>,
    #[doc = "Defines the details of wiki."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiki: Option<Wiki>,
}
impl WikiResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a wiki search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiSearchRequest {
    #[serde(flatten)]
    pub entity_search_request: EntitySearchRequest,
}
impl WikiSearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a wiki search response item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiSearchResponse {
    #[serde(flatten)]
    pub entity_search_response: EntitySearchResponse,
    #[doc = "Total number of matched wiki documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "List of top matched wiki documents."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<WikiResult>,
}
impl WikiSearchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the matched terms in the field of the work item result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemHit {
    #[doc = "Reference name of the highlighted field."]
    #[serde(
        rename = "fieldReferenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_reference_name: Option<String>,
    #[doc = "Matched/highlighted snippets of the field."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub highlights: Vec<String>,
}
impl WorkItemHit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the work item result that matched a work item search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemResult {
    #[doc = "A standard set of work item fields and their values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    #[doc = "Highlighted snippets of fields that match the search request. The list is sorted by relevance of the snippets."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hits: Vec<WorkItemHit>,
    #[doc = "Defines the details of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
    #[doc = "Reference to the work item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl WorkItemResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a work item search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemSearchRequest {
    #[serde(flatten)]
    pub entity_search_request: EntitySearchRequest,
}
impl WorkItemSearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a response item that is returned for a work item search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemSearchResponse {
    #[serde(flatten)]
    pub entity_search_response: EntitySearchResponse,
    #[doc = "Total number of matched work items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "List of top matched work items."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<WorkItemResult>,
}
impl WorkItemSearchResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
