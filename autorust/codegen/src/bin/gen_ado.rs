// cargo run --example storage_mgmt
// https://github.com/Azure/azure-rest-api-specs/tree/master/specification/storage/resource-manager

use autorust_codegen::*;
use std::collections::HashSet;
use std::path::PathBuf;

const API_VERSION: &str = "7.1";
const ROOT_SPEC_DIR: &str = "../../vsts-rest-api-specs.patched/specification";

const BOX_PROPERTIES: &[(&str, &str, &str)] = &[
    // build
    (
        "{ROOT_SPEC_DIR}/build/{VERSION}/build.json",
        "Build",
        "triggeredByBuild",
    ),
    (
        "{ROOT_SPEC_DIR}/distributedTask/{VERSION}/taskAgent.json",
        "TaskOrchestrationContainer",
        "rollback",
    ),
    (
        "{ROOT_SPEC_DIR}/distributedTask/{VERSION}/taskAgent.json",
        "JToken",
        "first",
    ),
    (
        "{ROOT_SPEC_DIR}/distributedTask/{VERSION}/taskAgent.json",
        "JToken",
        "item",
    ),
    (
        "{ROOT_SPEC_DIR}/distributedTask/{VERSION}/taskAgent.json",
        "JToken",
        "last",
    ),
    (
        "{ROOT_SPEC_DIR}/distributedTask/{VERSION}/taskAgent.json",
        "JToken",
        "next",
    ),
    (
        "{ROOT_SPEC_DIR}/distributedTask/{VERSION}/taskAgent.json",
        "JToken",
        "previous",
    ),
    (
        "{ROOT_SPEC_DIR}/distributedTask/{VERSION}/taskAgent.json",
        "JToken",
        "root",
    ),
    (
        "{ROOT_SPEC_DIR}/graph/{VERSION}/graph.json",
        "JToken",
        "first",
    ),
    (
        "{ROOT_SPEC_DIR}/graph/{VERSION}/graph.json",
        "JToken",
        "item",
    ),
    (
        "{ROOT_SPEC_DIR}/graph/{VERSION}/graph.json",
        "JToken",
        "last",
    ),
    (
        "{ROOT_SPEC_DIR}/graph/{VERSION}/graph.json",
        "JToken",
        "next",
    ),
    (
        "{ROOT_SPEC_DIR}/graph/{VERSION}/graph.json",
        "JToken",
        "previous",
    ),
    (
        "{ROOT_SPEC_DIR}/graph/{VERSION}/graph.json",
        "JToken",
        "root",
    ),
    (
        "{ROOT_SPEC_DIR}/serviceEndpoint/{VERSION}/serviceEndpoint.json",
        "JToken",
        "first",
    ),
    (
        "{ROOT_SPEC_DIR}/serviceEndpoint/{VERSION}/serviceEndpoint.json",
        "JToken",
        "item",
    ),
    (
        "{ROOT_SPEC_DIR}/serviceEndpoint/{VERSION}/serviceEndpoint.json",
        "JToken",
        "last",
    ),
    (
        "{ROOT_SPEC_DIR}/serviceEndpoint/{VERSION}/serviceEndpoint.json",
        "JToken",
        "next",
    ),
    (
        "{ROOT_SPEC_DIR}/serviceEndpoint/{VERSION}/serviceEndpoint.json",
        "JToken",
        "previous",
    ),
    (
        "{ROOT_SPEC_DIR}/serviceEndpoint/{VERSION}/serviceEndpoint.json",
        "JToken",
        "root",
    ),
];

