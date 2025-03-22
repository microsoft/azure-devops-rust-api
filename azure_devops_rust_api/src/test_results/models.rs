// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedDataForResultTrend {
    #[doc = "This is tests execution duration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(
        rename = "resultsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_by_outcome: Option<serde_json::Value>,
    #[serde(
        rename = "runSummaryByState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary_by_state: Option<serde_json::Value>,
    #[serde(
        rename = "testResultsContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_results_context: Option<TestResultsContext>,
    #[serde(
        rename = "totalTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_tests: Option<i32>,
}
impl AggregatedDataForResultTrend {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedDataForResultTrendList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AggregatedDataForResultTrend>,
}
impl AggregatedDataForResultTrendList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result deatils for a particular test result outcome."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedResultDetailsByOutcome {
    #[doc = "Number of results for current outcome."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Time taken by results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Test result outcome"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<aggregated_result_details_by_outcome::Outcome>,
    #[doc = "Number of results on rerun"]
    #[serde(
        rename = "rerunResultCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rerun_result_count: Option<i32>,
}
impl AggregatedResultDetailsByOutcome {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aggregated_result_details_by_outcome {
    use super::*;
    #[doc = "Test result outcome"]
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
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedResultsAnalysis {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(
        rename = "notReportedResultsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_reported_results_by_outcome: Option<serde_json::Value>,
    #[serde(
        rename = "previousContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_context: Option<TestResultsContext>,
    #[serde(
        rename = "resultsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_by_outcome: Option<serde_json::Value>,
    #[serde(
        rename = "resultsDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_difference: Option<AggregatedResultsDifference>,
    #[serde(
        rename = "runSummaryByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary_by_outcome: Option<serde_json::Value>,
    #[serde(
        rename = "runSummaryByState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary_by_state: Option<serde_json::Value>,
    #[serde(
        rename = "totalTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_tests: Option<i32>,
}
impl AggregatedResultsAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedResultsByOutcome {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(
        rename = "groupByField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_field: Option<String>,
    #[serde(
        rename = "groupByValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<aggregated_results_by_outcome::Outcome>,
    #[serde(
        rename = "rerunResultCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rerun_result_count: Option<i32>,
}
impl AggregatedResultsByOutcome {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aggregated_results_by_outcome {
    use super::*;
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
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedResultsDifference {
    #[serde(
        rename = "increaseInDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_duration: Option<String>,
    #[serde(
        rename = "increaseInFailures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_failures: Option<i32>,
    #[serde(
        rename = "increaseInNonImpactedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_non_impacted_tests: Option<i32>,
    #[serde(
        rename = "increaseInOtherTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_other_tests: Option<i32>,
    #[serde(
        rename = "increaseInPassedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_passed_tests: Option<i32>,
    #[serde(
        rename = "increaseInTotalTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub increase_in_total_tests: Option<i32>,
}
impl AggregatedResultsDifference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedRunsByOutcome {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<aggregated_runs_by_outcome::Outcome>,
    #[serde(rename = "runsCount", default, skip_serializing_if = "Option::is_none")]
    pub runs_count: Option<i32>,
}
impl AggregatedRunsByOutcome {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aggregated_runs_by_outcome {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Outcome {
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "notImpacted")]
        NotImpacted,
        #[serde(rename = "others")]
        Others,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregatedRunsByState {
    #[serde(
        rename = "resultsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_by_outcome: Option<serde_json::Value>,
    #[serde(rename = "runsCount", default, skip_serializing_if = "Option::is_none")]
    pub runs_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<aggregated_runs_by_state::State>,
}
impl AggregatedRunsByState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aggregated_runs_by_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "notStarted")]
        NotStarted,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "aborted")]
        Aborted,
        #[serde(rename = "waiting")]
        Waiting,
        #[serde(rename = "needsInvestigation")]
        NeedsInvestigation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attachment {
    #[serde(
        rename = "compressionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compression_type: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}
impl Attachment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BuildConfiguration Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildConfiguration {
    #[doc = "Branch name for which build is generated."]
    #[serde(
        rename = "branchName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_name: Option<String>,
    #[doc = "BuildDefinitionId for build."]
    #[serde(
        rename = "buildDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_definition_id: Option<i32>,
    #[doc = "Build system."]
    #[serde(
        rename = "buildSystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_system: Option<String>,
    #[doc = "Build Creation Date."]
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Build flavor (eg Build/Release)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,
    #[doc = "BuildConfiguration Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Build Number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "BuildConfiguration Platform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ShallowReference>,
    #[doc = "Repository Guid for the Build."]
    #[serde(
        rename = "repositoryGuid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_guid: Option<String>,
    #[doc = "Repository Type (eg. TFSGit)."]
    #[serde(
        rename = "repositoryType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_type: Option<String>,
    #[doc = "Source Version(/first commit) for the build was triggered."]
    #[serde(
        rename = "sourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_version: Option<String>,
    #[doc = "Target BranchName."]
    #[serde(
        rename = "targetBranchName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_branch_name: Option<String>,
    #[doc = "Build Uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl BuildConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build Coverage Detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildCoverage {
    #[doc = "Code Coverage File Url"]
    #[serde(
        rename = "codeCoverageFileUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code_coverage_file_url: Option<String>,
    #[doc = "BuildConfiguration Details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<BuildConfiguration>,
    #[doc = "Last Error"]
    #[serde(rename = "lastError", default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[doc = "List of Modules"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub modules: Vec<ModuleCoverage>,
    #[doc = "State"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl BuildCoverage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildCoverageList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BuildCoverage>,
}
impl BuildCoverageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildReference {
    #[doc = "Branch name."]
    #[serde(
        rename = "branchName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_name: Option<String>,
    #[doc = "Build system."]
    #[serde(
        rename = "buildSystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_system: Option<String>,
    #[doc = "Build Definition ID."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "Build ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Build Number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "Repository ID."]
    #[serde(
        rename = "repositoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id: Option<String>,
    #[doc = "Build URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl BuildReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the build configuration (platform, flavor) and coverage data for the build"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeCoverageData {
    #[doc = "Flavor of build for which data is retrieved/published"]
    #[serde(
        rename = "buildFlavor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_flavor: Option<String>,
    #[doc = "Platform of build for which data is retrieved/published"]
    #[serde(
        rename = "buildPlatform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_platform: Option<String>,
    #[doc = "List of coverage data for the build"]
    #[serde(
        rename = "coverageStats",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub coverage_stats: Vec<CodeCoverageStatistics>,
}
impl CodeCoverageData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the code coverage statistics for a particular coverage label (modules, statements, blocks, etc.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeCoverageStatistics {
    #[doc = "Covered units"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub covered: Option<i32>,
    #[doc = "Delta of coverage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    #[doc = "Is delta valid"]
    #[serde(
        rename = "isDeltaAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_delta_available: Option<bool>,
    #[doc = "Label of coverage data (\"Blocks\", \"Statements\", \"Modules\", etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Position of label"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[doc = "Total units"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
impl CodeCoverageStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the code coverage summary results Used to publish or retrieve code coverage summary against a build"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeCoverageSummary {
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<ShallowReference>,
    #[doc = "List of coverage data and details for the build"]
    #[serde(
        rename = "coverageData",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub coverage_data: Vec<CodeCoverageData>,
    #[serde(
        rename = "coverageDetailedSummaryStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub coverage_detailed_summary_status:
        Option<code_coverage_summary::CoverageDetailedSummaryStatus>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "deltaBuild",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delta_build: Option<ShallowReference>,
    #[doc = "Uri of build against which difference in coverage is computed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<code_coverage_summary::Status>,
}
impl CodeCoverageSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod code_coverage_summary {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CoverageDetailedSummaryStatus {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "finalized")]
        Finalized,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "updateRequestQueued")]
        UpdateRequestQueued,
        #[serde(rename = "noModulesFound")]
        NoModulesFound,
        #[serde(rename = "numberOfFilesExceeded")]
        NumberOfFilesExceeded,
        #[serde(rename = "noInputFiles")]
        NoInputFiles,
        #[serde(rename = "buildCancelled")]
        BuildCancelled,
        #[serde(rename = "failedJobs")]
        FailedJobs,
        #[serde(rename = "moduleMergeJobTimeout")]
        ModuleMergeJobTimeout,
        #[serde(rename = "codeCoverageSuccess")]
        CodeCoverageSuccess,
        #[serde(rename = "invalidBuildConfiguration")]
        InvalidBuildConfiguration,
        #[serde(rename = "coverageAnalyzerBuildNotFound")]
        CoverageAnalyzerBuildNotFound,
        #[serde(rename = "failedToRequeue")]
        FailedToRequeue,
        #[serde(rename = "buildBailedOut")]
        BuildBailedOut,
        #[serde(rename = "noCodeCoverageTask")]
        NoCodeCoverageTask,
        #[serde(rename = "mergeJobFailed")]
        MergeJobFailed,
        #[serde(rename = "mergeInvokerJobFailed")]
        MergeInvokerJobFailed,
        #[serde(rename = "monitorJobFailed")]
        MonitorJobFailed,
        #[serde(rename = "moduleMergeInvokerJobTimeout")]
        ModuleMergeInvokerJobTimeout,
        #[serde(rename = "monitorJobTimeout")]
        MonitorJobTimeout,
        #[serde(rename = "invalidCoverageInput")]
        InvalidCoverageInput,
    }
    #[doc = "Uri of build against which difference in coverage is computed"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "finalized")]
        Finalized,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "updateRequestQueued")]
        UpdateRequestQueued,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CoverageStatistics {
    #[serde(
        rename = "blocksCovered",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocks_covered: Option<i32>,
    #[serde(
        rename = "blocksNotCovered",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocks_not_covered: Option<i32>,
    #[serde(
        rename = "linesCovered",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lines_covered: Option<i32>,
    #[serde(
        rename = "linesNotCovered",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lines_not_covered: Option<i32>,
    #[serde(
        rename = "linesPartiallyCovered",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lines_partially_covered: Option<i32>,
}
impl CoverageStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom field information. Allowed Key : Value pairs - ( AttemptId: int value, IsTestResultFlaky: bool)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomTestField {
    #[doc = "Field Name."]
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[doc = "Field value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl CustomTestField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is a temporary class to provide the details for the test run environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DtlEnvironmentDetails {
    #[serde(
        rename = "csmContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub csm_content: Option<String>,
    #[serde(
        rename = "csmParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub csm_parameters: Option<String>,
    #[serde(
        rename = "subscriptionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_name: Option<String>,
}
impl DtlEnvironmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Failing since information of a test result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailingSince {
    #[doc = "Reference to a build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildReference>,
    #[doc = "Time since failing(UTC)."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "Reference to a release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseReference>,
}
impl FailingSince {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldDetailsForTestResults {
    #[doc = "Group by field name"]
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[doc = "Group by field values"]
    #[serde(
        rename = "groupsForField",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups_for_field: Vec<serde_json::Value>,
}
impl FieldDetailsForTestResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldDetailsForTestResultsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FieldDetailsForTestResults>,
}
impl FieldDetailsForTestResultsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileCoverageRequest {
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(
        rename = "pullRequestBaseIterationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_base_iteration_id: Option<i32>,
    #[serde(
        rename = "pullRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_id: Option<i32>,
    #[serde(
        rename = "pullRequestIterationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_iteration_id: Option<i32>,
    #[serde(rename = "repoId", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
}
impl FileCoverageRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlakyDetection {
    #[serde(
        rename = "flakyDetectionPipelines",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flaky_detection_pipelines: Option<FlakyDetectionPipelines>,
    #[doc = "FlakyDetectionType defines Detection type i.e. 1. System or 2. Manual."]
    #[serde(
        rename = "flakyDetectionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flaky_detection_type: Option<flaky_detection::FlakyDetectionType>,
}
impl FlakyDetection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod flaky_detection {
    use super::*;
    #[doc = "FlakyDetectionType defines Detection type i.e. 1. System or 2. Manual."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FlakyDetectionType {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "system")]
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlakyDetectionPipelines {
    #[doc = "AllowedPipelines - List All Pipelines allowed for detection."]
    #[serde(
        rename = "allowedPipelines",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_pipelines: Vec<i32>,
    #[doc = "IsAllPipelinesAllowed if users configure all system's pipelines."]
    #[serde(
        rename = "isAllPipelinesAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_all_pipelines_allowed: Option<bool>,
}
impl FlakyDetectionPipelines {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlakySettings {
    #[serde(
        rename = "flakyDetection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flaky_detection: Option<FlakyDetection>,
    #[doc = "FlakyInSummaryReport defines flaky data should show in summary report or not."]
    #[serde(
        rename = "flakyInSummaryReport",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flaky_in_summary_report: Option<bool>,
    #[doc = "IsFlakyBugCreated defines if there is any bug that has been created with flaky testresult."]
    #[serde(
        rename = "isFlakyBugCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_flaky_bug_created: Option<bool>,
    #[doc = "ManualMarkUnmarkFlaky defines manual marking unmarking of flaky testcase."]
    #[serde(
        rename = "manualMarkUnmarkFlaky",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manual_mark_unmark_flaky: Option<bool>,
}
impl FlakySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionCoverage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "sourceFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<CoverageStatistics>,
}
impl FunctionCoverage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubjectBase {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ReferenceLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
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
    #[serde(
        rename = "directoryAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub directory_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[serde(
        rename = "isAadIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_aad_identity: Option<bool>,
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
    #[serde(
        rename = "profileUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_url: Option<String>,
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
#[doc = "Job in pipeline. This is related to matrixing in YAML."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobReference {
    #[doc = "Attempt number of the job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Matrixing in YAML generates copies of a job with different inputs in matrix. JobName is the name of those input. Maximum supported length for name is 256 character."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}
impl JobReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleCoverage {
    #[serde(
        rename = "blockCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_count: Option<i32>,
    #[serde(
        rename = "blockData",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub block_data: Vec<String>,
    #[doc = "Code Coverage File Url"]
    #[serde(rename = "fileUrl", default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub functions: Vec<FunctionCoverage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(
        rename = "signatureAge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signature_age: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<CoverageStatistics>,
}
impl ModuleCoverage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewTestResultLoggingSettings {
    #[doc = "LogNewTests defines whether or not we will record new test cases coming into the system"]
    #[serde(
        rename = "logNewTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_new_tests: Option<bool>,
}
impl NewTestResultLoggingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Phase in pipeline"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PhaseReference {
    #[doc = "Attempt number of the phase"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Name of the phase. Maximum supported length for name is 256 character."]
    #[serde(rename = "phaseName", default, skip_serializing_if = "Option::is_none")]
    pub phase_name: Option<String>,
}
impl PhaseReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pipeline reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineReference {
    #[doc = "Job in pipeline. This is related to matrixing in YAML."]
    #[serde(
        rename = "jobReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_reference: Option<JobReference>,
    #[doc = "Phase in pipeline"]
    #[serde(
        rename = "phaseReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phase_reference: Option<PhaseReference>,
    #[doc = "Reference of the pipeline with which this pipeline instance is related."]
    #[serde(
        rename = "pipelineId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pipeline_id: Option<i32>,
    #[doc = "Stage in pipeline"]
    #[serde(
        rename = "stageReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_reference: Option<StageReference>,
}
impl PipelineReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test summary of a pipeline instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTestMetrics {
    #[doc = "Pipeline reference"]
    #[serde(
        rename = "currentContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_context: Option<PipelineReference>,
    #[doc = "Results insights for runs with state completed and NeedInvestigation."]
    #[serde(
        rename = "resultsAnalysis",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_analysis: Option<ResultsAnalysis>,
    #[doc = "Summary of results for a pipeline instance."]
    #[serde(
        rename = "resultSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_summary: Option<ResultSummary>,
    #[doc = "Summary of runs for a pipeline instance."]
    #[serde(
        rename = "runSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary: Option<RunSummary>,
    #[doc = "Summary at child node."]
    #[serde(
        rename = "summaryAtChild",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub summary_at_child: Vec<PipelineTestMetrics>,
}
impl PipelineTestMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl QueryModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
impl ReferenceLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a release."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReleaseReference {
    #[doc = "Number of Release Attempt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Release Creation Date(UTC)."]
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Release definition ID."]
    #[serde(
        rename = "definitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition_id: Option<i32>,
    #[doc = "Environment creation Date(UTC)."]
    #[serde(
        rename = "environmentCreationDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub environment_creation_date: Option<time::OffsetDateTime>,
    #[doc = "Release environment definition ID."]
    #[serde(
        rename = "environmentDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_definition_id: Option<i32>,
    #[doc = "Release environment definition name."]
    #[serde(
        rename = "environmentDefinitionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_definition_name: Option<String>,
    #[doc = "Release environment ID."]
    #[serde(
        rename = "environmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_id: Option<i32>,
    #[doc = "Release environment name."]
    #[serde(
        rename = "environmentName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_name: Option<String>,
    #[doc = "Release ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Release name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ReleaseReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of results for a pipeline instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResultSummary {
    #[doc = "Result summary of pipeline, group by TestRun state."]
    #[serde(
        rename = "resultSummaryByRunState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_summary_by_run_state: Option<serde_json::Value>,
}
impl ResultSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results insights for runs with state completed and NeedInvestigation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResultsAnalysis {
    #[doc = "Pipeline reference"]
    #[serde(
        rename = "previousContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_context: Option<PipelineReference>,
    #[serde(
        rename = "resultsDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_difference: Option<AggregatedResultsDifference>,
    #[serde(
        rename = "testFailuresAnalysis",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_failures_analysis: Option<TestResultFailuresAnalysis>,
}
impl ResultsAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResultsFilter {
    #[serde(
        rename = "automatedTestName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(
        rename = "executedIn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_in: Option<results_filter::ExecutedIn>,
    #[serde(rename = "groupBy", default, skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    #[serde(
        rename = "maxCompleteDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub max_complete_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "resultsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_count: Option<i32>,
    #[serde(
        rename = "testCaseId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_id: Option<i32>,
    #[serde(
        rename = "testCaseReferenceIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_case_reference_ids: Vec<i32>,
    #[serde(
        rename = "testPlanId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_plan_id: Option<i32>,
    #[serde(
        rename = "testPointIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_point_ids: Vec<i32>,
    #[serde(
        rename = "testResultsContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_results_context: Option<TestResultsContext>,
    #[serde(rename = "trendDays", default, skip_serializing_if = "Option::is_none")]
    pub trend_days: Option<i32>,
}
impl ResultsFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod results_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutedIn {
        #[serde(rename = "any")]
        Any,
        #[serde(rename = "tcm")]
        Tcm,
        #[serde(rename = "tfs")]
        Tfs,
    }
}
#[doc = "Result summary by the outcome of test results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResultsSummaryByOutcome {
    #[doc = "Aggregated result details for each test result outcome."]
    #[serde(
        rename = "aggregatedResultDetailsByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aggregated_result_details_by_outcome: Option<serde_json::Value>,
    #[doc = "Time taken by results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Total number of not reported test results."]
    #[serde(
        rename = "notReportedTestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_reported_test_count: Option<i32>,
    #[doc = "Total number of test results. (It includes NotImpacted test results as well which need to exclude while calculating pass/fail test result percentage)."]
    #[serde(
        rename = "totalTestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_test_count: Option<i32>,
}
impl ResultsSummaryByOutcome {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test run create details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunCreateModel {
    #[doc = "true if test run is automated, false otherwise. By default it will be false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automated: Option<bool>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<ShallowReference>,
    #[doc = "Drop location of the build used for test run."]
    #[serde(
        rename = "buildDropLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_drop_location: Option<String>,
    #[doc = "Flavor of the build used for test run. (E.g: Release, Debug)"]
    #[serde(
        rename = "buildFlavor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_flavor: Option<String>,
    #[doc = "Platform of the build used for test run. (E.g.: x86, amd64)"]
    #[serde(
        rename = "buildPlatform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_platform: Option<String>,
    #[doc = "BuildConfiguration Details."]
    #[serde(
        rename = "buildReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_reference: Option<BuildConfiguration>,
    #[doc = "Comments entered by those analyzing the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Completed date time of the run."]
    #[serde(
        rename = "completeDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complete_date: Option<String>,
    #[doc = "IDs of the test configurations associated with the run."]
    #[serde(
        rename = "configurationIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configuration_ids: Vec<i32>,
    #[doc = "Name of the test controller used for automated run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<String>,
    #[doc = "Additional properties of test Run."]
    #[serde(
        rename = "customTestFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_test_fields: Vec<CustomTestField>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "dtlAutEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_aut_environment: Option<ShallowReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "dtlTestEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_test_environment: Option<ShallowReference>,
    #[doc = "Due date and time for test run."]
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[doc = "This is a temporary class to provide the details for the test run environment."]
    #[serde(
        rename = "environmentDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_details: Option<DtlEnvironmentDetails>,
    #[doc = "Error message associated with the run."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "This class is used to provide the filters used for discovery"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<RunFilter>,
    #[doc = "The iteration in which to create the run. Root iteration of the team project will be default"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iteration: Option<String>,
    #[doc = "Name of the test run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "Pipeline reference"]
    #[serde(
        rename = "pipelineReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pipeline_reference: Option<PipelineReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<ShallowReference>,
    #[doc = "IDs of the test points to use in the run."]
    #[serde(
        rename = "pointIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub point_ids: Vec<i32>,
    #[doc = "URI of release environment associated with the run."]
    #[serde(
        rename = "releaseEnvironmentUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_environment_uri: Option<String>,
    #[doc = "Reference to a release."]
    #[serde(
        rename = "releaseReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_reference: Option<ReleaseReference>,
    #[doc = "URI of release associated with the run."]
    #[serde(
        rename = "releaseUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_uri: Option<String>,
    #[doc = "Run summary for run Type = NoConfigRun."]
    #[serde(
        rename = "runSummary",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub run_summary: Vec<RunSummaryModel>,
    #[doc = "Timespan till the run times out."]
    #[serde(
        rename = "runTimeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_timeout: Option<String>,
    #[doc = "SourceWorkFlow(CI/CD) of the test run."]
    #[serde(
        rename = "sourceWorkflow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_workflow: Option<String>,
    #[doc = "Start date time of the run."]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "The state of the run. Type TestRunState Valid states - NotStarted, InProgress, Waiting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Tags to attach with the test run, maximum of 5 tags can be added to run."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<TestTag>,
    #[doc = "TestConfigurationMapping of the test run."]
    #[serde(
        rename = "testConfigurationsMapping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_configurations_mapping: Option<String>,
    #[doc = "ID of the test environment associated with the run."]
    #[serde(
        rename = "testEnvironmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_environment_id: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "testSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_settings: Option<ShallowReference>,
    #[doc = "Type of the run(RunType) Valid Values : (Unspecified, Normal, Blocking, Web, MtrRunInitiatedFromWeb, RunWithDtlEnv, NoConfigRun)"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RunCreateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class is used to provide the filters used for discovery"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunFilter {
    #[doc = "filter for the test case sources (test containers)"]
    #[serde(
        rename = "sourceFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_filter: Option<String>,
    #[doc = "filter for the test cases"]
    #[serde(
        rename = "testCaseFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_filter: Option<String>,
}
impl RunFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test run statistics per outcome."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunStatistic {
    #[doc = "Test result count fo the given outcome."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Test result outcome"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[doc = "Test Resolution State Details."]
    #[serde(
        rename = "resolutionState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_state: Option<TestResolutionState>,
    #[doc = "ResultMetadata for the given outcome/count."]
    #[serde(
        rename = "resultMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_metadata: Option<run_statistic::ResultMetadata>,
    #[doc = "State of the test run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl RunStatistic {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_statistic {
    use super::*;
    #[doc = "ResultMetadata for the given outcome/count."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultMetadata {
        #[serde(rename = "rerun")]
        Rerun,
        #[serde(rename = "flaky")]
        Flaky,
    }
}
#[doc = "Summary of runs for a pipeline instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunSummary {
    #[doc = "Total time taken by runs with state completed and NeedInvestigation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "NoConfig runs count."]
    #[serde(
        rename = "noConfigRunsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_config_runs_count: Option<i32>,
    #[doc = "Runs count by outcome for runs with state completed and NeedInvestigation runs."]
    #[serde(
        rename = "runSummaryByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary_by_outcome: Option<serde_json::Value>,
    #[doc = "Runs count by state."]
    #[serde(
        rename = "runSummaryByState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_summary_by_state: Option<serde_json::Value>,
    #[doc = "Total runs count."]
    #[serde(
        rename = "totalRunsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_runs_count: Option<i32>,
}
impl RunSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Run summary for each output type of test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunSummaryModel {
    #[doc = "Total time taken in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "Number of results for Outcome TestOutcome"]
    #[serde(
        rename = "resultCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_count: Option<i32>,
    #[doc = "Summary is based on outcome"]
    #[serde(
        rename = "testOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_outcome: Option<run_summary_model::TestOutcome>,
}
impl RunSummaryModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_summary_model {
    use super::*;
    #[doc = "Summary is based on outcome"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TestOutcome {
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
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunUpdateModel {
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<ShallowReference>,
    #[doc = "Drop location of the build used for test run."]
    #[serde(
        rename = "buildDropLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_drop_location: Option<String>,
    #[doc = "Flavor of the build used for test run. (E.g: Release, Debug)"]
    #[serde(
        rename = "buildFlavor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_flavor: Option<String>,
    #[doc = "Platform of the build used for test run. (E.g.: x86, amd64)"]
    #[serde(
        rename = "buildPlatform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_platform: Option<String>,
    #[doc = "Comments entered by those analyzing the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Completed date time of the run."]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_date: Option<String>,
    #[doc = "Name of the test controller used for automated run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<String>,
    #[doc = "true to delete inProgess Results , false otherwise."]
    #[serde(
        rename = "deleteInProgressResults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_in_progress_results: Option<bool>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "dtlAutEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_aut_environment: Option<ShallowReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "dtlEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_environment: Option<ShallowReference>,
    #[doc = "This is a temporary class to provide the details for the test run environment."]
    #[serde(
        rename = "dtlEnvironmentDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_environment_details: Option<DtlEnvironmentDetails>,
    #[doc = "Due date and time for test run."]
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[doc = "Error message associated with the run."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "The iteration in which to create the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iteration: Option<String>,
    #[doc = "Log entries associated with the run. Use a comma-separated list of multiple log entry objects. { logEntry }, { logEntry }, ..."]
    #[serde(
        rename = "logEntries",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_entries: Vec<TestMessageLogDetails>,
    #[doc = "Name of the test run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "URI of release environment associated with the run."]
    #[serde(
        rename = "releaseEnvironmentUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_environment_uri: Option<String>,
    #[doc = "URI of release associated with the run."]
    #[serde(
        rename = "releaseUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_uri: Option<String>,
    #[doc = "Run summary for run Type = NoConfigRun."]
    #[serde(
        rename = "runSummary",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub run_summary: Vec<RunSummaryModel>,
    #[doc = "SourceWorkFlow(CI/CD) of the test run."]
    #[serde(
        rename = "sourceWorkflow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_workflow: Option<String>,
    #[doc = "Start date time of the run."]
    #[serde(
        rename = "startedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub started_date: Option<String>,
    #[doc = "The state of the test run Below are the valid values - NotStarted, InProgress, Completed, Aborted, Waiting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The types of sub states for test run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substate: Option<run_update_model::Substate>,
    #[doc = "Tags to attach with the test run."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<TestTag>,
    #[doc = "ID of the test environment associated with the run."]
    #[serde(
        rename = "testEnvironmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_environment_id: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "testSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_settings: Option<ShallowReference>,
}
impl RunUpdateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_update_model {
    use super::*;
    #[doc = "The types of sub states for test run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Substate {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "creatingEnvironment")]
        CreatingEnvironment,
        #[serde(rename = "runningTests")]
        RunningTests,
        #[serde(rename = "canceledByUser")]
        CanceledByUser,
        #[serde(rename = "abortedBySystem")]
        AbortedBySystem,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "pendingAnalysis")]
        PendingAnalysis,
        #[serde(rename = "analyzed")]
        Analyzed,
        #[serde(rename = "cancellationInProgress")]
        CancellationInProgress,
    }
}
#[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShallowReference {
    #[doc = "ID of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the linked resource (definition name, controller name, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Full http link to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ShallowReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShallowTestCaseResult {
    #[serde(
        rename = "automatedTestName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_name: Option<String>,
    #[serde(
        rename = "automatedTestStorage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_storage: Option<String>,
    #[serde(
        rename = "durationInMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_in_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isReRun", default, skip_serializing_if = "Option::is_none")]
    pub is_re_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "refId", default, skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<i32>,
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[serde(
        rename = "testCaseTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_title: Option<String>,
}
impl ShallowTestCaseResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShallowTestCaseResultList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ShallowTestCaseResult>,
}
impl ShallowTestCaseResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to shared step workitem."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedStepModel {
    #[doc = "WorkItem shared step ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Shared step workitem revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl SharedStepModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stage in pipeline"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageReference {
    #[doc = "Attempt number of stage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i32>,
    #[doc = "Name of the stage. Maximum supported length for name is 256 character."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}
