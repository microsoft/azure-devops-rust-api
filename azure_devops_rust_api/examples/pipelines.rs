// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// pipelines.rs
// Pipelines example.
use azure_devops_rust_api::pipelines;
use azure_devops_rust_api::pipelines::models::{Pipeline, RunPipelineParameters};
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let credential = Arc::new(azure_identity::AzureCliCredential {});

    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let pipeline_name = env::args().nth(1).expect("Usage: pipelines <name>");

    let client = pipelines::operations::Client::new(service_endpoint, credential, vec![]);

    let pipelines = client
        .pipelines()
        .list(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("Total pipelines: {}", pipelines.len());

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

        let pipeline = client
            .pipelines()
            .get(&organization, &project, pipeline.id)
            .into_future()
            .await?;
        println!("\nExample pipeline struct from get:");
        println!("{:#?}", pipeline);

        let runs = client
            .runs()
            .list(&organization, &project, pipeline.id)
            .into_future()
            .await?
            .value;

        println!("Pipeline runs: {}", runs.len());
        for run in runs.iter() {
            println!(
                "{:8} {:16} {:14} {:14}",
                run.run_reference.id,
                run.run_reference.name,
                format!("{:?}", run.result),
                format!("{:?}", run.state)
            );
        }

        let run_pipeline_params = RunPipelineParameters {
            preview_run: Some(true),
            resources: None,
            stages_to_skip: vec![],
            template_parameters: None,
            variables: None,
            yaml_override: None,
        };

        let preview_client = client.preview();
        let preview = preview_client
            .preview(&organization, run_pipeline_params, &project, pipeline.id)
            .into_future()
            .await?;

        if let Some(final_yaml) = preview.final_yaml {
            println!("Pipeline preview:\n{}", final_yaml);
        }
    }

    Ok(())
}
