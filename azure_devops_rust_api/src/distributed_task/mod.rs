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
        self.options.retry = Some(retry.into());
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::http::TransportOptions>) -> Self {
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
    ) -> azure_core::Result<azure_core::http::BufResponse> {
        let context = azure_core::http::Context::default();
        self.pipeline.send(&context, request).await
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
    pub fn agentclouds_client(&self) -> agentclouds::Client {
        agentclouds::Client(self.clone())
    }
    pub fn agentcloudtypes_client(&self) -> agentcloudtypes::Client {
        agentcloudtypes::Client(self.clone())
    }
    pub fn agents_client(&self) -> agents::Client {
        agents::Client(self.clone())
    }
    pub fn deploymentgroups_client(&self) -> deploymentgroups::Client {
        deploymentgroups::Client(self.clone())
    }
    pub fn elasticpoollogs_client(&self) -> elasticpoollogs::Client {
        elasticpoollogs::Client(self.clone())
    }
    pub fn elasticpools_client(&self) -> elasticpools::Client {
        elasticpools::Client(self.clone())
    }
    pub fn environmentdeployment_records_client(&self) -> environmentdeployment_records::Client {
        environmentdeployment_records::Client(self.clone())
    }
    pub fn environments_client(&self) -> environments::Client {
        environments::Client(self.clone())
    }
    pub fn events_client(&self) -> events::Client {
        events::Client(self.clone())
    }
    pub fn kubernetes_client(&self) -> kubernetes::Client {
        kubernetes::Client(self.clone())
    }
    pub fn logs_client(&self) -> logs::Client {
        logs::Client(self.clone())
    }
    pub fn nodes_client(&self) -> nodes::Client {
        nodes::Client(self.clone())
    }
    pub fn oidctoken_client(&self) -> oidctoken::Client {
        oidctoken::Client(self.clone())
    }
    pub fn pools_client(&self) -> pools::Client {
        pools::Client(self.clone())
    }
    pub fn queues_client(&self) -> queues::Client {
        queues::Client(self.clone())
    }
    pub fn records_client(&self) -> records::Client {
        records::Client(self.clone())
    }
    pub fn requests_client(&self) -> requests::Client {
        requests::Client(self.clone())
    }
    pub fn targets_client(&self) -> targets::Client {
        targets::Client(self.clone())
    }
    pub fn taskgroups_client(&self) -> taskgroups::Client {
        taskgroups::Client(self.clone())
    }
    pub fn variablegroups_client(&self) -> variablegroups::Client {
        variablegroups::Client(self.clone())
    }
    pub fn yamlschema_client(&self) -> yamlschema::Client {
        yamlschema::Client(self.clone())
    }
}
pub mod events {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Send a pipeline job event to be processed by the execution plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The event data to be processed by the plan."]
        #[doc = "* `scope_identifier`: The project GUID to scope the request"]
        #[doc = "* `hub_name`: The name of the server hub. Common examples: \"build\", \"rm\", \"checks\""]
        #[doc = "* `plan_id`: The ID of the plan."]
        pub fn post_event(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::JobEvent>,
            scope_identifier: impl Into<String>,
            hub_name: impl Into<String>,
            plan_id: impl Into<String>,
        ) -> post_event::RequestBuilder {
            post_event::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                scope_identifier: scope_identifier.into(),
                hub_name: hub_name.into(),
                plan_id: plan_id.into(),
            }
        }
    }
    pub mod post_event {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<(), azure_core::http::NoFormat>);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::JobEvent,
            pub(crate) scope_identifier: String,
            pub(crate) hub_name: String,
            pub(crate) plan_id: String,
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
                    "{}/{}/{}/_apis/distributedtask/hubs/{}/plans/{}/events",
                    self.client.endpoint(),
                    &self.organization,
                    &self.scope_identifier,
                    &self.hub_name,
                    &self.plan_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod oidctoken {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `scope_identifier`: The project GUID to scope the request"]
        #[doc = "* `hub_name`: The name of the server hub. Common examples: \"build\", \"rm\", \"checks\""]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<serde_json::Value>,
            scope_identifier: impl Into<String>,
            hub_name: impl Into<String>,
            plan_id: impl Into<String>,
            job_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                scope_identifier: scope_identifier.into(),
                hub_name: hub_name.into(),
                plan_id: plan_id.into(),
                job_id: job_id.into(),
                service_connection_id: None,
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
            azure_core::http::Response<models::TaskHubOidcToken, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskHubOidcToken> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: serde_json::Value,
            pub(crate) scope_identifier: String,
            pub(crate) hub_name: String,
            pub(crate) plan_id: String,
            pub(crate) job_id: String,
            pub(crate) service_connection_id: Option<String>,
        }
        impl RequestBuilder {
            pub fn service_connection_id(
                mut self,
                service_connection_id: impl Into<String>,
            ) -> Self {
                self.service_connection_id = Some(service_connection_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(service_connection_id) = &this.service_connection_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("serviceConnectionId", service_connection_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/hubs/{}/plans/{}/jobs/{}/oidctoken",
                    self.client.endpoint(),
                    &self.organization,
                    &self.scope_identifier,
                    &self.hub_name,
                    &self.plan_id,
                    &self.job_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskHubOidcToken>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskHubOidcToken>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod logs {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create a log and connect it to a pipeline run's execution plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: An object that contains information about log's path."]
        #[doc = "* `scope_identifier`: The project GUID to scope the request"]
        #[doc = "* `hub_name`: The name of the server hub. Common examples: \"build\", \"rm\", \"checks\""]
        #[doc = "* `plan_id`: The ID of the plan."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskLog>,
            scope_identifier: impl Into<String>,
            hub_name: impl Into<String>,
            plan_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                scope_identifier: scope_identifier.into(),
                hub_name: hub_name.into(),
                plan_id: plan_id.into(),
            }
        }
        #[doc = "Append a log to a task's log. The log should be sent in the body of the request as a TaskLog object stream."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Stream to upload"]
        #[doc = "* `scope_identifier`: The project GUID to scope the request"]
        #[doc = "* `hub_name`: The name of the server hub. Common examples: \"build\", \"rm\", \"checks\""]
        #[doc = "* `plan_id`: The ID of the plan."]
        #[doc = "* `log_id`: The ID of the log."]
        pub fn append_log_content(
            &self,
            organization: impl Into<String>,
            body: impl Into<String>,
            scope_identifier: impl Into<String>,
            hub_name: impl Into<String>,
            plan_id: impl Into<String>,
            log_id: i32,
        ) -> append_log_content::RequestBuilder {
            append_log_content::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                scope_identifier: scope_identifier.into(),
                hub_name: hub_name.into(),
                plan_id: plan_id.into(),
                log_id,
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
            azure_core::http::Response<models::TaskLog, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskLog> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskLog,
            pub(crate) scope_identifier: String,
            pub(crate) hub_name: String,
            pub(crate) plan_id: String,
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
                    "{}/{}/{}/_apis/distributedtask/hubs/{}/plans/{}/logs",
                    self.client.endpoint(),
                    &self.organization,
                    &self.scope_identifier,
                    &self.hub_name,
                    &self.plan_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskLog>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskLog>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod append_log_content {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskLog, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskLog> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: String,
            pub(crate) scope_identifier: String,
            pub(crate) hub_name: String,
            pub(crate) plan_id: String,
            pub(crate) log_id: i32,
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
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::json::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/hubs/{}/plans/{}/logs/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.scope_identifier,
                    &self.hub_name,
                    &self.plan_id,
                    &self.log_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskLog>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskLog>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod records {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Update timeline records if they already exist, otherwise create new ones for the same timeline."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The array of timeline records to be updated."]
        #[doc = "* `scope_identifier`: The project GUID to scope the request"]
        #[doc = "* `hub_name`: The name of the server hub. Common examples: \"build\", \"rm\", \"checks\""]
        #[doc = "* `plan_id`: The ID of the plan."]
        #[doc = "* `timeline_id`: The ID of the timeline."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::VssJsonCollectionWrapper>,
            scope_identifier: impl Into<String>,
            hub_name: impl Into<String>,
            plan_id: impl Into<String>,
            timeline_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                scope_identifier: scope_identifier.into(),
                hub_name: hub_name.into(),
                plan_id: plan_id.into(),
                timeline_id: timeline_id.into(),
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
            azure_core::http::Response<models::TimelineRecordList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TimelineRecordList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::VssJsonCollectionWrapper,
            pub(crate) scope_identifier: String,
            pub(crate) hub_name: String,
            pub(crate) plan_id: String,
            pub(crate) timeline_id: String,
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
                    "{}/{}/{}/_apis/distributedtask/hubs/{}/plans/{}/timelines/{}/records",
                    self.client.endpoint(),
                    &self.organization,
                    &self.scope_identifier,
                    &self.hub_name,
                    &self.plan_id,
                    &self.timeline_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TimelineRecordList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TimelineRecordList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod pools {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of agent pools."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_agent_pools(
            &self,
            organization: impl Into<String>,
        ) -> get_agent_pools::RequestBuilder {
            get_agent_pools::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_name: None,
                properties: None,
                pool_type: None,
                action_filter: None,
            }
        }
        #[doc = "Get a list of agent pools."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_ids`: pool Ids to fetch"]
        pub fn get_agent_pools_by_ids(
            &self,
            organization: impl Into<String>,
            pool_ids: impl Into<String>,
        ) -> get_agent_pools_by_ids::RequestBuilder {
            get_agent_pools_by_ids::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_ids: pool_ids.into(),
                action_filter: None,
            }
        }
        #[doc = "Create an agent pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Details about the new agent pool"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgentPool>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Get information about an agent pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: An agent pool ID"]
        pub fn get(&self, organization: impl Into<String>, pool_id: i32) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
                properties: None,
                action_filter: None,
            }
        }
        #[doc = "Update properties on an agent pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Updated agent pool details"]
        #[doc = "* `pool_id`: The agent pool to update"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgentPool>,
            pool_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                pool_id,
            }
        }
        #[doc = "Delete an agent pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: ID of the agent pool to delete"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            pool_id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
            }
        }
    }
    pub mod get_agent_pools {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskAgentPoolList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentPoolList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_name: Option<String>,
            pub(crate) properties: Option<String>,
            pub(crate) pool_type: Option<String>,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter by name"]
            pub fn pool_name(mut self, pool_name: impl Into<String>) -> Self {
                self.pool_name = Some(pool_name.into());
                self
            }
            #[doc = "Filter by agent pool properties (comma-separated)"]
            pub fn properties(mut self, properties: impl Into<String>) -> Self {
                self.properties = Some(properties.into());
                self
            }
            #[doc = "Filter by pool type"]
            pub fn pool_type(mut self, pool_type: impl Into<String>) -> Self {
                self.pool_type = Some(pool_type.into());
                self
            }
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(pool_name) = &this.pool_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("poolName", pool_name);
                        }
                        if let Some(properties) = &this.properties {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("properties", properties);
                        }
                        if let Some(pool_type) = &this.pool_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("poolType", pool_type);
                        }
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/pools?",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentPoolList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentPoolList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_agent_pools_by_ids {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskAgentPoolList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentPoolList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_ids: String,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        let pool_ids = &this.pool_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("poolIds", pool_ids);
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/pools",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentPoolList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentPoolList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentPool, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentPool> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgentPool,
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
                    "{}/{}/_apis/distributedtask/pools",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentPool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentPool>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentPool, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentPool> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
            pub(crate) properties: Option<String>,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Agent pool properties (comma-separated)"]
            pub fn properties(mut self, properties: impl Into<String>) -> Self {
                self.properties = Some(properties.into());
                self
            }
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(properties) = &this.properties {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("properties", properties);
                        }
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/pools/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentPool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentPool>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentPool, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentPool> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgentPool,
            pub(crate) pool_id: i32,
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
                    "{}/{}/_apis/distributedtask/pools/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentPool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentPool>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
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
                    "{}/{}/_apis/distributedtask/pools/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod queues {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of agent queues by their names"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `queue_names`: A comma-separated list of agent names to retrieve"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_agent_queues_by_names(
            &self,
            organization: impl Into<String>,
            queue_names: impl Into<String>,
            project: impl Into<String>,
        ) -> get_agent_queues_by_names::RequestBuilder {
            get_agent_queues_by_names::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                queue_names: queue_names.into(),
                project: project.into(),
                action_filter: None,
            }
        }
        #[doc = "Get a list of agent queues by their IDs"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `queue_ids`: A comma-separated list of agent queue IDs to retrieve"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_agent_queues_by_ids(
            &self,
            organization: impl Into<String>,
            queue_ids: impl Into<String>,
            project: impl Into<String>,
        ) -> get_agent_queues_by_ids::RequestBuilder {
            get_agent_queues_by_ids::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                queue_ids: queue_ids.into(),
                project: project.into(),
                action_filter: None,
            }
        }
        #[doc = "Get a list of agent queues."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_agent_queues(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_agent_queues::RequestBuilder {
            get_agent_queues::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                queue_name: None,
                action_filter: None,
            }
        }
        #[doc = "Get a list of agent queues by pool ids"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_ids`: A comma-separated list of pool ids to get the corresponding queues for"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_agent_queues_for_pools(
            &self,
            organization: impl Into<String>,
            pool_ids: impl Into<String>,
            project: impl Into<String>,
        ) -> get_agent_queues_for_pools::RequestBuilder {
            get_agent_queues_for_pools::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_ids: pool_ids.into(),
                project: project.into(),
                action_filter: None,
            }
        }
        #[doc = "Create a new agent queue to connect a project to an agent pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Details about the queue to create"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgentQueue>,
            project: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                authorize_pipelines: None,
            }
        }
        #[doc = "Get information about an agent queue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `queue_id`: The agent queue to get information about"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            queue_id: i32,
            project: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                queue_id,
                project: project.into(),
                action_filter: None,
            }
        }
        #[doc = "Removes an agent queue from a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `queue_id`: The agent queue to remove"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            queue_id: i32,
            project: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                queue_id,
                project: project.into(),
            }
        }
    }
    pub mod get_agent_queues_by_names {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskAgentQueueList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentQueueList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) queue_names: String,
            pub(crate) project: String,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        let queue_names = &this.queue_names;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("queueNames", queue_names);
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/queues?queueNames={}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.queue_names
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentQueueList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentQueueList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_agent_queues_by_ids {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskAgentQueueList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentQueueList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) queue_ids: String,
            pub(crate) project: String,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        let queue_ids = &this.queue_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("queueIds", queue_ids);
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/queues?queueIds={}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.queue_ids
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentQueueList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentQueueList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_agent_queues {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskAgentQueueList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentQueueList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) queue_name: Option<String>,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter on the agent queue name"]
            pub fn queue_name(mut self, queue_name: impl Into<String>) -> Self {
                self.queue_name = Some(queue_name.into());
                self
            }
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(queue_name) = &this.queue_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("queueName", queue_name);
                        }
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/queues?",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentQueueList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentQueueList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_agent_queues_for_pools {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskAgentQueueList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentQueueList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_ids: String,
            pub(crate) project: String,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        let pool_ids = &this.pool_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("poolIds", pool_ids);
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/queues",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentQueueList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentQueueList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentQueue, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentQueue> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgentQueue,
            pub(crate) project: String,
            pub(crate) authorize_pipelines: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Automatically authorize this queue when using YAML"]
            pub fn authorize_pipelines(mut self, authorize_pipelines: bool) -> Self {
                self.authorize_pipelines = Some(authorize_pipelines);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(authorize_pipelines) = &this.authorize_pipelines {
                            req.url_mut().query_pairs_mut().append_pair(
                                "authorizePipelines",
                                &authorize_pipelines.to_string(),
                            );
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/queues",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentQueue>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentQueue>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentQueue, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentQueue> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) queue_id: i32,
            pub(crate) project: String,
            pub(crate) action_filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter by whether the calling user has use or manage permissions"]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/queues/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.queue_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentQueue>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentQueue>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) queue_id: i32,
            pub(crate) project: String,
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
                    "{}/{}/{}/_apis/distributedtask/queues/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.queue_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod variablegroups {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get variable groups."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_variable_groups(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_variable_groups::RequestBuilder {
            get_variable_groups::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                group_name: None,
                action_filter: None,
                top: None,
                continuation_token: None,
                query_order: None,
            }
        }
        #[doc = "Add a variable group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::VariableGroupParameters>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Add a variable group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn share_variable_group(
            &self,
            organization: impl Into<String>,
            body: Vec<models::VariableGroupProjectReference>,
            variable_group_id: i32,
        ) -> share_variable_group::RequestBuilder {
            share_variable_group::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                variable_group_id,
            }
        }
        #[doc = "Update a variable group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `group_id`: Id of the variable group to update."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::VariableGroupParameters>,
            group_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                group_id,
            }
        }
        #[doc = "Delete a variable group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `group_id`: Id of the variable group."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            group_id: i32,
            project_ids: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                group_id,
                project_ids: project_ids.into(),
            }
        }
        #[doc = "Get variable groups by ids."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `group_ids`: Comma separated list of Ids of variable groups."]
        pub fn get_variable_groups_by_id(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            group_ids: impl Into<String>,
        ) -> get_variable_groups_by_id::RequestBuilder {
            get_variable_groups_by_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                group_ids: group_ids.into(),
            }
        }
        #[doc = "Get a variable group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `group_id`: Id of the variable group."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            group_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                group_id,
            }
        }
    }
    pub mod get_variable_groups {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::VariableGroupList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VariableGroupList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) group_name: Option<String>,
            pub(crate) action_filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<i32>,
            pub(crate) query_order: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Name of variable group."]
            pub fn group_name(mut self, group_name: impl Into<String>) -> Self {
                self.group_name = Some(group_name.into());
                self
            }
            #[doc = "Action filter for the variable group. It specifies the action which can be performed on the variable groups."]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Number of variable groups to get."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Gets the variable groups after the continuation token provided."]
            pub fn continuation_token(mut self, continuation_token: i32) -> Self {
                self.continuation_token = Some(continuation_token);
                self
            }
            #[doc = "Gets the results in the defined order. Default is 'IdDescending'."]
            pub fn query_order(mut self, query_order: impl Into<String>) -> Self {
                self.query_order = Some(query_order.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(group_name) = &this.group_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("groupName", group_name);
                        }
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", &continuation_token.to_string());
                        }
                        if let Some(query_order) = &this.query_order {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("queryOrder", query_order);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/variablegroups?",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VariableGroupList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VariableGroupList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::VariableGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VariableGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::VariableGroupParameters,
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
                    "{}/{}/_apis/distributedtask/variablegroups",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VariableGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VariableGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod share_variable_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<(), azure_core::http::NoFormat>);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::VariableGroupProjectReference>,
            pub(crate) variable_group_id: i32,
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
                        let variable_group_id = &this.variable_group_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("variableGroupId", &variable_group_id.to_string());
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/variablegroups",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::VariableGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VariableGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::VariableGroupParameters,
            pub(crate) group_id: i32,
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
                    "{}/{}/_apis/distributedtask/variablegroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VariableGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VariableGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) group_id: i32,
            pub(crate) project_ids: String,
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
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/variablegroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
    pub mod get_variable_groups_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::VariableGroupList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VariableGroupList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) group_ids: String,
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
                        let group_ids = &this.group_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("groupIds", group_ids);
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/variablegroups",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VariableGroupList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VariableGroupList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::VariableGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VariableGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) group_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/variablegroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VariableGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VariableGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod agentclouds {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgentCloud>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            agent_cloud_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                agent_cloud_id,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgentCloud>,
            agent_cloud_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                agent_cloud_id,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            agent_cloud_id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                agent_cloud_id,
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
            azure_core::http::Response<models::TaskAgentCloudList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentCloudList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                    "{}/{}/_apis/distributedtask/agentclouds",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentCloudList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentCloudList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentCloud, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentCloud> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgentCloud,
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
                    "{}/{}/_apis/distributedtask/agentclouds",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentCloud>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentCloud>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentCloud, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentCloud> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) agent_cloud_id: i32,
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
                    "{}/{}/_apis/distributedtask/agentclouds/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.agent_cloud_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentCloud>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentCloud>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentCloud, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentCloud> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgentCloud,
            pub(crate) agent_cloud_id: i32,
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
                    "{}/{}/_apis/distributedtask/agentclouds/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.agent_cloud_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentCloud>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentCloud>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgentCloud, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentCloud> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) agent_cloud_id: i32,
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
                    "{}/{}/_apis/distributedtask/agentclouds/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.agent_cloud_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentCloud>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentCloud>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod requests {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            agent_cloud_id: i32,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                agent_cloud_id,
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
                models::TaskAgentCloudRequestList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentCloudRequestList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) agent_cloud_id: i32,
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
                    "{}/{}/_apis/distributedtask/agentclouds/{}/requests",
                    self.client.endpoint(),
                    &self.organization,
                    &self.agent_cloud_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentCloudRequestList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::TaskAgentCloudRequestList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod agentcloudtypes {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get agent cloud types."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
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
                models::TaskAgentCloudTypeList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentCloudTypeList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                    "{}/{}/_apis/distributedtask/agentcloudtypes",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentCloudTypeList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::TaskAgentCloudTypeList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod agents {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of agents."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: The agent pool containing the agents"]
        pub fn list(&self, organization: impl Into<String>, pool_id: i32) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
                agent_name: None,
                include_capabilities: None,
                include_assigned_request: None,
                include_last_completed_request: None,
                property_filters: None,
                demands: None,
            }
        }
        #[doc = "Adds an agent to a pool.  You probably don't want to call this endpoint directly. Instead, [configure an agent](https://docs.microsoft.com/azure/devops/pipelines/agents/agents) using the agent download package."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Details about the agent being added"]
        #[doc = "* `pool_id`: The agent pool in which to add the agent"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgent>,
            pool_id: i32,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                pool_id,
            }
        }
        #[doc = "Get information about an agent."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: The agent pool containing the agent"]
        #[doc = "* `agent_id`: The agent ID to get information about"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            pool_id: i32,
            agent_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
                agent_id,
                include_capabilities: None,
                include_assigned_request: None,
                include_last_completed_request: None,
                property_filters: None,
            }
        }
        #[doc = "Replace an agent.  You probably don't want to call this endpoint directly. Instead, [use the agent configuration script](https://docs.microsoft.com/azure/devops/pipelines/agents/agents) to remove and reconfigure an agent from your organization."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Updated details about the replacing agent"]
        #[doc = "* `pool_id`: The agent pool to use"]
        #[doc = "* `agent_id`: The agent to replace"]
        pub fn replace_agent(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgent>,
            pool_id: i32,
            agent_id: i32,
        ) -> replace_agent::RequestBuilder {
            replace_agent::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                pool_id,
                agent_id,
            }
        }
        #[doc = "Update agent details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Updated details about the agent"]
        #[doc = "* `pool_id`: The agent pool to use"]
        #[doc = "* `agent_id`: The agent to update"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskAgent>,
            pool_id: i32,
            agent_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                pool_id,
                agent_id,
            }
        }
        #[doc = "Delete an agent.  You probably don't want to call this endpoint directly. Instead, [use the agent configuration script](https://docs.microsoft.com/azure/devops/pipelines/agents/agents) to remove an agent from your organization."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: The pool ID to remove the agent from"]
        #[doc = "* `agent_id`: The agent ID to remove"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            pool_id: i32,
            agent_id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
                agent_id,
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
            azure_core::http::Response<models::TaskAgentList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgentList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
            pub(crate) agent_name: Option<String>,
            pub(crate) include_capabilities: Option<bool>,
            pub(crate) include_assigned_request: Option<bool>,
            pub(crate) include_last_completed_request: Option<bool>,
            pub(crate) property_filters: Option<String>,
            pub(crate) demands: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filter on agent name"]
            pub fn agent_name(mut self, agent_name: impl Into<String>) -> Self {
                self.agent_name = Some(agent_name.into());
                self
            }
            #[doc = "Whether to include the agents' capabilities in the response"]
            pub fn include_capabilities(mut self, include_capabilities: bool) -> Self {
                self.include_capabilities = Some(include_capabilities);
                self
            }
            #[doc = "Whether to include details about the agents' current work"]
            pub fn include_assigned_request(mut self, include_assigned_request: bool) -> Self {
                self.include_assigned_request = Some(include_assigned_request);
                self
            }
            #[doc = "Whether to include details about the agents' most recent completed work"]
            pub fn include_last_completed_request(
                mut self,
                include_last_completed_request: bool,
            ) -> Self {
                self.include_last_completed_request = Some(include_last_completed_request);
                self
            }
            #[doc = "Filter which custom properties will be returned"]
            pub fn property_filters(mut self, property_filters: impl Into<String>) -> Self {
                self.property_filters = Some(property_filters.into());
                self
            }
            #[doc = "Filter by demands the agents can satisfy"]
            pub fn demands(mut self, demands: impl Into<String>) -> Self {
                self.demands = Some(demands.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(agent_name) = &this.agent_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("agentName", agent_name);
                        }
                        if let Some(include_capabilities) = &this.include_capabilities {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeCapabilities",
                                &include_capabilities.to_string(),
                            );
                        }
                        if let Some(include_assigned_request) = &this.include_assigned_request {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeAssignedRequest",
                                &include_assigned_request.to_string(),
                            );
                        }
                        if let Some(include_last_completed_request) =
                            &this.include_last_completed_request
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeLastCompletedRequest",
                                &include_last_completed_request.to_string(),
                            );
                        }
                        if let Some(property_filters) = &this.property_filters {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("propertyFilters", property_filters);
                        }
                        if let Some(demands) = &this.demands {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("demands", demands);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/pools/{}/agents",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgentList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgentList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgent, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgent> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgent,
            pub(crate) pool_id: i32,
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
                    "{}/{}/_apis/distributedtask/pools/{}/agents",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgent>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgent>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgent, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgent> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
            pub(crate) agent_id: i32,
            pub(crate) include_capabilities: Option<bool>,
            pub(crate) include_assigned_request: Option<bool>,
            pub(crate) include_last_completed_request: Option<bool>,
            pub(crate) property_filters: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Whether to include the agent's capabilities in the response"]
            pub fn include_capabilities(mut self, include_capabilities: bool) -> Self {
                self.include_capabilities = Some(include_capabilities);
                self
            }
            #[doc = "Whether to include details about the agent's current work"]
            pub fn include_assigned_request(mut self, include_assigned_request: bool) -> Self {
                self.include_assigned_request = Some(include_assigned_request);
                self
            }
            #[doc = "Whether to include details about the agents' most recent completed work"]
            pub fn include_last_completed_request(
                mut self,
                include_last_completed_request: bool,
            ) -> Self {
                self.include_last_completed_request = Some(include_last_completed_request);
                self
            }
            #[doc = "Filter which custom properties will be returned"]
            pub fn property_filters(mut self, property_filters: impl Into<String>) -> Self {
                self.property_filters = Some(property_filters.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(include_capabilities) = &this.include_capabilities {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeCapabilities",
                                &include_capabilities.to_string(),
                            );
                        }
                        if let Some(include_assigned_request) = &this.include_assigned_request {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeAssignedRequest",
                                &include_assigned_request.to_string(),
                            );
                        }
                        if let Some(include_last_completed_request) =
                            &this.include_last_completed_request
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeLastCompletedRequest",
                                &include_last_completed_request.to_string(),
                            );
                        }
                        if let Some(property_filters) = &this.property_filters {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("propertyFilters", property_filters);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/pools/{}/agents/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id,
                    &self.agent_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgent>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgent>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod replace_agent {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::TaskAgent, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgent> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgent,
            pub(crate) pool_id: i32,
            pub(crate) agent_id: i32,
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
                    "{}/{}/_apis/distributedtask/pools/{}/agents/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id,
                    &self.agent_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgent>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgent>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskAgent, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskAgent> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskAgent,
            pub(crate) pool_id: i32,
            pub(crate) agent_id: i32,
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
                    "{}/{}/_apis/distributedtask/pools/{}/agents/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id,
                    &self.agent_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskAgent>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskAgent>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
            pub(crate) agent_id: i32,
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
                    "{}/{}/_apis/distributedtask/pools/{}/agents/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id,
                    &self.agent_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod yamlschema {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "GET the Yaml schema used for Yaml file validation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(&self, organization: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                validate_task_names: None,
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
            azure_core::http::Response<serde_json::Value, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<serde_json::Value> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) validate_task_names: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Whether the schema should validate that tasks are actually installed (useful for offline tools where you don't want validation)."]
            pub fn validate_task_names(mut self, validate_task_names: bool) -> Self {
                self.validate_task_names = Some(validate_task_names);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(validate_task_names) = &this.validate_task_names {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("validateTaskNames", &validate_task_names.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/yamlschema",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<serde_json::Value>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<serde_json::Value>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod deploymentgroups {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of deployment groups by name or IDs."]
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
                name: None,
                action_filter: None,
                expand: None,
                continuation_token: None,
                top: None,
                ids: None,
            }
        }
        #[doc = "Create a deployment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Deployment group to create."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::DeploymentGroupCreateParameter>,
            project: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get a deployment group by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `deployment_group_id`: ID of the deployment group."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            deployment_group_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                deployment_group_id,
                action_filter: None,
                expand: None,
            }
        }
        #[doc = "Update a deployment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Deployment group to update."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `deployment_group_id`: ID of the deployment group."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::DeploymentGroupUpdateParameter>,
            project: impl Into<String>,
            deployment_group_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                deployment_group_id,
            }
        }
        #[doc = "Delete a deployment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `deployment_group_id`: ID of the deployment group to be deleted."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            deployment_group_id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                deployment_group_id,
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
            azure_core::http::Response<models::DeploymentGroupList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentGroupList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) name: Option<String>,
            pub(crate) action_filter: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) ids: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Name of the deployment group."]
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            #[doc = "Get only deployment groups on which this action can be performed."]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Include these additional details in the returned objects."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Get deployment groups with names greater than this continuation token lexicographically."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Maximum number of deployment groups to return. Default is **1000**."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Comma separated list of IDs of the deployment groups."]
            pub fn ids(mut self, ids: impl Into<String>) -> Self {
                self.ids = Some(ids.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(name) = &this.name {
                            req.url_mut().query_pairs_mut().append_pair("name", name);
                        }
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(ids) = &this.ids {
                            req.url_mut().query_pairs_mut().append_pair("ids", ids);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentGroupList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeploymentGroupList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::DeploymentGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::DeploymentGroupCreateParameter,
            pub(crate) project: String,
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
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeploymentGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::DeploymentGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) deployment_group_id: i32,
            pub(crate) action_filter: Option<String>,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Get the deployment group only if this action can be performed on it."]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            #[doc = "Include these additional details in the returned object."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                                .append_pair("actionFilter", action_filter);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.deployment_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeploymentGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::DeploymentGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::DeploymentGroupUpdateParameter,
            pub(crate) project: String,
            pub(crate) deployment_group_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.deployment_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeploymentGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) deployment_group_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.deployment_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod targets {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of deployment targets in a deployment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `deployment_group_id`: ID of the deployment group."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            deployment_group_id: i32,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                deployment_group_id,
                tags: None,
                name: None,
                partial_name_match: None,
                expand: None,
                agent_status: None,
                agent_job_result: None,
                continuation_token: None,
                top: None,
                enabled: None,
                property_filters: None,
            }
        }
        #[doc = "Update tags of a list of deployment targets in a deployment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Deployment targets with tags to udpdate."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `deployment_group_id`: ID of the deployment group in which deployment targets are updated."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: Vec<models::DeploymentTargetUpdateParameter>,
            project: impl Into<String>,
            deployment_group_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                deployment_group_id,
            }
        }
        #[doc = "Get a deployment target by its ID in a deployment group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `deployment_group_id`: ID of the deployment group to which deployment target belongs."]
        #[doc = "* `target_id`: ID of the deployment target to return."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            deployment_group_id: i32,
            target_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                deployment_group_id,
                target_id,
                expand: None,
            }
        }
        #[doc = "Delete a deployment target in a deployment group. This deletes the agent from associated deployment pool too."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `deployment_group_id`: ID of the deployment group in which deployment target is deleted."]
        #[doc = "* `target_id`: ID of the deployment target to delete."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            deployment_group_id: i32,
            target_id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                deployment_group_id,
                target_id,
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
            azure_core::http::Response<models::DeploymentMachineList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentMachineList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) deployment_group_id: i32,
            pub(crate) tags: Option<String>,
            pub(crate) name: Option<String>,
            pub(crate) partial_name_match: Option<bool>,
            pub(crate) expand: Option<String>,
            pub(crate) agent_status: Option<String>,
            pub(crate) agent_job_result: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) enabled: Option<bool>,
            pub(crate) property_filters: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Get only the deployment targets that contain all these comma separted list of tags."]
            pub fn tags(mut self, tags: impl Into<String>) -> Self {
                self.tags = Some(tags.into());
                self
            }
            #[doc = "Name pattern of the deployment targets to return."]
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            #[doc = "When set to true, treats **name** as pattern. Else treats it as absolute match. Default is **false**."]
            pub fn partial_name_match(mut self, partial_name_match: bool) -> Self {
                self.partial_name_match = Some(partial_name_match);
                self
            }
            #[doc = "Include these additional details in the returned objects."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Get only deployment targets that have this status."]
            pub fn agent_status(mut self, agent_status: impl Into<String>) -> Self {
                self.agent_status = Some(agent_status.into());
                self
            }
            #[doc = "Get only deployment targets that have this last job result."]
            pub fn agent_job_result(mut self, agent_job_result: impl Into<String>) -> Self {
                self.agent_job_result = Some(agent_job_result.into());
                self
            }
            #[doc = "Get deployment targets with names greater than this continuation token lexicographically."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Maximum number of deployment targets to return. Default is **1000**."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Get only deployment targets that are enabled or disabled. Default is 'null' which returns all the targets."]
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.enabled = Some(enabled);
                self
            }
            pub fn property_filters(mut self, property_filters: impl Into<String>) -> Self {
                self.property_filters = Some(property_filters.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(tags) = &this.tags {
                            req.url_mut().query_pairs_mut().append_pair("tags", tags);
                        }
                        if let Some(name) = &this.name {
                            req.url_mut().query_pairs_mut().append_pair("name", name);
                        }
                        if let Some(partial_name_match) = &this.partial_name_match {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("partialNameMatch", &partial_name_match.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(agent_status) = &this.agent_status {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("agentStatus", agent_status);
                        }
                        if let Some(agent_job_result) = &this.agent_job_result {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("agentJobResult", agent_job_result);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(enabled) = &this.enabled {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("enabled", &enabled.to_string());
                        }
                        if let Some(property_filters) = &this.property_filters {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("propertyFilters", property_filters);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups/{}/targets",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.deployment_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentMachineList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeploymentMachineList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::DeploymentMachineList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentMachineList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::DeploymentTargetUpdateParameter>,
            pub(crate) project: String,
            pub(crate) deployment_group_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups/{}/targets",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.deployment_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentMachineList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeploymentMachineList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::DeploymentMachine, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentMachine> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) deployment_group_id: i32,
            pub(crate) target_id: i32,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Include these additional details in the returned objects."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups/{}/targets/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.deployment_group_id,
                    &self.target_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentMachine>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeploymentMachine>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) deployment_group_id: i32,
            pub(crate) target_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/deploymentgroups/{}/targets/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.deployment_group_id,
                    &self.target_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod environments {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all environments."]
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
                name: None,
                continuation_token: None,
                top: None,
            }
        }
        #[doc = "Create an environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Environment to create."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::EnvironmentCreateParameter>,
            project: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get an environment by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `environment_id`: ID of the environment."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            environment_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                environment_id,
                expands: None,
            }
        }
        #[doc = "Update the specified environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Environment data to update."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `environment_id`: ID of the environment."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::EnvironmentUpdateParameter>,
            project: impl Into<String>,
            environment_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                environment_id,
            }
        }
        #[doc = "Delete the specified environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `environment_id`: ID of the environment."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            environment_id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                environment_id,
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
                models::EnvironmentInstanceList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EnvironmentInstanceList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) name: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(name) = &this.name {
                            req.url_mut().query_pairs_mut().append_pair("name", name);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/environments",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::EnvironmentInstanceList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::EnvironmentInstanceList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::EnvironmentInstance, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EnvironmentInstance> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::EnvironmentCreateParameter,
            pub(crate) project: String,
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
                    "{}/{}/{}/_apis/distributedtask/environments",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::EnvironmentInstance>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::EnvironmentInstance>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::EnvironmentInstance, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EnvironmentInstance> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) environment_id: i32,
            pub(crate) expands: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Include these additional details in the returned objects."]
            pub fn expands(mut self, expands: impl Into<String>) -> Self {
                self.expands = Some(expands.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(expands) = &this.expands {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("expands", expands);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/environments/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.environment_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::EnvironmentInstance>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::EnvironmentInstance>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::EnvironmentInstance, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EnvironmentInstance> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::EnvironmentUpdateParameter,
            pub(crate) project: String,
            pub(crate) environment_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/environments/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.environment_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::EnvironmentInstance>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::EnvironmentInstance>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) environment_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/environments/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.environment_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod environmentdeployment_records {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get environment deployment execution history"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            environment_id: i32,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                environment_id,
                continuation_token: None,
                top: None,
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
                models::EnvironmentDeploymentExecutionRecordList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::EnvironmentDeploymentExecutionRecordList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) environment_id: i32,
            pub(crate) continuation_token: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("top", &top.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/environments/{}/environmentdeploymentrecords",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.environment_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::EnvironmentDeploymentExecutionRecordList>;
            type IntoFuture = BoxFuture<
                'static,
                azure_core::Result<models::EnvironmentDeploymentExecutionRecordList>,
            >;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod kubernetes {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::KubernetesResourceCreateParameters>,
            project: impl Into<String>,
            environment_id: i32,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                environment_id,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            environment_id: i32,
            resource_id: i32,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                environment_id,
                resource_id,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            environment_id: i32,
            resource_id: i32,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                environment_id,
                resource_id,
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
            azure_core::http::Response<models::KubernetesResource, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::KubernetesResource> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::KubernetesResourceCreateParameters,
            pub(crate) project: String,
            pub(crate) environment_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/environments/{}/providers/kubernetes",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.environment_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::KubernetesResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::KubernetesResource>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::KubernetesResource, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::KubernetesResource> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) environment_id: i32,
            pub(crate) resource_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/environments/{}/providers/kubernetes/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.environment_id,
                    &self.resource_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::KubernetesResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::KubernetesResource>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) environment_id: i32,
            pub(crate) resource_id: i32,
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
                    "{}/{}/{}/_apis/distributedtask/environments/{}/providers/kubernetes/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.environment_id,
                    &self.resource_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod taskgroups {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create a task group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Task group object to create."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskGroupCreateParameter>,
            project: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "List task groups."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `task_group_id`: Id of the task group."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            task_group_id: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                task_group_id: task_group_id.into(),
                expanded: None,
                task_id_filter: None,
                deleted: None,
                top: None,
                continuation_token: None,
                query_order: None,
            }
        }
        #[doc = "Update a task group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Task group to update."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `task_group_id`: Id of the task group to update."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TaskGroupUpdateParameter>,
            project: impl Into<String>,
            task_group_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                task_group_id: task_group_id.into(),
            }
        }
        #[doc = "Delete a task group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `task_group_id`: Id of the task group to be deleted."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            task_group_id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                task_group_id: task_group_id.into(),
                comment: None,
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
            azure_core::http::Response<models::TaskGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskGroupCreateParameter,
            pub(crate) project: String,
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
                    "{}/{}/{}/_apis/distributedtask/taskgroups",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskGroupList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskGroupList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) task_group_id: String,
            pub(crate) expanded: Option<bool>,
            pub(crate) task_id_filter: Option<String>,
            pub(crate) deleted: Option<bool>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<time::OffsetDateTime>,
            pub(crate) query_order: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "'true' to recursively expand task groups. Default is 'false'."]
            pub fn expanded(mut self, expanded: bool) -> Self {
                self.expanded = Some(expanded);
                self
            }
            #[doc = "Guid of the taskId to filter."]
            pub fn task_id_filter(mut self, task_id_filter: impl Into<String>) -> Self {
                self.task_id_filter = Some(task_id_filter.into());
                self
            }
            #[doc = "'true'to include deleted task groups. Default is 'false'."]
            pub fn deleted(mut self, deleted: bool) -> Self {
                self.deleted = Some(deleted);
                self
            }
            #[doc = "Number of task groups to get."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Gets the task groups after the continuation token provided."]
            pub fn continuation_token(
                mut self,
                continuation_token: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Gets the results in the defined order. Default is 'CreatedOnDescending'."]
            pub fn query_order(mut self, query_order: impl Into<String>) -> Self {
                self.query_order = Some(query_order.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(expanded) = &this.expanded {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("expanded", &expanded.to_string());
                        }
                        if let Some(task_id_filter) = &this.task_id_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("taskIdFilter", task_id_filter);
                        }
                        if let Some(deleted) = &this.deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deleted", &deleted.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            let formatted_date_time =
                                crate::date_time::format_date_time(continuation_token)?;
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", &formatted_date_time);
                        }
                        if let Some(query_order) = &this.query_order {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("queryOrder", query_order);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/distributedtask/taskgroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.task_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskGroupList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskGroupList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::TaskGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TaskGroup> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TaskGroupUpdateParameter,
            pub(crate) project: String,
            pub(crate) task_group_id: String,
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
                    "{}/{}/{}/_apis/distributedtask/taskgroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.task_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::TaskGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::TaskGroup>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) task_group_id: String,
            pub(crate) comment: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Comments to delete."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                    "{}/{}/{}/_apis/distributedtask/taskgroups/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.project,
                    &self.task_group_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
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
pub mod elasticpools {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of all Elastic Pools."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Create a new elastic pool. This will create a new TaskAgentPool at the organization level. If a project id is provided, this will create a new TaskAgentQueue in the specified project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Elastic pool to create. Contains the properties necessary for configuring a new ElasticPool."]
        #[doc = "* `pool_name`: Name to use for the new TaskAgentPool"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ElasticPool>,
            pool_name: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                pool_name: pool_name.into(),
                authorize_all_pipelines: None,
                auto_provision_project_pools: None,
                project_id: None,
            }
        }
        #[doc = "Returns the Elastic Pool with the specified Pool Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: Pool Id of the associated TaskAgentPool"]
        pub fn get(&self, organization: impl Into<String>, pool_id: i32) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
            }
        }
        #[doc = "Update settings on a specified Elastic Pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: New Elastic Pool settings data"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ElasticPoolSettings>,
            pool_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                pool_id,
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
            azure_core::http::Response<models::ElasticPoolList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ElasticPoolList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                    "{}/{}/_apis/distributedtask/elasticpools",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ElasticPoolList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ElasticPoolList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<
                models::ElasticPoolCreationResult,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ElasticPoolCreationResult> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ElasticPool,
            pub(crate) pool_name: String,
            pub(crate) authorize_all_pipelines: Option<bool>,
            pub(crate) auto_provision_project_pools: Option<bool>,
            pub(crate) project_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Setting to determine if all pipelines are authorized to use this TaskAgentPool by default."]
            pub fn authorize_all_pipelines(mut self, authorize_all_pipelines: bool) -> Self {
                self.authorize_all_pipelines = Some(authorize_all_pipelines);
                self
            }
            #[doc = "Setting to automatically provision TaskAgentQueues in every project for the new pool."]
            pub fn auto_provision_project_pools(
                mut self,
                auto_provision_project_pools: bool,
            ) -> Self {
                self.auto_provision_project_pools = Some(auto_provision_project_pools);
                self
            }
            #[doc = "Optional: If provided, a new TaskAgentQueue will be created in the specified project."]
            pub fn project_id(mut self, project_id: impl Into<String>) -> Self {
                self.project_id = Some(project_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        let pool_name = &this.pool_name;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("poolName", pool_name);
                        if let Some(authorize_all_pipelines) = &this.authorize_all_pipelines {
                            req.url_mut().query_pairs_mut().append_pair(
                                "authorizeAllPipelines",
                                &authorize_all_pipelines.to_string(),
                            );
                        }
                        if let Some(auto_provision_project_pools) =
                            &this.auto_provision_project_pools
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "autoProvisionProjectPools",
                                &auto_provision_project_pools.to_string(),
                            );
                        }
                        if let Some(project_id) = &this.project_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("projectId", project_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/elasticpools",
                    self.client.endpoint(),
                    &self.organization
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ElasticPoolCreationResult>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::ElasticPoolCreationResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::ElasticPool, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ElasticPool> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
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
                    "{}/{}/_apis/distributedtask/elasticpools/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ElasticPool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ElasticPool>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::ElasticPool, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ElasticPool> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ElasticPoolSettings,
            pub(crate) pool_id: i32,
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
                    "{}/{}/_apis/distributedtask/elasticpools/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ElasticPool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ElasticPool>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod elasticpoollogs {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get elastic pool diagnostics logs for a specified Elastic Pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: Pool Id of the Elastic Pool"]
        pub fn list(&self, organization: impl Into<String>, pool_id: i32) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
                top: None,
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
            azure_core::http::Response<models::ElasticPoolLogList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ElasticPoolLogList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "Number of elastic pool logs to retrieve"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                                .append_pair("$top", &top.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/elasticpools/{}/logs",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ElasticPoolLogList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ElasticPoolLogList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod nodes {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of ElasticNodes currently in the ElasticPool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `pool_id`: Pool id of the ElasticPool"]
        pub fn list(&self, organization: impl Into<String>, pool_id: i32) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                pool_id,
                state: None,
            }
        }
        #[doc = "Update properties on a specified ElasticNode"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ElasticNodeSettings>,
            pool_id: i32,
            elastic_node_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                pool_id,
                elastic_node_id,
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
            azure_core::http::Response<models::ElasticNodeList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ElasticNodeList> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) pool_id: i32,
            pub(crate) state: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Optional: Filter to only retrieve ElasticNodes in the given ElasticNodeState"]
            pub fn state(mut self, state: impl Into<String>) -> Self {
                self.state = Some(state.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
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
                        if let Some(state) = &this.state {
                            req.url_mut().query_pairs_mut().append_pair("$state", state);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/distributedtask/elasticpools/{}/nodes",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ElasticNodeList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ElasticNodeList>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
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
            azure_core::http::Response<models::ElasticNode, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ElasticNode> {
                self.0.into_body().await
            }
            pub fn into_raw_response(self) -> azure_core::http::BufResponse {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ElasticNodeSettings,
            pub(crate) pool_id: i32,
            pub(crate) elastic_node_id: i32,
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
                    "{}/{}/_apis/distributedtask/elasticpools/{}/nodes/{}",
                    self.client.endpoint(),
                    &self.organization,
                    &self.pool_id,
                    &self.elastic_node_id
                ))?;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(
                        azure_core::http::headers::query_param::API_VERSION,
                        "7.1-preview",
                    );
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ElasticNode>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ElasticNode>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
