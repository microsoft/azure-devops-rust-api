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
azure_core::static_url!(DEFAULT_ENDPOINT, "https://dev.azure.com");
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
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets an operation from the operationId using the given pluginId.\n\nSome scenarios don’t require a pluginId. If a pluginId is not included in the call then just the operationId will be used to find an operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `operation_id`: The ID for the operation."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            operation_id: impl Into<String>,
            organization: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                operation_id: operation_id.into(),
                organization: organization.into(),
                plugin_id: None,
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
            azure_core::http::Response<models::Operation, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Operation> {
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
        pub struct RequestBuilder<M = ()> {
            pub(crate) client: super::super::Client,
            pub(crate) mode: std::marker::PhantomData<M>,
            pub(crate) operation_id: String,
            pub(crate) organization: String,
            pub(crate) plugin_id: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "The ID for the plugin."]
            pub fn plugin_id(mut self, plugin_id: impl Into<String>) -> Self {
                self.plugin_id = Some(plugin_id.into());
                self
            }
        }
        impl<M: Clone + Send + 'static> RequestBuilder<M> {
            fn send_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req =
                            azure_core::http::Request::new(url, azure_core::http::Method::Get);
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
                        if let Some(plugin_id) = &this.plugin_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("pluginId", plugin_id);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/operations/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.operation_id
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
                self.send_future()
            }
        }
        #[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
        impl RequestBuilder<crate::blocking::Blocking> {
            #[doc = "Sends the request, blocking until the response is received, and returns a [`Response`] object."]
            pub fn send(self) -> azure_core::Result<Response> {
                crate::blocking::block_on(self.send_future())
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Operation>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Operation>>;
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
#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
pub mod blocking {
    #![doc = "Blocking (non-async) client."]
    #![doc = ""]
    #![doc = "This wraps the async [`Client`](super::Client), running each request on a hidden tokio runtime."]
    #![doc = "It must not be used from within an async runtime."]
    #[derive(Clone)]
    pub struct Client(pub(crate) super::Client);
    #[derive(Clone)]
    pub struct ClientBuilder(super::ClientBuilder);
    impl ClientBuilder {
        #[doc = "Create a new instance of `ClientBuilder`."]
        #[must_use]
        pub fn new(credential: crate::Credential) -> Self {
            Self(super::ClientBuilder::new(credential))
        }
        #[doc = "Set the endpoint."]
        #[must_use]
        pub fn endpoint(self, endpoint: impl Into<azure_core::http::Url>) -> Self {
            Self(self.0.endpoint(endpoint))
        }
        #[doc = "Set the scopes."]
        #[must_use]
        pub fn scopes(self, scopes: &[&str]) -> Self {
            Self(self.0.scopes(scopes))
        }
        #[doc = "Set the retry options."]
        #[must_use]
        pub fn retry(self, retry: impl Into<azure_core::http::RetryOptions>) -> Self {
            Self(self.0.retry(retry))
        }
        #[doc = "Set the transport options."]
        #[must_use]
        pub fn transport(self, transport: impl Into<azure_core::http::Transport>) -> Self {
            Self(self.0.transport(transport))
        }
        #[doc = "Set per-call policies."]
        #[must_use]
        pub fn per_call_policies(
            self,
            policies: impl Into<Vec<std::sync::Arc<dyn azure_core::http::policies::Policy>>>,
        ) -> Self {
            Self(self.0.per_call_policies(policies))
        }
        #[doc = "Set per-try policies."]
        #[must_use]
        pub fn per_try_policies(
            self,
            policies: impl Into<Vec<std::sync::Arc<dyn azure_core::http::policies::Policy>>>,
        ) -> Self {
            Self(self.0.per_try_policies(policies))
        }
        #[doc = "Convert the builder into a `Client` instance."]
        pub fn build(self) -> Client {
            Client(self.0.build())
        }
    }
    impl Client {
        #[doc = "Create a new `ClientBuilder`."]
        #[must_use]
        pub fn builder(credential: crate::Credential) -> ClientBuilder {
            ClientBuilder::new(credential)
        }
        pub fn operations_client(&self) -> operations::Client {
            operations::Client(self.clone())
        }
    }
    impl From<super::Client> for Client {
        #[doc = "Create a blocking client that sends requests using an existing async client."]
        fn from(client: super::Client) -> Self {
            Self(client)
        }
    }
    pub mod operations {
        use super::super::models;
        use super::super::operations::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Gets an operation from the operationId using the given pluginId.\n\nSome scenarios don’t require a pluginId. If a pluginId is not included in the call then just the operationId will be used to find an operation."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::operations::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                operation_id: impl Into<String>,
                organization: impl Into<String>,
            ) -> get::RequestBuilder<crate::blocking::Blocking> {
                get::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    operation_id: operation_id.into(),
                    organization: organization.into(),
                    plugin_id: None,
                }
            }
        }
    }
}