fn main() -> Result<()> {
    let root_spec_folder: std::path::PathBuf = format!("{ROOT_SPEC_DIR}").into();
    let root_output_folder: std::path::PathBuf = "../../azure_devops_rust_api/src".into();
    let modules = [
        (vec!["account/{VERSION}/accounts.json"], "accounts"),
        (
            vec![
                "artifacts/{VERSION}/feed.json",
                "artifacts/{VERSION}/provenance.json",
            ],
            "artifacts",
        ),
        (
            vec![
                "artifactsPackageTypes/{VERSION}/maven.json",
                "artifactsPackageTypes/{VERSION}/maven.json",
                "artifactsPackageTypes/{VERSION}/npm.json",
                "artifactsPackageTypes/{VERSION}/nuGet.json",
                "artifactsPackageTypes/{VERSION}/pyPiApi.json",
                "artifactsPackageTypes/{VERSION}/universal.json",
            ],
            "artifacts_package_types",
        ),
        (vec!["audit/{VERSION}/audit.json"], "audit"),
        (vec!["build/{VERSION}/build.json"], "build"),
        (vec!["clt/{VERSION}/cloudLoadTest.json"], "clt"),
        (vec!["core/{VERSION}/core.json"], "core"),
        (vec!["dashboard/{VERSION}/dashboard.json"], "dashboard"),
        (
            vec!["distributedTask/{VERSION}/taskAgent.json"],
            "distributed_task",
        ),
        (
            vec!["extensionManagement/{VERSION}/extensionManagement.json"],
            "extension_management",
        ),
        (vec!["git/{VERSION}/git.json"], "git"),
        (vec!["graph/{VERSION}/graph.json"], "graph"),
        (vec!["hooks/{VERSION}/serviceHooks.json"], "hooks"),
        (vec!["ims/{VERSION}/identities.json"], "ims"),
        (
            vec!["memberEntitlementManagement/{VERSION}/memberEntitlementManagement.json"],
            "member_entitlement_management",
        ),
        //(vec!["notification/{VERSION}/notification.json"], "notification"),
        (vec!["operations/{VERSION}/operations.json"], "operations"),
        (
            vec!["permissionsReport/{VERSION}/permissionsReport.json"],
            "permissions_report",
        ),
        (vec!["pipelines/{VERSION}/pipelines.json"], "pipelines"),
        (vec!["policy/{VERSION}/policy.json"], "policy"),
        (
            vec!["processadmin/{VERSION}/workItemTrackingProcessTemplate.json"],
            "processadmin",
        ),
        (
            vec!["processes/{VERSION}/workItemTrackingProcess.json"],
            "processes",
        ),
        (vec!["profile/{VERSION}/profile.json"], "profile"),
        (vec!["release/{VERSION}/release.json"], "release"),
        (vec!["search/{VERSION}/search.json"], "search"),
        (vec!["security/{VERSION}/security.json"], "security"),
        (
            vec!["serviceEndpoint/{VERSION}/serviceEndpoint.json"],
            "service_endpoint",
        ),
        (vec!["status/{VERSION}/status.json"], "status"),
        (vec!["symbol/{VERSION}/symbol.json"], "symbol"),
        (vec!["test/{VERSION}/test.json"], "test"),
        (vec!["testPlan/{VERSION}/testPlan.json"], "test_plan"),
        (
            vec!["testResults/{VERSION}/testResults.json"],
            "test_results",
        ),
        (vec!["tfvc/{VERSION}/tfvc.json"], "tfvc"),
        (vec!["tokenAdmin/{VERSION}/tokenAdmin.json"], "token_admin"),
        (vec!["wiki/{VERSION}/wiki.json"], "wiki"),
        (vec!["wit/{VERSION}/workItemTracking.json"], "wit"),
        (vec!["work/{VERSION}/work.json"], "work"),
    ];

    let mut box_properties = HashSet::new();
    for (file_path, schema_name, property_name) in BOX_PROPERTIES {
        box_properties.insert(PropertyName {
            file_path: PathBuf::from(
                file_path
                    .replace("{VERSION}", API_VERSION)
                    .replace("{ROOT_SPEC_DIR}", ROOT_SPEC_DIR),
            ),
            schema_name: schema_name.to_string(),
            property_name: property_name.to_string(),
        });
    }

    for (input_files, module_name) in modules {
        let mut output_folder = root_output_folder.clone();
        output_folder.push(module_name);
        run(Config {
            output_folder,
            input_files: input_files
                .iter()
                .map(|filename| {
                    let mut input_file = root_spec_folder.clone();
                    input_file.push(
                        filename
                            .replace("{VERSION}", API_VERSION)
                            .replace("{ROOT_SPEC_DIR}", ROOT_SPEC_DIR),
                    );
                    input_file
                })
                .collect(),
            box_properties: box_properties.clone(),
            ..Config::default()
        })?;
    }

    Ok(())
}
