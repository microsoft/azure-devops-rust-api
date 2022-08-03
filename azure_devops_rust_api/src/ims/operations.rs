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
pub const DEFAULT_ENDPOINT: &str = "https://vssps.dev.azure.com";
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
    pub fn identities(&self) -> identities::Client {
        identities::Client(self.clone())
    }
}
pub mod identities {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Resolve legacy identity information for use with older APIs such as the Security APIs"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn read_identities(&self, organization: impl Into<String>) -> read_identities::Builder {
            read_identities::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                descriptors: None,
                identity_ids: None,
                subject_descriptors: None,
                search_filter: None,
                filter_value: None,
                query_membership: None,
            }
        }
    }
    pub mod read_identities {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::IdentityList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) descriptors: Option<String>,
            pub(crate) identity_ids: Option<String>,
            pub(crate) subject_descriptors: Option<String>,
            pub(crate) search_filter: Option<String>,
            pub(crate) filter_value: Option<String>,
            pub(crate) query_membership: Option<String>,
        }
        impl Builder {
            pub fn descriptors(mut self, descriptors: impl Into<String>) -> Self {
                self.descriptors = Some(descriptors.into());
                self
            }
            pub fn identity_ids(mut self, identity_ids: impl Into<String>) -> Self {
                self.identity_ids = Some(identity_ids.into());
                self
            }
            pub fn subject_descriptors(mut self, subject_descriptors: impl Into<String>) -> Self {
                self.subject_descriptors = Some(subject_descriptors.into());
                self
            }
            pub fn search_filter(mut self, search_filter: impl Into<String>) -> Self {
                self.search_filter = Some(search_filter.into());
                self
            }
            pub fn filter_value(mut self, filter_value: impl Into<String>) -> Self {
                self.filter_value = Some(filter_value.into());
                self
            }
            pub fn query_membership(mut self, query_membership: impl Into<String>) -> Self {
                self.query_membership = Some(query_membership.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/identities",
                            this.client.endpoint(),
                            &this.organization
                        );
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
                        if let Some(descriptors) = &this.descriptors {
                            url.query_pairs_mut()
                                .append_pair("descriptors", descriptors);
                        }
                        if let Some(identity_ids) = &this.identity_ids {
                            url.query_pairs_mut()
                                .append_pair("identityIds", identity_ids);
                        }
                        if let Some(subject_descriptors) = &this.subject_descriptors {
                            url.query_pairs_mut()
                                .append_pair("subjectDescriptors", subject_descriptors);
                        }
                        if let Some(search_filter) = &this.search_filter {
                            url.query_pairs_mut()
                                .append_pair("searchFilter", search_filter);
                        }
                        if let Some(filter_value) = &this.filter_value {
                            url.query_pairs_mut()
                                .append_pair("filterValue", filter_value);
                        }
                        if let Some(query_membership) = &this.query_membership {
                            url.query_pairs_mut()
                                .append_pair("queryMembership", query_membership);
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
                                let rsp_value: models::IdentityList =
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
