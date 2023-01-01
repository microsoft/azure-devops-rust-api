// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// pipelines.rs
// Pipelines example.
use anyhow::Result;
use azure_devops_rust_api::pipelines;
use azure_devops_rust_api::pipelines::models::Pipeline;
use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential
    let credential = utils::get_credential();

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let pipeline_name = env::args().nth(1).expect("Usage: pipelines <name>");

    // Create a pipelines client
    let pipelines_client = pipelines::ClientBuilder::new(credential).build();

    // List all pipelines in the specified organization/project
    let pipelines = pipelines_client
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

    for pipeline in matched_pipelines.iter() {
        println!("{:4} {}", pipeline.id, pipeline.name);
    }

    if let Some(pipeline) = matched_pipelines.iter().next() {
        println!("\nExample pipeline struct from list:");
        println!("{:#?}", pipeline);

        // The pipeline struct returned from list is different from that returned by get.
        // Query and display the struct returned by get for comparison.
        let pipeline = pipelines_client
            .pipelines_client()
            .get(&organization, &project, pipeline.id)
            .into_future()
            .await?;
        println!("\nExample pipeline struct from get:");
        println!("{:#?}", pipeline);

        // Use the client to list all runs of the selected pipeline
        let runs = pipelines_client
            .runs_client()
            .list(&organization, &project, pipeline.id)
            .into_future()
            .await?
            .value;

        println!("\nPipeline runs: {}", runs.len());
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
    }

    Ok(())
}
