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
pub const DEFAULT_ENDPOINT: &str = "https://artifacts.dev.azure.com";
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
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
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
    pub fn availability_client(&self) -> availability::Client {
        availability::Client(self.clone())
    }
    pub fn client_client(&self) -> client::Client {
        client::Client(self.clone())
    }
    pub fn contents_client(&self) -> contents::Client {
        contents::Client(self.clone())
    }
    pub fn requests_client(&self) -> requests::Client {
        requests::Client(self.clone())
    }
    pub fn symsrv_client(&self) -> symsrv::Client {
        symsrv::Client(self.clone())
    }
}
pub mod requests {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create debug entries for a symbol request as specified by its name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A batch that contains debug entries to create."]
        #[doc = "* `request_name`: The symbol request name."]
        #[doc = "* `collection`: A valid debug entry collection name. Must be \"debugentries\"."]
        pub fn create_requests_request_name_debug_entries(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::DebugEntryCreateBatch>,
            request_name: impl Into<String>,
            collection: impl Into<String>,
        ) -> create_requests_request_name_debug_entries::RequestBuilder {
            create_requests_request_name_debug_entries::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                request_name: request_name.into(),
                collection: collection.into(),
            }
        }
        #[doc = "Get a symbol request by request name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `request_name`: The symbol request name."]
        pub fn get_requests_request_name(
            &self,
            organization: impl Into<String>,
            request_name: impl Into<String>,
        ) -> get_requests_request_name::RequestBuilder {
            get_requests_request_name::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                request_name: request_name.into(),
            }
        }
        #[doc = "Create a new symbol request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The symbol request to create."]
        pub fn create_requests(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Request>,
        ) -> create_requests::RequestBuilder {
            create_requests::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Update a symbol request by request name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The symbol request."]
        #[doc = "* `request_name`: The symbol request name."]
        pub fn update_requests_request_name(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Request>,
            request_name: impl Into<String>,
        ) -> update_requests_request_name::RequestBuilder {
            update_requests_request_name::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                request_name: request_name.into(),
            }
        }
        #[doc = "Delete a symbol request by request name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `request_name`: The symbol request name."]
        pub fn delete_requests_request_name(
            &self,
            organization: impl Into<String>,
            request_name: impl Into<String>,
        ) -> delete_requests_request_name::RequestBuilder {
            delete_requests_request_name::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                request_name: request_name.into(),
                synchronous: None,
            }
        }
        #[doc = "Get a symbol request by request identifier."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `request_id`: The symbol request identifier."]
        pub fn get_requests_request_id(
            &self,
            organization: impl Into<String>,
            request_id: impl Into<String>,
        ) -> get_requests_request_id::RequestBuilder {
            get_requests_request_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                request_id: request_id.into(),
            }
        }
        #[doc = "Create debug entries for a symbol request as specified by its identifier."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A batch that contains debug entries to create."]
        #[doc = "* `request_id`: The symbol request identifier."]
        #[doc = "* `collection`: A valid debug entry collection name. Must be \"debugentries\"."]
        pub fn create_requests_request_id_debug_entries(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::DebugEntryCreateBatch>,
            request_id: impl Into<String>,
            collection: impl Into<String>,
        ) -> create_requests_request_id_debug_entries::RequestBuilder {
            create_requests_request_id_debug_entries::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                request_id: request_id.into(),
                collection: collection.into(),
            }
        }
        #[doc = "Update a symbol request by request identifier."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The symbol request."]
        #[doc = "* `request_id`: The symbol request identifier."]
        pub fn update_requests_request_id(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Request>,
            request_id: impl Into<String>,
        ) -> update_requests_request_id::RequestBuilder {
            update_requests_request_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                request_id: request_id.into(),
            }
        }
        #[doc = "Delete a symbol request by request identifier."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `request_id`: The symbol request identifier."]
        pub fn delete_requests_request_id(
            &self,
            organization: impl Into<String>,
            request_id: impl Into<String>,
        ) -> delete_requests_request_id::RequestBuilder {
            delete_requests_request_id::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                request_id: request_id.into(),
                synchronous: None,
            }
        }
    }
    pub mod create_requests_request_name_debug_entries {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DebugEntryList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DebugEntryList = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::DebugEntryCreateBatch,
            pub(crate) request_name: String,
            pub(crate) collection: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests?requestName={}&collection={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.request_name,
                            &this.collection
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
                        let request_name = &this.request_name;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("requestName", request_name);
                        let collection = &this.collection;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("collection", collection);
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::DebugEntryList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_requests_request_name {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Request> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Request = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) request_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests",
                            this.client.endpoint(),
                            &this.organization
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
                        let request_name = &this.request_name;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("requestName", request_name);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Request>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_requests {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Request> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Request = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Request,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests",
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
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Request>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update_requests_request_name {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Request> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Request = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Request,
            pub(crate) request_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                        let request_name = &this.request_name;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("requestName", request_name);
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Request>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_requests_request_name {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) request_name: String,
            pub(crate) synchronous: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "If true, delete all the debug entries under this request synchronously in the current session. If false, the deletion will be postponed to a later point and be executed automatically by the system."]
            pub fn synchronous(mut self, synchronous: bool) -> Self {
                self.synchronous = Some(synchronous);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests",
                            this.client.endpoint(),
                            &this.organization
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
                        let request_name = &this.request_name;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("requestName", request_name);
                        if let Some(synchronous) = &this.synchronous {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("synchronous", &synchronous.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_requests_request_id {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Request> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Request = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) request_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.request_id
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Request>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_requests_request_id_debug_entries {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DebugEntryList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DebugEntryList = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::DebugEntryCreateBatch,
            pub(crate) request_id: String,
            pub(crate) collection: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.request_id
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
                        let collection = &this.collection;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("collection", collection);
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::DebugEntryList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update_requests_request_id {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Request> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Request = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Request,
            pub(crate) request_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.request_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Request>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_requests_request_id {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) request_id: String,
            pub(crate) synchronous: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "If true, delete all the debug entries under this request synchronously in the current session. If false, the deletion will be postponed to a later point and be executed automatically by the system."]
            pub fn synchronous(mut self, synchronous: bool) -> Self {
                self.synchronous = Some(synchronous);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.request_id
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
                        if let Some(synchronous) = &this.synchronous {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("synchronous", &synchronous.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod availability {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Check the availability of symbol service. This includes checking for feature flag, and possibly license in future. Note this is NOT an anonymous endpoint, and the caller will be redirected to authentication before hitting it."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn check_availability(
            &self,
            organization: impl Into<String>,
        ) -> check_availability::RequestBuilder {
            check_availability::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
    }
    pub mod check_availability {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/availability",
                            this.client.endpoint(),
                            &this.organization
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod client {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get client version information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn head_client(&self, organization: impl Into<String>) -> head_client::RequestBuilder {
            head_client::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Get the client package."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `client_type`: Either \"EXE\" for a zip file containing a Windows symbol client (a.k.a. symbol.exe) along with dependencies, or \"TASK\" for a VSTS task that can be run on a VSTS build agent. All the other values are invalid. The parameter is case-insensitive."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            client_type: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                client_type: client_type.into(),
            }
        }
    }
    pub mod head_client {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/client",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<String> {
                let bytes = self.0.into_body().collect().await?;
                let body: String = serde_json::from_slice(&bytes).map_err(|e| {
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) client_type: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/client/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.client_type
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<String>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod contents {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a stitched debug entry for a symbol request as specified by symbol request identifier and debug entry identifier."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `request_id`: The symbol request identifier."]
        #[doc = "* `debug_entry_id`: The debug entry identifier."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            request_id: impl Into<String>,
            debug_entry_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                request_id: request_id.into(),
                debug_entry_id: debug_entry_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) request_id: String,
            pub(crate) debug_entry_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/requests/{}/contents/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.request_id,
                            &this.debug_entry_id
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod symsrv {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Given a client key, returns the best matched debug entry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `debug_entry_client_key`: A \"client key\" used by both ends of Microsoft's symbol protocol to identify a debug entry. The semantics of client key is governed by symsrv and is beyond the scope of this documentation."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            debug_entry_client_key: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                debug_entry_client_key: debug_entry_client_key.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) debug_entry_client_key: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/symbol/symsrv/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.debug_entry_client_key
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
