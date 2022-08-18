// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// pipeline_preview.rs
// Pipeline preview example.
use anyhow::Result;
use azure_core::ClientOptions;
use azure_devops_rust_api::pipelines;
use azure_devops_rust_api::pipelines::models::{Pipeline, RunPipelineParameters};
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli.
    let credential = match env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential {}))
        }
    };

    // Get ADO server configuration via environment variables
    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let pipeline_name = env::args().nth(1).expect("Usage: pipeline_preview <name>");

    // Create a `pipelines` client
    let client = pipelines::Client::new(
        service_endpoint,
        credential,
        vec![],
        ClientOptions::default(),
    );

    // Use the client to list all pipelines in the specified organization/project
    let pipelines = client
        .pipelines_client()
        .list(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("Total pipelines: {}", pipelines.len());

    // Filter the pipelines to just those that contain the specified name
    println!("\nMatched pipelines:");
    let matched_pipelines: Vec<Pipeline> = pipelines
        .iter()
        .filter(|pipeline| pipeline.name.contains(&pipeline_name))
        .cloned()
        .collect();

    if let Some(pipeline) = matched_pipelines.iter().next() {
        // Demonstrate how to query a preview of pipeline YAML...
        // Define the pipeline params
        let run_pipeline_params = RunPipelineParameters {
            preview_run: Some(true),
            resources: None,
            stages_to_skip: vec![],
            template_parameters: None,
            variables: None,
            yaml_override: None,
        };

        // Create a preview client
        let preview_client = client.preview_client();

        // Request a preview of the specified pipeline
        let preview = preview_client
            .preview(&organization, run_pipeline_params, &project, pipeline.id)
            .into_future()
            .await?;

        // Display the full pipeline YAML
        if let Some(final_yaml) = preview.final_yaml {
            println!("Pipeline preview:\n{}", final_yaml);
        }
    }

    Ok(())
}
