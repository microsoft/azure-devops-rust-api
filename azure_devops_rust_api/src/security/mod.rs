// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: crate::Credential,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: crate::Credential,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://dev.azure.com";
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: crate::Credential) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
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
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Set per-call policies."]
    #[must_use]
    pub fn per_call_policies(
        mut self,
        policies: impl Into<Vec<std::sync::Arc<dyn azure_core::Policy>>>,
    ) -> Self {
        self.options = self.options.per_call_policies(policies);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![crate::ADO_SCOPE.to_string()]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &crate::Credential {
        &self.credential
    }
    #[allow(dead_code)]
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: &mut azure_core::Request,
    ) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: crate::Credential) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: crate::Credential,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
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
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
                token: None,
                descriptors: None,
            }
        }
    }
    pub mod set_access_control_entries {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AccessControlEntryList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AccessControlEntryList =
                    serde_json::from_slice(&bytes).map_err(|e| {
                        azure_core::error::Error::full(
                            azure_core::error::ErrorKind::DataConversion,
                            e,
                            format!(
                                "Failed to deserialize response:\n{}",
                                String::from_utf8_lossy(&bytes)
                            ),
                        )
                    })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) body: serde_json::Value,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/accesscontrolentries/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AccessControlEntryList>;
            type IntoFuture = futures::future::BoxFuture<
                'static,
                azure_core::Result<models::AccessControlEntryList>,
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
    pub mod remove_access_control_entries {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bool> {
                let bytes = self.0.into_body().collect().await?;
                let body: bool = serde_json::from_slice(&bytes).map_err(|e| {
                    azure_core::error::Error::full(
                        azure_core::error::ErrorKind::DataConversion,
                        e,
                        format!(
                            "Failed to deserialize response:\n{}",
                            String::from_utf8_lossy(&bytes)
                        ),
                    )
                })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) token: Option<String>,
            pub(crate) descriptors: Option<String>,
        }
        impl RequestBuilder {
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
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/accesscontrolentries/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(token) = &this.token {
                            req.url_mut().query_pairs_mut().append_pair("token", token);
                        }
                        if let Some(descriptors) = &this.descriptors {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("descriptors", descriptors);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<bool>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<bool>>;
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
pub mod access_control_lists {
    use super::models;
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
                security_namespace_id: security_namespace_id.into(),
                organization: organization.into(),
                tokens: None,
                recurse: None,
            }
        }
    }
    pub mod query {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AccessControlListList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AccessControlListList =
                    serde_json::from_slice(&bytes).map_err(|e| {
                        azure_core::error::Error::full(
                            azure_core::error::ErrorKind::DataConversion,
                            e,
                            format!(
                                "Failed to deserialize response:\n{}",
                                String::from_utf8_lossy(&bytes)
                            ),
                        )
                    })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) token: Option<String>,
            pub(crate) descriptors: Option<String>,
            pub(crate) include_extended_info: Option<bool>,
            pub(crate) recurse: Option<bool>,
        }
        impl RequestBuilder {
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
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/accesscontrollists/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AccessControlListList>;
            type IntoFuture = futures::future::BoxFuture<
                'static,
                azure_core::Result<models::AccessControlListList>,
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
    pub mod set_access_control_lists {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::AccessControlListBody,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/accesscontrollists/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<()>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<()>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
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
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bool> {
                let bytes = self.0.into_body().collect().await?;
                let body: bool = serde_json::from_slice(&bytes).map_err(|e| {
                    azure_core::error::Error::full(
                        azure_core::error::ErrorKind::DataConversion,
                        e,
                        format!(
                            "Failed to deserialize response:\n{}",
                            String::from_utf8_lossy(&bytes)
                        ),
                    )
                })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) tokens: Option<String>,
            pub(crate) recurse: Option<bool>,
        }
        impl RequestBuilder {
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
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/accesscontrollists/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<bool>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<bool>>;
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
pub mod permissions {
    use super::models;
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
                body: body.into(),
                organization: organization.into(),
            }
        }
    }
    pub mod has_permissions {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<Vec<bool>> {
                let bytes = self.0.into_body().collect().await?;
                let body: Vec<bool> = serde_json::from_slice(&bytes).map_err(|e| {
                    azure_core::error::Error::full(
                        azure_core::error::ErrorKind::DataConversion,
                        e,
                        format!(
                            "Failed to deserialize response:\n{}",
                            String::from_utf8_lossy(&bytes)
                        ),
                    )
                })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) security_namespace_id: String,
            pub(crate) organization: String,
            pub(crate) permissions: i32,
            pub(crate) tokens: Option<String>,
            pub(crate) always_allow_administrators: Option<bool>,
            pub(crate) delimiter: Option<String>,
        }
        impl RequestBuilder {
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
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/permissions/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id,
                            &this.permissions
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<Vec<bool>>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<Vec<bool>>>;
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
    pub mod remove_permission {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AccessControlEntry> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AccessControlEntry =
                    serde_json::from_slice(&bytes).map_err(|e| {
                        azure_core::error::Error::full(
                            azure_core::error::ErrorKind::DataConversion,
                            e,
                            format!(
                                "Failed to deserialize response:\n{}",
                                String::from_utf8_lossy(&bytes)
                            ),
                        )
                    })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) security_namespace_id: String,
            pub(crate) descriptor: String,
            pub(crate) organization: String,
            pub(crate) permissions: i32,
            pub(crate) token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Security token to remove permissions for."]
            pub fn token(mut self, token: impl Into<String>) -> Self {
                self.token = Some(token.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/permissions/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id,
                            &this.permissions
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let descriptor = &this.descriptor;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("descriptor", descriptor);
                        if let Some(token) = &this.token {
                            req.url_mut().query_pairs_mut().append_pair("token", token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AccessControlEntry>;
            type IntoFuture =
                futures::future::BoxFuture<'static, azure_core::Result<models::AccessControlEntry>>;
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
    pub mod has_permissions_batch {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PermissionEvaluationBatch> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PermissionEvaluationBatch = serde_json::from_slice(&bytes)
                    .map_err(|e| {
                        azure_core::error::Error::full(
                            azure_core::error::ErrorKind::DataConversion,
                            e,
                            format!(
                                "Failed to deserialize response:\n{}",
                                String::from_utf8_lossy(&bytes)
                            ),
                        )
                    })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::PermissionEvaluationBatch,
            pub(crate) organization: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/security/permissionevaluationbatch",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::PermissionEvaluationBatch>;
            type IntoFuture = futures::future::BoxFuture<
                'static,
                azure_core::Result<models::PermissionEvaluationBatch>,
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
pub mod security_namespaces {
    use super::models;
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
                organization: organization.into(),
                security_namespace_id: security_namespace_id.into(),
                local_only: None,
            }
        }
    }
    pub mod query {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::SecurityNamespaceDescriptionList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SecurityNamespaceDescriptionList = serde_json::from_slice(&bytes)
                    .map_err(|e| {
                        azure_core::error::Error::full(
                            azure_core::error::ErrorKind::DataConversion,
                            e,
                            format!(
                                "Failed to deserialize response:\n{}",
                                String::from_utf8_lossy(&bytes)
                            ),
                        )
                    })?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) security_namespace_id: String,
            pub(crate) local_only: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "If true, retrieve only local security namespaces."]
            pub fn local_only(mut self, local_only: bool) -> Self {
                self.local_only = Some(local_only);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/securitynamespaces/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.security_namespace_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(local_only) = &this.local_only {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("localOnly", &local_only.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SecurityNamespaceDescriptionList>;
            type IntoFuture = futures::future::BoxFuture<
                'static,
                azure_core::Result<models::SecurityNamespaceDescriptionList>,
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
