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
pub const DEFAULT_ENDPOINT: &str = "https://app.vssps.visualstudio.com";
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
    #[doc = "Set per-call policies."]
    #[must_use]
    pub fn per_call_policies(
        mut self,
        policies: impl Into<Vec<std::sync::Arc<dyn azure_core::Policy>>>,
    ) -> Self {
        self.options = self.options.per_call_policies(policies);
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
    pub fn profiles_client(&self) -> profiles::Client {
        profiles::Client(self.clone())
    }
}
pub mod profiles {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a user profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The ID of the target user profile within the same organization, or 'me' to get the profile of the current authenticated user."]
        pub fn get(&self, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                id: id.into(),
                details: None,
                with_attributes: None,
                partition: None,
                core_attributes: None,
                force_refresh: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Profile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) details: Option<bool>,
            pub(crate) with_attributes: Option<bool>,
            pub(crate) partition: Option<String>,
            pub(crate) core_attributes: Option<String>,
            pub(crate) force_refresh: Option<bool>,
        }
        impl Builder {
            #[doc = "Return public profile information such as display name, email address, country, etc. If false, the withAttributes parameter is ignored."]
            pub fn details(mut self, details: bool) -> Self {
                self.details = Some(details);
                self
            }
            #[doc = "If true, gets the attributes (named key-value pairs of arbitrary data) associated with the profile. The partition parameter must also have a value."]
            pub fn with_attributes(mut self, with_attributes: bool) -> Self {
                self.with_attributes = Some(with_attributes);
                self
            }
            #[doc = "The partition (named group) of attributes to return."]
            pub fn partition(mut self, partition: impl Into<String>) -> Self {
                self.partition = Some(partition.into());
                self
            }
            #[doc = "A comma-delimited list of core profile attributes to return. Valid values are Email, Avatar, DisplayName, and ContactWithOffers."]
            pub fn core_attributes(mut self, core_attributes: impl Into<String>) -> Self {
                self.core_attributes = Some(core_attributes.into());
                self
            }
            #[doc = "Not used in this version of the API."]
            pub fn force_refresh(mut self, force_refresh: bool) -> Self {
                self.force_refresh = Some(force_refresh);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/_apis/profile/profiles/{}",
                            this.client.endpoint(),
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(details) = &this.details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("details", &details.to_string());
                        }
                        if let Some(with_attributes) = &this.with_attributes {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("withAttributes", &with_attributes.to_string());
                        }
                        if let Some(partition) = &this.partition {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("partition", partition);
                        }
                        if let Some(core_attributes) = &this.core_attributes {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("coreAttributes", core_attributes);
                        }
                        if let Some(force_refresh) = &this.force_refresh {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("forceRefresh", &force_refresh.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)
                                    .map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
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
