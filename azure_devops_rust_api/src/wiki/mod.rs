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
    pub fn attachments_client(&self) -> attachments::Client {
        attachments::Client(self.clone())
    }
    pub fn page_moves_client(&self) -> page_moves::Client {
        page_moves::Client(self.clone())
    }
    pub fn page_stats_client(&self) -> page_stats::Client {
        page_stats::Client(self.clone())
    }
    pub fn pages_client(&self) -> pages::Client {
        pages::Client(self.clone())
    }
    pub fn pages_batch_client(&self) -> pages_batch::Client {
        pages_batch::Client(self.clone())
    }
    pub fn wikis_client(&self) -> wikis::Client {
        wikis::Client(self.clone())
    }
}
pub mod wikis {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all wikis in a project or collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
            }
        }
        #[doc = "Creates the wiki resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Parameters for the wiki creation."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiCreateParametersV2>,
            project: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Gets the wiki corresponding to the wiki ID or wiki name provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            wiki_identifier: impl Into<String>,
            project: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                wiki_identifier: wiki_identifier.into(),
                project: project.into(),
            }
        }
        #[doc = "Updates the wiki corresponding to the wiki ID or wiki name provided using the update parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Update parameters."]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiUpdateParameters>,
            wiki_identifier: impl Into<String>,
            project: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                wiki_identifier: wiki_identifier.into(),
                project: project.into(),
            }
        }
        #[doc = "Deletes the wiki corresponding to the wiki ID or wiki name provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            wiki_identifier: impl Into<String>,
            project: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                wiki_identifier: wiki_identifier.into(),
                project: project.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiV2List, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiV2List> {
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
            pub(crate) organization: String,
            pub(crate) project: String,
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
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis",
                    self.client.endpoint(),
                    self.organization,
                    self.project
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
            type Output = azure_core::Result<models::WikiV2List>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiV2List>>;
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
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiV2, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiV2> {
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
            pub(crate) organization: String,
            pub(crate) body: models::WikiCreateParametersV2,
            pub(crate) project: String,
        }
        impl<M: Clone + Send + 'static> RequestBuilder<M> {
            fn send_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req =
                            azure_core::http::Request::new(url, azure_core::http::Method::Post);
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis",
                    self.client.endpoint(),
                    self.organization,
                    self.project
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
            type Output = azure_core::Result<models::WikiV2>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiV2>>;
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
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiV2, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiV2> {
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
            pub(crate) organization: String,
            pub(crate) wiki_identifier: String,
            pub(crate) project: String,
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
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiV2>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiV2>>;
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
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiV2, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiV2> {
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
            pub(crate) organization: String,
            pub(crate) body: models::WikiUpdateParameters,
            pub(crate) wiki_identifier: String,
            pub(crate) project: String,
        }
        impl<M: Clone + Send + 'static> RequestBuilder<M> {
            fn send_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req =
                            azure_core::http::Request::new(url, azure_core::http::Method::Patch);
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiV2>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiV2>>;
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
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiV2, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiV2> {
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
            pub(crate) organization: String,
            pub(crate) wiki_identifier: String,
            pub(crate) project: String,
        }
        impl<M: Clone + Send + 'static> RequestBuilder<M> {
            fn send_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req =
                            azure_core::http::Request::new(url, azure_core::http::Method::Delete);
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
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiV2>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiV2>>;
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
pub mod attachments {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Creates an attachment in the wiki."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Stream to upload"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `name`: Wiki attachment name."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            name: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                name: name.into(),
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiAttachment, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiAttachment> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) body: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) name: String,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<models::GitVersionOptions>,
            pub(crate) version_descriptor_version_type: Option<models::GitVersionType>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<models::GitVersionOptions>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<models::GitVersionType>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
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
                            azure_core::http::Request::new(url, azure_core::http::Method::Put);
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
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        let name = &this.name;
                        req.url_mut().query_pairs_mut().append_pair("name", name);
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                &version_descriptor_version_options.to_string(),
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                &version_descriptor_version_type.to_string(),
                            );
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/attachments",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiAttachment>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiAttachment>>;
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
pub mod page_moves {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Creates a page move operation that updates the path and order of the page as provided in the parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Page more operation parameters."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPageMoveParameters>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                comment: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiPageMove, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPageMove> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) body: models::WikiPageMoveParameters,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) comment: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<models::GitVersionOptions>,
            pub(crate) version_descriptor_version_type: Option<models::GitVersionType>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Comment that is to be associated with this page move."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<models::GitVersionOptions>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<models::GitVersionType>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
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
                            azure_core::http::Request::new(url, azure_core::http::Method::Post);
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                &version_descriptor_version_options.to_string(),
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                &version_descriptor_version_type.to_string(),
                            );
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pagemoves",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiPageMove>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPageMove>>;
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
pub mod pages {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets metadata or content of the wiki page for the provided path. Content negotiation is done based on the `Accept` header sent in the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        pub fn get_page(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
        ) -> get_page::RequestBuilder {
            get_page::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                path: None,
                recursion_level: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
                include_content: None,
            }
        }
        #[doc = "Creates or edits a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Wiki create or update operation parameters."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `path`: Wiki page path."]
        #[doc = "* `if_match`: Version of the page on which the change is to be made. Mandatory for `Edit` scenario. To be populated in the If-Match header of the request."]
        pub fn create_or_update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPageCreateOrUpdateParameters>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            path: impl Into<String>,
            if_match: impl Into<String>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                path: path.into(),
                if_match: if_match.into(),
                comment: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
        #[doc = "Deletes a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `path`: Wiki page path."]
        pub fn delete_page(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            path: impl Into<String>,
        ) -> delete_page::RequestBuilder {
            delete_page::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                path: path.into(),
                comment: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
        #[doc = "Gets metadata or content of the wiki page for the provided page id. Content negotiation is done based on the `Accept` header sent in the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name.."]
        #[doc = "* `id`: Wiki page ID."]
        pub fn get_page_by_id(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            id: i32,
        ) -> get_page_by_id::RequestBuilder {
            get_page_by_id::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                id,
                recursion_level: None,
                include_content: None,
            }
        }
        #[doc = "Edits a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Wiki update operation parameters."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `id`: Wiki page ID."]
        #[doc = "* `if_match`: Version of the page on which the change is to be made. Mandatory for `Edit` scenario. To be populated in the If-Match header of the request."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPageCreateOrUpdateParameters>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            id: i32,
            if_match: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                id,
                if_match: if_match.into(),
                comment: None,
            }
        }
        #[doc = "Deletes a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `id`: Wiki page ID."]
        pub fn delete_page_by_id(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            id: i32,
        ) -> delete_page_by_id::RequestBuilder {
            delete_page_by_id::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                id,
                comment: None,
            }
        }
    }
    pub mod get_page {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiPage, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPage> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) path: Option<String>,
            pub(crate) recursion_level: Option<models::VersionControlRecursionType>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<models::GitVersionOptions>,
            pub(crate) version_descriptor_version_type: Option<models::GitVersionType>,
            pub(crate) include_content: Option<bool>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Wiki page path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Recursion level for subpages retrieval. Defaults to `None` (Optional)."]
            pub fn recursion_level(
                mut self,
                recursion_level: impl Into<models::VersionControlRecursionType>,
            ) -> Self {
                self.recursion_level = Some(recursion_level.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<models::GitVersionOptions>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<models::GitVersionType>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
                self
            }
            #[doc = "Set to true to include the content of the page in the response for Json content type. Defaults to false (Optional)"]
            pub fn include_content(mut self, include_content: bool) -> Self {
                self.include_content = Some(include_content);
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
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        if let Some(recursion_level) = &this.recursion_level {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("recursionLevel", &recursion_level.to_string());
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                &version_descriptor_version_options.to_string(),
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                &version_descriptor_version_type.to_string(),
                            );
                        }
                        if let Some(include_content) = &this.include_content {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeContent", &include_content.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pages",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiPage>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPage>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiPage, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPage> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) body: models::WikiPageCreateOrUpdateParameters,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) path: String,
            pub(crate) if_match: String,
            pub(crate) comment: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<models::GitVersionOptions>,
            pub(crate) version_descriptor_version_type: Option<models::GitVersionType>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Comment to be associated with the page operation."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<models::GitVersionOptions>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<models::GitVersionType>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
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
                            azure_core::http::Request::new(url, azure_core::http::Method::Put);
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        let path = &this.path;
                        req.url_mut().query_pairs_mut().append_pair("path", path);
                        req.insert_header("if-match", &this.if_match);
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                &version_descriptor_version_options.to_string(),
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                &version_descriptor_version_type.to_string(),
                            );
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pages",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiPage>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPage>>;
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
    pub mod delete_page {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiPage, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPage> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) path: String,
            pub(crate) comment: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<models::GitVersionOptions>,
            pub(crate) version_descriptor_version_type: Option<models::GitVersionType>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Comment to be associated with this page delete."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<models::GitVersionOptions>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<models::GitVersionType>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
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
                            azure_core::http::Request::new(url, azure_core::http::Method::Delete);
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
                        let path = &this.path;
                        req.url_mut().query_pairs_mut().append_pair("path", path);
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                &version_descriptor_version_options.to_string(),
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                &version_descriptor_version_type.to_string(),
                            );
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pages",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiPage>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPage>>;
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
    pub mod get_page_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiPage, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPage> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) id: i32,
            pub(crate) recursion_level: Option<models::VersionControlRecursionType>,
            pub(crate) include_content: Option<bool>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Recursion level for subpages retrieval. Defaults to `None` (Optional)."]
            pub fn recursion_level(
                mut self,
                recursion_level: impl Into<models::VersionControlRecursionType>,
            ) -> Self {
                self.recursion_level = Some(recursion_level.into());
                self
            }
            #[doc = "Set to true to include the content of the page in the response for Json content type. Defaults to false (Optional)"]
            pub fn include_content(mut self, include_content: bool) -> Self {
                self.include_content = Some(include_content);
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
                        if let Some(recursion_level) = &this.recursion_level {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("recursionLevel", &recursion_level.to_string());
                        }
                        if let Some(include_content) = &this.include_content {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeContent", &include_content.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier,
                    self.id
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
            type Output = azure_core::Result<models::WikiPage>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPage>>;
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
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiPage, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPage> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) body: models::WikiPageCreateOrUpdateParameters,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) id: i32,
            pub(crate) if_match: String,
            pub(crate) comment: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Comment to be associated with the page operation."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
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
                            azure_core::http::Request::new(url, azure_core::http::Method::Patch);
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        req.insert_header("if-match", &this.if_match);
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier,
                    self.id
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
            type Output = azure_core::Result<models::WikiPage>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPage>>;
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
    pub mod delete_page_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiPage, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPage> {
                self.0.into_model()
            }
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl Headers<'_> {
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("etag"))
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
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) id: i32,
            pub(crate) comment: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Comment to be associated with this page delete."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
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
                            azure_core::http::Request::new(url, azure_core::http::Method::Delete);
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
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier,
                    self.id
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
            type Output = azure_core::Result<models::WikiPage>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPage>>;
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
pub mod page_stats {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns page detail corresponding to Page ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `page_id`: Wiki page ID."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            page_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                page_id,
                page_views_for_days: None,
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
            azure_core::http::Response<models::WikiPageDetail, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPageDetail> {
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
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) page_id: i32,
            pub(crate) page_views_for_days: Option<i32>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "last N days from the current day for which page views is to be returned. It's inclusive of current day."]
            pub fn page_views_for_days(mut self, page_views_for_days: i32) -> Self {
                self.page_views_for_days = Some(page_views_for_days);
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
                        if let Some(page_views_for_days) = &this.page_views_for_days {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("pageViewsForDays", &page_views_for_days.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}/stats",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier,
                    self.page_id
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
            type Output = azure_core::Result<models::WikiPageDetail>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPageDetail>>;
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
pub mod pages_batch {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns pageable list of Wiki Pages"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Wiki batch page request."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPagesBatchRequest>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
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
            azure_core::http::Response<models::WikiPageDetailList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiPageDetailList> {
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
            pub(crate) organization: String,
            pub(crate) body: models::WikiPagesBatchRequest,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<models::GitVersionOptions>,
            pub(crate) version_descriptor_version_type: Option<models::GitVersionType>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<models::GitVersionOptions>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<models::GitVersionType>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
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
                            azure_core::http::Request::new(url, azure_core::http::Method::Post);
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                &version_descriptor_version_options.to_string(),
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                &version_descriptor_version_type.to_string(),
                            );
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/wiki/wikis/{}/pagesbatch",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.wiki_identifier
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
            type Output = azure_core::Result<models::WikiPageDetailList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiPageDetailList>>;
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
        pub fn attachments_client(&self) -> attachments::Client {
            attachments::Client(self.clone())
        }
        pub fn page_moves_client(&self) -> page_moves::Client {
            page_moves::Client(self.clone())
        }
        pub fn page_stats_client(&self) -> page_stats::Client {
            page_stats::Client(self.clone())
        }
        pub fn pages_client(&self) -> pages::Client {
            pages::Client(self.clone())
        }
        pub fn pages_batch_client(&self) -> pages_batch::Client {
            pages_batch::Client(self.clone())
        }
        pub fn wikis_client(&self) -> wikis::Client {
            wikis::Client(self.clone())
        }
    }
    impl From<super::Client> for Client {
        #[doc = "Create a blocking client that sends requests using an existing async client."]
        fn from(client: super::Client) -> Self {
            Self(client)
        }
    }
    pub mod wikis {
        use super::super::models;
        use super::super::wikis::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Gets all wikis in a project or collection."]
            #[doc = ""]
            #[doc = "See [`list`](super::super::wikis::Client::list) for details of the arguments."]
            pub fn list(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
            ) -> list::RequestBuilder<crate::blocking::Blocking> {
                list::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                }
            }
            #[doc = "Creates the wiki resource."]
            #[doc = ""]
            #[doc = "See [`create`](super::super::wikis::Client::create) for details of the arguments."]
            pub fn create(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WikiCreateParametersV2>,
                project: impl Into<String>,
            ) -> create::RequestBuilder<crate::blocking::Blocking> {
                create::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                }
            }
            #[doc = "Gets the wiki corresponding to the wiki ID or wiki name provided."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::wikis::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                wiki_identifier: impl Into<String>,
                project: impl Into<String>,
            ) -> get::RequestBuilder<crate::blocking::Blocking> {
                get::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    wiki_identifier: wiki_identifier.into(),
                    project: project.into(),
                }
            }
            #[doc = "Updates the wiki corresponding to the wiki ID or wiki name provided using the update parameters."]
            #[doc = ""]
            #[doc = "See [`update`](super::super::wikis::Client::update) for details of the arguments."]
            pub fn update(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WikiUpdateParameters>,
                wiki_identifier: impl Into<String>,
                project: impl Into<String>,
            ) -> update::RequestBuilder<crate::blocking::Blocking> {
                update::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    wiki_identifier: wiki_identifier.into(),
                    project: project.into(),
                }
            }
            #[doc = "Deletes the wiki corresponding to the wiki ID or wiki name provided."]
            #[doc = ""]
            #[doc = "See [`delete`](super::super::wikis::Client::delete) for details of the arguments."]
            pub fn delete(
                &self,
                organization: impl Into<String>,
                wiki_identifier: impl Into<String>,
                project: impl Into<String>,
            ) -> delete::RequestBuilder<crate::blocking::Blocking> {
                delete::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    wiki_identifier: wiki_identifier.into(),
                    project: project.into(),
                }
            }
        }
    }
    pub mod attachments {
        use super::super::attachments::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Creates an attachment in the wiki."]
            #[doc = ""]
            #[doc = "See [`create`](super::super::attachments::Client::create) for details of the arguments."]
            pub fn create(
                &self,
                organization: impl Into<String>,
                body: impl Into<String>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
                name: impl Into<String>,
            ) -> create::RequestBuilder<crate::blocking::Blocking> {
                create::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    name: name.into(),
                    version_descriptor_version: None,
                    version_descriptor_version_options: None,
                    version_descriptor_version_type: None,
                }
            }
        }
    }
    pub mod page_moves {
        use super::super::models;
        use super::super::page_moves::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Creates a page move operation that updates the path and order of the page as provided in the parameters."]
            #[doc = ""]
            #[doc = "See [`create`](super::super::page_moves::Client::create) for details of the arguments."]
            pub fn create(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WikiPageMoveParameters>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
            ) -> create::RequestBuilder<crate::blocking::Blocking> {
                create::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    comment: None,
                    version_descriptor_version: None,
                    version_descriptor_version_options: None,
                    version_descriptor_version_type: None,
                }
            }
        }
    }
    pub mod pages {
        use super::super::models;
        use super::super::pages::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Gets metadata or content of the wiki page for the provided path. Content negotiation is done based on the `Accept` header sent in the request."]
            #[doc = ""]
            #[doc = "See [`get_page`](super::super::pages::Client::get_page) for details of the arguments."]
            pub fn get_page(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
            ) -> get_page::RequestBuilder<crate::blocking::Blocking> {
                get_page::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    path: None,
                    recursion_level: None,
                    version_descriptor_version: None,
                    version_descriptor_version_options: None,
                    version_descriptor_version_type: None,
                    include_content: None,
                }
            }
            #[doc = "Creates or edits a wiki page."]
            #[doc = ""]
            #[doc = "See [`create_or_update`](super::super::pages::Client::create_or_update) for details of the arguments."]
            pub fn create_or_update(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WikiPageCreateOrUpdateParameters>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
                path: impl Into<String>,
                if_match: impl Into<String>,
            ) -> create_or_update::RequestBuilder<crate::blocking::Blocking> {
                create_or_update::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    path: path.into(),
                    if_match: if_match.into(),
                    comment: None,
                    version_descriptor_version: None,
                    version_descriptor_version_options: None,
                    version_descriptor_version_type: None,
                }
            }
            #[doc = "Deletes a wiki page."]
            #[doc = ""]
            #[doc = "See [`delete_page`](super::super::pages::Client::delete_page) for details of the arguments."]
            pub fn delete_page(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
                path: impl Into<String>,
            ) -> delete_page::RequestBuilder<crate::blocking::Blocking> {
                delete_page::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    path: path.into(),
                    comment: None,
                    version_descriptor_version: None,
                    version_descriptor_version_options: None,
                    version_descriptor_version_type: None,
                }
            }
            #[doc = "Gets metadata or content of the wiki page for the provided page id. Content negotiation is done based on the `Accept` header sent in the request."]
            #[doc = ""]
            #[doc = "See [`get_page_by_id`](super::super::pages::Client::get_page_by_id) for details of the arguments."]
            pub fn get_page_by_id(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
                id: i32,
            ) -> get_page_by_id::RequestBuilder<crate::blocking::Blocking> {
                get_page_by_id::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    id,
                    recursion_level: None,
                    include_content: None,
                }
            }
            #[doc = "Edits a wiki page."]
            #[doc = ""]
            #[doc = "See [`update`](super::super::pages::Client::update) for details of the arguments."]
            pub fn update(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WikiPageCreateOrUpdateParameters>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
                id: i32,
                if_match: impl Into<String>,
            ) -> update::RequestBuilder<crate::blocking::Blocking> {
                update::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    id,
                    if_match: if_match.into(),
                    comment: None,
                }
            }
            #[doc = "Deletes a wiki page."]
            #[doc = ""]
            #[doc = "See [`delete_page_by_id`](super::super::pages::Client::delete_page_by_id) for details of the arguments."]
            pub fn delete_page_by_id(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
                id: i32,
            ) -> delete_page_by_id::RequestBuilder<crate::blocking::Blocking> {
                delete_page_by_id::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    id,
                    comment: None,
                }
            }
        }
    }
    pub mod page_stats {
        use super::super::models;
        use super::super::page_stats::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Returns page detail corresponding to Page ID."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::page_stats::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
                page_id: i32,
            ) -> get::RequestBuilder<crate::blocking::Blocking> {
                get::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    page_id,
                    page_views_for_days: None,
                }
            }
        }
    }
    pub mod pages_batch {
        use super::super::models;
        use super::super::pages_batch::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Returns pageable list of Wiki Pages"]
            #[doc = ""]
            #[doc = "See [`get`](super::super::pages_batch::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WikiPagesBatchRequest>,
                project: impl Into<String>,
                wiki_identifier: impl Into<String>,
            ) -> get::RequestBuilder<crate::blocking::Blocking> {
                get::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    wiki_identifier: wiki_identifier.into(),
                    version_descriptor_version: None,
                    version_descriptor_version_options: None,
                    version_descriptor_version_type: None,
                }
            }
        }
    }
}
