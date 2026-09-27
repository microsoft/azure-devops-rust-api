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
    pub fn endpointproxy_client(&self) -> endpointproxy::Client {
        endpointproxy::Client(self.clone())
    }
    pub fn endpoints_client(&self) -> endpoints::Client {
        endpoints::Client(self.clone())
    }
    pub fn executionhistory_client(&self) -> executionhistory::Client {
        executionhistory::Client(self.clone())
    }
    pub fn types_client(&self) -> types::Client {
        types::Client(self.clone())
    }
}
pub mod endpointproxy {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Proxy for a GET request defined by a service endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Service endpoint request."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `endpoint_id`: Id of the service endpoint."]
        pub fn execute_service_endpoint_request(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ServiceEndpointRequest>,
            project: impl Into<String>,
            endpoint_id: impl Into<String>,
        ) -> execute_service_endpoint_request::RequestBuilder {
            execute_service_endpoint_request::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                endpoint_id: endpoint_id.into(),
            }
        }
        #[doc = "Use ExecuteServiceEndpointRequest API Instead"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Describes the data source to fetch."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn query(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::DataSourceBinding>,
            project: impl Into<String>,
        ) -> query::RequestBuilder {
            query::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod execute_service_endpoint_request {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::ServiceEndpointRequestResult,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpointRequestResult> {
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
            pub(crate) body: models::ServiceEndpointRequest,
            pub(crate) project: String,
            pub(crate) endpoint_id: String,
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
                        let endpoint_id = &this.endpoint_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("endpointId", endpoint_id);
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/serviceendpoint/endpointproxy?endpointId={}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.endpoint_id
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
            type Output = azure_core::Result<models::ServiceEndpointRequestResult>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::ServiceEndpointRequestResult>>;
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
    pub mod query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<Vec<String>, azure_core::http::JsonFormat>);
        impl Response {
            pub fn into_body(self) -> azure_core::Result<Vec<String>> {
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
            pub(crate) body: models::DataSourceBinding,
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
                    "{}/{}/{}/_apis/serviceendpoint/endpointproxy",
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
            type Output = azure_core::Result<Vec<String>>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<Vec<String>>>;
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
pub mod endpoints {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the service endpoints."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_service_endpoints(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_service_endpoints::RequestBuilder {
            get_service_endpoints::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                type_: None,
                auth_schemes: None,
                endpoint_ids: None,
                owner: None,
                include_failed: None,
                include_details: None,
                action_filter: None,
            }
        }
        #[doc = "Creates a new service endpoint"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Service endpoint to create"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ServiceEndpoint>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Update the service endpoints."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Names of the service endpoints to update."]
        pub fn update_service_endpoints(
            &self,
            organization: impl Into<String>,
            body: Vec<models::ServiceEndpoint>,
        ) -> update_service_endpoints::RequestBuilder {
            update_service_endpoints::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body,
            }
        }
        #[doc = "Update the service endpoint"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Updated data for the endpoint"]
        #[doc = "* `endpoint_id`: Endpoint Id of the endpoint to update"]
        pub fn update_service_endpoint(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ServiceEndpoint>,
            endpoint_id: impl Into<String>,
        ) -> update_service_endpoint::RequestBuilder {
            update_service_endpoint::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                endpoint_id: endpoint_id.into(),
                operation: None,
            }
        }
        #[doc = "Share service endpoint across projects"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Project reference details of the target project"]
        #[doc = "* `endpoint_id`: Endpoint Id of the endpoint to share"]
        pub fn share_service_endpoint(
            &self,
            organization: impl Into<String>,
            body: Vec<models::ServiceEndpointProjectReference>,
            endpoint_id: impl Into<String>,
        ) -> share_service_endpoint::RequestBuilder {
            share_service_endpoint::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body,
                endpoint_id: endpoint_id.into(),
            }
        }
        #[doc = "Delete a service endpoint"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `endpoint_id`: Endpoint Id of endpoint to delete"]
        #[doc = "* `project_ids`: project Ids from which endpoint needs to be deleted"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            endpoint_id: impl Into<String>,
            project_ids: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                endpoint_id: endpoint_id.into(),
                project_ids: project_ids.into(),
                deep: None,
            }
        }
        #[doc = "Get the service endpoints by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `endpoint_names`: Names of the service endpoints."]
        pub fn get_service_endpoints_by_names(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            endpoint_names: impl Into<String>,
        ) -> get_service_endpoints_by_names::RequestBuilder {
            get_service_endpoints_by_names::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                endpoint_names: endpoint_names.into(),
                type_: None,
                auth_schemes: None,
                owner: None,
                include_failed: None,
                include_details: None,
            }
        }
        #[doc = "Gets the service endpoints and patch new authorization parameters"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Scope, Validity of Token requested."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `endpoint_ids`: Ids of the service endpoints."]
        pub fn get_service_endpoints_with_refreshed_authentication(
            &self,
            organization: impl Into<String>,
            body: Vec<models::RefreshAuthenticationParameters>,
            project: impl Into<String>,
            endpoint_ids: impl Into<String>,
        ) -> get_service_endpoints_with_refreshed_authentication::RequestBuilder {
            get_service_endpoints_with_refreshed_authentication::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body,
                project: project.into(),
                endpoint_ids: endpoint_ids.into(),
            }
        }
        #[doc = "Get the service endpoint details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `endpoint_id`: Id of the service endpoint."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            endpoint_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                endpoint_id: endpoint_id.into(),
                action_filter: None,
            }
        }
    }
    pub mod get_service_endpoints {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::ServiceEndpointList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpointList> {
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
            pub(crate) type_: Option<String>,
            pub(crate) auth_schemes: Option<String>,
            pub(crate) endpoint_ids: Option<String>,
            pub(crate) owner: Option<String>,
            pub(crate) include_failed: Option<bool>,
            pub(crate) include_details: Option<bool>,
            pub(crate) action_filter: Option<models::ServiceEndpointActionFilter>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Type of the service endpoints."]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            #[doc = "Authorization schemes used for service endpoints."]
            pub fn auth_schemes(mut self, auth_schemes: impl Into<String>) -> Self {
                self.auth_schemes = Some(auth_schemes.into());
                self
            }
            #[doc = "Ids of the service endpoints."]
            pub fn endpoint_ids(mut self, endpoint_ids: impl Into<String>) -> Self {
                self.endpoint_ids = Some(endpoint_ids.into());
                self
            }
            #[doc = "Owner for service endpoints."]
            pub fn owner(mut self, owner: impl Into<String>) -> Self {
                self.owner = Some(owner.into());
                self
            }
            #[doc = "Failed flag for service endpoints."]
            pub fn include_failed(mut self, include_failed: bool) -> Self {
                self.include_failed = Some(include_failed);
                self
            }
            #[doc = "Flag to include more details for service endpoints. This is for internal use only and the flag will be treated as false for all other requests"]
            pub fn include_details(mut self, include_details: bool) -> Self {
                self.include_details = Some(include_details);
                self
            }
            #[doc = "The \"actionFilter\" parameter allows users to evaluate requestor permissions and retrieve a list of endpoints that match the specified conditions, ensuring that only relevant endpoints are returned based on their permissions"]
            pub fn action_filter(
                mut self,
                action_filter: impl Into<models::ServiceEndpointActionFilter>,
            ) -> Self {
                self.action_filter = Some(action_filter.into());
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
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        if let Some(auth_schemes) = &this.auth_schemes {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("authSchemes", auth_schemes);
                        }
                        if let Some(endpoint_ids) = &this.endpoint_ids {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("endpointIds", endpoint_ids);
                        }
                        if let Some(owner) = &this.owner {
                            req.url_mut().query_pairs_mut().append_pair("owner", owner);
                        }
                        if let Some(include_failed) = &this.include_failed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeFailed", &include_failed.to_string());
                        }
                        if let Some(include_details) = &this.include_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDetails", &include_details.to_string());
                        }
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", &action_filter.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/serviceendpoint/endpoints?",
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
            type Output = azure_core::Result<models::ServiceEndpointList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceEndpointList>>;
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
            azure_core::http::Response<models::ServiceEndpoint, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpoint> {
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
            pub(crate) body: models::ServiceEndpoint,
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
                    "{}/{}/_apis/serviceendpoint/endpoints",
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
            type Output = azure_core::Result<models::ServiceEndpoint>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceEndpoint>>;
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
    pub mod update_service_endpoints {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::ServiceEndpointList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpointList> {
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
            pub(crate) body: Vec<models::ServiceEndpoint>,
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
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/serviceendpoint/endpoints",
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
            type Output = azure_core::Result<models::ServiceEndpointList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceEndpointList>>;
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
    pub mod update_service_endpoint {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::ServiceEndpoint, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpoint> {
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
            pub(crate) body: models::ServiceEndpoint,
            pub(crate) endpoint_id: String,
            pub(crate) operation: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "operation type"]
            pub fn operation(mut self, operation: impl Into<String>) -> Self {
                self.operation = Some(operation.into());
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
                        if let Some(operation) = &this.operation {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("operation", operation);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/serviceendpoint/endpoints/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.endpoint_id
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
            type Output = azure_core::Result<models::ServiceEndpoint>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceEndpoint>>;
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
    pub mod share_service_endpoint {
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
        pub struct RequestBuilder<M = ()> {
            pub(crate) client: super::super::Client,
            pub(crate) mode: std::marker::PhantomData<M>,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::ServiceEndpointProjectReference>,
            pub(crate) endpoint_id: String,
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
                    "{}/{}/_apis/serviceendpoint/endpoints/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.endpoint_id
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
        pub struct RequestBuilder<M = ()> {
            pub(crate) client: super::super::Client,
            pub(crate) mode: std::marker::PhantomData<M>,
            pub(crate) organization: String,
            pub(crate) endpoint_id: String,
            pub(crate) project_ids: String,
            pub(crate) deep: Option<bool>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "delete the spn created by endpoint"]
            pub fn deep(mut self, deep: bool) -> Self {
                self.deep = Some(deep);
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
                        let project_ids = &this.project_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("projectIds", project_ids);
                        if let Some(deep) = &this.deep {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deep", &deep.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/serviceendpoint/endpoints/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.endpoint_id
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
    pub mod get_service_endpoints_by_names {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::ServiceEndpointList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpointList> {
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
            pub(crate) endpoint_names: String,
            pub(crate) type_: Option<String>,
            pub(crate) auth_schemes: Option<String>,
            pub(crate) owner: Option<String>,
            pub(crate) include_failed: Option<bool>,
            pub(crate) include_details: Option<bool>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Type of the service endpoints."]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            #[doc = "Authorization schemes used for service endpoints."]
            pub fn auth_schemes(mut self, auth_schemes: impl Into<String>) -> Self {
                self.auth_schemes = Some(auth_schemes.into());
                self
            }
            #[doc = "Owner for service endpoints."]
            pub fn owner(mut self, owner: impl Into<String>) -> Self {
                self.owner = Some(owner.into());
                self
            }
            #[doc = "Failed flag for service endpoints."]
            pub fn include_failed(mut self, include_failed: bool) -> Self {
                self.include_failed = Some(include_failed);
                self
            }
            #[doc = "Flag to include more details for service endpoints. This is for internal use only and the flag will be treated as false for all other requests"]
            pub fn include_details(mut self, include_details: bool) -> Self {
                self.include_details = Some(include_details);
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
                        let endpoint_names = &this.endpoint_names;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("endpointNames", endpoint_names);
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        if let Some(auth_schemes) = &this.auth_schemes {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("authSchemes", auth_schemes);
                        }
                        if let Some(owner) = &this.owner {
                            req.url_mut().query_pairs_mut().append_pair("owner", owner);
                        }
                        if let Some(include_failed) = &this.include_failed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeFailed", &include_failed.to_string());
                        }
                        if let Some(include_details) = &this.include_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDetails", &include_details.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/serviceendpoint/endpoints",
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
            type Output = azure_core::Result<models::ServiceEndpointList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceEndpointList>>;
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
    pub mod get_service_endpoints_with_refreshed_authentication {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::ServiceEndpointList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpointList> {
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
            pub(crate) body: Vec<models::RefreshAuthenticationParameters>,
            pub(crate) project: String,
            pub(crate) endpoint_ids: String,
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
                        let endpoint_ids = &this.endpoint_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("endpointIds", endpoint_ids);
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/serviceendpoint/endpoints",
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
            type Output = azure_core::Result<models::ServiceEndpointList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceEndpointList>>;
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
            azure_core::http::Response<models::ServiceEndpoint, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpoint> {
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
            pub(crate) endpoint_id: String,
            pub(crate) action_filter: Option<models::ServiceEndpointActionFilter>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Action filter for the service connection. It specifies the action which can be performed on the service connection."]
            pub fn action_filter(
                mut self,
                action_filter: impl Into<models::ServiceEndpointActionFilter>,
            ) -> Self {
                self.action_filter = Some(action_filter.into());
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
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", &action_filter.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/serviceendpoint/endpoints/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.endpoint_id
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
            type Output = azure_core::Result<models::ServiceEndpoint>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceEndpoint>>;
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
pub mod types {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get service endpoint types."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                type_: None,
                scheme: None,
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
                models::ServiceEndpointTypeList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::ServiceEndpointTypeList> {
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
            pub(crate) type_: Option<String>,
            pub(crate) scheme: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Type of service endpoint."]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            #[doc = "Scheme of service endpoint."]
            pub fn scheme(mut self, scheme: impl Into<String>) -> Self {
                self.scheme = Some(scheme.into());
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
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        if let Some(scheme) = &this.scheme {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("scheme", scheme);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/serviceendpoint/types",
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
            type Output = azure_core::Result<models::ServiceEndpointTypeList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::ServiceEndpointTypeList>>;
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
pub mod executionhistory {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get service endpoint execution records."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `endpoint_id`: Id of the service endpoint."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            endpoint_id: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                endpoint_id: endpoint_id.into(),
                top: None,
                continuation_token: None,
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
                models::ServiceEndpointExecutionRecordList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(
                self,
            ) -> azure_core::Result<models::ServiceEndpointExecutionRecordList> {
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
            pub(crate) endpoint_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<i64>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Number of service endpoint execution records to get."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "A continuation token, returned by a previous call to this method, that can be used to return the next set of records"]
            pub fn continuation_token(mut self, continuation_token: i64) -> Self {
                self.continuation_token = Some(continuation_token);
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
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("top", &top.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", &continuation_token.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/serviceendpoint/{}/executionhistory",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.endpoint_id
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
            type Output = azure_core::Result<models::ServiceEndpointExecutionRecordList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::ServiceEndpointExecutionRecordList>>;
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
        pub fn endpointproxy_client(&self) -> endpointproxy::Client {
            endpointproxy::Client(self.clone())
        }
        pub fn endpoints_client(&self) -> endpoints::Client {
            endpoints::Client(self.clone())
        }
        pub fn executionhistory_client(&self) -> executionhistory::Client {
            executionhistory::Client(self.clone())
        }
        pub fn types_client(&self) -> types::Client {
            types::Client(self.clone())
        }
    }
    impl From<super::Client> for Client {
        #[doc = "Create a blocking client that sends requests using an existing async client."]
        fn from(client: super::Client) -> Self {
            Self(client)
        }
    }
    pub mod endpointproxy {
        use super::super::endpointproxy::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Proxy for a GET request defined by a service endpoint."]
            #[doc = ""]
            #[doc = "See [`execute_service_endpoint_request`](super::super::endpointproxy::Client::execute_service_endpoint_request) for details of the arguments."]
            pub fn execute_service_endpoint_request(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::ServiceEndpointRequest>,
                project: impl Into<String>,
                endpoint_id: impl Into<String>,
            ) -> execute_service_endpoint_request::RequestBuilder<crate::blocking::Blocking>
            {
                execute_service_endpoint_request::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    endpoint_id: endpoint_id.into(),
                }
            }
            #[doc = "Use ExecuteServiceEndpointRequest API Instead"]
            #[doc = ""]
            #[doc = "See [`query`](super::super::endpointproxy::Client::query) for details of the arguments."]
            pub fn query(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::DataSourceBinding>,
                project: impl Into<String>,
            ) -> query::RequestBuilder<crate::blocking::Blocking> {
                query::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                }
            }
        }
    }
    pub mod endpoints {
        use super::super::endpoints::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Get the service endpoints."]
            #[doc = ""]
            #[doc = "See [`get_service_endpoints`](super::super::endpoints::Client::get_service_endpoints) for details of the arguments."]
            pub fn get_service_endpoints(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
            ) -> get_service_endpoints::RequestBuilder<crate::blocking::Blocking> {
                get_service_endpoints::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    type_: None,
                    auth_schemes: None,
                    endpoint_ids: None,
                    owner: None,
                    include_failed: None,
                    include_details: None,
                    action_filter: None,
                }
            }
            #[doc = "Creates a new service endpoint"]
            #[doc = ""]
            #[doc = "See [`create`](super::super::endpoints::Client::create) for details of the arguments."]
            pub fn create(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::ServiceEndpoint>,
            ) -> create::RequestBuilder<crate::blocking::Blocking> {
                create::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                }
            }
            #[doc = "Update the service endpoints."]
            #[doc = ""]
            #[doc = "See [`update_service_endpoints`](super::super::endpoints::Client::update_service_endpoints) for details of the arguments."]
            pub fn update_service_endpoints(
                &self,
                organization: impl Into<String>,
                body: Vec<models::ServiceEndpoint>,
            ) -> update_service_endpoints::RequestBuilder<crate::blocking::Blocking> {
                update_service_endpoints::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body,
                }
            }
            #[doc = "Update the service endpoint"]
            #[doc = ""]
            #[doc = "See [`update_service_endpoint`](super::super::endpoints::Client::update_service_endpoint) for details of the arguments."]
            pub fn update_service_endpoint(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::ServiceEndpoint>,
                endpoint_id: impl Into<String>,
            ) -> update_service_endpoint::RequestBuilder<crate::blocking::Blocking> {
                update_service_endpoint::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    endpoint_id: endpoint_id.into(),
                    operation: None,
                }
            }
            #[doc = "Share service endpoint across projects"]
            #[doc = ""]
            #[doc = "See [`share_service_endpoint`](super::super::endpoints::Client::share_service_endpoint) for details of the arguments."]
            pub fn share_service_endpoint(
                &self,
                organization: impl Into<String>,
                body: Vec<models::ServiceEndpointProjectReference>,
                endpoint_id: impl Into<String>,
            ) -> share_service_endpoint::RequestBuilder<crate::blocking::Blocking> {
                share_service_endpoint::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body,
                    endpoint_id: endpoint_id.into(),
                }
            }
            #[doc = "Delete a service endpoint"]
            #[doc = ""]
            #[doc = "See [`delete`](super::super::endpoints::Client::delete) for details of the arguments."]
            pub fn delete(
                &self,
                organization: impl Into<String>,
                endpoint_id: impl Into<String>,
                project_ids: impl Into<String>,
            ) -> delete::RequestBuilder<crate::blocking::Blocking> {
                delete::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    endpoint_id: endpoint_id.into(),
                    project_ids: project_ids.into(),
                    deep: None,
                }
            }
            #[doc = "Get the service endpoints by name."]
            #[doc = ""]
            #[doc = "See [`get_service_endpoints_by_names`](super::super::endpoints::Client::get_service_endpoints_by_names) for details of the arguments."]
            pub fn get_service_endpoints_by_names(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                endpoint_names: impl Into<String>,
            ) -> get_service_endpoints_by_names::RequestBuilder<crate::blocking::Blocking>
            {
                get_service_endpoints_by_names::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    endpoint_names: endpoint_names.into(),
                    type_: None,
                    auth_schemes: None,
                    owner: None,
                    include_failed: None,
                    include_details: None,
                }
            }
            #[doc = "Gets the service endpoints and patch new authorization parameters"]
            #[doc = ""]
            #[doc = "See [`get_service_endpoints_with_refreshed_authentication`](super::super::endpoints::Client::get_service_endpoints_with_refreshed_authentication) for details of the arguments."]
            pub fn get_service_endpoints_with_refreshed_authentication(
                &self,
                organization: impl Into<String>,
                body: Vec<models::RefreshAuthenticationParameters>,
                project: impl Into<String>,
                endpoint_ids: impl Into<String>,
            ) -> get_service_endpoints_with_refreshed_authentication::RequestBuilder<
                crate::blocking::Blocking,
            > {
                get_service_endpoints_with_refreshed_authentication::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body,
                    project: project.into(),
                    endpoint_ids: endpoint_ids.into(),
                }
            }
            #[doc = "Get the service endpoint details."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::endpoints::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                endpoint_id: impl Into<String>,
            ) -> get::RequestBuilder<crate::blocking::Blocking> {
                get::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    endpoint_id: endpoint_id.into(),
                    action_filter: None,
                }
            }
        }
    }
    pub mod types {
        use super::super::models;
        use super::super::types::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Get service endpoint types."]
            #[doc = ""]
            #[doc = "See [`list`](super::super::types::Client::list) for details of the arguments."]
            pub fn list(
                &self,
                organization: impl Into<String>,
            ) -> list::RequestBuilder<crate::blocking::Blocking> {
                list::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    type_: None,
                    scheme: None,
                }
            }
        }
    }
    pub mod executionhistory {
        use super::super::executionhistory::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Get service endpoint execution records."]
            #[doc = ""]
            #[doc = "See [`list`](super::super::executionhistory::Client::list) for details of the arguments."]
            pub fn list(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                endpoint_id: impl Into<String>,
            ) -> list::RequestBuilder<crate::blocking::Blocking> {
                list::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    endpoint_id: endpoint_id.into(),
                    top: None,
                    continuation_token: None,
                }
            }
        }
    }
}
