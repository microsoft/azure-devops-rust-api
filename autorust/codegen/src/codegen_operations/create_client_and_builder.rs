use crate::{identifier::SnakeCaseIdent, Result};
use proc_macro2::TokenStream;
use quote::quote;

pub fn create_client(modules: &[String], endpoint: Option<&str>) -> Result<TokenStream> {
    let mut clients = TokenStream::new();
    for md in modules {
        let client = format!("{md}_client").to_snake_case_ident()?;
        let md = md.to_snake_case_ident()?;
        clients.extend(quote! {
            pub fn #client(&self) -> #md::Client {
                #md::Client(self.clone())
            }
        });
    }

    let public_cloud = quote! {
        pub use azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD as DEFAULT_ENDPOINT;
    };
    let default_endpoint_code = if let Some(endpoint) = endpoint {
        if endpoint == "https://management.azure.com" {
            public_cloud
        } else {
            quote! {
                azure_core::static_url!(DEFAULT_ENDPOINT, #endpoint);
            }
        }
    } else {
        public_cloud
    };

    let mut code = TokenStream::new();
    code.extend(quote! {

        #[derive(Clone)]
        pub struct Client {
            endpoint: azure_core::http::Url,
            credential: crate::Credential,
            scopes: Vec<String>,
            pipeline: azure_core::http::Pipeline,
        }

        #[derive(Clone)]
        pub struct ClientBuilder {
            credential: crate::Credential,
            endpoint: Option<azure_core::http::Url>,
            scopes: Option<Vec<String>>,
            options: azure_core::http::ClientOptions,
        }

        #default_endpoint_code

        impl ClientBuilder {
            #[doc = "Create a new instance of `ClientBuilder`."]
            #[must_use]
            pub fn new(credential: crate::Credential) -> Self {
                Self {
                    credential,
                    endpoint: None,
                    scopes: None,
                    options: azure_core::http::ClientOptions::default(),
                }
            }

            #[doc = "Set the endpoint."]
            #[must_use]
            pub fn endpoint(mut self, endpoint: impl Into<azure_core::http::Url>) -> Self {
                self.endpoint = Some(endpoint.into());
                self
            }

            #[doc = "Set the scopes."]
            #[must_use]
            pub fn scopes(mut self, scopes: &[&str]) -> Self {
                self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
                self
            }

            #[doc = "Set the retry options."]
            #[must_use]
            pub fn retry(mut self, retry: impl Into<azure_core::http::RetryOptions>) -> Self {
                self.options.retry = Some(retry.into());
                self
            }

            #[doc = "Set the transport options."]
            #[must_use]
            pub fn transport(mut self, transport: impl Into<azure_core::http::TransportOptions>) -> Self {
                self.options.transport = Some(transport.into());
                self
            }

            #[doc = "Set per-call policies."]
            #[must_use]
            pub fn per_call_policies(mut self, policies: impl Into<Vec<std::sync::Arc<dyn azure_core::http::policies::Policy>>>) -> Self {
                self.options.per_call_policies = policies.into();
                self
            }

            #[doc = "Set per-try policies."]
            #[must_use]
            pub fn per_try_policies(
                mut self,
                policies: impl Into<Vec<std::sync::Arc<dyn azure_core::http::policies::Policy>>>,
            ) -> Self {
                self.options.per_try_policies = policies.into();
                self
            }

            #[doc = "Convert the builder into a `Client` instance."]
            pub fn build(self) -> Client {
                let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
                let scopes = self
                            .scopes
                            .unwrap_or_else(|| vec![crate::ADO_SCOPE.to_string()]);
                Client::new(endpoint, self.credential, scopes, self.options)
            }
        }

        impl Client {
            // Note: Not used for azure-devops-rust-api
            // pub(crate) async fn bearer_token(&self) -> azure_core::Result<azure_core::auth::Secret> {
            //     let credential = self.token_credential();
            //     let response = credential.get_token(&self.scopes()).await?;
            //     Ok(response.token)
            // }

            pub(crate) fn endpoint(&self) -> &azure_core::http::Url {
                &self.endpoint
            }
            pub(crate) fn token_credential(&self) -> &crate::Credential {
                &self.credential
            }
            pub(crate) fn scopes(&self) -> Vec<&str> {
                self.scopes.iter().map(String::as_str).collect()
            }
            pub(crate) async fn send(&self, request: &mut azure_core::http::Request) -> azure_core::Result<azure_core::http::BufResponse> {
                let context = azure_core::http::Context::default();
                self.pipeline.send(&context, request).await
            }

            #[doc = "Create a new `ClientBuilder`."]
            #[must_use]
            pub fn builder(credential: crate::Credential) -> ClientBuilder {
                ClientBuilder::new(credential)
            }

            #[doc = "Create a new `Client`."]
            #[must_use]
            pub fn new(endpoint: impl Into<azure_core::http::Url>, credential: crate::Credential, scopes: Vec<String>, options: azure_core::http::ClientOptions) -> Self {
                let endpoint = endpoint.into();
                let pipeline = azure_core::http::Pipeline::new(
                    option_env!("CARGO_PKG_NAME"),
                    option_env!("CARGO_PKG_VERSION"),
                    options,
                    Vec::new(),
                    Vec::new(),
                    None
                );
                Self {
                    endpoint,
                    credential,
                    scopes,
                    pipeline,
                }
            }

            #clients
        }
    });
    Ok(code)
}
