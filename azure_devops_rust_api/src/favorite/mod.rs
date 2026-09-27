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
    pub fn favorites_client(&self) -> favorites::Client {
        favorites::Client(self.clone())
    }
}
pub mod favorites {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_favorites_of_owner(
            &self,
            organization: impl Into<String>,
            owner_scope_type: impl Into<String>,
            owner_scope_id: impl Into<String>,
        ) -> get_favorites_of_owner::RequestBuilder {
            get_favorites_of_owner::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                owner_scope_type: owner_scope_type.into(),
                owner_scope_id: owner_scope_id.into(),
                artifact_type: None,
                artifact_scope_type: None,
                artifact_scope_id: None,
                include_extended_details: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn create_favorite_of_owner(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::FavoriteCreateParameters>,
            owner_scope_type: impl Into<String>,
            owner_scope_id: impl Into<String>,
        ) -> create_favorite_of_owner::RequestBuilder {
            create_favorite_of_owner::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                owner_scope_type: owner_scope_type.into(),
                owner_scope_id: owner_scope_id.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_favorite_by_artifact(
            &self,
            organization: impl Into<String>,
            artifact_type: impl Into<String>,
            artifact_id: impl Into<String>,
            artifact_scope_type: impl Into<String>,
        ) -> get_favorite_by_artifact::RequestBuilder {
            get_favorite_by_artifact::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                artifact_type: artifact_type.into(),
                artifact_id: artifact_id.into(),
                artifact_scope_type: artifact_scope_type.into(),
                artifact_scope_id: None,
                include_extended_details: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn delete_favorite_of_owner_by_id(
            &self,
            organization: impl Into<String>,
            favorite_id: impl Into<String>,
            owner_scope_type: impl Into<String>,
            owner_scope_id: impl Into<String>,
            artifact_type: impl Into<String>,
            artifact_scope_type: impl Into<String>,
        ) -> delete_favorite_of_owner_by_id::RequestBuilder {
            delete_favorite_of_owner_by_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                favorite_id: favorite_id.into(),
                owner_scope_type: owner_scope_type.into(),
                owner_scope_id: owner_scope_id.into(),
                artifact_type: artifact_type.into(),
                artifact_scope_type: artifact_scope_type.into(),
                artifact_scope_id: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_favorite_of_owner_by_id(
            &self,
            organization: impl Into<String>,
            favorite_id: impl Into<String>,
            owner_scope_type: impl Into<String>,
            owner_scope_id: impl Into<String>,
            artifact_scope_type: impl Into<String>,
            artifact_type: impl Into<String>,
        ) -> get_favorite_of_owner_by_id::RequestBuilder {
            get_favorite_of_owner_by_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                favorite_id: favorite_id.into(),
                owner_scope_type: owner_scope_type.into(),
                owner_scope_id: owner_scope_id.into(),
                artifact_scope_type: artifact_scope_type.into(),
                artifact_type: artifact_type.into(),
                artifact_scope_id: None,
                include_extended_details: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_favorites(
            &self,
            organization: impl Into<String>,
        ) -> get_favorites::RequestBuilder {
            get_favorites::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                artifact_type: None,
                artifact_scope_type: None,
                artifact_scope_id: None,
                include_extended_details: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn create_favorite(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::FavoriteCreateParameters>,
        ) -> create_favorite::RequestBuilder {
            create_favorite::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_favorite_by_id(
            &self,
            organization: impl Into<String>,
            favorite_id: impl Into<String>,
            artifact_scope_type: impl Into<String>,
            artifact_type: impl Into<String>,
        ) -> get_favorite_by_id::RequestBuilder {
            get_favorite_by_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                favorite_id: favorite_id.into(),
                artifact_scope_type: artifact_scope_type.into(),
                artifact_type: artifact_type.into(),
                artifact_scope_id: None,
                include_extended_details: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn delete_favorite_by_id(
            &self,
            organization: impl Into<String>,
            favorite_id: impl Into<String>,
            artifact_type: impl Into<String>,
            artifact_scope_type: impl Into<String>,
        ) -> delete_favorite_by_id::RequestBuilder {
            delete_favorite_by_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                favorite_id: favorite_id.into(),
                artifact_type: artifact_type.into(),
                artifact_scope_type: artifact_scope_type.into(),
                artifact_scope_id: None,
            }
        }
    }
    pub mod get_favorites_of_owner {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::FavoriteList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::FavoriteList> {
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
            pub(crate) owner_scope_type: String,
            pub(crate) owner_scope_id: String,
            pub(crate) artifact_type: Option<String>,
            pub(crate) artifact_scope_type: Option<String>,
            pub(crate) artifact_scope_id: Option<String>,
            pub(crate) include_extended_details: Option<bool>,
        }
        impl<C> RequestBuilder<C> {
            pub fn artifact_type(mut self, artifact_type: impl Into<String>) -> Self {
                self.artifact_type = Some(artifact_type.into());
                self
            }
            pub fn artifact_scope_type(mut self, artifact_scope_type: impl Into<String>) -> Self {
                self.artifact_scope_type = Some(artifact_scope_type.into());
                self
            }
            pub fn artifact_scope_id(mut self, artifact_scope_id: impl Into<String>) -> Self {
                self.artifact_scope_id = Some(artifact_scope_id.into());
                self
            }
            pub fn include_extended_details(mut self, include_extended_details: bool) -> Self {
                self.include_extended_details = Some(include_extended_details);
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
                let owner_scope_type = &this.owner_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeType", owner_scope_type);
                let owner_scope_id = &this.owner_scope_id;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeId", owner_scope_id);
                if let Some(artifact_type) = &this.artifact_type {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactType", artifact_type);
                }
                if let Some(artifact_scope_type) = &this.artifact_scope_type {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeType", artifact_scope_type);
                }
                if let Some(artifact_scope_id) = &this.artifact_scope_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeId", artifact_scope_id);
                }
                if let Some(include_extended_details) = &this.include_extended_details {
                    req.url_mut().query_pairs_mut().append_pair(
                        "includeExtendedDetails",
                        &include_extended_details.to_string(),
                    );
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/favorite/favorites?ownerScopeType={}&ownerScopeId={}",
                    self.client.endpoint(),
                    self.organization,
                    self.owner_scope_type,
                    self.owner_scope_id
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
            type Output = azure_core::Result<models::FavoriteList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::FavoriteList>>;
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
    pub mod create_favorite_of_owner {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Favorite, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Favorite> {
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
            pub(crate) body: models::FavoriteCreateParameters,
            pub(crate) owner_scope_type: String,
            pub(crate) owner_scope_id: String,
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
                let owner_scope_type = &this.owner_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeType", owner_scope_type);
                let owner_scope_id = &this.owner_scope_id;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeId", owner_scope_id);
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/favorite/favorites?ownerScopeType={}&ownerScopeId={}",
                    self.client.endpoint(),
                    self.organization,
                    self.owner_scope_type,
                    self.owner_scope_id
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
            type Output = azure_core::Result<models::Favorite>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Favorite>>;
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
    pub mod get_favorite_by_artifact {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Favorite, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Favorite> {
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
            pub(crate) artifact_type: String,
            pub(crate) artifact_id: String,
            pub(crate) artifact_scope_type: String,
            pub(crate) artifact_scope_id: Option<String>,
            pub(crate) include_extended_details: Option<bool>,
        }
        impl<C> RequestBuilder<C> {
            pub fn artifact_scope_id(mut self, artifact_scope_id: impl Into<String>) -> Self {
                self.artifact_scope_id = Some(artifact_scope_id.into());
                self
            }
            pub fn include_extended_details(mut self, include_extended_details: bool) -> Self {
                self.include_extended_details = Some(include_extended_details);
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
                let artifact_type = &this.artifact_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactType", artifact_type);
                let artifact_id = &this.artifact_id;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactId", artifact_id);
                let artifact_scope_type = &this.artifact_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactScopeType", artifact_scope_type);
                if let Some(artifact_scope_id) = &this.artifact_scope_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeId", artifact_scope_id);
                }
                if let Some(include_extended_details) = &this.include_extended_details {
                    req.url_mut().query_pairs_mut().append_pair(
                        "includeExtendedDetails",
                        &include_extended_details.to_string(),
                    );
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core :: http :: Url :: parse (& format ! ("{}/{}/_apis/favorite/favorites?artifactType={}&artifactId={}&artifactScopeType={}" , self . client . endpoint () , self . organization , self . artifact_type , self . artifact_id , self . artifact_scope_type)) ? ;
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
            type Output = azure_core::Result<models::Favorite>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Favorite>>;
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
    pub mod delete_favorite_of_owner_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<(), azure_core::http::NoFormat>);
        impl Response {
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
            pub(crate) favorite_id: String,
            pub(crate) owner_scope_type: String,
            pub(crate) owner_scope_id: String,
            pub(crate) artifact_type: String,
            pub(crate) artifact_scope_type: String,
            pub(crate) artifact_scope_id: Option<String>,
        }
        impl<C> RequestBuilder<C> {
            pub fn artifact_scope_id(mut self, artifact_scope_id: impl Into<String>) -> Self {
                self.artifact_scope_id = Some(artifact_scope_id.into());
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
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                let owner_scope_type = &this.owner_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeType", owner_scope_type);
                let owner_scope_id = &this.owner_scope_id;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeId", owner_scope_id);
                let artifact_type = &this.artifact_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactType", artifact_type);
                let artifact_scope_type = &this.artifact_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactScopeType", artifact_scope_type);
                if let Some(artifact_scope_id) = &this.artifact_scope_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeId", artifact_scope_id);
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core :: http :: Url :: parse (& format ! ("{}/{}/_apis/favorite/favorites/{}?ownerScopeType={}&ownerScopeId={}&artifactType={}&artifactScopeType={}" , self . client . endpoint () , self . organization , self . favorite_id , self . owner_scope_type , self . owner_scope_id , self . artifact_type , self . artifact_scope_type)) ? ;
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
            type Output = azure_core::Result<()>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<()>>;
            #[doc = "Returns a future that sends the request and waits for the response."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    let _rsp = self.send().await?;
                    Ok(())
                })
            }
        }
    }
    pub mod get_favorite_of_owner_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Favorite, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Favorite> {
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
            pub(crate) favorite_id: String,
            pub(crate) owner_scope_type: String,
            pub(crate) owner_scope_id: String,
            pub(crate) artifact_scope_type: String,
            pub(crate) artifact_type: String,
            pub(crate) artifact_scope_id: Option<String>,
            pub(crate) include_extended_details: Option<bool>,
        }
        impl<C> RequestBuilder<C> {
            pub fn artifact_scope_id(mut self, artifact_scope_id: impl Into<String>) -> Self {
                self.artifact_scope_id = Some(artifact_scope_id.into());
                self
            }
            pub fn include_extended_details(mut self, include_extended_details: bool) -> Self {
                self.include_extended_details = Some(include_extended_details);
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
                let owner_scope_type = &this.owner_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeType", owner_scope_type);
                let owner_scope_id = &this.owner_scope_id;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("ownerScopeId", owner_scope_id);
                let artifact_scope_type = &this.artifact_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactScopeType", artifact_scope_type);
                let artifact_type = &this.artifact_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactType", artifact_type);
                if let Some(artifact_scope_id) = &this.artifact_scope_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeId", artifact_scope_id);
                }
                if let Some(include_extended_details) = &this.include_extended_details {
                    req.url_mut().query_pairs_mut().append_pair(
                        "includeExtendedDetails",
                        &include_extended_details.to_string(),
                    );
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core :: http :: Url :: parse (& format ! ("{}/{}/_apis/favorite/favorites/{}?ownerScopeType={}&ownerScopeId={}&artifactScopeType={}&artifactType={}" , self . client . endpoint () , self . organization , self . favorite_id , self . owner_scope_type , self . owner_scope_id , self . artifact_scope_type , self . artifact_type)) ? ;
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
            type Output = azure_core::Result<models::Favorite>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Favorite>>;
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
    pub mod get_favorites {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::FavoriteList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::FavoriteList> {
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
            pub(crate) artifact_type: Option<String>,
            pub(crate) artifact_scope_type: Option<String>,
            pub(crate) artifact_scope_id: Option<String>,
            pub(crate) include_extended_details: Option<bool>,
        }
        impl<C> RequestBuilder<C> {
            pub fn artifact_type(mut self, artifact_type: impl Into<String>) -> Self {
                self.artifact_type = Some(artifact_type.into());
                self
            }
            pub fn artifact_scope_type(mut self, artifact_scope_type: impl Into<String>) -> Self {
                self.artifact_scope_type = Some(artifact_scope_type.into());
                self
            }
            pub fn artifact_scope_id(mut self, artifact_scope_id: impl Into<String>) -> Self {
                self.artifact_scope_id = Some(artifact_scope_id.into());
                self
            }
            pub fn include_extended_details(mut self, include_extended_details: bool) -> Self {
                self.include_extended_details = Some(include_extended_details);
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
                if let Some(artifact_type) = &this.artifact_type {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactType", artifact_type);
                }
                if let Some(artifact_scope_type) = &this.artifact_scope_type {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeType", artifact_scope_type);
                }
                if let Some(artifact_scope_id) = &this.artifact_scope_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeId", artifact_scope_id);
                }
                if let Some(include_extended_details) = &this.include_extended_details {
                    req.url_mut().query_pairs_mut().append_pair(
                        "includeExtendedDetails",
                        &include_extended_details.to_string(),
                    );
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/favorite/favorites",
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
            type Output = azure_core::Result<models::FavoriteList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::FavoriteList>>;
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
    pub mod create_favorite {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Favorite, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Favorite> {
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
            pub(crate) body: models::FavoriteCreateParameters,
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
                    "{}/{}/_apis/favorite/favorites",
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
            type Output = azure_core::Result<models::Favorite>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Favorite>>;
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
    pub mod get_favorite_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Favorite, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Favorite> {
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
            pub(crate) favorite_id: String,
            pub(crate) artifact_scope_type: String,
            pub(crate) artifact_type: String,
            pub(crate) artifact_scope_id: Option<String>,
            pub(crate) include_extended_details: Option<bool>,
        }
        impl<C> RequestBuilder<C> {
            pub fn artifact_scope_id(mut self, artifact_scope_id: impl Into<String>) -> Self {
                self.artifact_scope_id = Some(artifact_scope_id.into());
                self
            }
            pub fn include_extended_details(mut self, include_extended_details: bool) -> Self {
                self.include_extended_details = Some(include_extended_details);
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
                let artifact_scope_type = &this.artifact_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactScopeType", artifact_scope_type);
                let artifact_type = &this.artifact_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactType", artifact_type);
                if let Some(artifact_scope_id) = &this.artifact_scope_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeId", artifact_scope_id);
                }
                if let Some(include_extended_details) = &this.include_extended_details {
                    req.url_mut().query_pairs_mut().append_pair(
                        "includeExtendedDetails",
                        &include_extended_details.to_string(),
                    );
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/favorite/favorites/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.favorite_id
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
            type Output = azure_core::Result<models::Favorite>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Favorite>>;
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
    pub mod delete_favorite_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<(), azure_core::http::NoFormat>);
        impl Response {
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
            pub(crate) favorite_id: String,
            pub(crate) artifact_type: String,
            pub(crate) artifact_scope_type: String,
            pub(crate) artifact_scope_id: Option<String>,
        }
        impl<C> RequestBuilder<C> {
            pub fn artifact_scope_id(mut self, artifact_scope_id: impl Into<String>) -> Self {
                self.artifact_scope_id = Some(artifact_scope_id.into());
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
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                let artifact_type = &this.artifact_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactType", artifact_type);
                let artifact_scope_type = &this.artifact_scope_type;
                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("artifactScopeType", artifact_scope_type);
                if let Some(artifact_scope_id) = &this.artifact_scope_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("artifactScopeId", artifact_scope_id);
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/favorite/favorites/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.favorite_id
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
            type Output = azure_core::Result<()>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<()>>;
            #[doc = "Returns a future that sends the request and waits for the response."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    let _rsp = self.send().await?;
                    Ok(())
                })
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
        pub fn favorites_client(&self) -> favorites::Client {
            favorites::Client(self.clone())
        }
    }
    impl crate::ClientEndpoint for Client {
        fn endpoint(&self) -> &azure_core::http::Url {
            &self.endpoint
        }
    }
    pub mod favorites {
        use super::super::favorites::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "See [`get_favorites_of_owner`](super::super::favorites::Client::get_favorites_of_owner) for details of the arguments."]
            pub fn get_favorites_of_owner(
                &self,
                organization: impl Into<String>,
                owner_scope_type: impl Into<String>,
                owner_scope_id: impl Into<String>,
            ) -> get_favorites_of_owner::RequestBuilder<super::Client> {
                get_favorites_of_owner::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    owner_scope_type: owner_scope_type.into(),
                    owner_scope_id: owner_scope_id.into(),
                    artifact_type: None,
                    artifact_scope_type: None,
                    artifact_scope_id: None,
                    include_extended_details: None,
                }
            }
            #[doc = "See [`create_favorite_of_owner`](super::super::favorites::Client::create_favorite_of_owner) for details of the arguments."]
            pub fn create_favorite_of_owner(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::FavoriteCreateParameters>,
                owner_scope_type: impl Into<String>,
                owner_scope_id: impl Into<String>,
            ) -> create_favorite_of_owner::RequestBuilder<super::Client> {
                create_favorite_of_owner::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    owner_scope_type: owner_scope_type.into(),
                    owner_scope_id: owner_scope_id.into(),
                }
            }
            #[doc = "See [`get_favorite_by_artifact`](super::super::favorites::Client::get_favorite_by_artifact) for details of the arguments."]
            pub fn get_favorite_by_artifact(
                &self,
                organization: impl Into<String>,
                artifact_type: impl Into<String>,
                artifact_id: impl Into<String>,
                artifact_scope_type: impl Into<String>,
            ) -> get_favorite_by_artifact::RequestBuilder<super::Client> {
                get_favorite_by_artifact::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    artifact_type: artifact_type.into(),
                    artifact_id: artifact_id.into(),
                    artifact_scope_type: artifact_scope_type.into(),
                    artifact_scope_id: None,
                    include_extended_details: None,
                }
            }
            #[doc = "See [`delete_favorite_of_owner_by_id`](super::super::favorites::Client::delete_favorite_of_owner_by_id) for details of the arguments."]
            pub fn delete_favorite_of_owner_by_id(
                &self,
                organization: impl Into<String>,
                favorite_id: impl Into<String>,
                owner_scope_type: impl Into<String>,
                owner_scope_id: impl Into<String>,
                artifact_type: impl Into<String>,
                artifact_scope_type: impl Into<String>,
            ) -> delete_favorite_of_owner_by_id::RequestBuilder<super::Client> {
                delete_favorite_of_owner_by_id::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    favorite_id: favorite_id.into(),
                    owner_scope_type: owner_scope_type.into(),
                    owner_scope_id: owner_scope_id.into(),
                    artifact_type: artifact_type.into(),
                    artifact_scope_type: artifact_scope_type.into(),
                    artifact_scope_id: None,
                }
            }
            #[doc = "See [`get_favorite_of_owner_by_id`](super::super::favorites::Client::get_favorite_of_owner_by_id) for details of the arguments."]
            pub fn get_favorite_of_owner_by_id(
                &self,
                organization: impl Into<String>,
                favorite_id: impl Into<String>,
                owner_scope_type: impl Into<String>,
                owner_scope_id: impl Into<String>,
                artifact_scope_type: impl Into<String>,
                artifact_type: impl Into<String>,
            ) -> get_favorite_of_owner_by_id::RequestBuilder<super::Client> {
                get_favorite_of_owner_by_id::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    favorite_id: favorite_id.into(),
                    owner_scope_type: owner_scope_type.into(),
                    owner_scope_id: owner_scope_id.into(),
                    artifact_scope_type: artifact_scope_type.into(),
                    artifact_type: artifact_type.into(),
                    artifact_scope_id: None,
                    include_extended_details: None,
                }
            }
            #[doc = "See [`get_favorites`](super::super::favorites::Client::get_favorites) for details of the arguments."]
            pub fn get_favorites(
                &self,
                organization: impl Into<String>,
            ) -> get_favorites::RequestBuilder<super::Client> {
                get_favorites::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    artifact_type: None,
                    artifact_scope_type: None,
                    artifact_scope_id: None,
                    include_extended_details: None,
                }
            }
            #[doc = "See [`create_favorite`](super::super::favorites::Client::create_favorite) for details of the arguments."]
            pub fn create_favorite(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::FavoriteCreateParameters>,
            ) -> create_favorite::RequestBuilder<super::Client> {
                create_favorite::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                }
            }
            #[doc = "See [`get_favorite_by_id`](super::super::favorites::Client::get_favorite_by_id) for details of the arguments."]
            pub fn get_favorite_by_id(
                &self,
                organization: impl Into<String>,
                favorite_id: impl Into<String>,
                artifact_scope_type: impl Into<String>,
                artifact_type: impl Into<String>,
            ) -> get_favorite_by_id::RequestBuilder<super::Client> {
                get_favorite_by_id::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    favorite_id: favorite_id.into(),
                    artifact_scope_type: artifact_scope_type.into(),
                    artifact_type: artifact_type.into(),
                    artifact_scope_id: None,
                    include_extended_details: None,
                }
            }
            #[doc = "See [`delete_favorite_by_id`](super::super::favorites::Client::delete_favorite_by_id) for details of the arguments."]
            pub fn delete_favorite_by_id(
                &self,
                organization: impl Into<String>,
                favorite_id: impl Into<String>,
                artifact_type: impl Into<String>,
                artifact_scope_type: impl Into<String>,
            ) -> delete_favorite_by_id::RequestBuilder<super::Client> {
                delete_favorite_by_id::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    favorite_id: favorite_id.into(),
                    artifact_type: artifact_type.into(),
                    artifact_scope_type: artifact_scope_type.into(),
                    artifact_scope_id: None,
                }
            }
        }
    }
}
