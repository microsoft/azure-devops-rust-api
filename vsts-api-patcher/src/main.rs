// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
use anyhow::{Context, Result};
use json::JsonValue;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

const ORIGINAL_VSTS_SPECS_DIR: &str = "../vsts-rest-api-specs/";
const JSON_INDENT: u16 = 2;

// List of documentation patches.
// Any matches of the first string are replaced with the second.
const DOC_PATCHES: &[(&str, &str)] = &[
    ("file name of item returned.", "File name of item returned."),
    ("definitionId", "definition_id"),
    ("[optional] True", "(optional) Set to true"),
    ("$format", "format"),
    ("fileName", "filename"),
    ("True to", "Set to true to"),
    ("getTopPackageVersions", "get_top_package_versions"),
    ("includeAllVersions", "include_all_versions"),
    ("detailsToInclude", "details_to_include"),
    ("continuationToken", "continuation token"),
    ("dislayed", "displayed"),
    (
        "Numbe of attachments reference",
        "Number of attachment references",
    ),
    (
        "Number of attachments reference to return",
        "Number of attachment references to return",
    ),
    (
        "directory path of attachments to get",
        "Directory path of attachments to get",
    ),
    (
        "file name prefix to filter the list of attachment",
        "Filename prefix to filter the list of attachments",
    ),
    (
        "Number of test points to skip..",
        "Number of test points to skip.",
    ),
    ("[Internal]", r"\[Internal\]"),
    ("[optional]", r"\[optional\]"),
    ("[Obsolete]", r"\[Obsolete\]"),
    ("[DEPRECATED]", r"\[DEPRECATED\]"),
    ("[Readonly]", r"\[Readonly\]"),
    ("[DataMember]", r"\[DataMember\]"),
    ("[n]", r"\[n\]"),
    (
        r#"<![CDATA[ @, ~, ;, {, }, \, +, =, <, >, |, /, \\, ?, :, &, $, *, \", #, [, ] ]]>"#,
        r#"<!\[CDATA\[ @, ~, ;, {, }, \\, +, =, <, >, |, /, \\\\, ?, :, &, $, *, \", #, \[, \] \]\]>"#,
    ),
    (
        "[Provided for legacy reasons]",
        r"\[Provided for legacy reasons\]",
    ),
];

const SPEC_DESCRIPTIONS: &[(&str, &str)] = &[
    ("git.json", "Git repositories"),
    ("workItemTracking.json", "Work item tracking"),
];

struct Patcher {
    spec_path: PathBuf,
    new_definitions: BTreeMap<String, JsonValue>,
}

// Return true if the specified `entry` appears to be a an OpenAPI specification filename
fn is_spec(entry: &DirEntry) -> bool {
    let path = entry.path().to_string_lossy().to_string();
    path.ends_with(".json") && !path.contains("httpExamples")
}

// Performs preprocessing of spec text immediately after loading
fn preprocess_spec(spec_path: &Path, data: String) -> String {
    if spec_path.ends_with("workItemTracking.json") {
        // Fix up formatting of `$filter` query parameter - codegen fails with the $ prefix in the template.
        data.replace(
            "/{organization}/{project}/_apis/wit/queries?$filter={$filter}",
            "/{organization}/{project}/_apis/wit/queries?$filter={filter}",
        )
    } else {
        data
    }
}

fn main() -> Result<()> {
    let path = Path::new(ORIGINAL_VSTS_SPECS_DIR).join("specification");
    let spec_paths: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_spec)
        .map(|dir_entry| dir_entry.path().to_path_buf())
        .collect();

    println!("{:#?}", spec_paths);

    for spec_path in &spec_paths {
        let bytes = std::fs::read(spec_path)?;

        // Strip BOM (Byte Order Mark) if present.
        // https://en.wikipedia.org/wiki/Byte_order_mark
        // Spoiler alert: It is present in vsts-rest-api-specs.
        let bytes = match bytes[..] {
            [0xEF, 0xBB, 0xBF, ..] => &bytes[3..],
            _ => &bytes,
        };

        let data =
            std::str::from_utf8(bytes).context(format!("File is not valid UTF8: {spec_path:?}"))?;

        let data = preprocess_spec(spec_path, data.to_string());

        let mut json = json::parse(&data)?;
        let mut patcher = Patcher::new(spec_path);
        patcher.run(&mut json);

        let new_spec_name = &spec_path
            .to_string_lossy()
            .replace("/vsts-rest-api-specs/", "/vsts-rest-api-specs.patched/");

        let new_spec_path = Path::new(&new_spec_name);
        let new_spec_dir = new_spec_path.parent().unwrap();
        std::fs::create_dir_all(new_spec_dir)
            .context(format!("Failed to create {}", new_spec_dir.display()))?;

        let json_data = json::stringify_pretty(json, JSON_INDENT);
        let mut f = File::create(new_spec_path)?;
        f.write_all(format!("{json_data}\n").as_bytes())?;
        println!("{spec_path:#?} -> {new_spec_path:#?}");
    }

    Ok(())
}

type PatcherFn = fn(&mut Patcher, &[&str], &JsonValue) -> Option<JsonValue>;

