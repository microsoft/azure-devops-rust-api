// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// pipelines.rs
// Pipelines example.
use azure_devops_rust_api::auth::Credential;
use azure_devops_rust_api::pipelines;
use azure_devops_rust_api::pipelines::models::{Pipeline, RunPipelineParameters};
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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
    let pipeline_name = env::args().nth(1).expect("Usage: pipelines <name>");

    // Create a `pipelines` client
    let client = pipelines::operations::Client::new(service_endpoint, credential, vec![]);

    // Use the client to list all pipelines in the specified organization/project
    let pipelines = client
        .pipelines()
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

    for pipeline in matched_pipelines.iter() {
        println!("{:4} {}", pipeline.id, pipeline.name);
    }

    if let Some(pipeline) = matched_pipelines.iter().next() {
        println!("\nExample pipeline struct from list:");
        println!("{:#?}", pipeline);

        // The pipeline struct returned from list is different from that returned by get.
        // Query and display the struct returned by get for comparison.
        let pipeline = client
            .pipelines()
            .get(&organization, &project, pipeline.id)
            .into_future()
            .await?;
        println!("\nExample pipeline struct from get:");
        println!("{:#?}", pipeline);

        // Use the client to list all runs of the selected pipeline
        let runs = client
            .runs()
            .list(&organization, &project, pipeline.id)
            .into_future()
            .await?
            .value;

        println!("Pipeline runs: {}", runs.len());
        // Display [result, state] for each pipeline run
        for run in runs.iter() {
            println!(
                "{:8} {:16} {:14} {:14}",
                run.run_reference.id,
                run.run_reference.name,
                format!("{:?}", run.result),
                format!("{:?}", run.state)
            );
        }

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
        let preview_client = client.preview();

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
