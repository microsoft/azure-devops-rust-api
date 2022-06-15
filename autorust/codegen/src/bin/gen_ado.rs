// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// gen_ado.rs
// Main Azure DevOps crate code generation entry point
use autorust_codegen::{self, Result, RunConfig, CrateConfig};
use autorust_codegen::autorust_toml;
use camino::Utf8PathBuf;

const API_VERSION: &str = "7.1";
const ROOT_SPEC_DIR: &str = "../../vsts-rest-api-specs.patched/specification";

fn main() -> Result<()> {
    let package_config = autorust_toml::read("autorust.toml".into())?;

    let run_config = &mut RunConfig::new("azure_devops_rust_api_");
    let root_spec_folder: Utf8PathBuf = ROOT_SPEC_DIR.into();
    let root_output_folder: Utf8PathBuf = "../../azure_devops_rust_api/src".into();
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

    for (input_files, module_name) in modules {
        let mut output_folder = root_output_folder.clone();

        let input_files = input_files
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
        .collect();

        output_folder.push(module_name);
        let crate_config = CrateConfig {
            run_config,
            output_folder,
            input_files
        };

        // Generate module
        let _cg = crate::autorust_codegen::run(&crate_config, &package_config)?;
    }

    Ok(())
}