impl Patcher {
    const PATCH_FNS: &'static [PatcherFn] = &[
        Patcher::patch_spec_descriptions,
        Patcher::patch_json_reference_links,
        Patcher::patch_teamproject_visibility_enum,
        Patcher::patch_array_array_schema,
        Patcher::patch_response_schema,
        Patcher::patch_git_reference_links,
        Patcher::patch_pipelines_reference_links,
        Patcher::patch_build_reference_links,
        Patcher::patch_pipelines_pipeline_configuration,
        Patcher::patch_pipeline,
        Patcher::patch_docs,
        Patcher::patch_git_commit_change_counts,
        Patcher::patch_git_change,
        Patcher::patch_git_pull_request_create,
        Patcher::patch_git_pull_request_update,
        Patcher::patch_ims_identity_base,
        Patcher::patch_input_validation_min_max,
        Patcher::patch_probation_retries_type,
        Patcher::patch_operation_status_in_releases,
        Patcher::patch_extension_flags,
        Patcher::patch_wit_create_update_item,
        Patcher::patch_wiki_pages_update,
        // This must be done after the other patches
        Patcher::patch_definition_required_fields,
    ];

    fn new(spec_path: &Path) -> Patcher {
        Patcher {
            spec_path: spec_path.into(),
            new_definitions: BTreeMap::new(),
        }
    }

    fn patch_json_reference_links(&mut self, key: &[&str], value: &JsonValue) -> Option<JsonValue> {
        match key {
            ["definitions", schema_name, "properties", "_links"] => {
                if *value != json::object! { "$ref": "#/definitions/ReferenceLinks" } {
                    println!("Replace _links definition for {}", schema_name);
                    Some(json::object! {
                        "description": "Links",
                        "type": "object",
                    })
                } else {
                    println!("Skip replacing _links definition for {}", schema_name);
                    None
                }
            }
            _ => None,
        }
    }

    fn add_link_definition(&mut self) {
        // Add `Link` definition
        self.new_definitions.insert(
            "Link".to_string(),
            json::object! {
                "description": "Link URL",
                "type": "object",
                "required": [ "href" ],
                "properties": {
                    "href": {
                        "type": "string"
                    }
                }
            },
        );
    }

    fn patch_pipeline(&mut self, key: &[&str], _value: &JsonValue) -> Option<JsonValue> {
        // Only applies to pipelines specs
        if !self.spec_path.ends_with("pipelines.json") {
            return None;
        }
        match key {
            ["definitions", "Pipeline"] => {
                println!("Replace Pipeline definition");
                Some(json::object! {
                    "description": "Definition of a pipeline.",
                    "type": "object",
                    "properties": {
                        "_links": {
                            "$ref": "#/definitions/ReferenceLinks"
                        },
                        "configuration": {
                            "$ref": "#/definitions/PipelineConfiguration"
                        },
                        "url": {
                            "description": "URL of the pipeline",
                            "type": "string"
                        },
                        "folder": {
                            "description": "Pipeline folder",
                            "type": "string"
                        },
                        "id": {
                            "description": "Pipeline ID",
                            "type": "integer",
                            "format": "int32"
                        },
                        "name": {
                            "description": "Pipeline name",
                            "type": "string"
                        },
                        "revision": {
                            "description": "Revision number",
                            "type": "integer",
                            "format": "int32"
                        }
                    },
                    "required": [
                        "_links",
                        "url",
                        "folder",
                        "id",
                        "name",
                        "revision"
                    ]
                })
            }
            _ => None,
        }
    }

    fn patch_spec_descriptions(&mut self, key: &[&str], _value: &JsonValue) -> Option<JsonValue> {
        for (filename, desc) in SPEC_DESCRIPTIONS {
            if !self.spec_path.ends_with(filename) {
                continue;
            }

            #[allow(clippy::single_match)]
            match key {
                ["info", "description"] => {
                    println!("Patching spec description: {}: {}", filename, desc);
                    return Some(JsonValue::from(*desc));
                }
                _ => (),
            }
        }
        None
    }

    fn patch_ims_identity_base(&mut self, key: &[&str], _value: &JsonValue) -> Option<JsonValue> {
        // Only applies to ims (identities.json) specs
        if !self.spec_path.ends_with("identities.json") {
            return None;
        }
        match key {
            ["definitions", "IdentityBase", "properties", "descriptor"] => {
                println!("Patching ims IdentityBase descriptor");
                Some(json::object! {
                    "description": "Identity descriptor",
                    "type": "string"
                })
            }
            _ => None,
        }
    }

    fn patch_pipelines_pipeline_configuration(
        &mut self,
        key: &[&str],
        _value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to pipelines specs
        if !self.spec_path.ends_with("pipelines.json") {
            return None;
        }
        match key {
            ["definitions", "PipelineConfiguration", "properties"] => {
                println!("Modify PipelineConfiguration definition");
                Some(json::object! {
                    "type": {
                        "enum": [
                            "unknown",
                            "yaml",
                            "designerJson",
                            "justInTime",
                            "designerHyphenJson"
                        ],
                        "x-ms-enum": {
                            "name": "ConfigurationType",
                            "values": [
                                {
                                "value": "unknown",
                                "description": "Unknown type."
                                },
                                {
                                "value": "yaml",
                                "description": "YAML."
                                },
                                {
                                "value": "designerJson",
                                "description": "Designer JSON."
                                },
                                {
                                "value": "justInTime",
                                "description": "Just-in-time."
                                },
                                {
                                "value": "designerHyphenJson",
                                "description": "Designer-JSON."
                                }
                            ]
                        }
                    },
                    "path": {
                        "type": "string"
                    },
                    "repository": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "string"
                            },
                            "type": {
                                "type": "string"
                            }
                        },
                        "required": ["id", "type"]
                    }
                })
            }
            _ => None,
        }
    }

    fn patch_operation_status_in_releases(
        &mut self,
        key: &[&str],
        _value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to release specs
        if !self.spec_path.ends_with("release.json") {
            return None;
        }
        match key {
            ["definitions", "Deployment", "properties", "operationStatus"] => {
                Some(self.patched_operation_status("Gets operation status of deployment."))
            }
            ["definitions", "DeploymentQueryParameters", "properties", "operationStatus"] => {
                Some(self.patched_operation_status(
                    "Query deployment based on deployment operation status.",
                ))
            }
            ["definitions", "DeploymentAttempt", "properties", "operationStatus"] => {
                Some(self.patched_operation_status("Deployment operation status."))
            }

            _ => None,
        }
    }

    fn patched_operation_status(&self, description: &str) -> JsonValue {
        json::object! {
            "description": description,
            "enum": [
                "Undefined",
                "Queued",
                "Scheduled",
                "Pending",
                "Approved",
                "Rejected",
                "Deferred",
                "QueuedForAgent",
                "PhaseInProgress",
                "PhaseSucceeded",
                "PhasePartiallySucceeded",
                "PhaseFailed",
                "Canceled",
                "PhaseCanceled",
                "ManualInterventionPending",
                "QueuedForPipeline",
                "Cancelling",
                "EvaluatingGates",
                "GateFailed",
                "All"
              ],
              "x-ms-enum": {
                "name": "DeploymentOperationStatus",
                "values": [
                  {
                    "value": "Undefined",
                    "description": "The deployment operation status is undefined."
                  },
                  {
                    "value": "Queued",
                    "description": "The deployment operation status is queued."
                  },
                  {
                    "value": "Scheduled",
                    "description": "The deployment operation status is scheduled."
                  },
                  {
                    "value": "Pending",
                    "description": "The deployment operation status is pending."
                  },
                  {
                    "value": "Approved",
                    "description": "The deployment operation status is approved."
                  },
                  {
                    "value": "Rejected",
                    "description": "The deployment operation status is rejected."
                  },
                  {
                    "value": "Deferred",
                    "description": "The deployment operation status is deferred."
                  },
                  {
                    "value": "QueuedForAgent",
                    "description": "The deployment operation status is queued for agent."
                  },
                  {
                    "value": "PhaseInProgress",
                    "description": "The deployment operation status is phase in progress."
                  },
                  {
                    "value": "PhaseSucceeded",
                    "description": "The deployment operation status is phase succeeded."
                  },
                  {
                    "value": "PhasePartiallySucceeded",
                    "description": "The deployment operation status is phase partially succeeded."
                  },
                  {
                    "value": "PhaseFailed",
                    "description": "The deployment operation status is phase failed."
                  },
                  {
                    "value": "Canceled",
                    "description": "The deployment operation status is canceled."
                  },
                  {
                    "value": "PhaseCanceled",
                    "description": "The deployment operation status is phase canceled."
                  },
                  {
                    "value": "ManualInterventionPending",
                    "description": "The deployment operation status is manualintervention pending."
                  },
                  {
                    "value": "QueuedForPipeline",
                    "description": "The deployment operation status is queued for pipeline."
                  },
                  {
                    "value": "Cancelling",
                    "description": "The deployment operation status is cancelling."
                  },
                  {
                    "value": "EvaluatingGates",
                    "description": "The deployment operation status is EvaluatingGates."
                  },
                  {
                    "value": "GateFailed",
                    "description": "The deployment operation status is GateFailed."
                  },
                  {
                    "value": "All",
                    "description": "The deployment operation status is all."
                  }
                ]
              }
        }
    }

    fn patch_pipelines_reference_links(
        &mut self,
        key: &[&str],
        _value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to pipelines specs
        if !self.spec_path.ends_with("pipelines.json") {
            return None;
        }
        match key {
            ["definitions", "ReferenceLinks", "properties"] => {
                println!("Replace pipelines ReferenceLinks definition");
                self.add_link_definition();

                // Add all links that we are aware of.
                Some(json::object! {
                     "self": {
                       "$ref": "#/definitions/Link"
                     },
                     "web": {
                       "$ref": "#/definitions/Link"
                     }
                })
            }
            _ => None,
        }
    }

    fn patch_git_commit_change_counts(
        &mut self,
        key: &[&str],
        _value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to git specs
        if !self.spec_path.ends_with("git.json") {
            return None;
        }
        match key {
            ["definitions", "GitCommitChanges" | "GitCommitRef", "properties", "changeCounts"] => {
                println!("Replace git GitCommitChanges/GitCommitRef changeCounts definition");

                Some(json::object! {
                    "type": "object",
                    "additionalProperties": {
                      "type": "integer",
                      "format": "int32"
                    }
                })
            }
            _ => None,
        }
    }

    fn patch_git_change(&mut self, key: &[&str], value: &JsonValue) -> Option<JsonValue> {
        // Only applies to git specs
        if !self.spec_path.ends_with("git.json") {
            return None;
        }
        match key {
            ["definitions", "GitChange", "properties"] => {
                println!("Remove unused git GitChange properties");
                // Remove properties that never seem to be used
                let mut value = value.clone();
                value.remove("changeId");
                value.remove("newContentTemplate");
                value.remove("originalPath");
                Some(value)
            }
            ["definitions", "Change", "properties"] => {
                println!("Remove unused git Change properties");
                // Remove properties that never seem to be used
                let mut value = value.clone();
                value.remove("newContent");
                value.remove("sourceServerItem");
                value.remove("url");
                Some(value)
            }
            ["definitions", "Change", "properties", "item"] => {
                println!("Replace git Change item definition");

                Some(json::object! {
                    "type": "object",
                    "additionalProperties": {
                      "type": "integer",
                      "format": "int32"
                    }
                })
            }
            _ => None,
        }
    }

    fn patch_git_reference_links(&mut self, key: &[&str], _value: &JsonValue) -> Option<JsonValue> {
        // Only applies to git specs
        if !self.spec_path.ends_with("git.json") {
            return None;
        }
        match key {
            ["definitions", "ReferenceLinks", "properties"] => {
                println!("Replace git ReferenceLinks definition");
                self.add_link_definition();

                // Add all links that we are aware of.
                Some(json::object! {
                   "commits": {
                       "$ref": "#/definitions/Link"
                     },
                     "items": {
                       "$ref": "#/definitions/Link"
                     },
                     "project": {
                       "$ref": "#/definitions/Link"
                     },
                     "pullRequests": {
                       "$ref": "#/definitions/Link"
                     },
                     "pushes": {
                       "$ref": "#/definitions/Link"
                     },
                     "refs": {
                       "$ref": "#/definitions/Link"
                     },
                     "self": {
                       "$ref": "#/definitions/Link"
                     },
                     "ssh": {
                       "$ref": "#/definitions/Link"
                     },
                     "web": {
                       "$ref": "#/definitions/Link"
                     }
                })
            }
            _ => None,
        }
    }

    fn patch_git_pull_request_create(
        &mut self,
        key: &[&str],
        _value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to git specs
        if !self.spec_path.ends_with("git.json") {
            return None;
        }
        println!("PR: {:?}", key);
        match key {
            ["paths", "/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullrequests", "post", "parameters"] =>
            {
                println!("Replace git create Pull Request parameters");
                self.new_definitions.insert(
                    "GitPullRequestCreateOptions".to_string(),
                    json::object! {
                        "description": "Pull Request create options",
                        "type": "object",
                        "required": [ "sourceRefName", "targetRefName", "title" ],
                        "properties": {
                              "description": {
                                "description": "The description of the pull request.",
                                "type": "string"
                              },
                              "isDraft": {
                                "description": "Draft / WIP pull request.",
                                "type": "boolean"
                              },
                              "labels": {
                                "description": "The labels associated with the pull request.",
                                "type": "array",
                                "items": {
                                  "$ref": "#/definitions/WebApiCreateTagRequestData"
                                }
                              },
                              "sourceRefName": {
                                "description": "The name of the source branch of the pull request.",
                                "type": "string"
                              },
                              "targetRefName": {
                                "description": "The name of the target branch of the pull request.",
                                "type": "string"
                              },
                              "title": {
                                "description": "The title of the pull request.",
                                "type": "string"
                              },
                              "mergeOptions": {
                                "description": "Options used when the pull request merge runs. These are separate from completion options since completion happens only once and a new merge will run every time the source branch of the pull request changes.",
                                "$ref": "#/definitions/GitPullRequestMergeOptions"
                              },
                              "completionOptions": {
                                "description": "Options which affect how the pull request will be merged when it is completed.",
                                "$ref": "#/definitions/GitPullRequestCompletionOptions"
                              },
                              "workItemRefs": {
                                "description": "Any work item references associated with this pull request.",
                                "type": "array",
                                "items": {
                                  "$ref": "#/definitions/ResourceRef"
                                }
                              },
                              "reviewers": {
                                "description": "A list of reviewers on the pull request.",
                                "type": "array",
                                "items": {
                                  "$ref": "#/definitions/IdentityId"
                                }
                              }
                            }
                        }
                    );

                self.new_definitions.insert(
                    "IdentityId".to_string(),
                    json::object! {
                        "description": "Identity id",
                        "type": "object",
                        "required": [ "id" ],
                        "properties": {
                              "id": {
                                "description": "The user identity",
                                "type": "string"
                              }
                        }
                    },
                );

                Some(json::array![
                    {
                        "in": "path",
                        "name": "organization",
                        "description": "The name of the Azure DevOps organization.",
                        "required": true,
                        "type": "string"
                    },
                    {
                        "in": "path",
                        "name": "repositoryId",
                        "description": "The repository ID of the pull request's target branch.",
                        "required": true,
                        "type": "string"
                    },
                    {
                        "in": "path",
                        "name": "project",
                        "description": "Project ID or project name",
                        "required": true,
                        "x-ms-required": false,
                        "type": "string"
                    },
                    {
                        "in": "body",
                        "name": "createOptions",
                        "description": "The pull request to create.",
                        "required": true,
                        "schema": {
                        "$ref": "#/definitions/GitPullRequestCreateOptions"
                        }
                    },
                    {
                        "in": "query",
                        "name": "supportsIterations",
                        "description": "If true, subsequent pushes to the pull request will be individually reviewable. Set this to false for large pull requests for performance reasons if this functionality is not needed.",
                        "required": false,
                        "type": "boolean"
                    },
                    {
                        "$ref": "#/parameters/api-Version-preview.1"
                    }
                ])
            }
            // The spec says that the response code is 200, but the server actually returns 201
            ["paths", "/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullrequests", "post", "responses"] => {
                Some(json::object! {
                    "201": {
                        "description": "successful operation",
                        "schema": {
                        "$ref": "#/definitions/GitPullRequest"
                        }
                    }
                })
            }
            _ => None,
        }
    }

    fn patch_git_pull_request_update(
        &mut self,
        key: &[&str],
        _value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to git specs
        if !self.spec_path.ends_with("git.json") {
            return None;
        }
        println!("PR: {:?}", key);
        match key {
            ["paths", "/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullrequests/{pullRequestId}", "patch", "parameters"] =>
            {
                println!("Replace git update Pull Request parameters");
                self.new_definitions.insert(
                    "GitPullRequestUpdateOptions".to_string(),
                    json::object! {
                    "description": "Pull Request update options",
                    "type": "object",
                    "properties": {
                          "description": {
                            "description": "The description of the pull request.",
                            "type": "string"
                          },
                          "title": {
                            "description": "The title of the pull request.",
                            "type": "string"
                          },
                          "reviewers": {
                            "description": "A list of reviewers on the pull request.",
                            "type": "array",
                            "items": {
                              "$ref": "#/definitions/IdentityId"
                            }
                          },
                          "mergeStatus": {
                            "description": "The current status of the pull request merge.",
                                "$ref": "#/definitions/GitPullRequestMergeOptions"
                          },
                          "status": {
                            "description": "The status of the pull request.",
                                "$ref": "#/definitions/PullRequestStatus",
                          }
                        }
                    },
                );

                self.new_definitions.insert(
                    "PullRequestStatus".to_string(),
                    json::object! {
                            "description": "Pull request status",
                                "enum": [
                                    "notSet",
                                    "active",
                                    "abandoned",
                                    "completed",
                                  ],
                                  "x-ms-enum": {
                                    "name": "PullRequestStatus",
                                    "values": [
                                      {
                                        "value": "notSet",
                                        "description": "Status not set. Default state."
                                      },
                                      {
                                        "value": "active",
                                        "description": "Pull request is active."
                                      },
                                      {
                                        "value": "abandoned",
                                        "description": "Pull request is abandoned."
                                      },
                                      {
                                        "value": "completed",
                                        "description": "Pull request is completed."
                                      }
                                    ]

                        },
                    },
                );

                Some(json::array![
                    {
                        "in": "path",
                        "name": "organization",
                        "description": "The name of the Azure DevOps organization.",
                        "required": true,
                        "type": "string"
                    },
                    {
                        "in": "path",
                        "name": "repositoryId",
                        "description": "The repository ID of the pull request's target branch.",
                        "required": true,
                        "type": "string"
                    },
                    {
                        "in": "path",
                        "name": "project",
                        "description": "Project ID or project name",
                        "required": true,
                        "x-ms-required": false,
                        "type": "string"
                    },
                    {
                        "in": "path",
                        "name": "pullRequestId",
                        "description": "The ID of the pull request to retrieve.",
                        "required": true,
                        "type": "integer",
                        "format": "int32"
                    },
                    {
                        "in": "query",
                        "name": "includeCommits",
                        "description": "If true, the pull request will be returned with the associated commits.",
                        "required": false,
                        "type": "boolean"
                    },
                    {
                        "in": "body",
                        "name": "updateOptions",
                        "description": "The pull request content to update.",
                        "required": true,
                        "schema": {
                        "$ref": "#/definitions/GitPullRequestUpdateOptions"
                        }
                    },
                    {
                        "in": "query",
                        "name": "includeWorkItemRefs",
                        "description": "If true, the pull request will be returned with the associated work item references.",
                        "required": false,
                        "type": "boolean"
                    },
                    {
                        "$ref": "#/parameters/api-Version-preview.1"
                    }
                ])
            }
            _ => None,
        }
    }

    fn patch_build_reference_links(
        &mut self,
        key: &[&str],
        _value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to build specs
        if !self.spec_path.ends_with("build.json") {
            return None;
        }
        match key {
            ["definitions", "ReferenceLinks", "properties"] => {
                println!("Replace build ReferenceLinks definition");
                self.add_link_definition();

                // Add all links that we are aware of.
                Some(json::object! {
                   "badge": {
                       "$ref": "#/definitions/Link"
                     },
                     "self": {
                       "$ref": "#/definitions/Link"
                     },
                     "sourceVersionDisplayUri": {
                       "$ref": "#/definitions/Link"
                     },
                     "timeline": {
                       "$ref": "#/definitions/Link"
                     },
                     "web": {
                       "$ref": "#/definitions/Link"
                     },
                })
            }
            _ => None,
        }
    }

    fn patch_input_validation_min_max(
        &mut self,
        key: &[&str],
        value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to hooks specs
        if !self.spec_path.ends_with("serviceHooks.json") {
            return None;
        }
        match key {
            ["definitions", "InputValidation", "properties", "minValue" | "maxValue"] => {
                let mut value = value.clone();
                value["type"] = JsonValue::from("number");
                value["format"] = JsonValue::from("float");
                Some(value)
            }
            _ => None,
        }
    }

    // extensionManagement declares several `flags` properties as enums, but the values
    // returned by the server are a comma-separated list of values, e.g. "builtIn, multiVersion, trusted"
    // This is not easy to handle with serde, so we patch these definitions to be strings.
    fn patch_extension_flags(&mut self, key: &[&str], value: &JsonValue) -> Option<JsonValue> {
        // Only applies to extensionManagement specs
        if !self.spec_path.ends_with("extensionManagement.json") {
            return None;
        }
        match key {
            ["definitions", _, "properties", "flags"] => {
                println!("Replace extensionManagement flags definition");
                Some(json::object! {
                    "description": value["description"].as_str().unwrap_or("").to_string(),
                    "type": "string",
                })
            }
            _ => None,
        }
    }

    fn patch_probation_retries_type(
        &mut self,
        key: &[&str],
        value: &JsonValue,
    ) -> Option<JsonValue> {
        // Only applies to hooks specs
        if !self.spec_path.ends_with("serviceHooks.json") {
            return None;
        }
        match key {
            ["definitions", "Subscription", "properties", "probationRetries"] => {
                let mut value = value.clone();
                value["type"] = JsonValue::from("integer");
                value["format"] = JsonValue::from("int32");
                Some(value)
            }
            _ => None,
        }
    }

    fn patch_definition_required_fields(
        &mut self,
        key: &[&str],
        value: &JsonValue,
    ) -> Option<JsonValue> {
        let patches = [
            (
                "serviceEndpoint.json",
                "ServiceEndpoint",
                // Excluded
                //   administratorsGroup
                //   operationStatus
                r#"[
                    "authorization",
                    "createdBy",
                    "data",
                    "description",
                    "id",
                    "isReady",
                    "isShared",
                    "name",
                    "owner",
                    "type",
                    "url"
                ]"#,
            ),
            (
                "serviceEndpoint.json",
                "ServiceEndpointProjectReference",
                r#"[
                    "description",
                    "name",
                    "projectReference"
                ]"#,
            ),
            (
                "serviceEndpoint.json",
                "ProjectReference",
                r#"[
                    "id",
                    "name"
                ]"#,
            ),
            // (
            //     "serviceEndpoint.json",
            //     "GraphSubjectBase",
            //     r#"[
            //         "links",
            //         "descriptor",
            //         "displayName",
            //         "url"
            //     ]"#,
            // ),
            (
                "serviceEndpoint.json",
                "IdentityRef",
                r#"[
                    "id"
                ]"#,
            ),
            (
                "pipelines.json",
                "PipelineBase",
                r#"[
                    "folder",
                    "id",
                    "name",
                    "revision"
                ]"#,
            ),
            (
                "pipelines.json",
                "PipelineConfiguration",
                r#"[
                    "type",
                    "path",
                    "repository"
                ]"#,
            ),
            (
                "pipelines.json",
                "Run",
                r#"[
                    "_links",
                    "createdDate",
                    "finishedDate",
                    "pipeline",
                    "result",
                    "state",
                    "url"
                ]"#,
            ),
            (
                "pipelines.json",
                "RunReference",
                r#"[
                    "id",
                    "name"
                ]"#,
            ),
            (
                "git.json",
                "GitRepository",
                // Excluded
                //   _links
                //   defaultBranch
                //   isDisabled
                //   remoteUrl
                //   size
                //   sshUrl
                //   webUrl
                r#"[
                    "id",
                    "name",
                    "project",
                    "url"
                ]"#,
            ),
            (
                "git.json",
                "GitPullRequest",
                // Excluded
                //   _links
                //   artifactId
                //   autoCompleteSetBy
                //   closedBy
                //   closedDate
                //   completionOptions
                //   completionQueueTime
                //   forkSource
                //   hasMultipleMergeBases
                //   mergeFailureMessage
                //   mergeFailureType
                //   mergeOptions
                //   remoteUrl
                r#"[
                    "createdBy",
                    "creationDate",
                    "isDraft",
                    "pullRequestId",
                    "repository",
                    "status",
                    "sourceRefName",
                    "targetRefName",
                    "url"
                ]"#,
            ),
            (
                "git.json",
                "GitRef",
                r#"[
                    "name",
                    "objectId"
                ]"#,
            ),
            (
                "git.json",
                "Change",
                r#"[
                    "changeType",
                    "item"
                ]"#,
            ),
            (
                "git.json",
                "GitCommitRef",
                r#"[
                    "commitId",
                    "url"
                ]"#,
            ),
            (
                "git.json",
                "IdentityRef",
                r#"[
                    "id"
                ]"#,
            ),
            (
                "git.json",
                "GitCommitRef",
                r#"[
                    "commitId"
                ]"#,
            ),
            (
                "git.json",
                "WebApiCreateTagRequestData",
                r#"[
                    "name"
                ]"#,
            ),
            (
                "*",
                "TeamProjectReference",
                // Excluded
                //   description
                //   revision
                //   url
                r#"[
                    "id",
                    "lastUpdateTime",
                    "name",
                    "state",
                    "visibility"
                ]"#,
            ),
            (
                "build.json",
                "Build",
                // Excluded
                //   agentSpecification
                //   buildNumber
                //   buildNumberRevision
                //   controller
                //   deleted
                //   deletedBy
                //   deletedDate
                //   deletedReason
                //   demands
                //   finishTime
                //   lastChangedBy
                //   lastChangedDate
                //   logs
                //   orchestrationPlan
                //   parameters
                //   plans
                //   properties
                //   quality
                //   queue
                //   queueOptions
                //   queuePosition
                //   queueTime
                //   repository
                //   requestedBy
                //   requestedFor
                //   result
                //   retainedByRelease
                //   sourceBranch
                //   sourceVersion
                //   startTime
                //   status
                //   tags
                //   templateParameters
                //   triggeredByBuild
                //   triggerInfo
                //   _links
                //   uri
                //   url
                //   validationResults
                r#"[
                    "definition",
                    "id",
                    "priority",
                    "project",
                    "reason"
                ]"#,
            ),
            (
                "build.json",
                "DefinitionReference",
                // Excluded
                //   createdDate
                //   name
                //   path
                //   type
                //   uri
                r#"[
                    "id",
                    "project",
                    "queueStatus",
                    "revision",
                    "url"
                ]"#,
            ),
            (
                "build.json",
                "BuildLogReference",
                r#"[
                    "id",
                    "type",
                    "url"
                ]"#,
            ),
            (
                "build.json",
                "AgentPoolQueue",
                // Excluded
                //   _links
                //   url
                r#"[
                    "id",
                    "pool",
                    "name"
                ]"#,
            ),
            (
                "build.json",
                "TaskAgentPoolReference",
                // Excluded
                //   isHosted
                r#"[
                    "id",
                    "name"
                ]"#,
            ),
            (
                "build.json",
                "TaskOrchestrationPlanReference",
                // Excluded
                //   orchestrationType
                r#"[
                    "planId"
                ]"#,
            ),
            (
                "build.json",
                "BuildRepository",
                // Excluded
                //   clean
                //   checkoutSubmodules
                //   defaultBranch
                //   properties
                //   rootFolder
                //   name
                //   url
                //   type
                r#"[
                    "id"
                ]"#,
            ),
            // (
            //     "*",
            //     "GraphSubjectBase",
            //     // Excluded
            //     //   _links
            //     //   "descriptor
            //     //   displayName
            //     //   url
            //     r#"[
            //     ]"#,
            // ),
            (
                "*",
                "IdentityRef",
                r#"[
                    "id",
                    "uniqueName"
                ]"#,
            ),
            (
                "workItemTracking.json",
                "Link",
                r#"[
                    "attributes",
                    "rel",
                    "url"
                ]"#,
            ),
            (
                "workItemTracking.json",
                "WorkItemTrackingResourceReference",
                r#"[
                    "url"
                ]"#,
            ),
            (
                "workItemTracking.json",
                "WorkItemTrackingResource",
                r#"[
                    "_links"
                ]"#,
            ),
            (
                "workItemTracking.json",
                "WorkItem",
                r#"[
                    "id",
                    "fields"
                ]"#,
            ),
            (
                "status.json",
                "ServiceHealth",
                r#"[
                    "id"
                ]"#,
            ),
            (
                "status.json",
                "GeographyWithHealth",
                r#"[
                    "health"
                ]"#,
            ),
            (
                "status.json",
                "Geography",
                r#"[
                    "id",
                    "name"
                ]"#,
            ),
            (
                "status.json",
                "Status",
                r#"[
                    "status"
                ]"#,
            ),
            (
                "status.json",
                "StatusSummary",
                r#"[
                    "health",
                    "message"
                ]"#,
            ),
            (
                "policy.json",
                "PolicyType",
                r#"[
                    "_links",
                    "description"
                ]"#,
            ),
            (
                "policy.json",
                "PolicyTypeRef",
                r#"[
                    "displayName",
                    "id",
                    "url"
                ]"#,
            ),
        ];

        match key {
            ["definitions", def] => {
                for (filename, def_name, required_fields) in patches.iter() {
                    if (*filename == "*" || self.spec_path.ends_with(filename)) && (def == def_name)
                    {
                        println!("Add required values to {} definition", def_name);
                        let mut value = value.to_owned();
                        value["required"] = json::parse(required_fields).unwrap();
                        return Some(value);
                    }
                }
                None
            }
            _ => None,
        }
    }

    // Add missing "organization" and "unchanged" values to the TeamProjectReference `visibility` enum
    //
    // Before patch:
    // enum: [
    //    "private",
    //    "public"
    // ]
    //
    // After patch:
    // enum: [
    //    "private",
    //    "public",
    //    "organization",
    //    "unchanged"
    // ]
    fn patch_teamproject_visibility_enum(
        &mut self,
        key: &[&str],
        value: &JsonValue,
    ) -> Option<JsonValue> {
        match key {
            ["definitions", "TeamProjectReference", "properties", "visibility", "enum"] => {
                if value.len() == 2 {
                    println!("Add 'organization' and 'unchanged' to TeamProjectReference visibility enum");
                    Some(json::array![
                        "private",
                        "public",
                        "organization",
                        "unchanged"
                    ])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Replace this invalid array of arrays with no item type declaration:
    //   "schema": {
    //     "type": "array",
    //     "items": {
    //       "type": "array"
    //     }
    //   }
    //
    // With:
    //   "schema": {
    //     "type": "array",
    //     "items": {
    //       "type": "string"
    //     }
    //   }
    fn patch_array_array_schema(&mut self, key: &[&str], value: &JsonValue) -> Option<JsonValue> {
        match key {
            ["paths" | "x-ms-paths", _path, _op, "responses", _rsp_code, "schema"] => {
                if (value["items"]["type"] == "array")
                    && (value["items"]["items"] == JsonValue::Null)
                {
                    println!("Replace array[array] with array[string]");
                    Some(json::object! {
                        "type": "array",
                        "items": {
                            "type": "string"
                        }
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // The spec defines many return values as arrays of a type, when in fact the API returns
    // the array wrapped in an object with `count` and `value` fields (where the value field
    // contains the array).
    //
    // This patch replaces the array definition with the wrapped version, and adds a new
    // definition called `<type>List` for the wrapped type.
    fn patch_response_schema(&mut self, key: &[&str], value: &JsonValue) -> Option<JsonValue> {
        match key {
            ["paths" | "x-ms-paths", _path, _op, "responses", _rsp_code, "schema"] => {
                if value["type"].as_str() == Some("array") {
                    if let Some(qualified_def) = value["items"]["$ref"].as_str() {
                        // The definition reference looks like this: `#/definitions/PipelineBase`.
                        // We want to extract the name segment which is after the last `/`.
                        let def_name = qualified_def.rsplit('/').next().unwrap();
                        let def_list = format!("{def_name}List");
                        println!("Convert array of {def_name} to {def_list}");

                        self.new_definitions.insert(
                            def_list.clone(),
                            json::object! {
                                "description": "",
                                "type": "object",
                                "properties": {
                                    "count": {
                                        "type": "integer",
                                        "format": "int32"
                                    },
                                    "value": {
                                        "type": "array",
                                        "items": {
                                            "$ref": qualified_def
                                        }
                                    }
                                }
                            },
                        );

                        Some(json::object! {
                            "$ref": format!("#/definitions/{def_list}")
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Applies all patches from DOC_PATCHES to supplied string.
    fn patch_docstring(&self, s: &str) -> String {
        let mut patched_s: String = s.to_string();
        for r in DOC_PATCHES {
            patched_s = patched_s.replace(r.0, r.1);
        }
        // Markup URL references
        let regex = regex::Regex::new(r"\s(?P<url>(http|https)://[_A-Za-z0-9/]+)").unwrap();
        let patched_s = regex.replace_all(&patched_s, "<${url}>");
        if patched_s != s {
            format!("[patched]{}", patched_s);
            println!("patch_docstring: {} => {}", s, patched_s);
        }
        patched_s.to_string()
    }

    // Patch documentation
    //
    // Does simple text replacement on description and summary fields.
    fn patch_docs(&mut self, key: &[&str], value: &JsonValue) -> Option<JsonValue> {
        match key {
            [.., "parameters"] => {
                // Parameters is an array of `JsonValue`s, each of which may have a `description` field
                let mut value = value.clone();
                for param in value.members_mut() {
                    if let Some(s) = param["description"].as_str() {
                        param["description"] = self.patch_docstring(s).into()
                    }
                }
                Some(value)
            }
            [.., "description" | "summary"] => {
                value.as_str().map(|v| self.patch_docstring(v).into())
            }
            _ => None,
        }
    }
    /// Patch the WorkItemTracking schema for creating or updating Work Items
    ///
    /// Creating or updating a work item can invole multiple operations in one go, e.g creating
    /// an item but also assigning a parent/iteration to it. This patch allows a Vector of operations
    /// to be supplied in a single API call so that multiple fields can be assigned
    fn patch_wit_create_update_item(
        &mut self,
        key: &[&str],
        value: &JsonValue,
    ) -> Option<JsonValue> {
        if !self.spec_path.ends_with("workItemTracking.json") {
            return None;
        }
        match key {
            ["paths", "/{organization}/{project}/_apis/wit/workitems/${type}", "post", "parameters"] =>
            {
                // Parameters is an array of `JsonValue`s, each of which has a `name` field
                let mut value = value.clone();
                for param in value.members_mut() {
                    if let Some(s) = param["name"].as_str() {
                        // Find the body so we can correct its fields
                        if s == "body" {
                            // Remove the schema as we're placing it with an array of items
                            let _ = param.remove("schema");
                            // Add a type field to the parameter
                            param.insert("type", "array").unwrap();
                            // Add an items field referring to a definition
                            param
                                .insert(
                                    "items",
                                    json::object! {
                                        "$ref": "#/definitions/JsonPatchOperation"
                                    },
                                )
                                .unwrap();
                            // Update the description to reflect the definition
                            param["description"] = JsonValue::from(
                                "A list of operations to perform when creating a Work Item",
                            );
                        }
                    }
                }
                Some(value)
            }
            ["paths", "/{organization}/{project}/_apis/wit/workitems/{id}", "patch", "parameters"] =>
            {
                // Parameters is an array of `JsonValue`s, each of which has a `name` field
                let mut value = value.clone();
                for param in value.members_mut() {
                    if let Some(s) = param["name"].as_str() {
                        // Find the body so we can correct its fields
                        if s == "body" {
                            // Remove the schema as we're placing it with an array of items
                            let _ = param.remove("schema");
                            // Add a type field to the parameter
                            param.insert("type", "array").unwrap();
                            // Add an items field referring to a definition
                            param
                                .insert(
                                    "items",
                                    json::object! {
                                        "$ref": "#/definitions/JsonPatchOperation"
                                    },
                                )
                                .unwrap();
                            // Update the description to reflect the definition
                            param["description"] = JsonValue::from(
                                "A list of operations to perform when updating a Work Item",
                            );
                        }
                    }
                }
                Some(value)
            }
            _ => None,
        }
    }
    /// Patch Wiki Pages
    ///
    /// To update a Wiki Page an `If-Match` header must be supplied with an `eTag` (page version)
    /// value. By default this header is generated with the name `Version`. This replaces the name
    /// `Version` with `If-Match`.
    fn patch_wiki_pages_update(&mut self, key: &[&str], value: &JsonValue) -> Option<JsonValue> {
        // Only applies to wiki specs
        if !self.spec_path.ends_with("wiki.json") {
            return None;
        }
        match key {
            ["paths", "/{organization}/{project}/_apis/wiki/wikis/{wikiIdentifier}/pages", "put", "parameters"] =>
            {
                // Parameters is an array of `JsonValue`s, each of which has a `name` field
                let mut value = value.clone();
                for param in value.members_mut() {
                    if let Some(s) = param["name"].as_str() {
                        // Only update the field which exactly matches "Version"
                        if s == "Version" {
                            println!("Replacing header `Version` with `If-Match`");
                            param["name"] = "If-Match".to_string().into();
                        }
                    }
                }
                Some(value)
            }
            ["paths", "/{organization}/{project}/_apis/wiki/wikis/{wikiIdentifier}/pages/{id}", "patch", "parameters"] =>
            {
                // Parameters is an array of `JsonValue`s, each of which has a `name` field
                let mut value = value.clone();
                for param in value.members_mut() {
                    if let Some(s) = param["name"].as_str() {
                        // Only update the field which exactly matches "Version"
                        if s == "Version" {
                            println!("Replacing header `Version` with `If-Match`");
                            param["name"] = "If-Match".to_string().into();
                        }
                    }
                }
                Some(value)
            }
            _ => None,
        }
    }

    // Main patching function, called for each object key in the object tree.
    // The patcher can replace the existing value by returning `Some<JsonValue>`,
    // or leave it unmodified by returning `None`.
    fn maybe_patch(&mut self, key: &[&str], value: &mut JsonValue) {
        //println!("patcher: {:#?}", key);
        for patch_fn in Patcher::PATCH_FNS.iter() {
            if let Some(patch) = patch_fn(self, key, value) {
                *value = patch;
            }
        }
    }

    // Walk all (key, values) in the object tree, invoking the `patcher` function for each.
    fn walker(&mut self, key: &[&str], value: &mut JsonValue) {
        self.maybe_patch(key, value);
        if let JsonValue::Object(_) = value {
            for (k, v) in value.entries_mut() {
                let mut new_key = key.to_owned();
                new_key.push(k);
                self.walker(&new_key, v);
            }
        }
    }

    // Adds all the accumulated new definitions to the definitions section of the schema
    fn add_new_definitions(&mut self, json: &mut JsonValue) {
        for (k, v) in mem::take(&mut self.new_definitions) {
            json["definitions"][k] = v
        }
    }

    fn run(&mut self, json: &mut JsonValue) {
        self.walker(&[], json);
        self.add_new_definitions(json);
    }
}
