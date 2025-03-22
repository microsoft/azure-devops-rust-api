// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The build definition reference resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildDefinitionReference {
    #[doc = "ID of the build definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the build definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl BuildDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common Response for clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneOperationCommonResponse {
    #[doc = "Clone Statistics Details."]
    #[serde(
        rename = "cloneStatistics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_statistics: Option<CloneStatistics>,
    #[doc = "Completion data of the operation"]
    #[serde(
        rename = "completionDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completion_date: Option<time::OffsetDateTime>,
    #[doc = "Creation data of the operation"]
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Message related to the job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Clone operation Id"]
    #[serde(rename = "opId", default, skip_serializing_if = "Option::is_none")]
    pub op_id: Option<i32>,
    #[doc = "Clone operation state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<clone_operation_common_response::State>,
}
impl CloneOperationCommonResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod clone_operation_common_response {
    use super::*;
    #[doc = "Clone operation state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "succeeded")]
        Succeeded,
    }
}
#[doc = "Clone options for cloning the test suite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneOptions {
    #[doc = "If set to true requirements will be cloned"]
    #[serde(
        rename = "cloneRequirements",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_requirements: Option<bool>,
    #[doc = "copy all suites from a source plan"]
    #[serde(
        rename = "copyAllSuites",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_all_suites: Option<bool>,
    #[doc = "copy ancestor hierarchy"]
    #[serde(
        rename = "copyAncestorHierarchy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_ancestor_hierarchy: Option<bool>,
    #[doc = "Name of the workitem type of the clone"]
    #[serde(
        rename = "destinationWorkItemType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_work_item_type: Option<String>,
    #[doc = "Key value pairs where the key value is overridden by the value."]
    #[serde(
        rename = "overrideParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub override_parameters: Option<serde_json::Value>,
    #[doc = "Comment on the link that will link the new clone  test case to the original Set null for no comment"]
    #[serde(
        rename = "relatedLinkComment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_link_comment: Option<String>,
}
impl CloneOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Clone Statistics Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneStatistics {
    #[doc = "Number of requirements cloned so far."]
    #[serde(
        rename = "clonedRequirementsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloned_requirements_count: Option<i32>,
    #[doc = "Number of shared steps cloned so far."]
    #[serde(
        rename = "clonedSharedStepsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloned_shared_steps_count: Option<i32>,
    #[doc = "Number of test cases cloned so far"]
    #[serde(
        rename = "clonedTestCasesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloned_test_cases_count: Option<i32>,
    #[doc = "Total number of requirements to be cloned"]
    #[serde(
        rename = "totalRequirementsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_requirements_count: Option<i32>,
    #[doc = "Total number of test cases to be cloned"]
    #[serde(
        rename = "totalTestCasesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_test_cases_count: Option<i32>,
}
impl CloneStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneTestCaseOperationInformation {
    #[doc = "Common Response for clone operation"]
    #[serde(
        rename = "cloneOperationResponse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_operation_response: Option<CloneOperationCommonResponse>,
    #[serde(
        rename = "cloneOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_options: Option<CloneTestCaseOptions>,
    #[doc = "Test Suite Reference with Project"]
    #[serde(
        rename = "destinationTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_test_suite: Option<TestSuiteReferenceWithProject>,
    #[doc = "Source Test Suite Response for Test Case clone operation"]
    #[serde(
        rename = "sourceTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_test_suite: Option<SourceTestSuiteResponse>,
}
impl CloneTestCaseOperationInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneTestCaseOptions {
    #[doc = "If set to true, include the attachments"]
    #[serde(
        rename = "includeAttachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_attachments: Option<bool>,
    #[doc = "If set to true, include the links"]
    #[serde(
        rename = "includeLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_links: Option<bool>,
    #[doc = "Comment on the link that will link the new clone  test case to the original Set null for no comment"]
    #[serde(
        rename = "relatedLinkComment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_link_comment: Option<String>,
}
impl CloneTestCaseOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for Test Suite clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneTestCaseParams {
    #[serde(
        rename = "cloneOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_options: Option<CloneTestCaseOptions>,
    #[doc = "The test plan reference resource."]
    #[serde(
        rename = "destinationTestPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_test_plan: Option<TestPlanReference>,
    #[doc = "Destination Test Suite information for Test Suite clone operation"]
    #[serde(
        rename = "destinationTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_test_suite: Option<DestinationTestSuiteInfo>,
    #[doc = "The test plan reference resource."]
    #[serde(
        rename = "sourceTestPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_test_plan: Option<TestPlanReference>,
    #[doc = "Source Test Suite information for Test Suite clone operation"]
    #[serde(
        rename = "sourceTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_test_suite: Option<SourceTestSuiteInfo>,
    #[doc = "Test Case IDs"]
    #[serde(
        rename = "testCaseIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_case_ids: Vec<i32>,
}
impl CloneTestCaseParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for Test Plan clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneTestPlanOperationInformation {
    #[doc = "Common Response for clone operation"]
    #[serde(
        rename = "cloneOperationResponse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_operation_response: Option<CloneOperationCommonResponse>,
    #[doc = "Clone options for cloning the test suite."]
    #[serde(
        rename = "cloneOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_options: Option<CloneOptions>,
    #[doc = "The test plan resource."]
    #[serde(
        rename = "destinationTestPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_test_plan: Option<TestPlan>,
    #[doc = "Source Test Plan Response for Test Plan clone operation"]
    #[serde(
        rename = "sourceTestPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_test_plan: Option<SourceTestplanResponse>,
}
impl CloneTestPlanOperationInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for Test Plan clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneTestPlanParams {
    #[doc = "Clone options for cloning the test suite."]
    #[serde(
        rename = "cloneOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_options: Option<CloneOptions>,
    #[doc = "Destination Test Plan create parameters"]
    #[serde(
        rename = "destinationTestPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_test_plan: Option<DestinationTestPlanCloneParams>,
    #[doc = "Source Test Plan information for Test Plan clone operation"]
    #[serde(
        rename = "sourceTestPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_test_plan: Option<SourceTestPlanInfo>,
}
impl CloneTestPlanParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for Test Suite clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneTestSuiteOperationInformation {
    #[doc = "Test Suite Reference with Project"]
    #[serde(
        rename = "clonedTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloned_test_suite: Option<TestSuiteReferenceWithProject>,
    #[doc = "Common Response for clone operation"]
    #[serde(
        rename = "cloneOperationResponse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_operation_response: Option<CloneOperationCommonResponse>,
    #[doc = "Clone options for cloning the test suite."]
    #[serde(
        rename = "cloneOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_options: Option<CloneOptions>,
    #[doc = "Test Suite Reference with Project"]
    #[serde(
        rename = "destinationTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_test_suite: Option<TestSuiteReferenceWithProject>,
    #[doc = "Test Suite Reference with Project"]
    #[serde(
        rename = "sourceTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_test_suite: Option<TestSuiteReferenceWithProject>,
}
impl CloneTestSuiteOperationInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for Test Suite clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloneTestSuiteParams {
    #[doc = "Clone options for cloning the test suite."]
    #[serde(
        rename = "cloneOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_options: Option<CloneOptions>,
    #[doc = "Destination Test Suite information for Test Suite clone operation"]
    #[serde(
        rename = "destinationTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_test_suite: Option<DestinationTestSuiteInfo>,
    #[doc = "Source Test Suite information for Test Suite clone operation"]
    #[serde(
        rename = "sourceTestSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_test_suite: Option<SourceTestSuiteInfo>,
}
impl CloneTestSuiteParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration of the Test Point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[doc = "Id of the Configuration Assigned to the Test Point"]
    #[serde(
        rename = "configurationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_id: Option<i32>,
}
impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Destination Test Plan create parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DestinationTestPlanCloneParams {
    #[serde(flatten)]
    pub test_plan_create_params: TestPlanCreateParams,
    #[doc = "Destination Project Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}
impl DestinationTestPlanCloneParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Destination Test Suite information for Test Suite clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DestinationTestSuiteInfo {
    #[doc = "Destination Suite Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Destination Project Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}
impl DestinationTestSuiteInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for test case export operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportTestCaseParams {
    #[doc = "Test Case IDs to exported"]
    #[serde(
        rename = "testCaseIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_case_ids: Vec<i32>,
    #[doc = "ID of test plan containing test cases"]
    #[serde(
        rename = "testPlanId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_plan_id: Option<i32>,
    #[doc = "ID of test suite containing test cases"]
    #[serde(
        rename = "testSuiteId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_suite_id: Option<i32>,
}
impl ExportTestCaseParams {
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
#[doc = "Last result details of test point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LastResultDetails {
    #[doc = "Completed date of last result."]
    #[serde(
        rename = "dateCompleted",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date_completed: Option<time::OffsetDateTime>,
    #[doc = "Duration of the last result in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "runBy", default, skip_serializing_if = "Option::is_none")]
    pub run_by: Option<IdentityRef>,
}
impl LastResultDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This data model is used in Work item-based tabs of Test Plans Library."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LibraryWorkItemsData {
    #[doc = "Specifies the column option field names"]
    #[serde(
        rename = "columnOptions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub column_options: Vec<String>,
    #[doc = "Continuation token to fetch next set of elements. Present only when HasMoreElements is true."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "Boolean indicating if the WIQL query has exceeded the limit of items returned."]
    #[serde(
        rename = "exceededWorkItemQueryLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exceeded_work_item_query_limit: Option<bool>,
    #[doc = "Boolean indicating if there are more elements present than what are being sent."]
    #[serde(
        rename = "hasMoreElements",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_more_elements: Option<bool>,
    #[doc = "Specifies if there was an error while execution of data provider."]
    #[serde(
        rename = "returnCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub return_code: Option<library_work_items_data::ReturnCode>,
    #[doc = "List of work items returned when OrderByField is sent something other than Id."]
    #[serde(
        rename = "workItemIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_ids: Vec<i32>,
    #[doc = "List of work items to be returned."]
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<WorkItemDetails>,
}
impl LibraryWorkItemsData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod library_work_items_data {
    use super::*;
    #[doc = "Specifies if there was an error while execution of data provider."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReturnCode {
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
}
#[doc = "This is the request data contract for LibraryTestCaseDataProvider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LibraryWorkItemsDataProviderRequest {
    #[doc = "Specifies the list of column options to show in test cases table."]
    #[serde(
        rename = "columnOptions",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub column_options: Vec<String>,
    #[doc = "The continuation token required for paging of work items. This is required when getting subsequent sets of work items when OrderByField is Id."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "List of filter values to be supplied. Currently supported filters are Title, State, AssignedTo, Priority, AreaPath."]
    #[serde(
        rename = "filterValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filter_values: Vec<TestPlansLibraryWorkItemFilter>,
    #[doc = "Whether the data is to be sorted in ascending or descending order. When not supplied, defaults to descending."]
    #[serde(
        rename = "isAscending",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ascending: Option<bool>,
    #[doc = "The type of query to run."]
    #[serde(
        rename = "libraryQueryType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_query_type: Option<library_work_items_data_provider_request::LibraryQueryType>,
    #[doc = "Work item field on which to order the results. When not supplied, defaults to work item IDs."]
    #[serde(
        rename = "orderByField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_by_field: Option<String>,
    #[doc = "List of work items to query for field details. This is required when getting subsequent sets of work item fields when OrderByField is other than Id."]
    #[serde(
        rename = "workItemIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_ids: Vec<i32>,
}
impl LibraryWorkItemsDataProviderRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod library_work_items_data_provider_request {
    use super::*;
    #[doc = "The type of query to run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LibraryQueryType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "allTestCases")]
        AllTestCases,
        #[serde(rename = "testCasesWithActiveBugs")]
        TestCasesWithActiveBugs,
        #[serde(rename = "testCasesNotLinkedToRequirements")]
        TestCasesNotLinkedToRequirements,
        #[serde(rename = "testCasesLinkedToRequirements")]
        TestCasesLinkedToRequirements,
        #[serde(rename = "allSharedSteps")]
        AllSharedSteps,
        #[serde(rename = "sharedStepsNotLinkedToRequirement")]
        SharedStepsNotLinkedToRequirement,
    }
}
#[doc = "Name value pair"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameValuePair {
    #[doc = "Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl NameValuePair {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assignments for the Test Point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PointAssignment {
    #[serde(flatten)]
    pub configuration: Configuration,
    #[doc = "Name of the Configuration Assigned to the Test Point"]
    #[serde(
        rename = "configurationName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_name: Option<String>,
    #[doc = "Id of the Test Point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tester: Option<IdentityRef>,
}
impl PointAssignment {
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
#[doc = "Reference to release environment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseEnvironmentDefinitionReference {
    #[doc = "ID of the release definition that contains the release environment definition."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "ID of the release environment definition."]
    #[serde(
        rename = "environmentDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_definition_id: Option<i32>,
}
impl ReleaseEnvironmentDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results class for Test Point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Results {
    #[doc = "Outcome of the Test Point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<results::Outcome>,
}
impl Results {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod results {
    use super::*;
    #[doc = "Outcome of the Test Point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Outcome {
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "inconclusive")]
        Inconclusive,
        #[serde(rename = "timeout")]
        Timeout,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "blocked")]
        Blocked,
        #[serde(rename = "notExecuted")]
        NotExecuted,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "notApplicable")]
        NotApplicable,
        #[serde(rename = "paused")]
        Paused,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "notImpacted")]
        NotImpacted,
        #[serde(rename = "maxValue")]
        MaxValue,
    }
}
#[doc = "Source Test Plan information for Test Plan clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceTestPlanInfo {
    #[doc = "ID of the source Test Plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Id of suites to be cloned inside source Test Plan"]
    #[serde(
        rename = "suiteIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suite_ids: Vec<i32>,
}
impl SourceTestPlanInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source Test Suite information for Test Suite clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceTestSuiteInfo {
    #[doc = "Id of the Source Test Suite"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl SourceTestSuiteInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source Test Suite Response for Test Case clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceTestSuiteResponse {
    #[serde(flatten)]
    pub test_suite_reference: TestSuiteReference,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[doc = "Id of suites to be cloned inside source Test Plan"]
    #[serde(
        rename = "testCaseIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_case_ids: Vec<i32>,
}
impl SourceTestSuiteResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source Test Plan Response for Test Plan clone operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceTestplanResponse {
    #[serde(flatten)]
    pub test_plan_reference: TestPlanReference,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[doc = "Id of suites to be cloned inside source Test Plan"]
    #[serde(
        rename = "suiteIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suite_ids: Vec<i32>,
}
impl SourceTestplanResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A suite entry defines properties for a test suite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuiteEntry {
    #[serde(flatten)]
    pub suite_entry_update_params: SuiteEntryUpdateParams,
    #[doc = "Id for the test suite."]
    #[serde(rename = "suiteId", default, skip_serializing_if = "Option::is_none")]
    pub suite_id: Option<i32>,
}
impl SuiteEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuiteEntryList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SuiteEntry>,
}
impl SuiteEntryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A suite entry defines properties for a test suite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuiteEntryUpdateParams {
    #[doc = "Id of the suite entry in the test suite: either a test case id or child suite id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Sequence number for the suite entry object in the test suite."]
    #[serde(
        rename = "sequenceNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence_number: Option<i32>,
    #[doc = "Defines whether the entry is of type test case or suite."]
    #[serde(
        rename = "suiteEntryType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suite_entry_type: Option<suite_entry_update_params::SuiteEntryType>,
}
impl SuiteEntryUpdateParams {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod suite_entry_update_params {
    use super::*;
    #[doc = "Defines whether the entry is of type test case or suite."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SuiteEntryType {
        #[serde(rename = "testCase")]
        TestCase,
        #[serde(rename = "suite")]
        Suite,
    }
}
#[doc = "Create and Update Suite Test Case Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuiteTestCaseCreateUpdateParameters {
    #[doc = "Configurations Ids"]
    #[serde(
        rename = "pointAssignments",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub point_assignments: Vec<Configuration>,
    #[doc = "Work Item"]
    #[serde(rename = "workItem", default, skip_serializing_if = "Option::is_none")]
    pub work_item: Option<WorkItem>,
}
impl SuiteTestCaseCreateUpdateParameters {
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
#[doc = "Test Case Class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCase {
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Order of the TestCase in the Suite"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "List of Points associated with the Test Case"]
    #[serde(
        rename = "pointAssignments",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub point_assignments: Vec<PointAssignment>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[doc = "The test plan reference resource."]
    #[serde(rename = "testPlan", default, skip_serializing_if = "Option::is_none")]
    pub test_plan: Option<TestPlanReference>,
    #[doc = "The test suite reference resource."]
    #[serde(rename = "testSuite", default, skip_serializing_if = "Option::is_none")]
    pub test_suite: Option<TestSuiteReference>,
    #[doc = "Work Item Class"]
    #[serde(rename = "workItem", default, skip_serializing_if = "Option::is_none")]
    pub work_item: Option<WorkItemDetails>,
}
impl TestCase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseAssociatedResult {
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Test Configuration Reference"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TestConfigurationReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<test_case_associated_result::Outcome>,
    #[doc = "The test plan reference resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TestPlanReference>,
    #[serde(rename = "pointId", default, skip_serializing_if = "Option::is_none")]
    pub point_id: Option<i32>,
    #[serde(rename = "resultId", default, skip_serializing_if = "Option::is_none")]
    pub result_id: Option<i32>,
    #[serde(rename = "runBy", default, skip_serializing_if = "Option::is_none")]
    pub run_by: Option<IdentityRef>,
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
    #[doc = "The test suite reference resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suite: Option<TestSuiteReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tester: Option<IdentityRef>,
}
impl TestCaseAssociatedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_case_associated_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Outcome {
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "blocked")]
        Blocked,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "notApplicable")]
        NotApplicable,
        #[serde(rename = "paused")]
        Paused,
        #[serde(rename = "timeout")]
        Timeout,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "notExecuted")]
        NotExecuted,
        #[serde(rename = "inconclusive")]
        Inconclusive,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "notImpacted")]
        NotImpacted,
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "maxValue")]
        MaxValue,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestCase>,
}
impl TestCaseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Case Reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseReference {
    #[serde(
        rename = "assignedTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_to: Option<IdentityRef>,
    #[doc = "Test Case Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Test Case Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "State of the test case work item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl TestCaseReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This data model is used in TestCaseResultsDataProvider and populates the data required for initial page load"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseResultsData {
    #[serde(
        rename = "contextPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_point: Option<TestPointDetailedReference>,
    #[doc = "Use to store the results displayed in the table"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<TestCaseAssociatedResult>,
    #[doc = "Test Case Name to be displayed in the table header"]
    #[serde(
        rename = "testCaseName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_name: Option<String>,
}
impl TestCaseResultsData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestConfiguration {
    #[serde(flatten)]
    pub test_configuration_create_update_parameters: TestConfigurationCreateUpdateParameters,
    #[doc = "Id of the configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
}
impl TestConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Configuration Create or Update Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestConfigurationCreateUpdateParameters {
    #[doc = "Description of the configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Is the configuration a default for the test plans"]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Name of the configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "State of the configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<test_configuration_create_update_parameters::State>,
    #[doc = "Dictionary of Test Variable, Selected Value"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<NameValuePair>,
}
impl TestConfigurationCreateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_configuration_create_update_parameters {
    use super::*;
    #[doc = "State of the configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "inactive")]
        Inactive,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestConfigurationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestConfiguration>,
}
impl TestConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Configuration Reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestConfigurationReference {
    #[doc = "Id of the configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestConfigurationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Entity Count Used to store test cases count (define tab) and test point count (execute tab) Used to store test cases count (define tab) and test point count (execute tab)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestEntityCount {
    #[doc = "Test Entity Count"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Test Plan under which the Test Entities are"]
    #[serde(
        rename = "testPlanId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_plan_id: Option<i32>,
    #[doc = "Test Suite under which the Test Entities are"]
    #[serde(
        rename = "testSuiteId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_suite_id: Option<i32>,
    #[doc = "Total test entities in the suite without the applied filters"]
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i32>,
}
impl TestEntityCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test environment Detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestEnvironment {
    #[doc = "Test Environment Id."]
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<String>,
    #[doc = "Test Environment Name."]
    #[serde(
        rename = "environmentName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_name: Option<String>,
}
impl TestEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test outcome settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestOutcomeSettings {
    #[doc = "Value to configure how test outcomes for the same tests across suites are shown"]
    #[serde(
        rename = "syncOutcomeAcrossSuites",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sync_outcome_across_suites: Option<bool>,
}
impl TestOutcomeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test plan resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlan {
    #[serde(flatten)]
    pub test_plan_update_params: TestPlanUpdateParams,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "ID of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Previous build Id associated with the test plan"]
    #[serde(
        rename = "previousBuildId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_build_id: Option<i32>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[doc = "The test suite reference resource."]
    #[serde(rename = "rootSuite", default, skip_serializing_if = "Option::is_none")]
    pub root_suite: Option<TestSuiteReference>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<IdentityRef>,
    #[doc = "Updated date of the test plan"]
    #[serde(
        rename = "updatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub updated_date: Option<time::OffsetDateTime>,
}
impl TestPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test plan create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlanCreateParams {
    #[doc = "Area of the test plan."]
    #[serde(rename = "areaPath", default, skip_serializing_if = "Option::is_none")]
    pub area_path: Option<String>,
    #[doc = "The build definition reference resource"]
    #[serde(
        rename = "buildDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_definition: Option<BuildDefinitionReference>,
    #[doc = "Build to be tested."]
    #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<i32>,
    #[doc = "Description of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "End date for the test plan."]
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "Iteration path of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iteration: Option<String>,
    #[doc = "Name of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "Reference to release environment resource."]
    #[serde(
        rename = "releaseEnvironmentDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_environment_definition: Option<ReleaseEnvironmentDefinitionReference>,
    #[doc = "Start date for the test plan."]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "State of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Test outcome settings"]
    #[serde(
        rename = "testOutcomeSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_outcome_settings: Option<TestOutcomeSettings>,
}
impl TestPlanCreateParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test plan detailed reference resource. Contains additional workitem realted information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlanDetailedReference {
    #[serde(flatten)]
    pub test_plan_reference: TestPlanReference,
    #[doc = "Area of the test plan."]
    #[serde(rename = "areaPath", default, skip_serializing_if = "Option::is_none")]
    pub area_path: Option<String>,
    #[doc = "End date for the test plan."]
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "Iteration path of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iteration: Option<String>,
    #[doc = "Root Suite Id"]
    #[serde(
        rename = "rootSuiteId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub root_suite_id: Option<i32>,
    #[doc = "Start date for the test plan."]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub start_date: Option<time::OffsetDateTime>,
}
impl TestPlanDetailedReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlanList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestPlan>,
}
impl TestPlanList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test plan reference resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlanReference {
    #[doc = "ID of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestPlanReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test plan update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlanUpdateParams {
    #[serde(flatten)]
    pub test_plan_create_params: TestPlanCreateParams,
    #[doc = "Revision of the test plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl TestPlanUpdateParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This data model is used in TestPlansHubRefreshDataProvider and populates the data required for initial page load"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlansHubRefreshData {
    #[serde(
        rename = "defineColumnOptionFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub define_column_option_fields: Vec<String>,
    #[serde(
        rename = "defineTabCustomColumnFieldMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub define_tab_custom_column_field_map: Option<serde_json::Value>,
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[serde(
        rename = "executeColumnOptionFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub execute_column_option_fields: Vec<String>,
    #[serde(
        rename = "executeTabCustomColumnFieldMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execute_tab_custom_column_field_map: Option<serde_json::Value>,
    #[serde(
        rename = "isAdvancedExtensionEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_advanced_extension_enabled: Option<bool>,
    #[serde(
        rename = "selectedPivotId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selected_pivot_id: Option<String>,
    #[serde(
        rename = "selectedSuiteId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selected_suite_id: Option<i32>,
    #[serde(
        rename = "testCasePageSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_page_size: Option<i32>,
    #[serde(
        rename = "testCases",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_cases: Vec<TestCase>,
    #[serde(
        rename = "testCasesContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_cases_continuation_token: Option<String>,
    #[doc = "The test plan detailed reference resource. Contains additional workitem realted information"]
    #[serde(rename = "testPlan", default, skip_serializing_if = "Option::is_none")]
    pub test_plan: Option<TestPlanDetailedReference>,
    #[serde(
        rename = "testPointPageSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_point_page_size: Option<i32>,
    #[serde(
        rename = "testPoints",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_points: Vec<TestPoint>,
    #[serde(
        rename = "testPointsContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_points_continuation_token: Option<String>,
    #[serde(
        rename = "testSuites",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_suites: Vec<TestSuite>,
    #[serde(
        rename = "testSuitesContinuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_suites_continuation_token: Option<String>,
}
impl TestPlansHubRefreshData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container to hold information about a filter being applied in Test Plans Library."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPlansLibraryWorkItemFilter {
    #[doc = "Work item field name on which the items are to be filtered."]
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[doc = "Work item field values corresponding to the field name."]
    #[serde(
        rename = "fieldValues",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_values: Vec<String>,
    #[doc = "Mode of the filter."]
    #[serde(
        rename = "filterMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_mode: Option<test_plans_library_work_item_filter::FilterMode>,
}
impl TestPlansLibraryWorkItemFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_plans_library_work_item_filter {
    use super::*;
    #[doc = "Mode of the filter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FilterMode {
        #[serde(rename = "or")]
        Or,
        #[serde(rename = "and")]
        And,
    }
}
#[doc = "Test Point Class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPoint {
    #[doc = "Comment associated to the Test Point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Test Configuration Reference"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TestConfigurationReference>,
    #[doc = "Id of the Test Point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Variable to decide whether the test case is Active or not"]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "Is the Test Point for Automated Test Case or Manual"]
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Last Reset to Active Time Stamp for the Test Point"]
    #[serde(
        rename = "lastResetToActive",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_reset_to_active: Option<time::OffsetDateTime>,
    #[serde(
        rename = "lastUpdatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_by: Option<IdentityRef>,
    #[doc = "Last Update Time Stamp for the Test Point"]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "The class to represent a collection of REST reference links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[doc = "Test Point Results"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<TestPointResults>,
    #[doc = "Test Case Reference"]
    #[serde(
        rename = "testCaseReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_reference: Option<TestCaseReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tester: Option<IdentityRef>,
    #[doc = "The test plan reference resource."]
    #[serde(rename = "testPlan", default, skip_serializing_if = "Option::is_none")]
    pub test_plan: Option<TestPlanReference>,
    #[doc = "The test suite reference resource."]
    #[serde(rename = "testSuite", default, skip_serializing_if = "Option::is_none")]
    pub test_suite: Option<TestSuiteReference>,
}
impl TestPoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPointDetailedReference {
    #[doc = "Test Configuration Reference"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TestConfigurationReference>,
    #[doc = "The test plan reference resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TestPlanReference>,
    #[serde(rename = "pointId", default, skip_serializing_if = "Option::is_none")]
    pub point_id: Option<i32>,
    #[doc = "The test suite reference resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suite: Option<TestSuiteReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tester: Option<IdentityRef>,
}
impl TestPointDetailedReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPointList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestPoint>,
}
impl TestPointList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Point Results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPointResults {
    #[doc = "Failure Type for the Test Point"]
    #[serde(
        rename = "failureType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_type: Option<test_point_results::FailureType>,
    #[doc = "Last Resolution State Id for the Test Point"]
    #[serde(
        rename = "lastResolutionState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_resolution_state: Option<test_point_results::LastResolutionState>,
    #[doc = "Last result details of test point."]
    #[serde(
        rename = "lastResultDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_result_details: Option<LastResultDetails>,
    #[doc = "Last Result Id"]
    #[serde(
        rename = "lastResultId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_result_id: Option<i32>,
    #[doc = "Last Result State of the Test Point"]
    #[serde(
        rename = "lastResultState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_result_state: Option<test_point_results::LastResultState>,
    #[doc = "Last RUn Build Number for the Test Point"]
    #[serde(
        rename = "lastRunBuildNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_run_build_number: Option<String>,
    #[doc = "Last Test Run Id for the Test Point"]
    #[serde(
        rename = "lastTestRunId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_test_run_id: Option<i32>,
    #[doc = "Outcome of the Test Point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<test_point_results::Outcome>,
    #[doc = "State of the Test Point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<test_point_results::State>,
}
impl TestPointResults {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_point_results {
    use super::*;
    #[doc = "Failure Type for the Test Point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FailureType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "regression")]
        Regression,
        #[serde(rename = "new_Issue")]
        NewIssue,
        #[serde(rename = "known_Issue")]
        KnownIssue,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "null_Value")]
        NullValue,
        #[serde(rename = "maxValue")]
        MaxValue,
    }
    #[doc = "Last Resolution State Id for the Test Point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastResolutionState {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "needsInvestigation")]
        NeedsInvestigation,
        #[serde(rename = "testIssue")]
        TestIssue,
        #[serde(rename = "productIssue")]
        ProductIssue,
        #[serde(rename = "configurationIssue")]
        ConfigurationIssue,
        #[serde(rename = "nullValue")]
        NullValue,
        #[serde(rename = "maxValue")]
        MaxValue,
    }
    #[doc = "Last Result State of the Test Point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastResultState {
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "paused")]
        Paused,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "maxValue")]
        MaxValue,
    }
    #[doc = "Outcome of the Test Point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Outcome {
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "inconclusive")]
        Inconclusive,
        #[serde(rename = "timeout")]
        Timeout,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "blocked")]
        Blocked,
        #[serde(rename = "notExecuted")]
        NotExecuted,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "notApplicable")]
        NotApplicable,
        #[serde(rename = "paused")]
        Paused,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "notImpacted")]
        NotImpacted,
        #[serde(rename = "maxValue")]
        MaxValue,
    }
    #[doc = "State of the Test Point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "notReady")]
        NotReady,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "maxValue")]
        MaxValue,
    }
}
#[doc = "Test Point Update Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestPointUpdateParams {
    #[doc = "Id of Test Point to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Reset the Test Point to Active"]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "Results class for Test Point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Results>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tester: Option<IdentityRef>,
}
impl TestPointUpdateParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the test settings of the run. Used to create test settings and fetch test settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSettings {
    #[doc = "Area path required to create test settings"]
    #[serde(rename = "areaPath", default, skip_serializing_if = "Option::is_none")]
    pub area_path: Option<String>,
    #[doc = "Description of the test settings. Used in create test settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicates if the tests settings is public or private.Used in create test settings."]
    #[serde(rename = "isPublic", default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[doc = "Xml string of machine roles. Used in create test settings."]
    #[serde(
        rename = "machineRoles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub machine_roles: Option<String>,
    #[doc = "Test settings content."]
    #[serde(
        rename = "testSettingsContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_settings_content: Option<String>,
    #[doc = "Test settings id."]
    #[serde(
        rename = "testSettingsId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_settings_id: Option<i32>,
    #[doc = "Test settings name."]
    #[serde(
        rename = "testSettingsName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_settings_name: Option<String>,
}
impl TestSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test suite"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSuite {
    #[serde(flatten)]
    pub test_suite_create_params: TestSuiteCreateParams,
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "Child test suites of current test suite."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<TestSuite>,
    #[doc = "Boolean value dictating if Child test suites are present"]
    #[serde(
        rename = "hasChildren",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_children: Option<bool>,
    #[doc = "Id of test suite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Last error for test suite."]
    #[serde(rename = "lastError", default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[doc = "Last populated date."]
    #[serde(
        rename = "lastPopulatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_populated_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "lastUpdatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_by: Option<IdentityRef>,
    #[doc = "Last update date."]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "The test plan reference resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TestPlanReference>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
    #[doc = "Test suite revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl TestSuite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test suite Create Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSuiteCreateParams {
    #[serde(flatten)]
    pub test_suite_create_update_common_params: TestSuiteCreateUpdateCommonParams,
    #[doc = "Test suite requirement id."]
    #[serde(
        rename = "requirementId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requirement_id: Option<i32>,
    #[doc = "Test suite type."]
    #[serde(rename = "suiteType", default, skip_serializing_if = "Option::is_none")]
    pub suite_type: Option<test_suite_create_params::SuiteType>,
}
impl TestSuiteCreateParams {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_suite_create_params {
    use super::*;
    #[doc = "Test suite type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SuiteType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "dynamicTestSuite")]
        DynamicTestSuite,
        #[serde(rename = "staticTestSuite")]
        StaticTestSuite,
        #[serde(rename = "requirementTestSuite")]
        RequirementTestSuite,
    }
}
#[doc = "Test Suite Create/Update Common Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSuiteCreateUpdateCommonParams {
    #[doc = "Test suite default configurations."]
    #[serde(
        rename = "defaultConfigurations",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub default_configurations: Vec<TestConfigurationReference>,
    #[doc = "Test suite default testers."]
    #[serde(
        rename = "defaultTesters",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub default_testers: Vec<IdentityRef>,
    #[doc = "Default configuration was inherited or not."]
    #[serde(
        rename = "inheritDefaultConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inherit_default_configurations: Option<bool>,
    #[doc = "Name of test suite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The test suite reference resource."]
    #[serde(
        rename = "parentSuite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_suite: Option<TestSuiteReference>,
    #[doc = "Test suite query string, for dynamic suites."]
    #[serde(
        rename = "queryString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_string: Option<String>,
}
impl TestSuiteCreateUpdateCommonParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSuiteList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestSuite>,
}
impl TestSuiteList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test suite reference resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSuiteReference {
    #[doc = "ID of the test suite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the test suite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestSuiteReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Suite Reference with Project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSuiteReferenceWithProject {
    #[serde(flatten)]
    pub test_suite_reference: TestSuiteReference,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
}
impl TestSuiteReferenceWithProject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Suite Update Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSuiteUpdateParams {
    #[serde(flatten)]
    pub test_suite_create_update_common_params: TestSuiteCreateUpdateCommonParams,
    #[doc = "Test suite revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl TestSuiteUpdateParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Variable"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestVariable {
    #[serde(flatten)]
    pub test_variable_create_update_parameters: TestVariableCreateUpdateParameters,
    #[doc = "Id of the test variable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Represents a shallow reference to a TeamProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<TeamProjectReference>,
}
impl TestVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Variable Create or Update Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestVariableCreateUpdateParameters {
    #[doc = "Description of the test variable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the test variable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of allowed values"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl TestVariableCreateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestVariableList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestVariable>,
}
impl TestVariableList {
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
#[doc = "Work Item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItem {
    #[doc = "Id of the Work Item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
impl WorkItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Work Item Class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemDetails {
    #[doc = "Work Item Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Work Item Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Work Item Fields"]
    #[serde(
        rename = "workItemFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_item_fields: Vec<serde_json::Value>,
}
impl WorkItemDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
