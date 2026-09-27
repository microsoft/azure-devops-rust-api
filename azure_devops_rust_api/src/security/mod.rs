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
    pub fn access_control_entries_client(&self) -> access_control_entries::Client {
        access_control_entries::Client(self.clone())
    }
    pub fn access_control_lists_client(&self) -> access_control_lists::Client {
        access_control_lists::Client(self.clone())
    }
    pub fn permissions_client(&self) -> permissions::Client {
        permissions::Client(self.clone())
    }
    pub fn security_namespaces_client(&self) -> security_namespaces::Client {
        security_namespaces::Client(self.clone())
    }
}
pub mod access_control_entries {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Add or update ACEs in the ACL for the provided token. The request body contains the target token, a list of [ACEs](https://docs.microsoft.com/en-us/rest/api/azure/devops/security/access%20control%20entries/set%20access%20control%20entries?#accesscontrolentry) and a optional merge parameter. In the case of a collision (by identity descriptor) with an existing ACE in the ACL, the \"merge\" parameter determines the behavior. If set, the existing ACE has its allow and deny merged with the incoming ACE's allow and deny. If unset, the existing ACE is displaced."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn set_access_control_entries(
            &self,
            body: impl Into<serde_json::Value>,
            security_namespace_id: impl Into<String>,
            organization: impl Into<String>,
        ) -> set_access_control_entries::RequestBuilder {
            set_access_control_entries::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                body: body.into(),
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
            }
        }
        #[doc = "Remove the specified ACEs from the ACL belonging to the specified token."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn remove_access_control_entries(
            &self,
            security_namespace_id: impl Into<String>,
            organization: impl Into<String>,
        ) -> remove_access_control_entries::RequestBuilder {
            remove_access_control_entries::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
                token: None,
                descriptors: None,
            }
        }
    }
    pub mod set_access_control_entries {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::AccessControlEntryList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::AccessControlEntryList> {
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
            pub(crate) body: serde_json::Value,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
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
                    "{}/{}/_apis/accesscontrolentries/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id
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
            type Output = azure_core::Result<models::AccessControlEntryList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::AccessControlEntryList>>;
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
    pub mod remove_access_control_entries {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<bool, azure_core::http::JsonFormat>);
        impl Response {
            pub fn into_body(self) -> azure_core::Result<bool> {
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
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) token: Option<String>,
            pub(crate) descriptors: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "The token whose ACL should be modified."]
            pub fn token(mut self, token: impl Into<String>) -> Self {
                self.token = Some(token.into());
                self
            }
            #[doc = "String containing a list of identity descriptors separated by ',' whose entries should be removed."]
            pub fn descriptors(mut self, descriptors: impl Into<String>) -> Self {
                self.descriptors = Some(descriptors.into());
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
                        if let Some(token) = &this.token {
                            req.url_mut().query_pairs_mut().append_pair("token", token);
                        }
                        if let Some(descriptors) = &this.descriptors {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("descriptors", descriptors);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/accesscontrolentries/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id
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
            type Output = azure_core::Result<bool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<bool>>;
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
pub mod access_control_lists {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return a list of access control lists for the specified security namespace and token. All ACLs in the security namespace will be retrieved if no optional parameters are provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn query(
            &self,
            security_namespace_id: impl Into<String>,
            organization: impl Into<String>,
        ) -> query::RequestBuilder {
            query::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
                token: None,
                descriptors: None,
                include_extended_info: None,
                recurse: None,
            }
        }
        #[doc = "Create or update one or more access control lists. All data that currently exists for the ACLs supplied will be overwritten."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: A list of ACLs to create or update."]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn set_access_control_lists(
            &self,
            body: impl Into<models::AccessControlListBody>,
            security_namespace_id: impl Into<String>,
            organization: impl Into<String>,
        ) -> set_access_control_lists::RequestBuilder {
            set_access_control_lists::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                body: body.into(),
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
            }
        }
        #[doc = "Remove access control lists under the specfied security namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn remove_access_control_lists(
            &self,
            security_namespace_id: impl Into<String>,
            organization: impl Into<String>,
        ) -> remove_access_control_lists::RequestBuilder {
            remove_access_control_lists::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
                tokens: None,
                recurse: None,
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
            azure_core::http::Response<models::AccessControlListList, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::AccessControlListList> {
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
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) token: Option<String>,
            pub(crate) descriptors: Option<String>,
            pub(crate) include_extended_info: Option<bool>,
            pub(crate) recurse: Option<bool>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Security token"]
            pub fn token(mut self, token: impl Into<String>) -> Self {
                self.token = Some(token.into());
                self
            }
            #[doc = "An optional filter string containing a list of identity descriptors separated by ',' whose ACEs should be retrieved. If this is left null, entire ACLs will be returned."]
            pub fn descriptors(mut self, descriptors: impl Into<String>) -> Self {
                self.descriptors = Some(descriptors.into());
                self
            }
            #[doc = "If true, populate the extended information properties for the access control entries contained in the returned lists."]
            pub fn include_extended_info(mut self, include_extended_info: bool) -> Self {
                self.include_extended_info = Some(include_extended_info);
                self
            }
            #[doc = "If true and this is a hierarchical namespace, return child ACLs of the specified token."]
            pub fn recurse(mut self, recurse: bool) -> Self {
                self.recurse = Some(recurse);
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
                        if let Some(token) = &this.token {
                            req.url_mut().query_pairs_mut().append_pair("token", token);
                        }
                        if let Some(descriptors) = &this.descriptors {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("descriptors", descriptors);
                        }
                        if let Some(include_extended_info) = &this.include_extended_info {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeExtendedInfo",
                                &include_extended_info.to_string(),
                            );
                        }
                        if let Some(recurse) = &this.recurse {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("recurse", &recurse.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/accesscontrollists/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id
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
            type Output = azure_core::Result<models::AccessControlListList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AccessControlListList>>;
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
    pub mod set_access_control_lists {
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
            pub(crate) body: models::AccessControlListBody,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
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
                    "{}/{}/_apis/accesscontrollists/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id
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
    pub mod remove_access_control_lists {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<bool, azure_core::http::JsonFormat>);
        impl Response {
            pub fn into_body(self) -> azure_core::Result<bool> {
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
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) tokens: Option<String>,
            pub(crate) recurse: Option<bool>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "One or more comma-separated security tokens"]
            pub fn tokens(mut self, tokens: impl Into<String>) -> Self {
                self.tokens = Some(tokens.into());
                self
            }
            #[doc = "If true and this is a hierarchical namespace, also remove child ACLs of the specified tokens."]
            pub fn recurse(mut self, recurse: bool) -> Self {
                self.recurse = Some(recurse);
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
                        if let Some(tokens) = &this.tokens {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("tokens", tokens);
                        }
                        if let Some(recurse) = &this.recurse {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("recurse", &recurse.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/accesscontrollists/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id
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
            type Output = azure_core::Result<bool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<bool>>;
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
pub mod permissions {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Evaluates whether the caller has the specified permissions on the specified set of security tokens."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `permissions`: Permissions to evaluate."]
        pub fn has_permissions(
            &self,
            security_namespace_id: impl Into<String>,
            organization: impl Into<String>,
            permissions: i32,
        ) -> has_permissions::RequestBuilder {
            has_permissions::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
                permissions,
                tokens: None,
                always_allow_administrators: None,
                delimiter: None,
            }
        }
        #[doc = "Removes the specified permissions on a security token for a user or group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        #[doc = "* `descriptor`: Identity descriptor of the user to remove permissions for."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `permissions`: Permissions to remove."]
        pub fn remove_permission(
            &self,
            security_namespace_id: impl Into<String>,
            descriptor: impl Into<String>,
            organization: impl Into<String>,
            permissions: i32,
        ) -> remove_permission::RequestBuilder {
            remove_permission::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                security_namespace_id: security_namespace_id.into(),
                descriptor: descriptor.into(),
                organization: organization.into(),
                permissions,
                token: None,
            }
        }
        #[doc = "Evaluates multiple permissions for the calling user.  Note: This method does not aggregate the results, nor does it short-circuit if one of the permissions evaluates to false."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: The set of evaluation requests."]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn has_permissions_batch(
            &self,
            body: impl Into<models::PermissionEvaluationBatch>,
            organization: impl Into<String>,
        ) -> has_permissions_batch::RequestBuilder {
            has_permissions_batch::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                body: body.into(),
                organization: organization.into(),
            }
        }
    }
    pub mod has_permissions {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::http::Response<Vec<bool>, azure_core::http::JsonFormat>);
        impl Response {
            pub fn into_body(self) -> azure_core::Result<Vec<bool>> {
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
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) permissions: i32,
            pub(crate) tokens: Option<String>,
            pub(crate) always_allow_administrators: Option<bool>,
            pub(crate) delimiter: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "One or more security tokens to evaluate."]
            pub fn tokens(mut self, tokens: impl Into<String>) -> Self {
                self.tokens = Some(tokens.into());
                self
            }
            #[doc = "If true and if the caller is an administrator, always return true."]
            pub fn always_allow_administrators(
                mut self,
                always_allow_administrators: bool,
            ) -> Self {
                self.always_allow_administrators = Some(always_allow_administrators);
                self
            }
            #[doc = "Optional security token separator. Defaults to \",\"."]
            pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
                self.delimiter = Some(delimiter.into());
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
                        if let Some(tokens) = &this.tokens {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("tokens", tokens);
                        }
                        if let Some(always_allow_administrators) = &this.always_allow_administrators
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "alwaysAllowAdministrators",
                                &always_allow_administrators.to_string(),
                            );
                        }
                        if let Some(delimiter) = &this.delimiter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("delimiter", delimiter);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/permissions/{}/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id,
                    self.permissions
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
            type Output = azure_core::Result<Vec<bool>>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<Vec<bool>>>;
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
    pub mod remove_permission {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<models::AccessControlEntry, azure_core::http::JsonFormat>,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::AccessControlEntry> {
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
            pub(crate) security_namespace_id: String,
            pub(crate) descriptor: String,
            pub(crate) organization: String,
            pub(crate) permissions: i32,
            pub(crate) token: Option<String>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "Security token to remove permissions for."]
            pub fn token(mut self, token: impl Into<String>) -> Self {
                self.token = Some(token.into());
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
                        let descriptor = &this.descriptor;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("descriptor", descriptor);
                        if let Some(token) = &this.token {
                            req.url_mut().query_pairs_mut().append_pair("token", token);
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/permissions/{}/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id,
                    self.permissions
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
            type Output = azure_core::Result<models::AccessControlEntry>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AccessControlEntry>>;
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
    pub mod has_permissions_batch {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(
            azure_core::http::Response<
                models::PermissionEvaluationBatch,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::PermissionEvaluationBatch> {
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
            pub(crate) body: models::PermissionEvaluationBatch,
            pub(crate) organization: String,
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
                    "{}/{}/_apis/security/permissionevaluationbatch",
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
            type Output = azure_core::Result<models::PermissionEvaluationBatch>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::PermissionEvaluationBatch>>;
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
pub mod security_namespaces {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List all security namespaces or just the specified namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `security_namespace_id`: Security namespace identifier."]
        pub fn query(
            &self,
            organization: impl Into<String>,
            security_namespace_id: impl Into<String>,
        ) -> query::RequestBuilder {
            query::RequestBuilder {
                client: self.0.clone(),
                mode: std::marker::PhantomData,
                organization: organization.into(),
                security_namespace_id: security_namespace_id.into(),
                local_only: None,
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
                models::SecurityNamespaceDescriptionList,
                azure_core::http::JsonFormat,
            >,
        );
        impl Response {
            pub fn into_body(self) -> azure_core::Result<models::SecurityNamespaceDescriptionList> {
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
            pub(crate) security_namespace_id: String,
            pub(crate) local_only: Option<bool>,
        }
        impl<M> RequestBuilder<M> {
            #[doc = "If true, retrieve only local security namespaces."]
            pub fn local_only(mut self, local_only: bool) -> Self {
                self.local_only = Some(local_only);
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
                        if let Some(local_only) = &this.local_only {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("localOnly", &local_only.to_string());
                        }
                        let req_body = azure_core::Bytes::new();
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?.into()))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = azure_core::http::Url::parse(&format!(
                    "{}/{}/_apis/securitynamespaces/{}",
                    self.client.endpoint(),
                    self.organization,
                    self.security_namespace_id
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
            type Output = azure_core::Result<models::SecurityNamespaceDescriptionList>;
            type IntoFuture =
                BoxFuture<'static, azure_core::Result<models::SecurityNamespaceDescriptionList>>;
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
        pub fn access_control_entries_client(&self) -> access_control_entries::Client {
            access_control_entries::Client(self.clone())
        }
        pub fn access_control_lists_client(&self) -> access_control_lists::Client {
            access_control_lists::Client(self.clone())
        }
        pub fn permissions_client(&self) -> permissions::Client {
            permissions::Client(self.clone())
        }
        pub fn security_namespaces_client(&self) -> security_namespaces::Client {
            security_namespaces::Client(self.clone())
        }
    }
    impl From<super::Client> for Client {
        #[doc = "Create a blocking client that sends requests using an existing async client."]
        fn from(client: super::Client) -> Self {
            Self(client)
        }
    }
    pub mod access_control_entries {
        use super::super::access_control_entries::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Add or update ACEs in the ACL for the provided token. The request body contains the target token, a list of [ACEs](https://docs.microsoft.com/en-us/rest/api/azure/devops/security/access%20control%20entries/set%20access%20control%20entries?#accesscontrolentry) and a optional merge parameter. In the case of a collision (by identity descriptor) with an existing ACE in the ACL, the \"merge\" parameter determines the behavior. If set, the existing ACE has its allow and deny merged with the incoming ACE's allow and deny. If unset, the existing ACE is displaced."]
            #[doc = ""]
            #[doc = "See [`set_access_control_entries`](super::super::access_control_entries::Client::set_access_control_entries) for details of the arguments."]
            pub fn set_access_control_entries(
                &self,
                body: impl Into<serde_json::Value>,
                security_namespace_id: impl Into<String>,
                organization: impl Into<String>,
            ) -> set_access_control_entries::RequestBuilder<crate::blocking::Blocking> {
                set_access_control_entries::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    body: body.into(),
                    security_namespace_id: security_namespace_id.into(),
                    organization: organization.into(),
                }
            }
            #[doc = "Remove the specified ACEs from the ACL belonging to the specified token."]
            #[doc = ""]
            #[doc = "See [`remove_access_control_entries`](super::super::access_control_entries::Client::remove_access_control_entries) for details of the arguments."]
            pub fn remove_access_control_entries(
                &self,
                security_namespace_id: impl Into<String>,
                organization: impl Into<String>,
            ) -> remove_access_control_entries::RequestBuilder<crate::blocking::Blocking>
            {
                remove_access_control_entries::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    security_namespace_id: security_namespace_id.into(),
                    organization: organization.into(),
                    token: None,
                    descriptors: None,
                }
            }
        }
    }
    pub mod access_control_lists {
        use super::super::access_control_lists::*;
        use super::super::models;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Return a list of access control lists for the specified security namespace and token. All ACLs in the security namespace will be retrieved if no optional parameters are provided."]
            #[doc = ""]
            #[doc = "See [`query`](super::super::access_control_lists::Client::query) for details of the arguments."]
            pub fn query(
                &self,
                security_namespace_id: impl Into<String>,
                organization: impl Into<String>,
            ) -> query::RequestBuilder<crate::blocking::Blocking> {
                query::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    security_namespace_id: security_namespace_id.into(),
                    organization: organization.into(),
                    token: None,
                    descriptors: None,
                    include_extended_info: None,
                    recurse: None,
                }
            }
            #[doc = "Create or update one or more access control lists. All data that currently exists for the ACLs supplied will be overwritten."]
            #[doc = ""]
            #[doc = "See [`set_access_control_lists`](super::super::access_control_lists::Client::set_access_control_lists) for details of the arguments."]
            pub fn set_access_control_lists(
                &self,
                body: impl Into<models::AccessControlListBody>,
                security_namespace_id: impl Into<String>,
                organization: impl Into<String>,
            ) -> set_access_control_lists::RequestBuilder<crate::blocking::Blocking> {
                set_access_control_lists::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    body: body.into(),
                    security_namespace_id: security_namespace_id.into(),
                    organization: organization.into(),
                }
            }
            #[doc = "Remove access control lists under the specfied security namespace."]
            #[doc = ""]
            #[doc = "See [`remove_access_control_lists`](super::super::access_control_lists::Client::remove_access_control_lists) for details of the arguments."]
            pub fn remove_access_control_lists(
                &self,
                security_namespace_id: impl Into<String>,
                organization: impl Into<String>,
            ) -> remove_access_control_lists::RequestBuilder<crate::blocking::Blocking>
            {
                remove_access_control_lists::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    security_namespace_id: security_namespace_id.into(),
                    organization: organization.into(),
                    tokens: None,
                    recurse: None,
                }
            }
        }
    }
    pub mod permissions {
        use super::super::models;
        use super::super::permissions::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "Evaluates whether the caller has the specified permissions on the specified set of security tokens."]
            #[doc = ""]
            #[doc = "See [`has_permissions`](super::super::permissions::Client::has_permissions) for details of the arguments."]
            pub fn has_permissions(
                &self,
                security_namespace_id: impl Into<String>,
                organization: impl Into<String>,
                permissions: i32,
            ) -> has_permissions::RequestBuilder<crate::blocking::Blocking> {
                has_permissions::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    security_namespace_id: security_namespace_id.into(),
                    organization: organization.into(),
                    permissions,
                    tokens: None,
                    always_allow_administrators: None,
                    delimiter: None,
                }
            }
            #[doc = "Removes the specified permissions on a security token for a user or group."]
            #[doc = ""]
            #[doc = "See [`remove_permission`](super::super::permissions::Client::remove_permission) for details of the arguments."]
            pub fn remove_permission(
                &self,
                security_namespace_id: impl Into<String>,
                descriptor: impl Into<String>,
                organization: impl Into<String>,
                permissions: i32,
            ) -> remove_permission::RequestBuilder<crate::blocking::Blocking> {
                remove_permission::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    security_namespace_id: security_namespace_id.into(),
                    descriptor: descriptor.into(),
                    organization: organization.into(),
                    permissions,
                    token: None,
                }
            }
            #[doc = "Evaluates multiple permissions for the calling user.  Note: This method does not aggregate the results, nor does it short-circuit if one of the permissions evaluates to false."]
            #[doc = ""]
            #[doc = "See [`has_permissions_batch`](super::super::permissions::Client::has_permissions_batch) for details of the arguments."]
            pub fn has_permissions_batch(
                &self,
                body: impl Into<models::PermissionEvaluationBatch>,
                organization: impl Into<String>,
            ) -> has_permissions_batch::RequestBuilder<crate::blocking::Blocking> {
                has_permissions_batch::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    body: body.into(),
                    organization: organization.into(),
                }
            }
        }
    }
    pub mod security_namespaces {
        use super::super::models;
        use super::super::security_namespaces::*;
        pub struct Client(pub(crate) super::Client);
        impl Client {
            #[doc = "List all security namespaces or just the specified namespace."]
            #[doc = ""]
            #[doc = "See [`query`](super::super::security_namespaces::Client::query) for details of the arguments."]
            pub fn query(
                &self,
                organization: impl Into<String>,
                security_namespace_id: impl Into<String>,
            ) -> query::RequestBuilder<crate::blocking::Blocking> {
                query::RequestBuilder {
                    client: self.0 .0.clone(),
                    mode: std::marker::PhantomData,
                    organization: organization.into(),
                    security_namespace_id: security_namespace_id.into(),
                    local_only: None,
                }
            }
        }
    }
}
