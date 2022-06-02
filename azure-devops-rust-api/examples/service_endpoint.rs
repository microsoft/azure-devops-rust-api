use azure_devops_rust_api::service_endpoint;
use azure_devops_rust_api::service_endpoint::models::ServiceEndpoint;
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

    let client = service_endpoint::operations::Client::new(service_endpoint, credential, vec![]);

    let service_endpoints = client
        .endpoints()
        .get_service_endpoints(&organization, &project)
        .into_future()
        .await?
        .value;
    println!("Total service_endpoints: {}", service_endpoints.len());

    for endpoint in service_endpoints.iter() {
        match endpoint {
            ServiceEndpoint {
                id: Some(id),
                name: Some(name),
                description: Some(description),
                ..
            } => {
                println!("{:38} {:40} {}", id, name, description);
            }
            _ => {
                println!("Endpoint missing expected fields: {:#?}", endpoint);
            }
        }
    }

    Ok(())
}
