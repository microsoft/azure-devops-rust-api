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
    pub fn approvals_client(&self) -> approvals::Client {
        approvals::Client(self.clone())
    }
    pub fn check_configurations_client(&self) -> check_configurations::Client {
        check_configurations::Client(self.clone())
    }
    pub fn check_evaluations_client(&self) -> check_evaluations::Client {
        check_evaluations::Client(self.clone())
    }
    pub fn pipeline_permissions_client(&self) -> pipeline_permissions::Client {
        pipeline_permissions::Client(self.clone())
    }
}
pub mod pipeline_permissions {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Batch API to authorize/unauthorize a list of definitions for a multiple resources."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_pipeline_permisions_for_resources(
            &self,
            organization: impl Into<String>,
            body: Vec<models::ResourcePipelinePermissions>,
            project: impl Into<String>,
        ) -> update_pipeline_permisions_for_resources::RequestBuilder {
            update_pipeline_permisions_for_resources::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
            }
        }
        #[doc = "Given a ResourceType and ResourceId, returns authorized definitions for that resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            resource_type: impl Into<String>,
            resource_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                resource_type: resource_type.into(),
                resource_id: resource_id.into(),
            }
        }
        #[doc = "Authorizes/Unauthorizes a list of definitions for a given resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_pipeline_permisions_for_resource(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ResourcePipelinePermissions>,
            project: impl Into<String>,
            resource_type: impl Into<String>,
            resource_id: impl Into<String>,
        ) -> update_pipeline_permisions_for_resource::RequestBuilder {
            update_pipeline_permisions_for_resource::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                resource_type: resource_type.into(),
                resource_id: resource_id.into(),
            }
        }
    }
    pub mod update_pipeline_permisions_for_resources {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::ResourcePipelinePermissionsList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ResourcePipelinePermissionsList> {
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
            pub(crate) body: Vec<models::ResourcePipelinePermissions>,
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
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/pipelinepermissions",
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
            type Output = azure_core::Result<models::ResourcePipelinePermissionsList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::ResourcePipelinePermissionsList>>;
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
            azure_core::http::Response<
                models::ResourcePipelinePermissions,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ResourcePipelinePermissions> {
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
            pub(crate) resource_type: String,
            pub(crate) resource_id: String,
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
                    "{}/{}/{}/_apis/pipelines/pipelinepermissions/{}/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.resource_type,
                    self.resource_id
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
            type Output = azure_core::Result<models::ResourcePipelinePermissions>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::ResourcePipelinePermissions>>;
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
    pub mod update_pipeline_permisions_for_resource {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::ResourcePipelinePermissions,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ResourcePipelinePermissions> {
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
            pub(crate) body: models::ResourcePipelinePermissions,
            pub(crate) project: String,
            pub(crate) resource_type: String,
            pub(crate) resource_id: String,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/pipelinepermissions/{}/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.resource_type,
                    self.resource_id
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
            type Output = azure_core::Result<models::ResourcePipelinePermissions>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::ResourcePipelinePermissions>>;
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
pub mod check_configurations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get Check configuration by resource type and id"]
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
                organization: organization.into(),
                project: project.into(),
                resource_type: None,
                resource_id: None,
                expand: None,
            }
        }
        #[doc = "Add a check configuration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CheckConfiguration>,
            project: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get Check configuration by Id"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id,
                expand: None,
            }
        }
        #[doc = "Update check configuration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: check configuration"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: check configuration id"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CheckConfiguration>,
            project: impl Into<String>,
            id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                id,
            }
        }
        #[doc = "Delete check configuration by id"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: check configuration id"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id,
            }
        }
        #[doc = "Get check configurations for multiple resources by resource type and id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: List of resources."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn query(
            &self,
            organization: impl Into<String>,
            body: Vec<models::Resource>,
            project: impl Into<String>,
        ) -> query::RequestBuilder {
            query::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                expand: None,
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
            azure_core::http::Response<
                models::CheckConfigurationList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CheckConfigurationList> {
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
            pub(crate) resource_type: Option<String>,
            pub(crate) resource_id: Option<String>,
            pub(crate) expand: Option<models::CheckConfigurationExpandParameter>,
        }
        impl<C> RequestBuilder<C> {
            #[doc = "resource type"]
            pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
                self.resource_type = Some(resource_type.into());
                self
            }
            #[doc = "resource id"]
            pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
                self.resource_id = Some(resource_id.into());
                self
            }
            pub fn expand(
                mut self,
                expand: impl Into<models::CheckConfigurationExpandParameter>,
            ) -> Self {
                self.expand = Some(expand.into());
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
                if let Some(resource_type) = &this.resource_type {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("resourceType", resource_type);
                }
                if let Some(resource_id) = &this.resource_id {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("resourceId", resource_id);
                }
                if let Some(expand) = &this.expand {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("$expand", &expand.to_string());
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/checks/configurations",
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
            type Output = azure_core::Result<models::CheckConfigurationList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::CheckConfigurationList>>;
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
    pub mod add {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::CheckConfiguration, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CheckConfiguration> {
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
            pub(crate) body: models::CheckConfiguration,
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
                    "{}/{}/{}/_apis/pipelines/checks/configurations",
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
            type Output = azure_core::Result<models::CheckConfiguration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckConfiguration>>;
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
            azure_core::http::Response<models::CheckConfiguration, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CheckConfiguration> {
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
            pub(crate) id: i32,
            pub(crate) expand: Option<models::CheckConfigurationExpandParameter>,
        }
        impl<C> RequestBuilder<C> {
            pub fn expand(
                mut self,
                expand: impl Into<models::CheckConfigurationExpandParameter>,
            ) -> Self {
                self.expand = Some(expand.into());
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
                if let Some(expand) = &this.expand {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("$expand", &expand.to_string());
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/checks/configurations/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
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
            type Output = azure_core::Result<models::CheckConfiguration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckConfiguration>>;
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
            azure_core::http::Response<models::CheckConfiguration, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CheckConfiguration> {
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
            pub(crate) body: models::CheckConfiguration,
            pub(crate) project: String,
            pub(crate) id: i32,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/checks/configurations/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
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
            type Output = azure_core::Result<models::CheckConfiguration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckConfiguration>>;
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
            pub(crate) project: String,
            pub(crate) id: i32,
        }
        impl<C> RequestBuilder<C> {}
        impl<C: crate::ClientEndpoint> RequestBuilder<C> {
            #[doc = "Builds the HTTP request for this operation without sending it."]
            #[doc = ""]
            #[doc = "The returned request does not include an `Authorization` header."]
            pub fn build_request(&self) -> azure_core::Result<azure_core::http::Request> {
                let this = self;
                let url = this.url()?;
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/checks/configurations/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
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
    pub mod query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::CheckConfigurationList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CheckConfigurationList> {
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
            pub(crate) body: Vec<models::Resource>,
            pub(crate) project: String,
            pub(crate) expand: Option<models::CheckConfigurationExpandParameter>,
        }
        impl<C> RequestBuilder<C> {
            #[doc = "The properties that should be expanded in the list of check configurations."]
            pub fn expand(
                mut self,
                expand: impl Into<models::CheckConfigurationExpandParameter>,
            ) -> Self {
                self.expand = Some(expand.into());
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
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                if let Some(expand) = &this.expand {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("$expand", &expand.to_string());
                }
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/checks/queryconfigurations",
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
            type Output = azure_core::Result<models::CheckConfigurationList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::CheckConfigurationList>>;
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
pub mod check_evaluations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Initiate an evaluation for a check in a pipeline"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn evaluate(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CheckSuiteRequest>,
            project: impl Into<String>,
        ) -> evaluate::RequestBuilder {
            evaluate::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                expand: None,
            }
        }
        #[doc = "Get details for a specific check evaluation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            check_suite_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                check_suite_id: check_suite_id.into(),
                expand: None,
            }
        }
    }
    pub mod evaluate {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::CheckSuite, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CheckSuite> {
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
            pub(crate) body: models::CheckSuiteRequest,
            pub(crate) project: String,
            pub(crate) expand: Option<models::CheckSuiteExpandParameter>,
        }
        impl<C> RequestBuilder<C> {
            pub fn expand(mut self, expand: impl Into<models::CheckSuiteExpandParameter>) -> Self {
                self.expand = Some(expand.into());
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
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                if let Some(expand) = &this.expand {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("$expand", &expand.to_string());
                }
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/checks/runs",
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
            type Output = azure_core::Result<models::CheckSuite>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckSuite>>;
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
            azure_core::http::Response<models::CheckSuite, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::CheckSuite> {
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
            pub(crate) check_suite_id: String,
            pub(crate) expand: Option<models::CheckSuiteExpandParameter>,
        }
        impl<C> RequestBuilder<C> {
            pub fn expand(mut self, expand: impl Into<models::CheckSuiteExpandParameter>) -> Self {
                self.expand = Some(expand.into());
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
                if let Some(expand) = &this.expand {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("$expand", &expand.to_string());
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/checks/runs/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.check_suite_id
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
            type Output = azure_core::Result<models::CheckSuite>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckSuite>>;
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
pub mod approvals {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List Approvals. This can be used to get a set of pending approvals in a pipeline, on an user or for a resource.."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn query(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> query::RequestBuilder {
            query::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                approval_ids: None,
                expand: None,
                user_ids: None,
                state: None,
                top: None,
            }
        }
        #[doc = "Update approvals."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: Vec<models::ApprovalUpdateParameters>,
            project: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
            }
        }
        #[doc = "Get an approval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `approval_id`: Id of the approval."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            approval_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                approval_id: approval_id.into(),
                expand: None,
            }
        }
    }
    pub mod query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::ApprovalList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ApprovalList> {
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
            pub(crate) approval_ids: Option<String>,
            pub(crate) expand: Option<models::ApprovalDetailsExpandParameter>,
            pub(crate) user_ids: Option<String>,
            pub(crate) state: Option<models::ApprovalStatus>,
            pub(crate) top: Option<i32>,
        }
        impl<C> RequestBuilder<C> {
            #[doc = "List of approval Ids to get."]
            pub fn approval_ids(mut self, approval_ids: impl Into<String>) -> Self {
                self.approval_ids = Some(approval_ids.into());
                self
            }
            #[doc = "Include these additional details in the returned objects."]
            pub fn expand(
                mut self,
                expand: impl Into<models::ApprovalDetailsExpandParameter>,
            ) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "List of user Ids approvals assigned to. Accepts either user Ids or user descriptors."]
            pub fn user_ids(mut self, user_ids: impl Into<String>) -> Self {
                self.user_ids = Some(user_ids.into());
                self
            }
            #[doc = "Approval status. Returns approvals of any status if not provided"]
            pub fn state(mut self, state: impl Into<models::ApprovalStatus>) -> Self {
                self.state = Some(state.into());
                self
            }
            #[doc = "Maximum number of approvals to get."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
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
                if let Some(approval_ids) = &this.approval_ids {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("approvalIds", approval_ids);
                }
                if let Some(expand) = &this.expand {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("$expand", &expand.to_string());
                }
                if let Some(user_ids) = &this.user_ids {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("userIds", user_ids);
                }
                if let Some(state) = &this.state {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("state", &state.to_string());
                }
                if let Some(top) = &this.top {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("top", &top.to_string());
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/approvals",
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
            type Output = azure_core::Result<models::ApprovalList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ApprovalList>>;
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
            azure_core::http::Response<models::ApprovalList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ApprovalList> {
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
            pub(crate) body: Vec<models::ApprovalUpdateParameters>,
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
                let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                req.insert_header("content-type", "application/json");
                let req_body = azure_core::json::to_json(&this.body)?;
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/approvals",
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
            type Output = azure_core::Result<models::ApprovalList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ApprovalList>>;
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
            azure_core::http::Response<models::Approval, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Approval> {
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
            pub(crate) approval_id: String,
            pub(crate) expand: Option<models::ApprovalDetailsExpandParameter>,
        }
        impl<C> RequestBuilder<C> {
            pub fn expand(
                mut self,
                expand: impl Into<models::ApprovalDetailsExpandParameter>,
            ) -> Self {
                self.expand = Some(expand.into());
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
                if let Some(expand) = &this.expand {
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair("$expand", &expand.to_string());
                }
                let req_body = azure_core::Bytes::new();
                req.set_body(req_body);
                Ok(req)
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/pipelines/approvals/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.approval_id
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
            type Output = azure_core::Result<models::Approval>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Approval>>;
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
        pub fn approvals_client(&self) -> approvals::Client {
            approvals::Client(self.clone())
        }
        pub fn check_configurations_client(&self) -> check_configurations::Client {
            check_configurations::Client(self.clone())
        }
        pub fn check_evaluations_client(&self) -> check_evaluations::Client {
            check_evaluations::Client(self.clone())
        }
        pub fn pipeline_permissions_client(&self) -> pipeline_permissions::Client {
            pipeline_permissions::Client(self.clone())
        }
    }
    impl crate::ClientEndpoint for Client {
        fn endpoint(&self) -> &azure_core::http::Url {
            &self.endpoint
        }
    }
    pub mod pipeline_permissions {
        use super::super::models;
        use super::super::pipeline_permissions::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Batch API to authorize/unauthorize a list of definitions for a multiple resources."]
            #[doc = ""]
            #[doc = "See [`update_pipeline_permisions_for_resources`](super::super::pipeline_permissions::Client::update_pipeline_permisions_for_resources) for details of the arguments."]
            pub fn update_pipeline_permisions_for_resources(
                &self,
                organization: impl Into<String>,
                body: Vec<models::ResourcePipelinePermissions>,
                project: impl Into<String>,
            ) -> update_pipeline_permisions_for_resources::RequestBuilder<super::Client>
            {
                update_pipeline_permisions_for_resources::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body,
                    project: project.into(),
                }
            }
            #[doc = "Given a ResourceType and ResourceId, returns authorized definitions for that resource."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::pipeline_permissions::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                resource_type: impl Into<String>,
                resource_id: impl Into<String>,
            ) -> get::RequestBuilder<super::Client> {
                get::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    resource_type: resource_type.into(),
                    resource_id: resource_id.into(),
                }
            }
            #[doc = "Authorizes/Unauthorizes a list of definitions for a given resource."]
            #[doc = ""]
            #[doc = "See [`update_pipeline_permisions_for_resource`](super::super::pipeline_permissions::Client::update_pipeline_permisions_for_resource) for details of the arguments."]
            pub fn update_pipeline_permisions_for_resource(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::ResourcePipelinePermissions>,
                project: impl Into<String>,
                resource_type: impl Into<String>,
                resource_id: impl Into<String>,
            ) -> update_pipeline_permisions_for_resource::RequestBuilder<super::Client>
            {
                update_pipeline_permisions_for_resource::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    resource_type: resource_type.into(),
                    resource_id: resource_id.into(),
                }
            }
        }
    }
    pub mod check_configurations {
        use super::super::check_configurations::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Get Check configuration by resource type and id"]
            #[doc = ""]
            #[doc = "See [`list`](super::super::check_configurations::Client::list) for details of the arguments."]
            pub fn list(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
            ) -> list::RequestBuilder<super::Client> {
                list::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    resource_type: None,
                    resource_id: None,
                    expand: None,
                }
            }
            #[doc = "Add a check configuration"]
            #[doc = ""]
            #[doc = "See [`add`](super::super::check_configurations::Client::add) for details of the arguments."]
            pub fn add(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::CheckConfiguration>,
                project: impl Into<String>,
            ) -> add::RequestBuilder<super::Client> {
                add::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                }
            }
            #[doc = "Get Check configuration by Id"]
            #[doc = ""]
            #[doc = "See [`get`](super::super::check_configurations::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                id: i32,
            ) -> get::RequestBuilder<super::Client> {
                get::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    id,
                    expand: None,
                }
            }
            #[doc = "Update check configuration"]
            #[doc = ""]
            #[doc = "See [`update`](super::super::check_configurations::Client::update) for details of the arguments."]
            pub fn update(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::CheckConfiguration>,
                project: impl Into<String>,
                id: i32,
            ) -> update::RequestBuilder<super::Client> {
                update::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    id,
                }
            }
            #[doc = "Delete check configuration by id"]
            #[doc = ""]
            #[doc = "See [`delete`](super::super::check_configurations::Client::delete) for details of the arguments."]
            pub fn delete(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                id: i32,
            ) -> delete::RequestBuilder<super::Client> {
                delete::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    id,
                }
            }
            #[doc = "Get check configurations for multiple resources by resource type and id."]
            #[doc = ""]
            #[doc = "See [`query`](super::super::check_configurations::Client::query) for details of the arguments."]
            pub fn query(
                &self,
                organization: impl Into<String>,
                body: Vec<models::Resource>,
                project: impl Into<String>,
            ) -> query::RequestBuilder<super::Client> {
                query::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body,
                    project: project.into(),
                    expand: None,
                }
            }
        }
    }
    pub mod check_evaluations {
        use super::super::check_evaluations::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Initiate an evaluation for a check in a pipeline"]
            #[doc = ""]
            #[doc = "See [`evaluate`](super::super::check_evaluations::Client::evaluate) for details of the arguments."]
            pub fn evaluate(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::CheckSuiteRequest>,
                project: impl Into<String>,
            ) -> evaluate::RequestBuilder<super::Client> {
                evaluate::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    expand: None,
                }
            }
            #[doc = "Get details for a specific check evaluation"]
            #[doc = ""]
            #[doc = "See [`get`](super::super::check_evaluations::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                check_suite_id: impl Into<String>,
            ) -> get::RequestBuilder<super::Client> {
                get::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    check_suite_id: check_suite_id.into(),
                    expand: None,
                }
            }
        }
    }
    pub mod approvals {
        use super::super::approvals::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "List Approvals. This can be used to get a set of pending approvals in a pipeline, on an user or for a resource.."]
            #[doc = ""]
            #[doc = "See [`query`](super::super::approvals::Client::query) for details of the arguments."]
            pub fn query(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
            ) -> query::RequestBuilder<super::Client> {
                query::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    approval_ids: None,
                    expand: None,
                    user_ids: None,
                    state: None,
                    top: None,
                }
            }
            #[doc = "Update approvals."]
            #[doc = ""]
            #[doc = "See [`update`](super::super::approvals::Client::update) for details of the arguments."]
            pub fn update(
                &self,
                organization: impl Into<String>,
                body: Vec<models::ApprovalUpdateParameters>,
                project: impl Into<String>,
            ) -> update::RequestBuilder<super::Client> {
                update::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    body,
                    project: project.into(),
                }
            }
            #[doc = "Get an approval."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::approvals::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                approval_id: impl Into<String>,
            ) -> get::RequestBuilder<super::Client> {
                get::RequestBuilder {
                    client: self.0.clone(),
                    organization: organization.into(),
                    project: project.into(),
                    approval_id: approval_id.into(),
                    expand: None,
                }
            }
        }
    }
}
