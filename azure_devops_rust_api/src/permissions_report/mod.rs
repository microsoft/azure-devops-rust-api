// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: crate::Credential,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: crate::Credential,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://dev.azure.com";
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: crate::Credential) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
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
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &crate::Credential {
        &self.credential
    }
    #[allow(dead_code)]
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: &mut azure_core::Request,
    ) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: crate::Credential) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: crate::Credential,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn permissions_report_client(&self) -> permissions_report::Client {
        permissions_report::Client(self.clone())
    }
    pub fn permissions_report_download_client(&self) -> permissions_report_download::Client {
        permissions_report_download::Client(self.clone())
    }
}
pub mod permissions_report {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of permissions reports"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Request a permissions report to be created asyncronously"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Request configuration to be included in the permissions report"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PermissionsReportRequest>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Get a specific permissions report"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: The ID (GUID) of the permissions report"]
        pub fn get(&self, organization: impl Into<String>, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id: id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PermissionsReportList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/permissionsreport",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PermissionsReportList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::ReferenceLinks;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PermissionsReportRequest,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/permissionsreport",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReferenceLinks =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::PermissionsReport;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/permissionsreport/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PermissionsReport =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod permissions_report_download {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Download the json results of a permissions report"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: The ID (GUID) of the permissions report"]
        pub fn download(
            &self,
            organization: impl Into<String>,
            id: impl Into<String>,
        ) -> download::Builder {
            download::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id: id.into(),
            }
        }
    }
    pub mod download {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/permissionsreport/{}/download",
                            this.client.endpoint(),
                            &this.organization,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
