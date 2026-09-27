// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
pub mod models;
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
azure_core::static_url!(DEFAULT_ENDPOINT, "https://status.dev.azure.com");
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
        self.options.retry = retry.into();
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::http::Transport>) -> Self {
        self.options.transport = Some(transport.into());
        self
    }
    #[doc = "Set per-call policies."]
    #[must_use]
    pub fn per_call_policies(
        mut self,
        policies: impl Into<Vec<std::sync::Arc<dyn azure_core::http::policies::Policy>>>,
    ) -> Self {
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
    pub(crate) fn endpoint(&self) -> &azure_core::http::Url {
        &self.endpoint
    }
    pub(crate) fn token_credential(&self) -> &crate::Credential {
        &self.credential
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: &mut azure_core::http::Request,
    ) -> azure_core::Result<azure_core::http::RawResponse> {
        let context = azure_core::http::Context::default();
        self.pipeline.send(&context, request, None).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: crate::Credential) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<azure_core::http::Url>,
        credential: crate::Credential,
        scopes: Vec<String>,
        options: azure_core::http::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
            None,
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn health_client(&self) -> health::Client {
        health::Client(self.clone())
    }
}
pub mod health {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Queries status information for the service all-up, or scoped to a particular service and/or geography"]
        pub fn get(&self) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                services: None,
                geographies: None,
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::ServiceStatus, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceStatus> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" converts the [`RequestBuilder`] into a future,"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        #[doc = r""]
        #[doc = r" When created from a [`blocking::Client`](super::super::blocking::Client),"]
        #[doc = r" finalize and submit the request using [`RequestBuilder::send()`], which"]
        #[doc = r" blocks until the request completes."]
        pub struct RequestBuilder<C = super::super::Client> {
            pub(crate) client: C,
            pub(crate) services: Option<String>,
            pub(crate) geographies: Option<String>,
        }
        impl<C> RequestBuilder<C> {
            #[doc = "A comma-separated list of services for which to get status information for. Supported values: Artifacts, Boards, Core services, Other services, Pipelines, Repos, Test Plans."]
            pub fn services(mut self, services: impl Into<String>) -> Self {
                self.services = Some(services.into());
                self
            }
            #[doc = "A comma-separated list of geographies for which to get status information for. Supported values: APAC, AU, BR, CA, EU, IN, UK, US."]
            pub fn geographies(mut self, geographies: impl Into<String>) -> Self {
                self.geographies = Some(geographies.into());
                self
            }
        }
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                if let Some(services) = &this.services {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("services", services);
                }
                if let Some(geographies) = &this.geographies {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("geographies", geographies);
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/_apis/status/health",
                    self.client.endpoint(),
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair("api-version", "7.1-preview");
                }
                Ok(url)
            }
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let mut req = this.build_request()?;
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes())
                            .await?
                        {
                            req.insert_header(
                                azure_core::http::headers::AUTHORIZATION,
                                auth_header,
                            );
                        }
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
        }
        #[cfg(feature = "blocking")]
        impl RequestBuilder<super::super::blocking::Client> {
            #[doc = "Sends the request, blocking until the response is received, and returns a [`Response`] object."]
            pub fn send(self) -> azure_core::Result<Response> {
                let mut req = self.build_request()?;
                if let Some(auth_header) = self
                    .client
                    .token_credential()
                    .http_authorization_header_blocking()?
                {
                    req.insert_header(azure_core::http::headers::AUTHORIZATION, auth_header);
                }
                Ok(Response(self.client.send(&mut req)?.into()))
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ServiceStatus>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceStatus>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body() })
            }
        }
    }
}
impl crate::ClientEndpoint for Client {
    fn endpoint(&self) -> &azure_core::http::Url {
        Client::endpoint(self)
    }
}
#[doc = "Blocking (non-async) client."]
#[doc = ""]
#[doc = "Requests are sent using the [`HttpClient`](crate::blocking::HttpClient) supplied to [`ClientBuilder::new`]."]
#[doc = "Only PAT and unauthenticated credentials are supported, and requests do not go through"]
#[doc = "the `azure_core` pipeline, so no retry policy is applied."]
#[cfg(feature = "blocking")]
pub mod blocking {
    #[derive(Clone)]
    pub struct Client {
        endpoint: azure_core::http::Url,
        credential: crate::Credential,
        http_client: std::sync::Arc<dyn crate::blocking::HttpClient>,
    }
    #[derive(Clone)]
    pub struct ClientBuilder {
        credential: crate::Credential,
        http_client: std::sync::Arc<dyn crate::blocking::HttpClient>,
        endpoint: Option<azure_core::http::Url>,
    }
    impl ClientBuilder {
        #[doc = "Create a new instance of `ClientBuilder`, using `http_client` to send requests."]
        #[must_use]
        pub fn new(
            credential: crate::Credential,
            http_client: impl crate::blocking::HttpClient + 'static,
        ) -> Self {
            Self {
                credential,
                http_client: std::sync::Arc::new(http_client),
                endpoint: None,
            }
        }
        #[doc = "Set the endpoint."]
        #[must_use]
        pub fn endpoint(mut self, endpoint: impl Into<azure_core::http::Url>) -> Self {
            self.endpoint = Some(endpoint.into());
            self
        }
        #[doc = "Convert the builder into a `Client` instance."]
        pub fn build(self) -> Client {
            Client {
                endpoint: self
                    .endpoint
                    .unwrap_or_else(|| super::DEFAULT_ENDPOINT.to_owned()),
                credential: self.credential,
                http_client: self.http_client,
            }
        }
    }
    impl Client {
        pub(crate) fn token_credential(&self) -> &crate::Credential {
            &self.credential
        }
        pub(crate) fn send(
            &self,
            request: &mut azure_core::http::Request,
        ) -> azure_core::Result<azure_core::http::RawResponse> {
            crate::blocking::send(self.http_client.as_ref(), request)
        }
        #[doc = "Create a new `ClientBuilder`."]
        #[must_use]
        pub fn builder(
            credential: crate::Credential,
            http_client: impl crate::blocking::HttpClient + 'static,
        ) -> ClientBuilder {
            ClientBuilder::new(credential, http_client)
        }
        pub fn health_client(&self) -> health::Client {
            health::Client(self.clone())
        }
    }
    impl crate::ClientEndpoint for Client {
        fn endpoint(&self) -> &azure_core::http::Url {
            &self.endpoint
        }
    }
    pub mod health {
        use super::super::health::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Queries status information for the service all-up, or scoped to a particular service and/or geography"]
            #[doc = ""]
            #[doc = "See [`get`](super::super::health::Client::get) for details of the arguments."]
            pub fn get(&self) -> get::RequestBuilder<super::Client> {
                get::RequestBuilder {
                    client: self.0.clone(),
                    services: None,
                    geographies: None,
                }
            }
        }
    }
}