impl StageReference {
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
#[doc = "Represents a test step result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestActionResultModel {
    #[serde(flatten)]
    pub test_result_model_base: TestResultModelBase,
    #[doc = "Path identifier for test step in test case workitem. Note: 1) It is represented in Hexadecimal format with 8 digits for a step. 2) Internally, the step ID value for first step starts with 2 so actionPath = 00000002 step 9, will have an ID = 10 and actionPath = 0000000a step 15, will have an ID =16 and actionPath = 00000010 3) actionPath of shared step is concatenated with the parent step of test case. Example, it would be something of type -  0000000300000001 where 00000003 denotes action path of test step and 00000001 denotes action path for shared step"]
    #[serde(
        rename = "actionPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_path: Option<String>,
    #[doc = "Iteration ID of test action result."]
    #[serde(
        rename = "iterationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_id: Option<i32>,
    #[doc = "Reference to shared step workitem."]
    #[serde(
        rename = "sharedStepModel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_step_model: Option<SharedStepModel>,
    #[doc = "This is step Id of test case. For shared step, it is step Id of shared step in test case workitem; step Id in shared step. Example: TestCase workitem has two steps: 1) Normal step with Id = 1 2) Shared Step with Id = 2. Inside shared step: a) Normal Step with Id = 1 Value for StepIdentifier for First step: \"1\" Second step: \"2;1\""]
    #[serde(
        rename = "stepIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub step_identifier: Option<String>,
    #[doc = "Url of test action result. Deprecated in hosted environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestActionResultModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAttachment {
    #[doc = "Attachment type."]
    #[serde(
        rename = "attachmentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachment_type: Option<test_attachment::AttachmentType>,
    #[doc = "Comment associated with attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Attachment created date."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Attachment file name"]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "ID of the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Attachment size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Attachment Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestAttachment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_attachment {
    use super::*;
    #[doc = "Attachment type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AttachmentType {
        #[serde(rename = "generalAttachment")]
        GeneralAttachment,
        #[serde(rename = "codeCoverage")]
        CodeCoverage,
        #[serde(rename = "consoleLog")]
        ConsoleLog,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAttachmentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestAttachment>,
}
impl TestAttachmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to test attachment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAttachmentReference {
    #[doc = "ID of the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Url to download the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestAttachmentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test attachment request model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAttachmentRequestModel {
    #[doc = "Attachment type By Default it will be GeneralAttachment. It can be one of the following type. { GeneralAttachment, AfnStrip, BugFilingData, CodeCoverage, IntermediateCollectorData, RunConfig, TestImpactDetails, TmiTestRunDeploymentFiles, TmiTestRunReverseDeploymentFiles, TmiTestResultDetail, TmiTestRunSummary }"]
    #[serde(
        rename = "attachmentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachment_type: Option<String>,
    #[doc = "Comment associated with attachment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Attachment filename"]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "Base64 encoded file stream"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}
impl TestAttachmentRequestModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a test result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseResult {
    #[doc = "Test attachment ID of action recording."]
    #[serde(
        rename = "afnStripId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub afn_strip_id: Option<i32>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<ShallowReference>,
    #[doc = "Reference to bugs linked to test result."]
    #[serde(
        rename = "associatedBugs",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub associated_bugs: Vec<ShallowReference>,
    #[doc = "ID representing test method in a dll."]
    #[serde(
        rename = "automatedTestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_id: Option<String>,
    #[doc = "Fully qualified name of test executed."]
    #[serde(
        rename = "automatedTestName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_name: Option<String>,
    #[doc = "Container to which test belongs."]
    #[serde(
        rename = "automatedTestStorage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_storage: Option<String>,
    #[doc = "Type of automated test."]
    #[serde(
        rename = "automatedTestType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_type: Option<String>,
    #[doc = "TypeId of automated test."]
    #[serde(
        rename = "automatedTestTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_type_id: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<ShallowReference>,
    #[doc = "Reference to a build."]
    #[serde(
        rename = "buildReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_reference: Option<BuildReference>,
    #[doc = "Comment in a test result with maxSize= 1000 chars."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Time when test execution completed(UTC). Completed date should be greater than StartedDate."]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Machine name where test executed."]
    #[serde(
        rename = "computerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub computer_name: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ShallowReference>,
    #[doc = "Timestamp when test result created(UTC)."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Additional properties of test result."]
    #[serde(
        rename = "customFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_fields: Vec<CustomTestField>,
    #[doc = "Duration of test execution in milliseconds. If not provided value will be set as CompletedDate - StartedDate"]
    #[serde(
        rename = "durationInMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_in_ms: Option<f64>,
    #[doc = "Error message in test execution."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Failing since information of a test result."]
    #[serde(
        rename = "failingSince",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failing_since: Option<FailingSince>,
    #[doc = "Failure type of test result. Valid Value= (Known Issue, New Issue, Regression, Unknown, None)"]
    #[serde(
        rename = "failureType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_type: Option<String>,
    #[doc = "ID of a test result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Test result details of test iterations used only for Manual Testing."]
    #[serde(
        rename = "iterationDetails",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub iteration_details: Vec<TestIterationDetailsModel>,
    #[serde(
        rename = "lastUpdatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_by: Option<IdentityRef>,
    #[doc = "Last updated datetime of test result(UTC)."]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "Test outcome of test result. Valid values = (Unspecified, None, Passed, Failed, Inconclusive, Timeout, Aborted, Blocked, NotExecuted, Warning, Error, NotApplicable, Paused, InProgress, NotImpacted)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "Priority of test executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ShallowReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ShallowReference>,
    #[doc = "Reference to a release."]
    #[serde(
        rename = "releaseReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_reference: Option<ReleaseReference>,
    #[doc = "ResetCount."]
    #[serde(
        rename = "resetCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reset_count: Option<i32>,
    #[doc = "Resolution state of test result."]
    #[serde(
        rename = "resolutionState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_state: Option<String>,
    #[doc = "ID of resolution state."]
    #[serde(
        rename = "resolutionStateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_state_id: Option<i32>,
    #[doc = "Hierarchy type of the result, default value of None means its leaf node."]
    #[serde(
        rename = "resultGroupType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_group_type: Option<test_case_result::ResultGroupType>,
    #[doc = "Revision number of test result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(rename = "runBy", default, skip_serializing_if = "Option::is_none")]
    pub run_by: Option<IdentityRef>,
    #[doc = "Stacktrace with maxSize= 1000 chars."]
    #[serde(
        rename = "stackTrace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<String>,
    #[doc = "Time when test execution started(UTC)."]
    #[serde(
        rename = "startedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_date: Option<time::OffsetDateTime>,
    #[doc = "State of test result. Type TestRunState."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "List of sub results inside a test result, if ResultGroupType is not None, it holds corresponding type sub results."]
    #[serde(
        rename = "subResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_results: Vec<TestSubResult>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(rename = "testCase", default, skip_serializing_if = "Option::is_none")]
    pub test_case: Option<ShallowReference>,
    #[doc = "Reference ID of test used by test result. Type TestResultMetaData"]
    #[serde(
        rename = "testCaseReferenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_reference_id: Option<i32>,
    #[doc = "TestCaseRevision Number."]
    #[serde(
        rename = "testCaseRevision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_revision: Option<i32>,
    #[doc = "Name of test."]
    #[serde(
        rename = "testCaseTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_title: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(rename = "testPlan", default, skip_serializing_if = "Option::is_none")]
    pub test_plan: Option<ShallowReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(rename = "testPoint", default, skip_serializing_if = "Option::is_none")]
    pub test_point: Option<ShallowReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(rename = "testRun", default, skip_serializing_if = "Option::is_none")]
    pub test_run: Option<ShallowReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(rename = "testSuite", default, skip_serializing_if = "Option::is_none")]
    pub test_suite: Option<ShallowReference>,
    #[doc = "Url of test result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestCaseResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_case_result {
    use super::*;
    #[doc = "Hierarchy type of the result, default value of None means its leaf node."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultGroupType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "rerun")]
        Rerun,
        #[serde(rename = "dataDriven")]
        DataDriven,
        #[serde(rename = "orderedTest")]
        OrderedTest,
        #[serde(rename = "generic")]
        Generic,
    }
}
#[doc = "Test attachment information in a test iteration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseResultAttachmentModel {
    #[doc = "Path identifier test step in test case workitem."]
    #[serde(
        rename = "actionPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_path: Option<String>,
    #[doc = "Attachment ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Iteration ID."]
    #[serde(
        rename = "iterationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_id: Option<i32>,
    #[doc = "Name of attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Attachment size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Url to attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestCaseResultAttachmentModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to a test result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseResultIdentifier {
    #[doc = "Test result ID."]
    #[serde(
        rename = "testResultId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_result_id: Option<i32>,
    #[doc = "Test run ID."]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<i32>,
}
impl TestCaseResultIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCaseResultList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestCaseResult>,
}
impl TestCaseResultList {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestFailureDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        rename = "testResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_results: Vec<TestCaseResultIdentifier>,
}
impl TestFailureDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestFailuresAnalysis {
    #[serde(
        rename = "existingFailures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_failures: Option<TestFailureDetails>,
    #[serde(
        rename = "fixedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_tests: Option<TestFailureDetails>,
    #[serde(
        rename = "newFailures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_failures: Option<TestFailureDetails>,
    #[serde(
        rename = "previousContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_context: Option<TestResultsContext>,
}
impl TestFailuresAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Flaky Identifier"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestFlakyIdentifier {
    #[doc = "Branch Name where Flakiness has to be Marked/Unmarked"]
    #[serde(
        rename = "branchName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_name: Option<String>,
    #[doc = "State for Flakiness"]
    #[serde(rename = "isFlaky", default, skip_serializing_if = "Option::is_none")]
    pub is_flaky: Option<bool>,
}
impl TestFlakyIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filter to get TestCase result history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestHistoryQuery {
    #[doc = "Automated test name of the TestCase."]
    #[serde(
        rename = "automatedTestName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_name: Option<String>,
    #[doc = "Results to be get for a particular branches."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "Get the results history only for this BuildDefinitionId. This to get used in query GroupBy should be Branch. If this is provided, Branch will have no use."]
    #[serde(
        rename = "buildDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_definition_id: Option<i32>,
    #[doc = "It will be filled by server. If not null means there are some results still to be get, and we need to call this REST API with this ContinuousToken. It is not supposed to be created (or altered, if received from server in last batch) by user."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_token: Option<String>,
    #[doc = "Group the result on the basis of TestResultGroupBy. This can be Branch, Environment or null(if results are fetched by BuildDefinitionId)"]
    #[serde(rename = "groupBy", default, skip_serializing_if = "Option::is_none")]
    pub group_by: Option<test_history_query::GroupBy>,
    #[doc = "History to get between time interval MaxCompleteDate and  (MaxCompleteDate - TrendDays). Default is current date time."]
    #[serde(
        rename = "maxCompleteDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub max_complete_date: Option<time::OffsetDateTime>,
    #[doc = "Get the results history only for this ReleaseEnvDefinitionId. This to get used in query GroupBy should be Environment."]
    #[serde(
        rename = "releaseEnvDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_env_definition_id: Option<i32>,
    #[doc = "List of TestResultHistoryForGroup which are grouped by GroupBy"]
    #[serde(
        rename = "resultsForGroup",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results_for_group: Vec<TestResultHistoryForGroup>,
    #[doc = "Get the results history only for this testCaseId. This to get used in query to filter the result along with automatedtestname"]
    #[serde(
        rename = "testCaseId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_id: Option<i32>,
    #[doc = "Number of days for which history to collect. Maximum supported value is 7 days. Default is 7 days."]
    #[serde(rename = "trendDays", default, skip_serializing_if = "Option::is_none")]
    pub trend_days: Option<i32>,
}
impl TestHistoryQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_history_query {
    use super::*;
    #[doc = "Group the result on the basis of TestResultGroupBy. This can be Branch, Environment or null(if results are fetched by BuildDefinitionId)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GroupBy {
        #[serde(rename = "branch")]
        Branch,
        #[serde(rename = "environment")]
        Environment,
    }
}
#[doc = "Represents a test iteration result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestIterationDetailsModel {
    #[doc = "Test step results in an iteration."]
    #[serde(
        rename = "actionResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_results: Vec<TestActionResultModel>,
    #[doc = "Reference to attachments in test iteration result."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attachments: Vec<TestCaseResultAttachmentModel>,
    #[doc = "Comment in test iteration result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Time when execution completed(UTC)."]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Duration of execution."]
    #[serde(
        rename = "durationInMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_in_ms: Option<f64>,
    #[doc = "Error message in test iteration result execution."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "ID of test iteration result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Test outcome if test iteration result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[doc = "Test parameters in an iteration."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<TestResultParameterModel>,
    #[doc = "Time when execution started(UTC)."]
    #[serde(
        rename = "startedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_date: Option<time::OffsetDateTime>,
    #[doc = "Url to test iteration result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestIterationDetailsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Test Log Result object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestLog {
    #[doc = "Test Log Reference object"]
    #[serde(
        rename = "logReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_reference: Option<TestLogReference>,
    #[doc = "Meta data for Log file"]
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<serde_json::Value>,
    #[doc = "LastUpdatedDate for Log file"]
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Size in Bytes for Log file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
impl TestLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestLogList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestLog>,
}
impl TestLogList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Log Reference object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestLogReference {
    #[doc = "BuildId for test log, if context is build"]
    #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<i32>,
    #[doc = "FileName for log file"]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "ReleaseEnvId for test log, if context is Release"]
    #[serde(
        rename = "releaseEnvId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_env_id: Option<i32>,
    #[doc = "ReleaseId for test log, if context is Release"]
    #[serde(rename = "releaseId", default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i32>,
    #[doc = "Resultid for test log, if context is run and log is related to result"]
    #[serde(rename = "resultId", default, skip_serializing_if = "Option::is_none")]
    pub result_id: Option<i32>,
    #[doc = "runid for test log, if context is run"]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i32>,
    #[doc = "Test Log Scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<test_log_reference::Scope>,
    #[doc = "SubResultid for test log, if context is run and log is related to subresult"]
    #[serde(
        rename = "subResultId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_result_id: Option<i32>,
    #[doc = "Log Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<test_log_reference::Type>,
}
impl TestLogReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_log_reference {
    use super::*;
    #[doc = "Test Log Scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        #[serde(rename = "run")]
        Run,
    }
    #[doc = "Log Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "generalAttachment")]
        GeneralAttachment,
    }
}
#[doc = "Attachment metadata for test attachments from LogStore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestLogStoreAttachment {
    #[doc = "Attachment type."]
    #[serde(
        rename = "attachmentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachment_type: Option<test_log_store_attachment::AttachmentType>,
    #[doc = "Comment associated with attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Attachment created date."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Attachment file name."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "Attachment size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Attachment Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestLogStoreAttachment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_log_store_attachment {
    use super::*;
    #[doc = "Attachment type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AttachmentType {
        #[serde(rename = "generalAttachment")]
        GeneralAttachment,
        #[serde(rename = "codeCoverage")]
        CodeCoverage,
        #[serde(rename = "consoleLog")]
        ConsoleLog,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestLogStoreAttachmentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestLogStoreAttachment>,
}
impl TestLogStoreAttachmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to test attachment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestLogStoreAttachmentReference {
    #[doc = "Url to download the attachment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestLogStoreAttachmentReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents Test Log store endpoint details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestLogStoreEndpointDetails {
    #[doc = "Test log store connection Uri."]
    #[serde(
        rename = "endpointSASUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_sas_uri: Option<String>,
    #[doc = "Test log store endpoint type."]
    #[serde(
        rename = "endpointType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_type: Option<test_log_store_endpoint_details::EndpointType>,
    #[doc = "Test log store status code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<test_log_store_endpoint_details::Status>,
}
impl TestLogStoreEndpointDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_log_store_endpoint_details {
    use super::*;
    #[doc = "Test log store endpoint type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointType {
        #[serde(rename = "root")]
        Root,
        #[serde(rename = "file")]
        File,
    }
    #[doc = "Test log store status code"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "fileAlreadyExists")]
        FileAlreadyExists,
        #[serde(rename = "invalidInput")]
        InvalidInput,
        #[serde(rename = "invalidFileName")]
        InvalidFileName,
        #[serde(rename = "invalidContainer")]
        InvalidContainer,
        #[serde(rename = "transferFailed")]
        TransferFailed,
        #[serde(rename = "featureDisabled")]
        FeatureDisabled,
        #[serde(rename = "buildDoesNotExist")]
        BuildDoesNotExist,
        #[serde(rename = "runDoesNotExist")]
        RunDoesNotExist,
        #[serde(rename = "containerNotCreated")]
        ContainerNotCreated,
        #[serde(rename = "apiNotSupported")]
        ApiNotSupported,
        #[serde(rename = "fileSizeExceeds")]
        FileSizeExceeds,
        #[serde(rename = "containerNotFound")]
        ContainerNotFound,
        #[serde(rename = "fileNotFound")]
        FileNotFound,
        #[serde(rename = "directoryNotFound")]
        DirectoryNotFound,
        #[serde(rename = "storageCapacityExceeded")]
        StorageCapacityExceeded,
    }
}
#[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestMessageLogDetails {
    #[doc = "Date when the resource is created"]
    #[serde(
        rename = "dateCreated",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub date_created: Option<time::OffsetDateTime>,
    #[doc = "Id of the resource"]
    #[serde(rename = "entryId", default, skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<i32>,
    #[doc = "Message of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl TestMessageLogDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestMessageLogDetailsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestMessageLogDetails>,
}
impl TestMessageLogDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestMethod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing a reference to an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestOperationReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestOperationReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test Resolution State Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResolutionState {
    #[doc = "Test Resolution state Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Test Resolution State Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ShallowReference>,
}
impl TestResolutionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultDocument {
    #[doc = "Class representing a reference to an operation."]
    #[serde(
        rename = "operationReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_reference: Option<TestOperationReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<TestResultPayload>,
}
impl TestResultDocument {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test failure type resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultFailureType {
    #[doc = "ID of the test failure type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Name of the test failure type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestResultFailureType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultFailureTypeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestResultFailureType>,
}
impl TestResultFailureTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test failure type request model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultFailureTypeRequestModel {
    #[doc = "Name of the test failure type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestResultFailureTypeRequestModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultFailuresAnalysis {
    #[serde(
        rename = "existingFailures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_failures: Option<TestFailureDetails>,
    #[serde(
        rename = "fixedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_tests: Option<TestFailureDetails>,
    #[serde(
        rename = "newFailures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_failures: Option<TestFailureDetails>,
}
impl TestResultFailuresAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultHistory {
    #[serde(
        rename = "groupByField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_field: Option<String>,
    #[serde(
        rename = "resultsForGroup",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results_for_group: Vec<TestResultHistoryDetailsForGroup>,
}
impl TestResultHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultHistoryDetailsForGroup {
    #[serde(
        rename = "groupByValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_value: Option<serde_json::Value>,
    #[doc = "Represents a test result."]
    #[serde(
        rename = "latestResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_result: Option<TestCaseResult>,
}
impl TestResultHistoryDetailsForGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of test results filtered on the basis of GroupByValue"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultHistoryForGroup {
    #[doc = "Display name of the group."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Name or Id of the group identifier by which results are grouped together."]
    #[serde(
        rename = "groupByValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_value: Option<String>,
    #[doc = "List of results for GroupByValue"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<TestCaseResult>,
}
impl TestResultHistoryForGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Meta Data of a test result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultMetaData {
    #[doc = "AutomatedTestName of test result."]
    #[serde(
        rename = "automatedTestName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_name: Option<String>,
    #[doc = "AutomatedTestStorage of test result."]
    #[serde(
        rename = "automatedTestStorage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automated_test_storage: Option<String>,
    #[doc = "List of Flaky Identifier for TestCaseReferenceId"]
    #[serde(
        rename = "flakyIdentifiers",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub flaky_identifiers: Vec<TestFlakyIdentifier>,
    #[doc = "Owner of test result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Priority of test result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "ID of TestCaseReference."]
    #[serde(
        rename = "testCaseReferenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_reference_id: Option<i32>,
    #[doc = "TestCaseTitle of test result."]
    #[serde(
        rename = "testCaseTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_case_title: Option<String>,
}
impl TestResultMetaData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultMetaDataList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestResultMetaData>,
}
impl TestResultMetaDataList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a TestResultMetaData Input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultMetaDataUpdateInput {
    #[doc = "List of Flaky Identifiers"]
    #[serde(
        rename = "flakyIdentifiers",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub flaky_identifiers: Vec<TestFlakyIdentifier>,
}
impl TestResultMetaDataUpdateInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultModelBase {
    #[doc = "Comment in result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Time when execution completed(UTC)."]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Duration of execution."]
    #[serde(
        rename = "durationInMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_in_ms: Option<f64>,
    #[doc = "Error message in result."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Test outcome of result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[doc = "Time when execution started(UTC)."]
    #[serde(
        rename = "startedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_date: Option<time::OffsetDateTime>,
}
impl TestResultModelBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test parameter information in a test iteration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultParameterModel {
    #[doc = "Test step path where parameter is referenced."]
    #[serde(
        rename = "actionPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_path: Option<String>,
    #[doc = "Iteration ID."]
    #[serde(
        rename = "iterationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iteration_id: Option<i32>,
    #[doc = "Name of parameter."]
    #[serde(
        rename = "parameterName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_name: Option<String>,
    #[doc = "This is step Id of test case. For shared step, it is step Id of shared step in test case workitem; step Id in shared step. Example: TestCase workitem has two steps: 1) Normal step with Id = 1 2) Shared Step with Id = 2. Inside shared step: a) Normal Step with Id = 1 Value for StepIdentifier for First step: \"1\" Second step: \"2;1\""]
    #[serde(
        rename = "stepIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub step_identifier: Option<String>,
    #[doc = "Url of test parameter. Deprecated in hosted environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Value of parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TestResultParameterModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}
impl TestResultPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultSummary {
    #[serde(
        rename = "aggregatedResultsAnalysis",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aggregated_results_analysis: Option<AggregatedResultsAnalysis>,
    #[serde(
        rename = "noConfigRunsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_config_runs_count: Option<i32>,
    #[serde(
        rename = "teamProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub team_project: Option<TeamProjectReference>,
    #[serde(
        rename = "testFailures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_failures: Option<TestFailuresAnalysis>,
    #[serde(
        rename = "testResultsContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_results_context: Option<TestResultsContext>,
    #[serde(
        rename = "totalRunsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_runs_count: Option<i32>,
}
impl TestResultSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultSummaryList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestResultSummary>,
}
impl TestResultSummaryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultTrendFilter {
    #[serde(
        rename = "branchNames",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_names: Vec<String>,
    #[serde(
        rename = "buildCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_count: Option<i32>,
    #[serde(
        rename = "definitionIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub definition_ids: Vec<i32>,
    #[serde(
        rename = "envDefinitionIds",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub env_definition_ids: Vec<i32>,
    #[serde(
        rename = "maxCompleteDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub max_complete_date: Option<time::OffsetDateTime>,
    #[serde(
        rename = "publishContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publish_context: Option<String>,
    #[serde(
        rename = "testRunTitles",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub test_run_titles: Vec<String>,
    #[serde(rename = "trendDays", default, skip_serializing_if = "Option::is_none")]
    pub trend_days: Option<i32>,
}
impl TestResultTrendFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsContext {
    #[doc = "Reference to a build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildReference>,
    #[serde(
        rename = "contextType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_type: Option<test_results_context::ContextType>,
    #[doc = "Pipeline reference"]
    #[serde(
        rename = "pipelineReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pipeline_reference: Option<PipelineReference>,
    #[doc = "Reference to a release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseReference>,
}
impl TestResultsContext {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_results_context {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ContextType {
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "release")]
        Release,
        #[serde(rename = "pipeline")]
        Pipeline,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsDetails {
    #[serde(
        rename = "groupByField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_field: Option<String>,
    #[serde(
        rename = "resultsForGroup",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results_for_group: Vec<TestResultsDetailsForGroup>,
}
impl TestResultsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsDetailsForGroup {
    #[serde(
        rename = "groupByValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_by_value: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<TestCaseResult>,
    #[serde(
        rename = "resultsCountByOutcome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_count_by_outcome: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl TestResultsDetailsForGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsQuery {
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
    pub results: Vec<TestCaseResult>,
    #[serde(
        rename = "resultsFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub results_filter: Option<ResultsFilter>,
}
impl TestResultsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsSettings {
    #[serde(
        rename = "flakySettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flaky_settings: Option<FlakySettings>,
    #[serde(
        rename = "newTestResultLoggingSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_test_result_logging_settings: Option<NewTestResultLoggingSettings>,
}
impl TestResultsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultsUpdateSettings {
    #[serde(
        rename = "flakySettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flaky_settings: Option<FlakySettings>,
    #[serde(
        rename = "newTestResultLoggingSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_test_result_logging_settings: Option<NewTestResultLoggingSettings>,
}
impl TestResultsUpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test run details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRun {
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<ShallowReference>,
    #[doc = "BuildConfiguration Details."]
    #[serde(
        rename = "buildConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_configuration: Option<BuildConfiguration>,
    #[doc = "Comments entered by those analyzing the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Completed date time of the run."]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Test Run Controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<String>,
    #[doc = "Test Run CreatedDate."]
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "List of Custom Fields for TestRun."]
    #[serde(
        rename = "customFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_fields: Vec<CustomTestField>,
    #[doc = "Drop Location for the test Run."]
    #[serde(
        rename = "dropLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_location: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "dtlAutEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_aut_environment: Option<ShallowReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "dtlEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_environment: Option<ShallowReference>,
    #[doc = "This is a temporary class to provide the details for the test run environment."]
    #[serde(
        rename = "dtlEnvironmentCreationDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dtl_environment_creation_details: Option<DtlEnvironmentDetails>,
    #[doc = "Due date and time for test run."]
    #[serde(
        rename = "dueDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub due_date: Option<time::OffsetDateTime>,
    #[doc = "Error message associated with the run."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "This class is used to provide the filters used for discovery"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<RunFilter>,
    #[doc = "ID of the test run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Number of Incomplete Tests."]
    #[serde(
        rename = "incompleteTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub incomplete_tests: Option<i32>,
    #[doc = "true if test run is automated, false otherwise."]
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "The iteration to which the run belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iteration: Option<String>,
    #[serde(
        rename = "lastUpdatedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_by: Option<IdentityRef>,
    #[doc = "Last updated date and time"]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "Name of the test run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Number of Not Applicable Tests."]
    #[serde(
        rename = "notApplicableTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_applicable_tests: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentityRef>,
    #[doc = "Number of passed tests in the run"]
    #[serde(
        rename = "passedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub passed_tests: Option<i32>,
    #[doc = "Phase/State for the testRun."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[doc = "Pipeline reference"]
    #[serde(
        rename = "pipelineReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pipeline_reference: Option<PipelineReference>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<ShallowReference>,
    #[doc = "Post Process State."]
    #[serde(
        rename = "postProcessState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub post_process_state: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ShallowReference>,
    #[doc = "Reference to a release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<ReleaseReference>,
    #[doc = "Release Environment Uri for TestRun."]
    #[serde(
        rename = "releaseEnvironmentUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_environment_uri: Option<String>,
    #[doc = "Release Uri for TestRun."]
    #[serde(
        rename = "releaseUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub release_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "RunSummary by outcome."]
    #[serde(
        rename = "runStatistics",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub run_statistics: Vec<RunStatistic>,
    #[doc = "Start date time of the run."]
    #[serde(
        rename = "startedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_date: Option<time::OffsetDateTime>,
    #[doc = "The state of the run. Type TestRunState Valid states - Unspecified ,NotStarted, InProgress, Completed, Waiting, Aborted, NeedsInvestigation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "TestRun Substate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substate: Option<test_run::Substate>,
    #[doc = "Tags attached with this test run."]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<TestTag>,
    #[doc = "Test environment Detail."]
    #[serde(
        rename = "testEnvironment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_environment: Option<TestEnvironment>,
    #[serde(
        rename = "testMessageLogId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_message_log_id: Option<i32>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(
        rename = "testSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_settings: Option<ShallowReference>,
    #[doc = "Total tests in the run"]
    #[serde(
        rename = "totalTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_tests: Option<i32>,
    #[doc = "Number of failed tests in the run."]
    #[serde(
        rename = "unanalyzedTests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unanalyzed_tests: Option<i32>,
    #[doc = "Url of the test run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Web Access Url for TestRun."]
    #[serde(
        rename = "webAccessUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_access_url: Option<String>,
}
impl TestRun {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_run {
    use super::*;
    #[doc = "TestRun Substate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Substate {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "creatingEnvironment")]
        CreatingEnvironment,
        #[serde(rename = "runningTests")]
        RunningTests,
        #[serde(rename = "canceledByUser")]
        CanceledByUser,
        #[serde(rename = "abortedBySystem")]
        AbortedBySystem,
        #[serde(rename = "timedOut")]
        TimedOut,
        #[serde(rename = "pendingAnalysis")]
        PendingAnalysis,
        #[serde(rename = "analyzed")]
        Analyzed,
        #[serde(rename = "cancellationInProgress")]
        CancellationInProgress,
    }
}
#[doc = "Test Run Code Coverage Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunCoverage {
    #[doc = "Last Error"]
    #[serde(rename = "lastError", default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[doc = "List of Modules Coverage"]
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub modules: Vec<ModuleCoverage>,
    #[doc = "State"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(rename = "testRun", default, skip_serializing_if = "Option::is_none")]
    pub test_run: Option<ShallowReference>,
}
impl TestRunCoverage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunCoverageList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestRunCoverage>,
}
impl TestRunCoverageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestRun>,
}
impl TestRunList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test run statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunStatistic {
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<ShallowReference>,
    #[serde(
        rename = "runStatistics",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub run_statistics: Vec<RunStatistic>,
}
impl TestRunStatistic {
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
#[doc = "Represents a sub result of a test result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSubResult {
    #[doc = "Comment in sub result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Time when test execution completed(UTC)."]
    #[serde(
        rename = "completedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub completed_date: Option<time::OffsetDateTime>,
    #[doc = "Machine where test executed."]
    #[serde(
        rename = "computerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub computer_name: Option<String>,
    #[doc = "An abstracted reference to some other resource. This class is used to provide the build data contracts with a uniform way to reference other resources in a way that provides easy traversal through links."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ShallowReference>,
    #[doc = "Additional properties of sub result."]
    #[serde(
        rename = "customFields",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_fields: Vec<CustomTestField>,
    #[doc = "Name of sub result."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Duration of test execution."]
    #[serde(
        rename = "durationInMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_in_ms: Option<i64>,
    #[doc = "Error message in sub result."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "ID of sub result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Time when result last updated(UTC)."]
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub last_updated_date: Option<time::OffsetDateTime>,
    #[doc = "Outcome of sub result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[doc = "Immediate parent ID of sub result."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    #[doc = "Hierarchy type of the result, default value of None means its leaf node."]
    #[serde(
        rename = "resultGroupType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_group_type: Option<test_sub_result::ResultGroupType>,
    #[doc = "Index number of sub result."]
    #[serde(
        rename = "sequenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence_id: Option<i32>,
    #[doc = "Stacktrace."]
    #[serde(
        rename = "stackTrace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<String>,
    #[doc = "Time when test execution started(UTC)."]
    #[serde(
        rename = "startedDate",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::date_time::rfc3339::option"
    )]
    pub started_date: Option<time::OffsetDateTime>,
    #[doc = "List of sub results inside a sub result, if ResultGroupType is not None, it holds corresponding type sub results."]
    #[serde(
        rename = "subResults",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_results: Vec<TestSubResult>,
    #[doc = "Reference to a test result."]
    #[serde(
        rename = "testResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub test_result: Option<TestCaseResultIdentifier>,
    #[doc = "Url of sub result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TestSubResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_sub_result {
    use super::*;
    #[doc = "Hierarchy type of the result, default value of None means its leaf node."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultGroupType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "rerun")]
        Rerun,
        #[serde(rename = "dataDriven")]
        DataDriven,
        #[serde(rename = "orderedTest")]
        OrderedTest,
        #[serde(rename = "generic")]
        Generic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSummaryForWorkItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<AggregatedDataForResultTrend>,
    #[doc = "WorkItem reference Details."]
    #[serde(rename = "workItem", default, skip_serializing_if = "Option::is_none")]
    pub work_item: Option<WorkItemReference>,
}
impl TestSummaryForWorkItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSummaryForWorkItemList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestSummaryForWorkItem>,
}
impl TestSummaryForWorkItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag attached to a run or result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestTag {
    #[doc = "Name of the tag, alphanumeric value less than 30 chars"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TestTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestTagList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TestTag>,
}
impl TestTagList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test tag summary for build or release grouped by test run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestTagSummary {
    #[doc = "Dictionary which contains tags associated with a test run."]
    #[serde(
        rename = "tagsGroupByTestArtifact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tags_group_by_test_artifact: Option<serde_json::Value>,
}
impl TestTagSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags to update to a run or result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestTagsUpdateModel {
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<serde_json::Value>,
}
impl TestTagsUpdateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestToWorkItemLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test: Option<TestMethod>,
    #[serde(
        rename = "workItems",
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub work_items: Vec<WorkItemReference>,
}
impl TestToWorkItemLinks {
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
#[doc = "WorkItem reference Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemReference {
    #[doc = "WorkItem Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "WorkItem Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "WorkItem Type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "WorkItem Url. Valid Values : (Bug, Task, User Story, Test Case)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "WorkItem WebUrl."]
    #[serde(rename = "webUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}
impl WorkItemReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemReferenceList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkItemReference>,
}
impl WorkItemReferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkItemToTestLinks {
    #[serde(
        rename = "executedIn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_in: Option<work_item_to_test_links::ExecutedIn>,
    #[serde(
        default,
        deserialize_with = "crate::serde::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tests: Vec<TestMethod>,
    #[doc = "WorkItem reference Details."]
    #[serde(rename = "workItem", default, skip_serializing_if = "Option::is_none")]
    pub work_item: Option<WorkItemReference>,
}
impl WorkItemToTestLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod work_item_to_test_links {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutedIn {
        #[serde(rename = "any")]
        Any,
        #[serde(rename = "tcm")]
        Tcm,
        #[serde(rename = "tfs")]
        Tfs,
    }
}
