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
azure_core::static_url!(DEFAULT_ENDPOINT, "https://almsearch.dev.azure.com");
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
    pub fn code_search_results_client(&self) -> code_search_results::Client {
        code_search_results::Client(self.clone())
    }
    pub fn package_search_results_client(&self) -> package_search_results::Client {
        package_search_results::Client(self.clone())
    }
    pub fn repositories_client(&self) -> repositories::Client {
        repositories::Client(self.clone())
    }
    pub fn tfvc_client(&self) -> tfvc::Client {
        tfvc::Client(self.clone())
    }
    pub fn wiki_search_results_client(&self) -> wiki_search_results::Client {
        wiki_search_results::Client(self.clone())
    }
    pub fn work_item_search_results_client(&self) -> work_item_search_results::Client {
        work_item_search_results::Client(self.clone())
    }
}
pub mod package_search_results {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a set of results for the search text."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The Package Search Request."]
        pub fn fetch_package_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageSearchRequest>,
        ) -> fetch_package_search_results::RequestBuilder {
            fetch_package_search_results::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod fetch_package_search_results {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::PackageSearchResponseContent,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::PackageSearchResponseContent> {
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
            pub fn activity_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static(
                        "activityid",
                    ))
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
            pub(crate) organization: String,
            pub(crate) body: models::PackageSearchRequest,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/search/packagesearchresults",
                    self.client.endpoint(),
                    self.organization
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
            type Output = azure_core::Result<models::PackageSearchResponseContent>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::PackageSearchResponseContent>>;
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
pub mod code_search_results {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a set of results for the search text."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The Code Search Request."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn fetch_code_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CodeSearchRequest>,
            project: impl Into<String>,
        ) -> fetch_code_search_results::RequestBuilder {
            fetch_code_search_results::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod fetch_code_search_results {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::CodeSearchResponse, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CodeSearchResponse> {
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
            pub(crate) organization: String,
            pub(crate) body: models::CodeSearchRequest,
            pub(crate) project: String,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/search/codesearchresults",
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
            type Output = azure_core::Result<models::CodeSearchResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CodeSearchResponse>>;
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
pub mod repositories {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides status of Repository."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `repository`: Repository ID or repository name."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            repository: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                repository: repository.into(),
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
            azure_core::http::Response<
                models::RepositoryStatusResponse,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::RepositoryStatusResponse> {
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
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) repository: String,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/search/status/repositories/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.repository
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
            type Output = azure_core::Result<models::RepositoryStatusResponse>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::RepositoryStatusResponse>>;
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
pub mod tfvc {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides status of TFVC Repository."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
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
            azure_core::http::Response<
                models::TfvcRepositoryStatusResponse,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::TfvcRepositoryStatusResponse> {
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
            pub(crate) organization: String,
            pub(crate) project: String,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/search/status/tfvc",
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
            type Output = azure_core::Result<models::TfvcRepositoryStatusResponse>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::TfvcRepositoryStatusResponse>>;
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
pub mod wiki_search_results {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a set of results for the search request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The Wiki Search Request."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn fetch_wiki_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiSearchRequest>,
            project: impl Into<String>,
        ) -> fetch_wiki_search_results::RequestBuilder {
            fetch_wiki_search_results::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod fetch_wiki_search_results {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WikiSearchResponse, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WikiSearchResponse> {
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
            pub(crate) organization: String,
            pub(crate) body: models::WikiSearchRequest,
            pub(crate) project: String,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/search/wikisearchresults",
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
            type Output = azure_core::Result<models::WikiSearchResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WikiSearchResponse>>;
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
pub mod work_item_search_results {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a set of results for the search text."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The Work Item Search Request."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn fetch_work_item_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemSearchRequest>,
            project: impl Into<String>,
        ) -> fetch_work_item_search_results::RequestBuilder {
            fetch_work_item_search_results::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod fetch_work_item_search_results {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::WorkItemSearchResponse,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WorkItemSearchResponse> {
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
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemSearchRequest,
            pub(crate) project: String,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/search/workitemsearchresults",
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
            type Output = azure_core::Result<models::WorkItemSearchResponse>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::WorkItemSearchResponse>>;
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
        pub fn code_search_results_client(&self) -> code_search_results::Client {
            code_search_results::Client(self.clone())
        }
        pub fn package_search_results_client(&self) -> package_search_results::Client {
            package_search_results::Client(self.clone())
        }
        pub fn repositories_client(&self) -> repositories::Client {
            repositories::Client(self.clone())
        }
        pub fn tfvc_client(&self) -> tfvc::Client {
            tfvc::Client(self.clone())
        }
        pub fn wiki_search_results_client(&self) -> wiki_search_results::Client {
            wiki_search_results::Client(self.clone())
        }
        pub fn work_item_search_results_client(&self) -> work_item_search_results::Client {
            work_item_search_results::Client(self.clone())
        }
    }
    impl crate::ClientEndpoint for Client {
        fn endpoint(&self) -> &azure_core::http::Url {
            &self.endpoint
        }
    }
    pub mod package_search_results {
        use super::super::models;
        use super::super::package_search_results::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Provides a set of results for the search text."]
            #[doc = ""]
            #[doc = "See [`fetch_package_search_results`](super::super::package_search_results::Client::fetch_package_search_results) for details of the arguments."]
            pub fn fetch_package_search_results(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::PackageSearchRequest>,
            ) -> fetch_package_search_results::RequestBuilder<super::Client> {
                fetch_package_search_results::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                }
            }
        }
    }
    pub mod code_search_results {
        use super::super::code_search_results::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Provides a set of results for the search text."]
            #[doc = ""]
            #[doc = "See [`fetch_code_search_results`](super::super::code_search_results::Client::fetch_code_search_results) for details of the arguments."]
            pub fn fetch_code_search_results(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::CodeSearchRequest>,
                project: impl Into<String>,
            ) -> fetch_code_search_results::RequestBuilder<super::Client> {
                fetch_code_search_results::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                }
            }
        }
    }
    pub mod repositories {
        use super::super::models;
        use super::super::repositories::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Provides status of Repository."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::repositories::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                repository: impl Into<String>,
            ) -> get::RequestBuilder<super::Client> {
                get::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    repository: repository.into(),
                }
            }
        }
    }
    pub mod tfvc {
        use super::super::models;
        use super::super::tfvc::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Provides status of TFVC Repository."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::tfvc::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
            ) -> get::RequestBuilder<super::Client> {
                get::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                }
            }
        }
    }
    pub mod wiki_search_results {
        use super::super::models;
        use super::super::wiki_search_results::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Provides a set of results for the search request."]
            #[doc = ""]
            #[doc = "See [`fetch_wiki_search_results`](super::super::wiki_search_results::Client::fetch_wiki_search_results) for details of the arguments."]
            pub fn fetch_wiki_search_results(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WikiSearchRequest>,
                project: impl Into<String>,
            ) -> fetch_wiki_search_results::RequestBuilder<super::Client> {
                fetch_wiki_search_results::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                }
            }
        }
    }
    pub mod work_item_search_results {
        use super::super::models;
        use super::super::work_item_search_results::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Provides a set of results for the search text."]
            #[doc = ""]
            #[doc = "See [`fetch_work_item_search_results`](super::super::work_item_search_results::Client::fetch_work_item_search_results) for details of the arguments."]
            pub fn fetch_work_item_search_results(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::WorkItemSearchRequest>,
                project: impl Into<String>,
            ) -> fetch_work_item_search_results::RequestBuilder<super::Client> {
                fetch_work_item_search_results::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                }
            }
        }
    }
}
