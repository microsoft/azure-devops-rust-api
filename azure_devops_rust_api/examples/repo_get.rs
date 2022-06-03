use azure_devops_rust_api::git;
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let credential = Arc::new(azure_identity::token_credentials::AzureCliCredential {});

    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");
    let repo = env::args()
        .nth(1)
        .expect("Usage: repo_get <repository-name>");

    let client = git::operations::Client::new(service_endpoint, credential, vec![]);

    let repo = client
        .repositories()
        .get_repository(organization, repo, project)
        .into_future()
        .await?;
    println!("{:#?}", repo);

    Ok(())
}
