use azure_devops_rust_api::pipelines;
use azure_devops_rust_api::pipelines::models::{Pipeline, RunPipelineParameters};
use std::env;
use std::error::Error;
use std::sync::Arc;

// ISSUE: The current OpenAPI spec does not include all the fields!
//
// Example pipeline get via curl returns this:
// {
//     "_links": {
//       "self": {
//         "href": "https://dev.azure.com/msazuredev/7f08d38b-6f75-4be5-8c43-b4b787d9c3e6/_apis/pipelines/295?revision=1"
//       },
//       "web": {
//         "href": "https://dev.azure.com/msazuredev/7f08d38b-6f75-4be5-8c43-b4b787d9c3e6/_build/definition?definitionId=295"
//       }
//     },
//     "configuration": {
//       "path": "/.pipelines/OneBranch.Official.yml",
//       "repository": {
//         "id": "4f698e75-d215-4ec3-be59-31b0d72284c9",
//         "type": "azureReposGit"
//       },
//       "type": "yaml"
//     },
//     "url": "https://dev.azure.com/msazuredev/7f08d38b-6f75-4be5-8c43-b4b787d9c3e6/_apis/pipelines/295?revision=1",
//     "id": 295,
//     "revision": 1,
//     "name": "qsinfra-base-dev-Official",
//     "folder": "\\OneBranch\\qsinfra-base-dev"
// }
//
// Current code returns:
// Pipeline {
//     pipeline_base: PipelineBase {
//         folder: Some(
//             "\\OneBranch\\qsinfra-base-dev",
//         ),
//         id: Some(
//             295,
//         ),
//         name: Some(
//             "qsinfra-base-dev-Official",
//         ),
//         revision: Some(
//             1,
//         ),
//     },
//     links: Some(
//         ReferenceLinks {
//             links: None,
//         },
//     ),
//     configuration: Some(
//         PipelineConfiguration {
//             type_: Some(
//                 Yaml,
//             ),
//         },
//     ),
//     url: Some(
//         "https://dev.azure.com/msazuredev/7f08d38b-6f75-4be5-8c43-b4b787d9c3e6/_apis/pipelines/295?revision=1",
//     ),
// }
//
// Notes:
// - missing configuration detail - only has type
//     "configuration": {
//       "path": "/.pipelines/OneBranch.Official.yml",
//       "repository": {
//         "id": "4f698e75-d215-4ec3-be59-31b0d72284c9",
//         "type": "azureReposGit"
//       },
//       "type": "yaml"
//     }
//
// - missing links detail:
//     "_links": {
//       "self": {
//         "href": "https://dev.azure.com/msazuredev/7f08d38b-6f75-4be5-8c43-b4b787d9c3e6/_apis/pipelines/295?revision=1"
//       },
//       "web": {
//         "href": "https://dev.azure.com/msazuredev/7f08d38b-6f75-4be5-8c43-b4b787d9c3e6/_build/definition?definitionId=295"
//       }
//     },

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let credential = Arc::new(azure_identity::token_credentials::AzureCliCredential {});

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
