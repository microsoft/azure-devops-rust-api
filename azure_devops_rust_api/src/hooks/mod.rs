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
    pub fn consumers_client(&self) -> consumers::Client {
        consumers::Client(self.clone())
    }
    pub fn diagnostics_client(&self) -> diagnostics::Client {
        diagnostics::Client(self.clone())
    }
    pub fn notifications_client(&self) -> notifications::Client {
        notifications::Client(self.clone())
    }
    pub fn publishers_client(&self) -> publishers::Client {
        publishers::Client(self.clone())
    }
    pub fn subscriptions_client(&self) -> subscriptions::Client {
        subscriptions::Client(self.clone())
    }
}
pub mod consumers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of available service hook consumer services. Optionally filter by consumers that support at least one event type from the specific publisher."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_id: None,
            }
        }
        #[doc = "Get a specific consumer service. Optionally filter out consumer actions that do not support any event types for the specified publisher."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `consumer_id`: ID for a consumer."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            consumer_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                consumer_id: consumer_id.into(),
                publisher_id: None,
            }
        }
        #[doc = "Get a list of consumer actions for a specific consumer."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `consumer_id`: ID for a consumer."]
        pub fn list_consumer_actions(
            &self,
            organization: impl Into<String>,
            consumer_id: impl Into<String>,
        ) -> list_consumer_actions::Builder {
            list_consumer_actions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                consumer_id: consumer_id.into(),
                publisher_id: None,
            }
        }
        #[doc = "Get details about a specific consumer action."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `consumer_id`: ID for a consumer."]
        #[doc = "* `consumer_action_id`: ID for a consumerActionId."]
        pub fn get_consumer_action(
            &self,
            organization: impl Into<String>,
            consumer_id: impl Into<String>,
            consumer_action_id: impl Into<String>,
        ) -> get_consumer_action::Builder {
            get_consumer_action::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                consumer_id: consumer_id.into(),
                consumer_action_id: consumer_action_id.into(),
                publisher_id: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ConsumerList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_id: Option<String>,
        }
        impl Builder {
            pub fn publisher_id(mut self, publisher_id: impl Into<String>) -> Self {
                self.publisher_id = Some(publisher_id.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/consumers",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(publisher_id) = &this.publisher_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherId", publisher_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConsumerList =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Consumer;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) consumer_id: String,
            pub(crate) publisher_id: Option<String>,
        }
        impl Builder {
            pub fn publisher_id(mut self, publisher_id: impl Into<String>) -> Self {
                self.publisher_id = Some(publisher_id.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/consumers/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.consumer_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(publisher_id) = &this.publisher_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherId", publisher_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Consumer =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod list_consumer_actions {
        use super::models;
        type Response = models::ConsumerActionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) consumer_id: String,
            pub(crate) publisher_id: Option<String>,
        }
        impl Builder {
            pub fn publisher_id(mut self, publisher_id: impl Into<String>) -> Self {
                self.publisher_id = Some(publisher_id.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/consumers/{}/actions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.consumer_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(publisher_id) = &this.publisher_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherId", publisher_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConsumerActionList =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod get_consumer_action {
        use super::models;
        type Response = models::ConsumerAction;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) consumer_id: String,
            pub(crate) consumer_action_id: String,
            pub(crate) publisher_id: Option<String>,
        }
        impl Builder {
            pub fn publisher_id(mut self, publisher_id: impl Into<String>) -> Self {
                self.publisher_id = Some(publisher_id.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/consumers/{}/actions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.consumer_id,
                            &this.consumer_action_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(publisher_id) = &this.publisher_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherId", publisher_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConsumerAction =
                                    serde_json::from_slice(&rsp_body)?;
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
pub mod notifications {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Query for notifications. A notification includes details about the event, the request to and the response from the consumer service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn query(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NotificationsQuery>,
        ) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Get a list of notifications for a specific subscription. A notification includes details about the event, the request to and the response from the consumer service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subscription_id`: ID for a subscription."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subscription_id: subscription_id.into(),
                max_results: None,
                status: None,
                result: None,
            }
        }
        #[doc = "Get a specific notification for a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subscription_id`: ID for a subscription."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            subscription_id: impl Into<String>,
            notification_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subscription_id: subscription_id.into(),
                notification_id,
            }
        }
        #[doc = "Sends a test notification. This is useful for verifying the configuration of an updated or new service hooks subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Notification>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                use_real_data: None,
            }
        }
    }
    pub mod query {
        use super::models;
        type Response = models::NotificationsQuery;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NotificationsQuery,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/notificationsquery",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
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
                                let rsp_value: models::NotificationsQuery =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::NotificationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subscription_id: String,
            pub(crate) max_results: Option<i32>,
            pub(crate) status: Option<String>,
            pub(crate) result: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of notifications to return. Default is **100**."]
            pub fn max_results(mut self, max_results: i32) -> Self {
                self.max_results = Some(max_results);
                self
            }
            #[doc = "Get only notifications with this status."]
            pub fn status(mut self, status: impl Into<String>) -> Self {
                self.status = Some(status.into());
                self
            }
            #[doc = "Get only notifications with this result type."]
            pub fn result(mut self, result: impl Into<String>) -> Self {
                self.result = Some(result.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions/{}/notifications",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(max_results) = &this.max_results {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("maxResults", &max_results.to_string());
                        }
                        if let Some(status) = &this.status {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("status", status);
                        }
                        if let Some(result) = &this.result {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("result", result);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NotificationList =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Notification;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subscription_id: String,
            pub(crate) notification_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions/{}/notifications/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subscription_id,
                            &this.notification_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Notification =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Notification;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Notification,
            pub(crate) use_real_data: Option<bool>,
        }
        impl Builder {
            #[doc = "Only allow testing with real data in existing subscriptions."]
            pub fn use_real_data(mut self, use_real_data: bool) -> Self {
                self.use_real_data = Some(use_real_data);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/testnotifications",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(use_real_data) = &this.use_real_data {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("useRealData", &use_real_data.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Notification =
                                    serde_json::from_slice(&rsp_body)?;
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
pub mod publishers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of publishers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Get a specific service hooks publisher."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `publisher_id`: ID for a publisher."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            publisher_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_id: publisher_id.into(),
            }
        }
        #[doc = "Get the event types for a specific publisher."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `publisher_id`: ID for a publisher."]
        pub fn list_event_types(
            &self,
            organization: impl Into<String>,
            publisher_id: impl Into<String>,
        ) -> list_event_types::Builder {
            list_event_types::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_id: publisher_id.into(),
            }
        }
        #[doc = "Get a specific event type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `publisher_id`: ID for a publisher."]
        pub fn get_event_type(
            &self,
            organization: impl Into<String>,
            publisher_id: impl Into<String>,
            event_type_id: impl Into<String>,
        ) -> get_event_type::Builder {
            get_event_type::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_id: publisher_id.into(),
                event_type_id: event_type_id.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn query_input_values(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::InputValuesQuery>,
            publisher_id: impl Into<String>,
        ) -> query_input_values::Builder {
            query_input_values::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                publisher_id: publisher_id.into(),
            }
        }
        #[doc = "Query for service hook publishers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn query_publishers(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PublishersQuery>,
        ) -> query_publishers::Builder {
            query_publishers::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PublisherList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/publishers",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PublisherList =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Publisher;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/publishers/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.publisher_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Publisher =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod list_event_types {
        use super::models;
        type Response = models::EventTypeDescriptorList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/publishers/{}/eventtypes",
                            this.client.endpoint(),
                            &this.organization,
                            &this.publisher_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventTypeDescriptorList =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod get_event_type {
        use super::models;
        type Response = models::EventTypeDescriptor;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_id: String,
            pub(crate) event_type_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/publishers/{}/eventtypes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.publisher_id,
                            &this.event_type_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventTypeDescriptor =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod query_input_values {
        use super::models;
        type Response = models::InputValuesQuery;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::InputValuesQuery,
            pub(crate) publisher_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/publishers/{}/inputValuesQuery",
                            this.client.endpoint(),
                            &this.organization,
                            &this.publisher_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
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
                                let rsp_value: models::InputValuesQuery =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod query_publishers {
        use super::models;
        type Response = models::PublishersQuery;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PublishersQuery,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/publishersquery",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
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
                                let rsp_value: models::PublishersQuery =
                                    serde_json::from_slice(&rsp_body)?;
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
pub mod subscriptions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of subscriptions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_id: None,
                event_type: None,
                consumer_id: None,
                consumer_action_id: None,
            }
        }
        #[doc = "Create a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Subscription to be created."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Subscription>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Get a specific service hooks subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subscription_id`: ID for a subscription."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Update a subscription. <param name=\"subscriptionId\">ID for a subscription that you wish to update.</param>"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn replace_subscription(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Subscription>,
            subscription_id: impl Into<String>,
        ) -> replace_subscription::Builder {
            replace_subscription::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Delete a specific service hooks subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subscription_id`: ID for a subscription."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Query for service hook subscriptions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn create_subscriptions_query(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::SubscriptionsQuery>,
        ) -> create_subscriptions_query::Builder {
            create_subscriptions_query::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SubscriptionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_id: Option<String>,
            pub(crate) event_type: Option<String>,
            pub(crate) consumer_id: Option<String>,
            pub(crate) consumer_action_id: Option<String>,
        }
        impl Builder {
            #[doc = "ID for a subscription."]
            pub fn publisher_id(mut self, publisher_id: impl Into<String>) -> Self {
                self.publisher_id = Some(publisher_id.into());
                self
            }
            #[doc = "The event type to filter on (if any)."]
            pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
                self.event_type = Some(event_type.into());
                self
            }
            #[doc = "ID for a consumer."]
            pub fn consumer_id(mut self, consumer_id: impl Into<String>) -> Self {
                self.consumer_id = Some(consumer_id.into());
                self
            }
            #[doc = "ID for a consumerActionId."]
            pub fn consumer_action_id(mut self, consumer_action_id: impl Into<String>) -> Self {
                self.consumer_action_id = Some(consumer_action_id.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(publisher_id) = &this.publisher_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherId", publisher_id);
                        }
                        if let Some(event_type) = &this.event_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("eventType", event_type);
                        }
                        if let Some(consumer_id) = &this.consumer_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("consumerId", consumer_id);
                        }
                        if let Some(consumer_action_id) = &this.consumer_action_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("consumerActionId", consumer_action_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionList =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Subscription;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Subscription,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
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
                                let rsp_value: models::Subscription =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Subscription;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Subscription =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod replace_subscription {
        use super::models;
        type Response = models::Subscription;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Subscription,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
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
                                let rsp_value: models::Subscription =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
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
    pub mod create_subscriptions_query {
        use super::models;
        type Response = models::SubscriptionsQuery;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::SubscriptionsQuery,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptionsquery",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
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
                                let rsp_value: models::SubscriptionsQuery =
                                    serde_json::from_slice(&rsp_body)?;
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
pub mod diagnostics {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdateSubscripitonDiagnosticsParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::SubscriptionDiagnostics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions/{}/diagnostics",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionDiagnostics =
                                    serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SubscriptionDiagnostics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::UpdateSubscripitonDiagnosticsParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/hooks/subscriptions/{}/diagnostics",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
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
                                let rsp_value: models::SubscriptionDiagnostics =
                                    serde_json::from_slice(&rsp_body)?;
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
