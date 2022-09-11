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
pub const DEFAULT_ENDPOINT: &str = "https://auditservice.dev.azure.com";
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
    pub fn actions_client(&self) -> actions::Client {
        actions::Client(self.clone())
    }
    pub fn audit_log_client(&self) -> audit_log::Client {
        audit_log::Client(self.clone())
    }
    pub fn download_log_client(&self) -> download_log::Client {
        download_log::Client(self.clone())
    }
    pub fn streams_client(&self) -> streams::Client {
        streams::Client(self.clone())
    }
}
pub mod actions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all auditable actions filterable by area."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                area_name: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AuditActionInfoList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) area_name: Option<String>,
        }
        impl Builder {
            #[doc = "Optional. Get actions scoped to area"]
            pub fn area_name(mut self, area_name: impl Into<String>) -> Self {
                self.area_name = Some(area_name.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/actions",
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
                        if let Some(area_name) = &this.area_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("areaName", area_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuditActionInfoList =
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
pub mod audit_log {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Queries audit log entries"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn query(&self, organization: impl Into<String>) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                start_time: None,
                end_time: None,
                batch_size: None,
                continuation_token: None,
                skip_aggregation: None,
            }
        }
    }
    pub mod query {
        use super::models;
        type Response = models::AuditLogQueryResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
            pub(crate) batch_size: Option<i32>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) skip_aggregation: Option<bool>,
        }
        impl Builder {
            #[doc = "Start time of download window. Optional"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "End time of download window. Optional"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            #[doc = "Max number of results to return. Optional"]
            pub fn batch_size(mut self, batch_size: i32) -> Self {
                self.batch_size = Some(batch_size);
                self
            }
            #[doc = "Token used for returning next set of results from previous query. Optional"]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Skips aggregating events and leaves them as individual entries instead. By default events are aggregated. Event types that are aggregated: AuditLog.AccessLog."]
            pub fn skip_aggregation(mut self, skip_aggregation: bool) -> Self {
                self.skip_aggregation = Some(skip_aggregation);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/auditlog",
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
                        if let Some(start_time) = &this.start_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("endTime", &end_time.to_string());
                        }
                        if let Some(batch_size) = &this.batch_size {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("batchSize", &batch_size.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(skip_aggregation) = &this.skip_aggregation {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("skipAggregation", &skip_aggregation.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuditLogQueryResult =
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
pub mod download_log {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Downloads audit log entries."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `format`: File format for download. Can be \"json\" or \"csv\"."]
        pub fn download_log(
            &self,
            organization: impl Into<String>,
            format: impl Into<String>,
        ) -> download_log::Builder {
            download_log::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                format: format.into(),
                start_time: None,
                end_time: None,
            }
        }
    }
    pub mod download_log {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) format: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "Start time of download window. Optional"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "End time of download window. Optional"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/downloadlog",
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
                        let format = &this.format;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("format", format);
                        if let Some(start_time) = &this.start_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("endTime", &end_time.to_string());
                        }
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
pub mod streams {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return all Audit Streams scoped to an organization"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn query_all_streams(
            &self,
            organization: impl Into<String>,
        ) -> query_all_streams::Builder {
            query_all_streams::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Create new Audit Stream"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Stream entry"]
        #[doc = "* `days_to_backfill`: The number of days of previously recorded audit data that will be replayed into the stream. A value of zero will result in only new events being streamed."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::AuditStream>,
            days_to_backfill: i32,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                days_to_backfill,
            }
        }
        #[doc = "Update existing Audit Stream"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Stream entry"]
        pub fn update_stream(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::AuditStream>,
        ) -> update_stream::Builder {
            update_stream::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Return Audit Stream with id of streamId if one exists otherwise throw"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `stream_id`: Id of stream entry to retrieve"]
        pub fn query_stream_by_id(
            &self,
            organization: impl Into<String>,
            stream_id: i32,
        ) -> query_stream_by_id::Builder {
            query_stream_by_id::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                stream_id,
            }
        }
        #[doc = "Update existing Audit Stream status"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `stream_id`: Id of stream entry to be updated"]
        #[doc = "* `status`: Status of the stream"]
        pub fn update_status(
            &self,
            organization: impl Into<String>,
            stream_id: i32,
            status: impl Into<String>,
        ) -> update_status::Builder {
            update_status::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                stream_id,
                status: status.into(),
            }
        }
        #[doc = "Delete Audit Stream"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `stream_id`: Id of stream entry to delete"]
        pub fn delete(&self, organization: impl Into<String>, stream_id: i32) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                stream_id,
            }
        }
    }
    pub mod query_all_streams {
        use super::models;
        type Response = models::AuditStreamList;
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
                            "{}/{}/_apis/audit/streams",
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
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuditStreamList =
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
        type Response = models::AuditStream;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::AuditStream,
            pub(crate) days_to_backfill: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/streams",
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
                        let days_to_backfill = &this.days_to_backfill;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("daysToBackfill", &days_to_backfill.to_string());
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuditStream =
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
    pub mod update_stream {
        use super::models;
        type Response = models::AuditStream;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::AuditStream,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/streams",
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
                                let rsp_value: models::AuditStream =
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
    pub mod query_stream_by_id {
        use super::models;
        type Response = models::AuditStream;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) stream_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/streams/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.stream_id
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
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuditStream =
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
    pub mod update_status {
        use super::models;
        type Response = models::AuditStream;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) stream_id: i32,
            pub(crate) status: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/streams/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.stream_id
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
                        let status = &this.status;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("status", status);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuditStream =
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) stream_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/audit/streams/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.stream_id
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
}
