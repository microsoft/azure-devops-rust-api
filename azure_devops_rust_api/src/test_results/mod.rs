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
pub const DEFAULT_ENDPOINT: &str = "https://vstmr.dev.azure.com";
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
    pub fn result_meta_data_client(&self) -> result_meta_data::Client {
        result_meta_data::Client(self.clone())
    }
    pub fn testlog_client(&self) -> testlog::Client {
        testlog::Client(self.clone())
    }
    pub fn testlogstoreendpoint_client(&self) -> testlogstoreendpoint::Client {
        testlogstoreendpoint::Client(self.clone())
    }
}
pub mod testlog {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of test subresult attachments reference"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the test run that contains the results"]
        #[doc = "* `result_id`: Id of the test result that contains subresult"]
        #[doc = "* `sub_result_id`: Id of the test subresult"]
        #[doc = "* `type_`: type of the attachments to get"]
        pub fn get_test_sub_result_logs(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            result_id: i32,
            sub_result_id: i32,
            type_: impl Into<String>,
        ) -> get_test_sub_result_logs::RequestBuilder {
            get_test_sub_result_logs::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                result_id,
                sub_result_id,
                type_: type_.into(),
                directory_path: None,
                file_name_prefix: None,
                fetch_meta_data: None,
                top: None,
                continuation_token: None,
            }
        }
        #[doc = "Get list of test result attachments reference"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the test run that contains the result"]
        #[doc = "* `result_id`: Id of the test result"]
        #[doc = "* `type_`: type of attachments to get"]
        pub fn get_test_result_logs(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            result_id: i32,
            type_: impl Into<String>,
        ) -> get_test_result_logs::RequestBuilder {
            get_test_result_logs::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                result_id,
                type_: type_.into(),
                directory_path: None,
                file_name_prefix: None,
                fetch_meta_data: None,
                top: None,
                continuation_token: None,
            }
        }
        #[doc = "Get list of test run attachments reference"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the test run"]
        #[doc = "* `type_`: type of the attachments to get"]
        pub fn get_test_run_logs(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            type_: impl Into<String>,
        ) -> get_test_run_logs::RequestBuilder {
            get_test_run_logs::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                type_: type_.into(),
                directory_path: None,
                file_name_prefix: None,
                fetch_meta_data: None,
                top: None,
                continuation_token: None,
            }
        }
    }
    pub mod get_test_sub_result_logs {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TestLogList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) result_id: i32,
            pub(crate) sub_result_id: i32,
            pub(crate) type_: String,
            pub(crate) directory_path: Option<String>,
            pub(crate) file_name_prefix: Option<String>,
            pub(crate) fetch_meta_data: Option<bool>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "directory path of the attachment to get"]
            pub fn directory_path(mut self, directory_path: impl Into<String>) -> Self {
                self.directory_path = Some(directory_path.into());
                self
            }
            #[doc = "Filename prefix to filter the list of attachmentss"]
            pub fn file_name_prefix(mut self, file_name_prefix: impl Into<String>) -> Self {
                self.file_name_prefix = Some(file_name_prefix.into());
                self
            }
            #[doc = "Default is false, set if metadata is needed"]
            pub fn fetch_meta_data(mut self, fetch_meta_data: bool) -> Self {
                self.fetch_meta_data = Some(fetch_meta_data);
                self
            }
            #[doc = "Number of attachment references to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Header to pass the continuation token"]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/testresults/runs/{}/results/{}/testlog?subResultId={}&type={}" , this . client . endpoint () , & this . organization , & this . project , & this . run_id , & this . result_id , & this . sub_result_id , & this . type_)) ? ;
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
                        let sub_result_id = &this.sub_result_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("subResultId", &sub_result_id.to_string());
                        let type_ = &this.type_;
                        req.url_mut().query_pairs_mut().append_pair("type", type_);
                        if let Some(directory_path) = &this.directory_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("directoryPath", directory_path);
                        }
                        if let Some(file_name_prefix) = &this.file_name_prefix {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fileNamePrefix", file_name_prefix);
                        }
                        if let Some(fetch_meta_data) = &this.fetch_meta_data {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fetchMetaData", &fetch_meta_data.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("top", &top.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.insert_header("continuationtoken", continuation_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::TestLogList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_test_result_logs {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TestLogList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) result_id: i32,
            pub(crate) type_: String,
            pub(crate) directory_path: Option<String>,
            pub(crate) file_name_prefix: Option<String>,
            pub(crate) fetch_meta_data: Option<bool>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Directory path of attachments to get"]
            pub fn directory_path(mut self, directory_path: impl Into<String>) -> Self {
                self.directory_path = Some(directory_path.into());
                self
            }
            #[doc = "Filename prefix to filter the list of attachments"]
            pub fn file_name_prefix(mut self, file_name_prefix: impl Into<String>) -> Self {
                self.file_name_prefix = Some(file_name_prefix.into());
                self
            }
            #[doc = "Default is false, set if metadata is needed"]
            pub fn fetch_meta_data(mut self, fetch_meta_data: bool) -> Self {
                self.fetch_meta_data = Some(fetch_meta_data);
                self
            }
            #[doc = "Number of attachment references to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Header to pass the continuation token"]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/runs/{}/results/{}/testlog",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.run_id,
                            &this.result_id
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
                        let type_ = &this.type_;
                        req.url_mut().query_pairs_mut().append_pair("type", type_);
                        if let Some(directory_path) = &this.directory_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("directoryPath", directory_path);
                        }
                        if let Some(file_name_prefix) = &this.file_name_prefix {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fileNamePrefix", file_name_prefix);
                        }
                        if let Some(fetch_meta_data) = &this.fetch_meta_data {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fetchMetaData", &fetch_meta_data.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("top", &top.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.insert_header("continuationtoken", continuation_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::TestLogList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_test_run_logs {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TestLogList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) type_: String,
            pub(crate) directory_path: Option<String>,
            pub(crate) file_name_prefix: Option<String>,
            pub(crate) fetch_meta_data: Option<bool>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "directory path for which attachments are needed"]
            pub fn directory_path(mut self, directory_path: impl Into<String>) -> Self {
                self.directory_path = Some(directory_path.into());
                self
            }
            #[doc = "Filename prefix to filter the list of attachments"]
            pub fn file_name_prefix(mut self, file_name_prefix: impl Into<String>) -> Self {
                self.file_name_prefix = Some(file_name_prefix.into());
                self
            }
            #[doc = "Default is false, set if metadata is needed"]
            pub fn fetch_meta_data(mut self, fetch_meta_data: bool) -> Self {
                self.fetch_meta_data = Some(fetch_meta_data);
                self
            }
            #[doc = "Number of attachment references to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Header to pass the continuation token"]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/runs/{}/testlog",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.run_id
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
                        let type_ = &this.type_;
                        req.url_mut().query_pairs_mut().append_pair("type", type_);
                        if let Some(directory_path) = &this.directory_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("directoryPath", directory_path);
                        }
                        if let Some(file_name_prefix) = &this.file_name_prefix {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fileNamePrefix", file_name_prefix);
                        }
                        if let Some(fetch_meta_data) = &this.fetch_meta_data {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fetchMetaData", &fetch_meta_data.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("top", &top.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.insert_header("continuationtoken", continuation_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::TestLogList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod testlogstoreendpoint {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get SAS Uri of a test subresults attachment"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the test run that contains result"]
        #[doc = "* `result_id`: Id of the test result that contains subresult"]
        #[doc = "* `sub_result_id`: Id of the test subresult whose file sas uri is needed"]
        #[doc = "* `type_`: type of the file"]
        #[doc = "* `file_path`: filePath for which sas uri is needed"]
        pub fn get_test_log_store_endpoint_details_for_sub_result_log(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            result_id: i32,
            sub_result_id: i32,
            type_: impl Into<String>,
            file_path: impl Into<String>,
        ) -> get_test_log_store_endpoint_details_for_sub_result_log::RequestBuilder {
            get_test_log_store_endpoint_details_for_sub_result_log::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                result_id,
                sub_result_id,
                type_: type_.into(),
                file_path: file_path.into(),
            }
        }
        #[doc = "Get SAS Uri of a test results attachment"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the test run that contains result"]
        #[doc = "* `result_id`: Id of the test result whose files need to be downloaded"]
        #[doc = "* `type_`: type of the file"]
        #[doc = "* `file_path`: filePath for which sas uri is needed"]
        pub fn get_test_log_store_endpoint_details_for_result_log(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            result_id: i32,
            type_: impl Into<String>,
            file_path: impl Into<String>,
        ) -> get_test_log_store_endpoint_details_for_result_log::RequestBuilder {
            get_test_log_store_endpoint_details_for_result_log::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                result_id,
                type_: type_.into(),
                file_path: file_path.into(),
            }
        }
        #[doc = "Create empty file for a result and Get Sas uri for the file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the test run that contains the result"]
        #[doc = "* `result_id`: Id of the test results that contains sub result"]
        #[doc = "* `sub_result_id`: Id of the test sub result whose file sas uri is needed"]
        #[doc = "* `file_path`: file path inside the sub result for which sas uri is needed"]
        #[doc = "* `type_`: Type of the file for download"]
        pub fn test_log_store_endpoint_details_for_result(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            result_id: i32,
            sub_result_id: i32,
            file_path: impl Into<String>,
            type_: impl Into<String>,
        ) -> test_log_store_endpoint_details_for_result::RequestBuilder {
            test_log_store_endpoint_details_for_result::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                result_id,
                sub_result_id,
                file_path: file_path.into(),
                type_: type_.into(),
            }
        }
        #[doc = "Get SAS Uri of a test run attachment"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the test run whose file has to be downloaded"]
        #[doc = "* `type_`: type of the file"]
        #[doc = "* `file_path`: filePath for which sas uri is needed"]
        pub fn get_test_log_store_endpoint_details_for_run_log(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            type_: impl Into<String>,
            file_path: impl Into<String>,
        ) -> get_test_log_store_endpoint_details_for_run_log::RequestBuilder {
            get_test_log_store_endpoint_details_for_run_log::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                type_: type_.into(),
                file_path: file_path.into(),
            }
        }
        #[doc = "Create empty file for a run and Get Sas uri for the file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `run_id`: Id of the run to get endpoint details"]
        #[doc = "* `test_log_store_operation_type`: Type of operation to perform using sas uri"]
        pub fn test_log_store_endpoint_details_for_run(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            run_id: i32,
            test_log_store_operation_type: impl Into<String>,
        ) -> test_log_store_endpoint_details_for_run::RequestBuilder {
            test_log_store_endpoint_details_for_run::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                run_id,
                test_log_store_operation_type: test_log_store_operation_type.into(),
                file_path: None,
                type_: None,
            }
        }
    }
    pub mod get_test_log_store_endpoint_details_for_sub_result_log {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::TestLogStoreEndpointDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogStoreEndpointDetails = serde_json::from_slice(&bytes)
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) result_id: i32,
            pub(crate) sub_result_id: i32,
            pub(crate) type_: String,
            pub(crate) file_path: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/testresults/runs/{}/results/{}/testlogstoreendpoint?subResultId={}&type={}&filePath={}" , this . client . endpoint () , & this . organization , & this . project , & this . run_id , & this . result_id , & this . sub_result_id , & this . type_ , & this . file_path)) ? ;
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
                        let sub_result_id = &this.sub_result_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("subResultId", &sub_result_id.to_string());
                        let type_ = &this.type_;
                        req.url_mut().query_pairs_mut().append_pair("type", type_);
                        let file_path = &this.file_path;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("filePath", file_path);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::TestLogStoreEndpointDetails>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_test_log_store_endpoint_details_for_result_log {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::TestLogStoreEndpointDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogStoreEndpointDetails = serde_json::from_slice(&bytes)
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) result_id: i32,
            pub(crate) type_: String,
            pub(crate) file_path: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/runs/{}/results/{}/testlogstoreendpoint",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.run_id,
                            &this.result_id
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
                        let type_ = &this.type_;
                        req.url_mut().query_pairs_mut().append_pair("type", type_);
                        let file_path = &this.file_path;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("filePath", file_path);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::TestLogStoreEndpointDetails>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod test_log_store_endpoint_details_for_result {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::TestLogStoreEndpointDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogStoreEndpointDetails = serde_json::from_slice(&bytes)
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) result_id: i32,
            pub(crate) sub_result_id: i32,
            pub(crate) file_path: String,
            pub(crate) type_: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/runs/{}/results/{}/testlogstoreendpoint",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.run_id,
                            &this.result_id
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
                        let sub_result_id = &this.sub_result_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("subResultId", &sub_result_id.to_string());
                        let file_path = &this.file_path;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("filePath", file_path);
                        let type_ = &this.type_;
                        req.url_mut().query_pairs_mut().append_pair("type", type_);
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::TestLogStoreEndpointDetails>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_test_log_store_endpoint_details_for_run_log {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::TestLogStoreEndpointDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogStoreEndpointDetails = serde_json::from_slice(&bytes)
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) type_: String,
            pub(crate) file_path: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/runs/{}/testlogstoreendpoint",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.run_id
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
                        let type_ = &this.type_;
                        req.url_mut().query_pairs_mut().append_pair("type", type_);
                        let file_path = &this.file_path;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("filePath", file_path);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::TestLogStoreEndpointDetails>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod test_log_store_endpoint_details_for_run {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::TestLogStoreEndpointDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestLogStoreEndpointDetails = serde_json::from_slice(&bytes)
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) run_id: i32,
            pub(crate) test_log_store_operation_type: String,
            pub(crate) file_path: Option<String>,
            pub(crate) type_: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "file path to create an empty file"]
            pub fn file_path(mut self, file_path: impl Into<String>) -> Self {
                self.file_path = Some(file_path.into());
                self
            }
            #[doc = "Default is GeneralAttachment, type of empty file to be created"]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/runs/{}/testlogstoreendpoint",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.run_id
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
                        let test_log_store_operation_type = &this.test_log_store_operation_type;
                        req.url_mut().query_pairs_mut().append_pair(
                            "testLogStoreOperationType",
                            test_log_store_operation_type,
                        );
                        if let Some(file_path) = &this.file_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("filePath", file_path);
                        }
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::TestLogStoreEndpointDetails>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod result_meta_data {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of test Result meta data details for corresponding testcasereferenceId"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: TestCaseReference Ids of the test Result to be queried, comma separated list of valid ids (limit no. of ids 200)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn query(
            &self,
            organization: impl Into<String>,
            body: Vec<String>,
            project: impl Into<String>,
        ) -> query::RequestBuilder {
            query::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                details_to_include: None,
            }
        }
        #[doc = "Update properties of test result meta data"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: TestResultMetaData update input TestResultMetaDataUpdateInput"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_case_reference_id`: TestCaseReference Id of Test Result to be updated."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestResultMetaDataUpdateInput>,
            project: impl Into<String>,
            test_case_reference_id: i32,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                test_case_reference_id,
            }
        }
    }
    pub mod query {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TestResultMetaDataList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestResultMetaDataList =
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<String>,
            pub(crate) project: String,
            pub(crate) details_to_include: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Details to include with test results metadata. Default is None. Other values are FlakyIdentifiers."]
            pub fn details_to_include(mut self, details_to_include: impl Into<String>) -> Self {
                self.details_to_include = Some(details_to_include.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/results/resultmetadata",
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
                        if let Some(details_to_include) = &this.details_to_include {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("detailsToInclude", details_to_include);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::TestResultMetaDataList>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::TestResultMetaData> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::TestResultMetaData =
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestResultMetaDataUpdateInput,
            pub(crate) project: String,
            pub(crate) test_case_reference_id: i32,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testresults/results/resultmetadata/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.test_case_reference_id
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::TestResultMetaData>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
