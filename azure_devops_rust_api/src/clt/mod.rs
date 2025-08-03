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
pub const DEFAULT_ENDPOINT: &str = "https://vsclt.dev.azure.com";
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
        request: &mut azure_core::http::Request,
    ) -> azure_core::Result<azure_core::http::RawResponse> {
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
    pub fn agent_groups_client(&self) -> agent_groups::Client {
        agent_groups::Client(self.clone())
    }
    pub fn agents_client(&self) -> agents::Client {
        agents::Client(self.clone())
    }
    pub fn applications_client(&self) -> applications::Client {
        applications::Client(self.clone())
    }
    pub fn counter_instances_client(&self) -> counter_instances::Client {
        counter_instances::Client(self.clone())
    }
    pub fn counter_samples_client(&self) -> counter_samples::Client {
        counter_samples::Client(self.clone())
    }
    pub fn counters_client(&self) -> counters::Client {
        counters::Client(self.clone())
    }
    pub fn errors_client(&self) -> errors::Client {
        errors::Client(self.clone())
    }
    pub fn messages_client(&self) -> messages::Client {
        messages::Client(self.clone())
    }
    pub fn plugins_client(&self) -> plugins::Client {
        plugins::Client(self.clone())
    }
    pub fn results_client(&self) -> results::Client {
        results::Client(self.clone())
    }
    pub fn test_definitions_client(&self) -> test_definitions::Client {
        test_definitions::Client(self.clone())
    }
    pub fn test_drops_client(&self) -> test_drops::Client {
        test_drops::Client(self.clone())
    }
    pub fn test_runs_client(&self) -> test_runs::Client {
        test_runs::Client(self.clone())
    }
}
pub mod agent_groups {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Agent group to be created"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::AgentGroup>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `agent_group_id`: The agent group identifier"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            agent_group_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                agent_group_id: agent_group_id.into(),
                machine_setup_input: None,
                machine_access_data: None,
                outgoing_request_urls: None,
                agent_group_name: None,
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::AgentGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::AgentGroup,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/agentgroups",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentGroup =
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
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) agent_group_id: String,
            pub(crate) machine_setup_input: Option<bool>,
            pub(crate) machine_access_data: Option<bool>,
            pub(crate) outgoing_request_urls: Option<bool>,
            pub(crate) agent_group_name: Option<String>,
        }
        impl Builder {
            pub fn machine_setup_input(mut self, machine_setup_input: bool) -> Self {
                self.machine_setup_input = Some(machine_setup_input);
                self
            }
            pub fn machine_access_data(mut self, machine_access_data: bool) -> Self {
                self.machine_access_data = Some(machine_access_data);
                self
            }
            pub fn outgoing_request_urls(mut self, outgoing_request_urls: bool) -> Self {
                self.outgoing_request_urls = Some(outgoing_request_urls);
                self
            }
            #[doc = "Name of the agent group"]
            pub fn agent_group_name(mut self, agent_group_name: impl Into<String>) -> Self {
                self.agent_group_name = Some(agent_group_name.into());
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/agentgroups/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.agent_group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        if let Some(machine_setup_input) = &this.machine_setup_input {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("machineSetupInput", &machine_setup_input.to_string());
                        }
                        if let Some(machine_access_data) = &this.machine_access_data {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("machineAccessData", &machine_access_data.to_string());
                        }
                        if let Some(outgoing_request_urls) = &this.outgoing_request_urls {
                            req.url_mut().query_pairs_mut().append_pair(
                                "outgoingRequestUrls",
                                &outgoing_request_urls.to_string(),
                            );
                        }
                        if let Some(agent_group_name) = &this.agent_group_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("agentGroupName", agent_group_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value =
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
pub mod agents {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `agent_group_id`: The agent group identifier"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            agent_group_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                agent_group_id: agent_group_id.into(),
                agent_name: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `agent_group_id`: The agent group identifier"]
        #[doc = "* `agent_name`: Name of the static agent"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            agent_group_id: impl Into<String>,
            agent_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                agent_group_id: agent_group_id.into(),
                agent_name: agent_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) agent_group_id: String,
            pub(crate) agent_name: Option<String>,
        }
        impl Builder {
            #[doc = "Name of the static agent"]
            pub fn agent_name(mut self, agent_name: impl Into<String>) -> Self {
                self.agent_name = Some(agent_name.into());
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/agentGroups/{}/agents",
                            this.client.endpoint(),
                            &this.organization,
                            &this.agent_group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        if let Some(agent_name) = &this.agent_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("agentName", agent_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value =
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
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) agent_group_id: String,
            pub(crate) agent_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/agentGroups/{}/agents",
                            this.client.endpoint(),
                            &this.organization,
                            &this.agent_group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let agent_name = &this.agent_name;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("agentName", agent_name);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String =
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
pub mod applications {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                type_: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `application_id`: Filter by APM application identifier."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            application_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                application_id: application_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::ApplicationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) type_: Option<String>,
        }
        impl Builder {
            #[doc = "Filters the results based on the plugin type."]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/apm/applications",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ApplicationList =
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
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::Application;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) application_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/apm/applications/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.application_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Application =
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
pub mod counters {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                application_id: None,
                plugintype: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::ApplicationCountersList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) application_id: Option<String>,
            pub(crate) plugintype: Option<String>,
        }
        impl Builder {
            #[doc = "Filter by APM application identifier."]
            pub fn application_id(mut self, application_id: impl Into<String>) -> Self {
                self.application_id = Some(application_id.into());
                self
            }
            #[doc = "Currently ApplicationInsights is the only available plugin type."]
            pub fn plugintype(mut self, plugintype: impl Into<String>) -> Self {
                self.plugintype = Some(plugintype.into());
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/apm/counters",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        if let Some(application_id) = &this.application_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("applicationId", application_id);
                        }
                        if let Some(plugintype) = &this.plugintype {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("plugintype", plugintype);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ApplicationCountersList =
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
pub mod plugins {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `type_`: Currently ApplicationInsights is the only available plugin type."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            type_: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                type_: type_.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::ApplicationTypeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/apm/plugins",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ApplicationTypeList =
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
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::ApplicationType;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) type_: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/apm/plugins/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.type_
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ApplicationType =
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
pub mod test_definitions {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                from_date: None,
                to_date: None,
                top: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Test definition to be created"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestDefinition>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestDefinition>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_definition_id`: The test definition identifier"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            test_definition_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_definition_id: test_definition_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestDefinitionBasicList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) from_date: Option<String>,
            pub(crate) to_date: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Date after which test definitions were created"]
            pub fn from_date(mut self, from_date: impl Into<String>) -> Self {
                self.from_date = Some(from_date.into());
                self
            }
            #[doc = "Date before which test definitions were crated"]
            pub fn to_date(mut self, to_date: impl Into<String>) -> Self {
                self.to_date = Some(to_date.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testdefinitions",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        if let Some(from_date) = &this.from_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fromDate", from_date);
                        }
                        if let Some(to_date) = &this.to_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("toDate", to_date);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("top", &top.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestDefinitionBasicList =
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
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestDefinition,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testdefinitions",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestDefinition =
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
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestDefinition,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testdefinitions",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestDefinition =
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
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_definition_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testdefinitions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_definition_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestDefinition =
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
pub mod test_drops {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Test drop to be created"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestDrop>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_drop_id`: The test drop identifier"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            test_drop_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_drop_id: test_drop_id.into(),
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestDrop;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestDrop,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testdrops",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestDrop = serde_json::from_slice(&rsp_body)
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
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestDrop;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_drop_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testdrops/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_drop_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestDrop = serde_json::from_slice(&rsp_body)
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
pub mod test_runs {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns test runs based on the filter specified. Returns all runs of the tenant if there is no filter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_test_runs(&self, organization: impl Into<String>) -> get_test_runs::Builder {
            get_test_runs::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                name: None,
                requested_by: None,
                status: None,
                run_type: None,
                from_date: None,
                to_date: None,
                detailed: None,
                top: None,
                runsourceidentifier: None,
                retention_state: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestRun>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_run_id`: Unique ID of the test run"]
        pub fn get_test_run(
            &self,
            organization: impl Into<String>,
            test_run_id: impl Into<String>,
        ) -> get_test_run::Builder {
            get_test_run::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_run_id: test_run_id.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestRun>,
            test_run_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                test_run_id: test_run_id.into(),
            }
        }
    }
    pub mod get_test_runs {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) name: Option<String>,
            pub(crate) requested_by: Option<String>,
            pub(crate) status: Option<String>,
            pub(crate) run_type: Option<String>,
            pub(crate) from_date: Option<String>,
            pub(crate) to_date: Option<String>,
            pub(crate) detailed: Option<bool>,
            pub(crate) top: Option<i32>,
            pub(crate) runsourceidentifier: Option<String>,
            pub(crate) retention_state: Option<String>,
        }
        impl Builder {
            #[doc = "Name for the test run. Names are not unique. Test runs with same name are assigned sequential rolling numbers."]
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            #[doc = "Filter by the user who requested the test run. Here requestedBy should be the display name of the user."]
            pub fn requested_by(mut self, requested_by: impl Into<String>) -> Self {
                self.requested_by = Some(requested_by.into());
                self
            }
            #[doc = "Filter by the test run status."]
            pub fn status(mut self, status: impl Into<String>) -> Self {
                self.status = Some(status.into());
                self
            }
            #[doc = "Valid values include: null, one of TestRunType, or \"*\""]
            pub fn run_type(mut self, run_type: impl Into<String>) -> Self {
                self.run_type = Some(run_type.into());
                self
            }
            #[doc = "Filter by the test runs that have been modified after the fromDate timestamp."]
            pub fn from_date(mut self, from_date: impl Into<String>) -> Self {
                self.from_date = Some(from_date.into());
                self
            }
            #[doc = "Filter by the test runs that have been modified before the toDate timestamp."]
            pub fn to_date(mut self, to_date: impl Into<String>) -> Self {
                self.to_date = Some(to_date.into());
                self
            }
            #[doc = "Include the detailed test run attributes."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            #[doc = "The maximum number of test runs to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn runsourceidentifier(mut self, runsourceidentifier: impl Into<String>) -> Self {
                self.runsourceidentifier = Some(runsourceidentifier.into());
                self
            }
            pub fn retention_state(mut self, retention_state: impl Into<String>) -> Self {
                self.retention_state = Some(retention_state.into());
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testruns",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        if let Some(name) = &this.name {
                            req.url_mut().query_pairs_mut().append_pair("name", name);
                        }
                        if let Some(requested_by) = &this.requested_by {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestedBy", requested_by);
                        }
                        if let Some(status) = &this.status {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("status", status);
                        }
                        if let Some(run_type) = &this.run_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("runType", run_type);
                        }
                        if let Some(from_date) = &this.from_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fromDate", from_date);
                        }
                        if let Some(to_date) = &this.to_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("toDate", to_date);
                        }
                        if let Some(detailed) = &this.detailed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("detailed", &detailed.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("top", &top.to_string());
                        }
                        if let Some(runsourceidentifier) = &this.runsourceidentifier {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("runsourceidentifier", runsourceidentifier);
                        }
                        if let Some(retention_state) = &this.retention_state {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("retentionState", retention_state);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value =
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
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestRun;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestRun,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testruns",
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRun = serde_json::from_slice(&rsp_body)
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
    pub mod get_test_run {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestRun;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testruns/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRun = serde_json::from_slice(&rsp_body)
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
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestRun,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testruns/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
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
}
pub mod counter_instances {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_run_id`: The test run identifier"]
        #[doc = "* `group_names`: Comma separated names of counter groups, such as 'Application', 'Performance' and 'Throughput'"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            test_run_id: impl Into<String>,
            group_names: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_run_id: test_run_id.into(),
                group_names: group_names.into(),
                include_summary: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestRunCounterInstanceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_run_id: String,
            pub(crate) group_names: String,
            pub(crate) include_summary: Option<bool>,
        }
        impl Builder {
            pub fn include_summary(mut self, include_summary: bool) -> Self {
                self.include_summary = Some(include_summary);
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testRuns/{}/counterinstances",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let group_names = &this.group_names;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("groupNames", group_names);
                        if let Some(include_summary) = &this.include_summary {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeSummary", &include_summary.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRunCounterInstanceList =
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
pub mod counter_samples {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_run_id`: The test run identifier"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::VssJsonCollectionWrapper>,
            test_run_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                test_run_id: test_run_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::CounterSamplesResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::VssJsonCollectionWrapper,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testRuns/{}/countersamples",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CounterSamplesResult =
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
pub mod errors {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_run_id`: The test run identifier"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            test_run_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_run_id: test_run_id.into(),
                type_: None,
                sub_type: None,
                detailed: None,
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::LoadTestErrors;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_run_id: String,
            pub(crate) type_: Option<String>,
            pub(crate) sub_type: Option<String>,
            pub(crate) detailed: Option<bool>,
        }
        impl Builder {
            #[doc = "Filter for the particular type of errors."]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            #[doc = "Filter for a particular subtype of errors. You should not provide error subtype without error type."]
            pub fn sub_type(mut self, sub_type: impl Into<String>) -> Self {
                self.sub_type = Some(sub_type.into());
                self
            }
            #[doc = "To include the details of test errors such as messagetext, request, stacktrace, testcasename, scenarioname, and lasterrordate."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testRuns/{}/errors",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        if let Some(sub_type) = &this.sub_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("subType", sub_type);
                        }
                        if let Some(detailed) = &this.detailed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("detailed", &detailed.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LoadTestErrors =
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
pub mod messages {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_run_id`: Id of the test run"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            test_run_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_run_id: test_run_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestRunMessageList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testRuns/{}/messages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRunMessageList =
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
pub mod results {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_run_id`: The test run identifier"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            test_run_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_run_id: test_run_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        type Response = models::TestResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/clt/testRuns/{}/results",
                            this.client.endpoint(),
                            &this.organization,
                            &this.test_run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "6.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestResults =
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
