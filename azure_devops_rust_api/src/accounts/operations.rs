// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
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
}
pub const DEFAULT_ENDPOINT: &str = "https://app.vssps.visualstudio.com";
impl ClientBuilder {
    pub fn new(credential: crate::Credential) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn credential(&self) -> &crate::Credential {
        &self.credential
    }
    pub(crate) async fn send(
        &self,
        request: impl Into<azure_core::Request>,
    ) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: crate::Credential,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
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
    pub fn accounts(&self) -> accounts::Client {
        accounts::Client(self.clone())
    }
}
pub mod accounts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of accounts for a specific owner or a specific member. One of the following parameters is required: ownerId, memberId."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                owner_id: None,
                member_id: None,
                properties: None,
            }
        }
    }
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::AccountList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) owner_id: Option<String>,
            pub(crate) member_id: Option<String>,
            pub(crate) properties: Option<String>,
        }
        impl Builder {
            pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
                self.owner_id = Some(owner_id.into());
                self
            }
            pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
                self.member_id = Some(member_id.into());
                self
            }
            pub fn properties(mut self, properties: impl Into<String>) -> Self {
                self.properties = Some(properties.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/_apis/accounts", this.client.endpoint(),);
                        let mut url = url::Url::parse(url_str)
                            .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        req_builder = req_builder.header(
                            http::header::AUTHORIZATION,
                            &this
                                .client
                                .credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        url.query_pairs_mut()
                            .append_pair("api-version", "7.1-preview");
                        if let Some(owner_id) = &this.owner_id {
                            url.query_pairs_mut().append_pair("ownerId", owner_id);
                        }
                        if let Some(member_id) = &this.member_id {
                            url.query_pairs_mut().append_pair("memberId", member_id);
                        }
                        if let Some(properties) = &this.properties {
                            url.query_pairs_mut().append_pair("properties", properties);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder
                            .body(req_body)
                            .context(azure_core::error::ErrorKind::Other, "build request")?;
                        let rsp = this
                            .client
                            .send(req)
                            .await
                            .context(azure_core::error::ErrorKind::Io, "execute request")?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body =
                                    azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::AccountList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code.as_u16(),
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
