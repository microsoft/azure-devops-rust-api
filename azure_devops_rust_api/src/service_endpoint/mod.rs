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
        ) -> execute_service_endpoint_request::Builder {
            execute_service_endpoint_request::Builder {
                client: self.0.clone(),
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
        ) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod execute_service_endpoint_request {
        use super::models;
        type Response = models::ServiceEndpointRequestResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ServiceEndpointRequest,
            pub(crate) project: String,
            pub(crate) endpoint_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/serviceendpoint/endpointproxy?endpointId={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.endpoint_id
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
                        let endpoint_id = &this.endpoint_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("endpointId", endpoint_id);
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpointRequestResult =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod query {
        use super::models;
        type Response = Vec<String>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::DataSourceBinding,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/serviceendpoint/endpointproxy",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<String> = serde_json::from_slice(&rsp_body)
                                    .map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod endpoints {
    use super::models;
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
        ) -> get_service_endpoints::Builder {
            get_service_endpoints::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                type_: None,
                auth_schemes: None,
                endpoint_ids: None,
                owner: None,
                include_failed: None,
                include_details: None,
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
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
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
        ) -> update_service_endpoints::Builder {
            update_service_endpoints::Builder {
                client: self.0.clone(),
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
        ) -> update_service_endpoint::Builder {
            update_service_endpoint::Builder {
                client: self.0.clone(),
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
        ) -> share_service_endpoint::Builder {
            share_service_endpoint::Builder {
                client: self.0.clone(),
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
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
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
        ) -> get_service_endpoints_by_names::Builder {
            get_service_endpoints_by_names::Builder {
                client: self.0.clone(),
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
        ) -> get_service_endpoints_with_refreshed_authentication::Builder {
            get_service_endpoints_with_refreshed_authentication::Builder {
                client: self.0.clone(),
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
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                endpoint_id: endpoint_id.into(),
                action_filter: None,
            }
        }
    }
    pub mod get_service_endpoints {
        use super::models;
        type Response = models::ServiceEndpointList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) type_: Option<String>,
            pub(crate) auth_schemes: Option<String>,
            pub(crate) endpoint_ids: Option<String>,
            pub(crate) owner: Option<String>,
            pub(crate) include_failed: Option<bool>,
            pub(crate) include_details: Option<bool>,
        }
        impl Builder {
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
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/serviceendpoint/endpoints?",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpointList =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::ServiceEndpoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ServiceEndpoint,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/serviceendpoint/endpoints",
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
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpoint =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_service_endpoints {
        use super::models;
        type Response = models::ServiceEndpointList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::ServiceEndpoint>,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/serviceendpoint/endpoints",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
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
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpointList =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_service_endpoint {
        use super::models;
        type Response = models::ServiceEndpoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ServiceEndpoint,
            pub(crate) endpoint_id: String,
            pub(crate) operation: Option<String>,
        }
        impl Builder {
            #[doc = "operation type"]
            pub fn operation(mut self, operation: impl Into<String>) -> Self {
                self.operation = Some(operation.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/serviceendpoint/endpoints/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.endpoint_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
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
                        if let Some(operation) = &this.operation {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("operation", operation);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpoint =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod share_service_endpoint {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::ServiceEndpointProjectReference>,
            pub(crate) endpoint_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/serviceendpoint/endpoints/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.endpoint_id
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
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) endpoint_id: String,
            pub(crate) project_ids: String,
            pub(crate) deep: Option<bool>,
        }
        impl Builder {
            #[doc = "delete the spn created by endpoint"]
            pub fn deep(mut self, deep: bool) -> Self {
                self.deep = Some(deep);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/serviceendpoint/endpoints/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.endpoint_id
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
                        let project_ids = &this.project_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("projectIds", project_ids);
                        if let Some(deep) = &this.deep {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deep", &deep.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_service_endpoints_by_names {
        use super::models;
        type Response = models::ServiceEndpointList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) endpoint_names: String,
            pub(crate) type_: Option<String>,
            pub(crate) auth_schemes: Option<String>,
            pub(crate) owner: Option<String>,
            pub(crate) include_failed: Option<bool>,
            pub(crate) include_details: Option<bool>,
        }
        impl Builder {
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
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/serviceendpoint/endpoints",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpointList =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_service_endpoints_with_refreshed_authentication {
        use super::models;
        type Response = models::ServiceEndpointList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::RefreshAuthenticationParameters>,
            pub(crate) project: String,
            pub(crate) endpoint_ids: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/serviceendpoint/endpoints",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        let endpoint_ids = &this.endpoint_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("endpointIds", endpoint_ids);
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpointList =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ServiceEndpoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) endpoint_id: String,
            pub(crate) action_filter: Option<String>,
        }
        impl Builder {
            #[doc = "Action filter for the service connection. It specifies the action which can be performed on the service connection."]
            pub fn action_filter(mut self, action_filter: impl Into<String>) -> Self {
                self.action_filter = Some(action_filter.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/serviceendpoint/endpoints/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.endpoint_id
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
                        if let Some(action_filter) = &this.action_filter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("actionFilter", action_filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpoint =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get service endpoint types."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                type_: None,
                scheme: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ServiceEndpointTypeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) type_: Option<String>,
            pub(crate) scheme: Option<String>,
        }
        impl Builder {
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
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/serviceendpoint/types",
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
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        if let Some(scheme) = &this.scheme {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("scheme", scheme);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpointTypeList =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod executionhistory {
    use super::models;
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
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
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
        type Response = models::ServiceEndpointExecutionRecordList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) endpoint_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<i64>,
        }
        impl Builder {
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
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/serviceendpoint/{}/executionhistory",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.endpoint_id
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceEndpointExecutionRecordList =
                                    serde_json::from_slice(&rsp_body).map_err(|e| {
                                        azure_core::error::Error::full(
                                            azure_core::error::ErrorKind::DataConversion,
                                            e,
                                            format!(
                                                "Failed to deserialize response:\n{}",
                                                String::from_utf8_lossy(&rsp_body)
                                            ),
                                        )
                                    })?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
