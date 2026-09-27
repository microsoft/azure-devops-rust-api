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
    pub fn dashboards_client(&self) -> dashboards::Client {
        dashboards::Client(self.clone())
    }
    pub fn widget_types_client(&self) -> widget_types::Client {
        widget_types::Client(self.clone())
    }
    pub fn widgets_client(&self) -> widgets::Client {
        widgets::Client(self.clone())
    }
}
pub mod widget_types {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all available widget metadata in alphabetical order, including widgets marked with isVisibleFromCatalog == false."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_widget_types(
            &self,
            organization: impl Into<String>,
            scope: impl Into<models::WidgetScope>,
            project: impl Into<String>,
        ) -> get_widget_types::RequestBuilder {
            get_widget_types::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                scope: scope.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the widget metadata satisfying the specified contribution ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `contribution_id`: The ID of Contribution for the Widget"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_widget_metadata(
            &self,
            organization: impl Into<String>,
            contribution_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_widget_metadata::RequestBuilder {
            get_widget_metadata::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                contribution_id: contribution_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_widget_types {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WidgetTypesResponse, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WidgetTypesResponse> {
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
            pub(crate) scope: models::WidgetScope,
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
                        let scope = &this.scope;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("$scope", &scope.to_string());
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/_apis/dashboard/widgettypes",
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
            type Output = azure_core::Result<models::WidgetTypesResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WidgetTypesResponse>>;
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
    pub mod get_widget_metadata {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::WidgetMetadataResponse,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WidgetMetadataResponse> {
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
            pub(crate) contribution_id: String,
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
                    "{}/{}/{}/_apis/dashboard/widgettypes/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.contribution_id
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
            type Output = azure_core::Result<models::WidgetMetadataResponse>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::WidgetMetadataResponse>>;
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
pub mod dashboards {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of dashboards under a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Create the supplied dashboard."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The initial state of the dashboard"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Dashboard>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Update the name and position of dashboards in the supplied group, and remove omitted dashboards. Does not modify dashboard content."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn replace_dashboards(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::DashboardGroup>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> replace_dashboards::RequestBuilder {
            replace_dashboards::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Get a dashboard by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            team: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Replace configuration for the specified dashboard. Replaces Widget list on Dashboard, only if property is supplied."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The Configuration of the dashboard to replace."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the dashboard to replace."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn replace_dashboard(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Dashboard>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            team: impl Into<String>,
        ) -> replace_dashboard::RequestBuilder {
            replace_dashboard::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Delete a dashboard given its ID. This also deletes the widgets associated with this dashboard."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the dashboard to delete."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            team: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                team: team.into(),
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
            azure_core::http::Response<models::DashboardList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::DashboardList> {
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
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team
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
            type Output = azure_core::Result<models::DashboardList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DashboardList>>;
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
            azure_core::http::Response<models::Dashboard, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Dashboard> {
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
            pub(crate) body: models::Dashboard,
            pub(crate) project: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team
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
            type Output = azure_core::Result<models::Dashboard>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Dashboard>>;
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
    pub mod replace_dashboards {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::DashboardGroup, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::DashboardGroup> {
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
            pub(crate) body: models::DashboardGroup,
            pub(crate) project: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team
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
            type Output = azure_core::Result<models::DashboardGroup>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DashboardGroup>>;
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
            azure_core::http::Response<models::Dashboard, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Dashboard> {
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
            pub(crate) dashboard_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id
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
            type Output = azure_core::Result<models::Dashboard>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Dashboard>>;
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
    pub mod replace_dashboard {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Dashboard, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Dashboard> {
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
            pub(crate) body: models::Dashboard,
            pub(crate) project: String,
            pub(crate) dashboard_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id
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
            type Output = azure_core::Result<models::Dashboard>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Dashboard>>;
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
        pub struct RequestBuilder<M = ()> {
            pub(crate) client: super::super::Client,
            pub(crate) mode: std::marker::PhantomData<M>,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) dashboard_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id
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
}
pub mod widgets {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get widgets contained on the specified dashboard."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the dashboard to read."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get_widgets(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            team: impl Into<String>,
        ) -> get_widgets::RequestBuilder {
            get_widgets::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                team: team.into(),
                e_tag: None,
            }
        }
        #[doc = "Create a widget on the specified dashboard."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: State of the widget to add"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of dashboard the widget will be added to."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Widget>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            team: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Replace the widgets on specified dashboard with the supplied widgets."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Revised state of widgets to store for the dashboard."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the Dashboard to modify."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn replace_widgets(
            &self,
            organization: impl Into<String>,
            body: Vec<models::Widget>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            team: impl Into<String>,
        ) -> replace_widgets::RequestBuilder {
            replace_widgets::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body,
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                team: team.into(),
                e_tag: None,
            }
        }
        #[doc = "Update the supplied widgets on the dashboard using supplied state. State of existing Widgets not passed in the widget list is preserved."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The set of widget states to update on the dashboard."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the Dashboard to modify."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update_widgets(
            &self,
            organization: impl Into<String>,
            body: Vec<models::Widget>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            team: impl Into<String>,
        ) -> update_widgets::RequestBuilder {
            update_widgets::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body,
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                team: team.into(),
                e_tag: None,
            }
        }
        #[doc = "Get the current state of the specified widget."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the dashboard containing the widget."]
        #[doc = "* `widget_id`: ID of the widget to read."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get_widget(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            widget_id: impl Into<String>,
            team: impl Into<String>,
        ) -> get_widget::RequestBuilder {
            get_widget::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                widget_id: widget_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Override the  state of the specified widget."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: State to be written for the widget."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the dashboard containing the widget."]
        #[doc = "* `widget_id`: ID of the widget to update."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn replace_widget(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Widget>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            widget_id: impl Into<String>,
            team: impl Into<String>,
        ) -> replace_widget::RequestBuilder {
            replace_widget::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                widget_id: widget_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Perform a partial update of the specified widget."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Description of the widget changes to apply. All non-null fields will be replaced."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the dashboard containing the widget."]
        #[doc = "* `widget_id`: ID of the widget to update."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update_widget(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Widget>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            widget_id: impl Into<String>,
            team: impl Into<String>,
        ) -> update_widget::RequestBuilder {
            update_widget::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                widget_id: widget_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Delete the specified widget."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `dashboard_id`: ID of the dashboard containing the widget."]
        #[doc = "* `widget_id`: ID of the widget to update."]
        #[doc = "* `team`: Team ID or team name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            dashboard_id: impl Into<String>,
            widget_id: impl Into<String>,
            team: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                project: project.into(),
                dashboard_id: dashboard_id.into(),
                widget_id: widget_id.into(),
                team: team.into(),
            }
        }
    }
    pub mod get_widgets {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WidgetList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WidgetList> {
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
            pub(crate) dashboard_id: String,
            pub(crate) team: String,
            pub(crate) e_tag: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Dashboard Widgets Version"]
            pub fn e_tag(mut self, e_tag: impl Into<String>) -> Self {
                self.e_tag = Some(e_tag.into());
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
                        if let Some(e_tag) = &this.e_tag {
                            req.insert_header("etag", e_tag);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id
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
            type Output = azure_core::Result<models::WidgetList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WidgetList>>;
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
            azure_core::http::Response<models::Widget, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Widget> {
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
            pub(crate) body: models::Widget,
            pub(crate) project: String,
            pub(crate) dashboard_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id
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
            type Output = azure_core::Result<models::Widget>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Widget>>;
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
    pub mod replace_widgets {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WidgetList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WidgetList> {
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
            pub(crate) body: Vec<models::Widget>,
            pub(crate) project: String,
            pub(crate) dashboard_id: String,
            pub(crate) team: String,
            pub(crate) e_tag: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Dashboard Widgets Version"]
            pub fn e_tag(mut self, e_tag: impl Into<String>) -> Self {
                self.e_tag = Some(e_tag.into());
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
                        if let Some(e_tag) = &this.e_tag {
                            req.insert_header("etag", e_tag);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id
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
            type Output = azure_core::Result<models::WidgetList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WidgetList>>;
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
    pub mod update_widgets {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::WidgetList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::WidgetList> {
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
            pub(crate) body: Vec<models::Widget>,
            pub(crate) project: String,
            pub(crate) dashboard_id: String,
            pub(crate) team: String,
            pub(crate) e_tag: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Dashboard Widgets Version"]
            pub fn e_tag(mut self, e_tag: impl Into<String>) -> Self {
                self.e_tag = Some(e_tag.into());
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
                        if let Some(e_tag) = &this.e_tag {
                            req.insert_header("etag", e_tag);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id
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
            type Output = azure_core::Result<models::WidgetList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::WidgetList>>;
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
    pub mod get_widget {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Widget, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Widget> {
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
            pub(crate) dashboard_id: String,
            pub(crate) widget_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id,
                    self.widget_id
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
            type Output = azure_core::Result<models::Widget>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Widget>>;
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
    pub mod replace_widget {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Widget, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Widget> {
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
            pub(crate) body: models::Widget,
            pub(crate) project: String,
            pub(crate) dashboard_id: String,
            pub(crate) widget_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id,
                    self.widget_id
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
            type Output = azure_core::Result<models::Widget>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Widget>>;
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
    pub mod update_widget {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::Widget, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Widget> {
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
            pub(crate) body: models::Widget,
            pub(crate) project: String,
            pub(crate) dashboard_id: String,
            pub(crate) widget_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id,
                    self.widget_id
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
            type Output = azure_core::Result<models::Widget>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Widget>>;
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
            azure_core::http::Response<models::Dashboard, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::Dashboard> {
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
            pub(crate) dashboard_id: String,
            pub(crate) widget_id: String,
            pub(crate) team: String,
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
                    "{}/{}/{}/{}/_apis/dashboard/dashboards/{}/widgets/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.project,
                    self.team,
                    self.dashboard_id,
                    self.widget_id
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
            type Output = azure_core::Result<models::Dashboard>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Dashboard>>;
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
        pub fn dashboards_client(&self) -> dashboards::Client {
            dashboards::Client(self.clone())
        }
        pub fn widget_types_client(&self) -> widget_types::Client {
            widget_types::Client(self.clone())
        }
        pub fn widgets_client(&self) -> widgets::Client {
            widgets::Client(self.clone())
        }
    }
    impl From<super::Client> for Client {
        #[doc = "Create a blocking client that sends requests using an existing async client."]
        fn from(client: super::Client) -> Self {
            Self(client)
        }
    }
    pub mod widget_types {
        use super::super::models;
        use super::super::widget_types::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Get all available widget metadata in alphabetical order, including widgets marked with isVisibleFromCatalog == false."]
            #[doc = ""]
            #[doc = "See [`get_widget_types`](super::super::widget_types::Client::get_widget_types) for details of the arguments."]
            pub fn get_widget_types(
                &self,
                organization: impl Into<String>,
                scope: impl Into<models::WidgetScope>,
                project: impl Into<String>,
            ) -> get_widget_types::RequestBuilder<crate::blocking::Blocking> {
                get_widget_types::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    scope: scope.into(),
                    project: project.into(),
                }
            }
            #[doc = "Get the widget metadata satisfying the specified contribution ID."]
            #[doc = ""]
            #[doc = "See [`get_widget_metadata`](super::super::widget_types::Client::get_widget_metadata) for details of the arguments."]
            pub fn get_widget_metadata(
                &self,
                organization: impl Into<String>,
                contribution_id: impl Into<String>,
                project: impl Into<String>,
            ) -> get_widget_metadata::RequestBuilder<crate::blocking::Blocking> {
                get_widget_metadata::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    contribution_id: contribution_id.into(),
                    project: project.into(),
                }
            }
        }
    }
    pub mod dashboards {
        use super::super::dashboards::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Get a list of dashboards under a project."]
            #[doc = ""]
            #[doc = "See [`list`](super::super::dashboards::Client::list) for details of the arguments."]
            pub fn list(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                team: impl Into<String>,
            ) -> list::RequestBuilder<crate::blocking::Blocking> {
                list::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    team: team.into(),
                }
            }
            #[doc = "Create the supplied dashboard."]
            #[doc = ""]
            #[doc = "See [`create`](super::super::dashboards::Client::create) for details of the arguments."]
            pub fn create(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::Dashboard>,
                project: impl Into<String>,
                team: impl Into<String>,
            ) -> create::RequestBuilder<crate::blocking::Blocking> {
                create::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    team: team.into(),
                }
            }
            #[doc = "Update the name and position of dashboards in the supplied group, and remove omitted dashboards. Does not modify dashboard content."]
            #[doc = ""]
            #[doc = "See [`replace_dashboards`](super::super::dashboards::Client::replace_dashboards) for details of the arguments."]
            pub fn replace_dashboards(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::DashboardGroup>,
                project: impl Into<String>,
                team: impl Into<String>,
            ) -> replace_dashboards::RequestBuilder<crate::blocking::Blocking> {
                replace_dashboards::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    team: team.into(),
                }
            }
            #[doc = "Get a dashboard by its ID."]
            #[doc = ""]
            #[doc = "See [`get`](super::super::dashboards::Client::get) for details of the arguments."]
            pub fn get(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                team: impl Into<String>,
            ) -> get::RequestBuilder<crate::blocking::Blocking> {
                get::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    team: team.into(),
                }
            }
            #[doc = "Replace configuration for the specified dashboard. Replaces Widget list on Dashboard, only if property is supplied."]
            #[doc = ""]
            #[doc = "See [`replace_dashboard`](super::super::dashboards::Client::replace_dashboard) for details of the arguments."]
            pub fn replace_dashboard(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::Dashboard>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                team: impl Into<String>,
            ) -> replace_dashboard::RequestBuilder<crate::blocking::Blocking> {
                replace_dashboard::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    team: team.into(),
                }
            }
            #[doc = "Delete a dashboard given its ID. This also deletes the widgets associated with this dashboard."]
            #[doc = ""]
            #[doc = "See [`delete`](super::super::dashboards::Client::delete) for details of the arguments."]
            pub fn delete(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                team: impl Into<String>,
            ) -> delete::RequestBuilder<crate::blocking::Blocking> {
                delete::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    team: team.into(),
                }
            }
        }
    }
    pub mod widgets {
        use super::super::models;
        use super::super::widgets::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Get widgets contained on the specified dashboard."]
            #[doc = ""]
            #[doc = "See [`get_widgets`](super::super::widgets::Client::get_widgets) for details of the arguments."]
            pub fn get_widgets(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                team: impl Into<String>,
            ) -> get_widgets::RequestBuilder<crate::blocking::Blocking> {
                get_widgets::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    team: team.into(),
                    e_tag: None,
                }
            }
            #[doc = "Create a widget on the specified dashboard."]
            #[doc = ""]
            #[doc = "See [`create`](super::super::widgets::Client::create) for details of the arguments."]
            pub fn create(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::Widget>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                team: impl Into<String>,
            ) -> create::RequestBuilder<crate::blocking::Blocking> {
                create::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    team: team.into(),
                }
            }
            #[doc = "Replace the widgets on specified dashboard with the supplied widgets."]
            #[doc = ""]
            #[doc = "See [`replace_widgets`](super::super::widgets::Client::replace_widgets) for details of the arguments."]
            pub fn replace_widgets(
                &self,
                organization: impl Into<String>,
                body: Vec<models::Widget>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                team: impl Into<String>,
            ) -> replace_widgets::RequestBuilder<crate::blocking::Blocking> {
                replace_widgets::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body,
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    team: team.into(),
                    e_tag: None,
                }
            }
            #[doc = "Update the supplied widgets on the dashboard using supplied state. State of existing Widgets not passed in the widget list is preserved."]
            #[doc = ""]
            #[doc = "See [`update_widgets`](super::super::widgets::Client::update_widgets) for details of the arguments."]
            pub fn update_widgets(
                &self,
                organization: impl Into<String>,
                body: Vec<models::Widget>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                team: impl Into<String>,
            ) -> update_widgets::RequestBuilder<crate::blocking::Blocking> {
                update_widgets::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body,
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    team: team.into(),
                    e_tag: None,
                }
            }
            #[doc = "Get the current state of the specified widget."]
            #[doc = ""]
            #[doc = "See [`get_widget`](super::super::widgets::Client::get_widget) for details of the arguments."]
            pub fn get_widget(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                widget_id: impl Into<String>,
                team: impl Into<String>,
            ) -> get_widget::RequestBuilder<crate::blocking::Blocking> {
                get_widget::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    widget_id: widget_id.into(),
                    team: team.into(),
                }
            }
            #[doc = "Override the  state of the specified widget."]
            #[doc = ""]
            #[doc = "See [`replace_widget`](super::super::widgets::Client::replace_widget) for details of the arguments."]
            pub fn replace_widget(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::Widget>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                widget_id: impl Into<String>,
                team: impl Into<String>,
            ) -> replace_widget::RequestBuilder<crate::blocking::Blocking> {
                replace_widget::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    widget_id: widget_id.into(),
                    team: team.into(),
                }
            }
            #[doc = "Perform a partial update of the specified widget."]
            #[doc = ""]
            #[doc = "See [`update_widget`](super::super::widgets::Client::update_widget) for details of the arguments."]
            pub fn update_widget(
                &self,
                organization: impl Into<String>,
                body: impl Into<models::Widget>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                widget_id: impl Into<String>,
                team: impl Into<String>,
            ) -> update_widget::RequestBuilder<crate::blocking::Blocking> {
                update_widget::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    body: body.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    widget_id: widget_id.into(),
                    team: team.into(),
                }
            }
            #[doc = "Delete the specified widget."]
            #[doc = ""]
            #[doc = "See [`delete`](super::super::widgets::Client::delete) for details of the arguments."]
            pub fn delete(
                &self,
                organization: impl Into<String>,
                project: impl Into<String>,
                dashboard_id: impl Into<String>,
                widget_id: impl Into<String>,
                team: impl Into<String>,
            ) -> delete::RequestBuilder<crate::blocking::Blocking> {
                delete::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    project: project.into(),
                    dashboard_id: dashboard_id.into(),
                    widget_id: widget_id.into(),
                    team: team.into(),
                }
            }
        }
    }
}
