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
    pub fn configurations_client(&self) -> configurations::Client {
        configurations::Client(self.clone())
    }
    pub fn suite_test_case_client(&self) -> suite_test_case::Client {
        suite_test_case::Client(self.clone())
    }
    pub fn test_case_clone_client(&self) -> test_case_clone::Client {
        test_case_clone::Client(self.clone())
    }
    pub fn test_cases_client(&self) -> test_cases::Client {
        test_cases::Client(self.clone())
    }
    pub fn test_plan_clone_client(&self) -> test_plan_clone::Client {
        test_plan_clone::Client(self.clone())
    }
    pub fn test_plans_client(&self) -> test_plans::Client {
        test_plans::Client(self.clone())
    }
    pub fn test_point_client(&self) -> test_point::Client {
        test_point::Client(self.clone())
    }
    pub fn test_suite_clone_client(&self) -> test_suite_clone::Client {
        test_suite_clone::Client(self.clone())
    }
    pub fn test_suite_entry_client(&self) -> test_suite_entry::Client {
        test_suite_entry::Client(self.clone())
    }
    pub fn test_suites_client(&self) -> test_suites::Client {
        test_suites::Client(self.clone())
    }
    pub fn variables_client(&self) -> variables::Client {
        variables::Client(self.clone())
    }
}
pub mod suite_test_case {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Removes test cases from a suite based on the list of test case Ids provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan from which test cases are to be removed."]
        #[doc = "* `suite_id`: ID of the test suite from which test cases are to be removed."]
        #[doc = "* `test_case_ids`: Test Case Ids to be removed."]
        pub fn remove_test_cases_from_suite(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
            test_case_ids: impl Into<String>,
        ) -> remove_test_cases_from_suite::Builder {
            remove_test_cases_from_suite::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
                test_case_ids: test_case_ids.into(),
            }
        }
        #[doc = "Get Test Case List return those test cases which have all the configuration Ids as mentioned in the optional parameter. If configuration Ids is null, it return all the test cases"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan for which test cases are requested."]
        #[doc = "* `suite_id`: ID of the test suite for which test cases are requested."]
        pub fn get_test_case_list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> get_test_case_list::Builder {
            get_test_case_list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
                test_ids: None,
                configuration_ids: None,
                wit_fields: None,
                continuation_token: None,
                return_identity_ref: None,
                expand: None,
                exclude_flags: None,
                is_recursive: None,
            }
        }
        #[doc = "Add test cases to a suite with specified configurations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: SuiteTestCaseCreateUpdateParameters object."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan to which test cases are to be added."]
        #[doc = "* `suite_id`: ID of the test suite to which test cases are to be added."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: Vec<models::SuiteTestCaseCreateUpdateParameters>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                plan_id,
                suite_id,
            }
        }
        #[doc = "Update the configurations for test cases"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A SuiteTestCaseCreateUpdateParameters object."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan to which test cases are to be updated."]
        #[doc = "* `suite_id`: ID of the test suite to which test cases are to be updated."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: Vec<models::SuiteTestCaseCreateUpdateParameters>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                plan_id,
                suite_id,
            }
        }
        #[doc = "Removes test cases from a suite based on the list of test case Ids provided. This API can be used to remove a larger number of test cases."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan from which test cases are to be removed."]
        #[doc = "* `suite_id`: ID of the test suite from which test cases are to be removed."]
        #[doc = "* `test_ids`: Comma separated string of Test Case Ids to be removed."]
        pub fn remove_test_cases_list_from_suite(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
            test_ids: impl Into<String>,
        ) -> remove_test_cases_list_from_suite::Builder {
            remove_test_cases_list_from_suite::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
                test_ids: test_ids.into(),
            }
        }
        #[doc = "Get a particular Test Case from a Suite."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan for which test cases are requested."]
        #[doc = "* `suite_id`: ID of the test suite for which test cases are requested."]
        #[doc = "* `test_case_id`: Test Case Id to be fetched."]
        pub fn get_test_case(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
            test_case_id: impl Into<String>,
        ) -> get_test_case::Builder {
            get_test_case::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
                test_case_id: test_case_id.into(),
                wit_fields: None,
                return_identity_ref: None,
            }
        }
    }
    pub mod remove_test_cases_from_suite {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) test_case_ids: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestCase?testCaseIds={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id,
                            &this.test_case_ids
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
                        let test_case_ids = &this.test_case_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("testCaseIds", test_case_ids);
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
    pub mod get_test_case_list {
        use super::models;
        type Response = models::TestCaseList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) test_ids: Option<String>,
            pub(crate) configuration_ids: Option<String>,
            pub(crate) wit_fields: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) return_identity_ref: Option<bool>,
            pub(crate) expand: Option<bool>,
            pub(crate) exclude_flags: Option<String>,
            pub(crate) is_recursive: Option<bool>,
        }
        impl Builder {
            #[doc = "Test Case Ids to be fetched."]
            pub fn test_ids(mut self, test_ids: impl Into<String>) -> Self {
                self.test_ids = Some(test_ids.into());
                self
            }
            #[doc = "Fetch Test Cases which contains all the configuration Ids specified."]
            pub fn configuration_ids(mut self, configuration_ids: impl Into<String>) -> Self {
                self.configuration_ids = Some(configuration_ids.into());
                self
            }
            #[doc = "Get the list of witFields."]
            pub fn wit_fields(mut self, wit_fields: impl Into<String>) -> Self {
                self.wit_fields = Some(wit_fields.into());
                self
            }
            #[doc = "If the list of test cases returned is not complete, a continuation token to query next batch of test cases is included in the response header as \"x-ms-continuationtoken\". Omit this parameter to get the first batch of test cases."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "If set to true, returns all identity fields, like AssignedTo, ActivatedBy etc., as IdentityRef objects. If set to false, these fields are returned as unique names in string format. This is false by default."]
            pub fn return_identity_ref(mut self, return_identity_ref: bool) -> Self {
                self.return_identity_ref = Some(return_identity_ref);
                self
            }
            #[doc = "If set to false, will get a smaller payload containing only basic details about the suite test case object"]
            pub fn expand(mut self, expand: bool) -> Self {
                self.expand = Some(expand);
                self
            }
            #[doc = "Flag to exclude various values from payload. For example to remove point assignments pass exclude = 1. To remove extra information (links, test plan , test suite) pass exclude = 2. To remove both extra information and point assignments pass exclude = 3 (1 + 2)."]
            pub fn exclude_flags(mut self, exclude_flags: impl Into<String>) -> Self {
                self.exclude_flags = Some(exclude_flags.into());
                self
            }
            pub fn is_recursive(mut self, is_recursive: bool) -> Self {
                self.is_recursive = Some(is_recursive);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestCase",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
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
                        if let Some(test_ids) = &this.test_ids {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("testIds", test_ids);
                        }
                        if let Some(configuration_ids) = &this.configuration_ids {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("configurationIds", configuration_ids);
                        }
                        if let Some(wit_fields) = &this.wit_fields {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("witFields", wit_fields);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(return_identity_ref) = &this.return_identity_ref {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("returnIdentityRef", &return_identity_ref.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("expand", &expand.to_string());
                        }
                        if let Some(exclude_flags) = &this.exclude_flags {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("excludeFlags", exclude_flags);
                        }
                        if let Some(is_recursive) = &this.is_recursive {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isRecursive", &is_recursive.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestCaseList =
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
    pub mod add {
        use super::models;
        type Response = models::TestCaseList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::SuiteTestCaseCreateUpdateParameters>,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestCase",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
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
                                let rsp_value: models::TestCaseList =
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
        type Response = models::TestCaseList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::SuiteTestCaseCreateUpdateParameters>,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestCase",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                                let rsp_value: models::TestCaseList =
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
    pub mod remove_test_cases_list_from_suite {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) test_ids: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestCase",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
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
                        let test_ids = &this.test_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("testIds", test_ids);
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
    pub mod get_test_case {
        use super::models;
        type Response = models::TestCaseList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) test_case_id: String,
            pub(crate) wit_fields: Option<String>,
            pub(crate) return_identity_ref: Option<bool>,
        }
        impl Builder {
            #[doc = "Get the list of witFields."]
            pub fn wit_fields(mut self, wit_fields: impl Into<String>) -> Self {
                self.wit_fields = Some(wit_fields.into());
                self
            }
            #[doc = "If set to true, returns all identity fields, like AssignedTo, ActivatedBy etc., as IdentityRef objects. If set to false, these fields are returned as unique names in string format. This is false by default."]
            pub fn return_identity_ref(mut self, return_identity_ref: bool) -> Self {
                self.return_identity_ref = Some(return_identity_ref);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestCase/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id,
                            &this.test_case_id
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
                        if let Some(wit_fields) = &this.wit_fields {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("witFields", wit_fields);
                        }
                        if let Some(return_identity_ref) = &this.return_identity_ref {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("returnIdentityRef", &return_identity_ref.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestCaseList =
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
pub mod test_point {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all the points inside a suite based on some filters"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan for which test points are requested."]
        #[doc = "* `suite_id`: ID of the test suite for which test points are requested"]
        pub fn get_points_list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> get_points_list::Builder {
            get_points_list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
                test_point_ids: None,
                test_case_id: None,
                continuation_token: None,
                return_identity_ref: None,
                include_point_details: None,
                is_recursive: None,
            }
        }
        #[doc = "Get a particular Test Point from a suite."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan for which test points are requested."]
        #[doc = "* `suite_id`: ID of the test suite for which test points are requested."]
        #[doc = "* `point_id`: ID of test point to be fetched."]
        pub fn get_points(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
            point_id: impl Into<String>,
        ) -> get_points::Builder {
            get_points::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
                point_id: point_id.into(),
                return_identity_ref: None,
                include_point_details: None,
            }
        }
        #[doc = "Update Test Points. This is used to Reset test point to active, update the outcome of a test point or update the tester of a test point"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A TestPointUpdateParams Object."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan for which test points are requested."]
        #[doc = "* `suite_id`: ID of the test suite for which test points are requested."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: Vec<models::TestPointUpdateParams>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                plan_id,
                suite_id,
                include_point_details: None,
                return_identity_ref: None,
            }
        }
    }
    pub mod get_points_list {
        use super::models;
        type Response = models::TestPointList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) test_point_ids: Option<String>,
            pub(crate) test_case_id: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) return_identity_ref: Option<bool>,
            pub(crate) include_point_details: Option<bool>,
            pub(crate) is_recursive: Option<bool>,
        }
        impl Builder {
            #[doc = "ID of test points to fetch."]
            pub fn test_point_ids(mut self, test_point_ids: impl Into<String>) -> Self {
                self.test_point_ids = Some(test_point_ids.into());
                self
            }
            #[doc = "Get Test Points for specific test case Ids."]
            pub fn test_case_id(mut self, test_case_id: impl Into<String>) -> Self {
                self.test_case_id = Some(test_case_id.into());
                self
            }
            #[doc = "If the list of test point returned is not complete, a continuation token to query next batch of test points is included in the response header as \"x-ms-continuationtoken\". Omit this parameter to get the first batch of test points."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "If set to true, returns the AssignedTo field in TestCaseReference as IdentityRef object."]
            pub fn return_identity_ref(mut self, return_identity_ref: bool) -> Self {
                self.return_identity_ref = Some(return_identity_ref);
                self
            }
            #[doc = "If set to false, will get a smaller payload containing only basic details about the test point object"]
            pub fn include_point_details(mut self, include_point_details: bool) -> Self {
                self.include_point_details = Some(include_point_details);
                self
            }
            #[doc = "If set to true, will also fetch test points belonging to child suites recursively."]
            pub fn is_recursive(mut self, is_recursive: bool) -> Self {
                self.is_recursive = Some(is_recursive);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestPoint?",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
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
                        if let Some(test_point_ids) = &this.test_point_ids {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("testPointIds", test_point_ids);
                        }
                        if let Some(test_case_id) = &this.test_case_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("testCaseId", test_case_id);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(return_identity_ref) = &this.return_identity_ref {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("returnIdentityRef", &return_identity_ref.to_string());
                        }
                        if let Some(include_point_details) = &this.include_point_details {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includePointDetails",
                                &include_point_details.to_string(),
                            );
                        }
                        if let Some(is_recursive) = &this.is_recursive {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isRecursive", &is_recursive.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestPointList =
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
    pub mod get_points {
        use super::models;
        type Response = models::TestPointList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) point_id: String,
            pub(crate) return_identity_ref: Option<bool>,
            pub(crate) include_point_details: Option<bool>,
        }
        impl Builder {
            #[doc = "If set to true, returns the AssignedTo field in TestCaseReference as IdentityRef object."]
            pub fn return_identity_ref(mut self, return_identity_ref: bool) -> Self {
                self.return_identity_ref = Some(return_identity_ref);
                self
            }
            #[doc = "If set to false, will get a smaller payload containing only basic details about the test point object"]
            pub fn include_point_details(mut self, include_point_details: bool) -> Self {
                self.include_point_details = Some(include_point_details);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestPoint",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
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
                        let point_id = &this.point_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("pointId", point_id);
                        if let Some(return_identity_ref) = &this.return_identity_ref {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("returnIdentityRef", &return_identity_ref.to_string());
                        }
                        if let Some(include_point_details) = &this.include_point_details {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includePointDetails",
                                &include_point_details.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestPointList =
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
        type Response = models::TestPointList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::TestPointUpdateParams>,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) include_point_details: Option<bool>,
            pub(crate) return_identity_ref: Option<bool>,
        }
        impl Builder {
            #[doc = "If set to false, will get a smaller payload containing only basic details about the test point object"]
            pub fn include_point_details(mut self, include_point_details: bool) -> Self {
                self.include_point_details = Some(include_point_details);
                self
            }
            #[doc = "If set to true, returns the AssignedTo field in TestCaseReference as IdentityRef object."]
            pub fn return_identity_ref(mut self, return_identity_ref: bool) -> Self {
                self.return_identity_ref = Some(return_identity_ref);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/Suites/{}/TestPoint",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                        if let Some(include_point_details) = &this.include_point_details {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includePointDetails",
                                &include_point_details.to_string(),
                            );
                        }
                        if let Some(return_identity_ref) = &this.return_identity_ref {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("returnIdentityRef", &return_identity_ref.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestPointList =
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
pub mod test_suites {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Find the list of all test suites in which a given test case is present. This is helpful if you need to find out which test suites are using a test case, when you need to make changes to a test case."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `test_case_id`: ID of the test case for which suites need to be fetched."]
        pub fn get_suites_by_test_case_id(
            &self,
            organization: impl Into<String>,
            test_case_id: i32,
        ) -> get_suites_by_test_case_id::Builder {
            get_suites_by_test_case_id::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                test_case_id,
            }
        }
        #[doc = "Get test suites for plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan for which suites are requested."]
        pub fn get_test_suites_for_plan(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
        ) -> get_test_suites_for_plan::Builder {
            get_test_suites_for_plan::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                expand: None,
                continuation_token: None,
                as_tree_view: None,
            }
        }
        #[doc = "Create test suite."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Parameters for suite creation"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan that contains the suites."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestSuiteCreateParams>,
            project: impl Into<String>,
            plan_id: i32,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                plan_id,
            }
        }
        #[doc = "Get test suite by suite id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan that contains the suites."]
        #[doc = "* `suite_id`: ID of the suite to get."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
                expand: None,
            }
        }
        #[doc = "Update test suite."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Parameters for suite updation"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan that contains the suites."]
        #[doc = "* `suite_id`: ID of the parent suite."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestSuiteUpdateParams>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                plan_id,
                suite_id,
            }
        }
        #[doc = "Delete test suite."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan that contains the suite."]
        #[doc = "* `suite_id`: ID of the test suite to delete."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
            suite_id: i32,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
                suite_id,
            }
        }
    }
    pub mod get_suites_by_test_case_id {
        use super::models;
        type Response = models::TestSuiteList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) test_case_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/testplan/suites",
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
                        let test_case_id = &this.test_case_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("testCaseId", &test_case_id.to_string());
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestSuiteList =
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
    pub mod get_test_suites_for_plan {
        use super::models;
        type Response = models::TestSuiteList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) expand: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) as_tree_view: Option<bool>,
        }
        impl Builder {
            #[doc = "Include the children suites and testers details."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "If the list of suites returned is not complete, a continuation token to query next batch of suites is included in the response header as \"x-ms-continuationtoken\". Omit this parameter to get the first batch of test suites."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "If the suites returned should be in a tree structure."]
            pub fn as_tree_view(mut self, as_tree_view: bool) -> Self {
                self.as_tree_view = Some(as_tree_view);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/suites",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("expand", expand);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(as_tree_view) = &this.as_tree_view {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("asTreeView", &as_tree_view.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestSuiteList =
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
        type Response = models::TestSuite;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestSuiteCreateParams,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/suites",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id
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
                                let rsp_value: models::TestSuite =
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
        type Response = models::TestSuite;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Include the children suites and testers details"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/suites/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestSuite =
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
        type Response = models::TestSuite;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestSuiteUpdateParams,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/suites/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                                let rsp_value: models::TestSuite =
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
            pub(crate) project: String,
            pub(crate) plan_id: i32,
            pub(crate) suite_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/{}/suites/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id,
                            &this.suite_id
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
}
pub mod configurations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of test configurations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                continuation_token: None,
            }
        }
        #[doc = "Create a test configuration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: TestConfigurationCreateUpdateParameters"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestConfigurationCreateUpdateParameters>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Update a test configuration by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: TestConfigurationCreateUpdateParameters"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_configuartion_id`: ID of the test configuration to update."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestConfigurationCreateUpdateParameters>,
            project: impl Into<String>,
            test_configuartion_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                test_configuartion_id,
            }
        }
        #[doc = "Delete a test configuration by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_configuartion_id`: ID of the test configuration to delete."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            test_configuartion_id: i32,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                test_configuartion_id,
            }
        }
        #[doc = "Get a test configuration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_configuration_id`: ID of the test configuration to get."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            test_configuration_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                test_configuration_id,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TestConfigurationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) continuation_token: Option<String>,
        }
        impl Builder {
            #[doc = "If the list of configurations returned is not complete, a continuation token to query next batch of configurations is included in the response header as \"x-ms-continuationtoken\". Omit this parameter to get the first batch of test configurations."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/configurations",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestConfigurationList =
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
        type Response = models::TestConfiguration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestConfigurationCreateUpdateParameters,
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
                            "{}/{}/{}/_apis/testplan/configurations",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                                let rsp_value: models::TestConfiguration =
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
        type Response = models::TestConfiguration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestConfigurationCreateUpdateParameters,
            pub(crate) project: String,
            pub(crate) test_configuartion_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/configurations",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                        let test_configuartion_id = &this.test_configuartion_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("testConfiguartionId", &test_configuartion_id.to_string());
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestConfiguration =
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
            pub(crate) project: String,
            pub(crate) test_configuartion_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/configurations",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        let test_configuartion_id = &this.test_configuartion_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("testConfiguartionId", &test_configuartion_id.to_string());
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
    pub mod get {
        use super::models;
        type Response = models::TestConfiguration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) test_configuration_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/configurations/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.test_configuration_id
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
                                let rsp_value: models::TestConfiguration =
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
pub mod test_plans {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of test plans"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                owner: None,
                continuation_token: None,
                include_plan_details: None,
                filter_active_plans: None,
            }
        }
        #[doc = "Create a test plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A testPlanCreateParams object.TestPlanCreateParams"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestPlanCreateParams>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get a test plan by Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan to get."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
            }
        }
        #[doc = "Update a test plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A testPlanUpdateParams object.TestPlanUpdateParams"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan to be updated."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestPlanUpdateParams>,
            project: impl Into<String>,
            plan_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                plan_id,
            }
        }
        #[doc = "Delete a test plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `plan_id`: ID of the test plan to be deleted."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            plan_id: i32,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                plan_id,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TestPlanList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) owner: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) include_plan_details: Option<bool>,
            pub(crate) filter_active_plans: Option<bool>,
        }
        impl Builder {
            #[doc = "Filter for test plan by owner ID or name"]
            pub fn owner(mut self, owner: impl Into<String>) -> Self {
                self.owner = Some(owner.into());
                self
            }
            #[doc = "If the list of plans returned is not complete, a continuation token to query next batch of plans is included in the response header as \"x-ms-continuationtoken\". Omit this parameter to get the first batch of test plans."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Get all properties of the test plan"]
            pub fn include_plan_details(mut self, include_plan_details: bool) -> Self {
                self.include_plan_details = Some(include_plan_details);
                self
            }
            #[doc = "Get just the active plans"]
            pub fn filter_active_plans(mut self, filter_active_plans: bool) -> Self {
                self.filter_active_plans = Some(filter_active_plans);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/plans",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        if let Some(owner) = &this.owner {
                            req.url_mut().query_pairs_mut().append_pair("owner", owner);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(include_plan_details) = &this.include_plan_details {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includePlanDetails",
                                &include_plan_details.to_string(),
                            );
                        }
                        if let Some(filter_active_plans) = &this.filter_active_plans {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("filterActivePlans", &filter_active_plans.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestPlanList =
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
        type Response = models::TestPlan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestPlanCreateParams,
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
                            "{}/{}/{}/_apis/testplan/plans",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                                let rsp_value: models::TestPlan =
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
        type Response = models::TestPlan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/plans/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id
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
                                let rsp_value: models::TestPlan =
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
        type Response = models::TestPlan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestPlanUpdateParams,
            pub(crate) project: String,
            pub(crate) plan_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/plans/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                                let rsp_value: models::TestPlan =
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
            pub(crate) project: String,
            pub(crate) plan_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/plans/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.plan_id
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
}
pub mod test_plan_clone {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Clone test plan"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Plan Clone Request Body detail TestPlanCloneRequest"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn clone_test_plan(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CloneTestPlanParams>,
            project: impl Into<String>,
        ) -> clone_test_plan::Builder {
            clone_test_plan::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                deep_clone: None,
            }
        }
        #[doc = "Get clone information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `clone_operation_id`: Operation ID returned when we queue a clone operation"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            clone_operation_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                clone_operation_id,
            }
        }
    }
    pub mod clone_test_plan {
        use super::models;
        type Response = models::CloneTestPlanOperationInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CloneTestPlanParams,
            pub(crate) project: String,
            pub(crate) deep_clone: Option<bool>,
        }
        impl Builder {
            #[doc = "Clones all the associated test cases as well"]
            pub fn deep_clone(mut self, deep_clone: bool) -> Self {
                self.deep_clone = Some(deep_clone);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/CloneOperation",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        if let Some(deep_clone) = &this.deep_clone {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deepClone", &deep_clone.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloneTestPlanOperationInformation =
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
        type Response = models::CloneTestPlanOperationInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) clone_operation_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Plans/CloneOperation/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.clone_operation_id
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
                                let rsp_value: models::CloneTestPlanOperationInformation =
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
pub mod test_suite_entry {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of test suite entries in the test suite."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `suite_id`: Id of the parent suite."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            suite_id: i32,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                suite_id,
                suite_entry_type: None,
            }
        }
        #[doc = "Reorder test suite entries in the test suite."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: List of SuiteEntry to reorder."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `suite_id`: Id of the parent test suite."]
        pub fn reorder_suite_entries(
            &self,
            organization: impl Into<String>,
            body: Vec<models::SuiteEntryUpdateParams>,
            project: impl Into<String>,
            suite_id: i32,
        ) -> reorder_suite_entries::Builder {
            reorder_suite_entries::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                suite_id,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SuiteEntryList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) suite_id: i32,
            pub(crate) suite_entry_type: Option<String>,
        }
        impl Builder {
            pub fn suite_entry_type(mut self, suite_entry_type: impl Into<String>) -> Self {
                self.suite_entry_type = Some(suite_entry_type.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/suiteentry/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.suite_id
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
                        if let Some(suite_entry_type) = &this.suite_entry_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("suiteEntryType", suite_entry_type);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SuiteEntryList =
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
    pub mod reorder_suite_entries {
        use super::models;
        type Response = models::SuiteEntryList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::SuiteEntryUpdateParams>,
            pub(crate) project: String,
            pub(crate) suite_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/suiteentry/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.suite_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                                let rsp_value: models::SuiteEntryList =
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
pub mod test_suite_clone {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Clone test suite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Suite Clone Request Body detail TestSuiteCloneRequest"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn clone_test_suite(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CloneTestSuiteParams>,
            project: impl Into<String>,
        ) -> clone_test_suite::Builder {
            clone_test_suite::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                deep_clone: None,
            }
        }
        #[doc = "Get clone information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `clone_operation_id`: Operation ID returned when we queue a clone operation"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            clone_operation_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                clone_operation_id,
            }
        }
    }
    pub mod clone_test_suite {
        use super::models;
        type Response = models::CloneTestSuiteOperationInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CloneTestSuiteParams,
            pub(crate) project: String,
            pub(crate) deep_clone: Option<bool>,
        }
        impl Builder {
            #[doc = "Clones all the associated test cases as well"]
            pub fn deep_clone(mut self, deep_clone: bool) -> Self {
                self.deep_clone = Some(deep_clone);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Suites/CloneOperation",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        if let Some(deep_clone) = &this.deep_clone {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deepClone", &deep_clone.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloneTestSuiteOperationInformation =
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
        type Response = models::CloneTestSuiteOperationInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) clone_operation_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/Suites/CloneOperation/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.clone_operation_id
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
                                let rsp_value: models::CloneTestSuiteOperationInformation =
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
pub mod test_cases {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Delete a test case."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_case_id`: Id of test case to be deleted."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            test_case_id: i32,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                test_case_id,
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
            pub(crate) project: String,
            pub(crate) test_case_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/testcases/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.test_case_id
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
}
pub mod test_case_clone {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn clone_test_case(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CloneTestCaseParams>,
            project: impl Into<String>,
        ) -> clone_test_case::Builder {
            clone_test_case::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get clone information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `clone_operation_id`: Operation ID returned when we queue a clone operation"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            clone_operation_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                clone_operation_id,
            }
        }
    }
    pub mod clone_test_case {
        use super::models;
        type Response = models::CloneTestCaseOperationInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CloneTestCaseParams,
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
                            "{}/{}/{}/_apis/testplan/TestCases/CloneTestCaseOperation",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                                let rsp_value: models::CloneTestCaseOperationInformation =
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
        type Response = models::CloneTestCaseOperationInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) clone_operation_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/TestCases/CloneTestCaseOperation/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.clone_operation_id
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
                                let rsp_value: models::CloneTestCaseOperationInformation =
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
pub mod variables {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of test variables."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                continuation_token: None,
            }
        }
        #[doc = "Create a test variable."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: TestVariableCreateUpdateParameters"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestVariableCreateUpdateParameters>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get a test variable by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_variable_id`: ID of the test variable to get."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            test_variable_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                test_variable_id,
            }
        }
        #[doc = "Update a test variable by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: TestVariableCreateUpdateParameters"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_variable_id`: ID of the test variable to update."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TestVariableCreateUpdateParameters>,
            project: impl Into<String>,
            test_variable_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                test_variable_id,
            }
        }
        #[doc = "Delete a test variable by its ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `test_variable_id`: ID of the test variable to delete."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            test_variable_id: i32,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                test_variable_id,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TestVariableList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) continuation_token: Option<String>,
        }
        impl Builder {
            #[doc = "If the list of variables returned is not complete, a continuation token to query next batch of variables is included in the response header as \"x-ms-continuationtoken\". Omit this parameter to get the first batch of test variables."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/variables",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestVariableList =
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
        type Response = models::TestVariable;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestVariableCreateUpdateParameters,
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
                            "{}/{}/{}/_apis/testplan/variables",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
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
                                let rsp_value: models::TestVariable =
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
        type Response = models::TestVariable;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) test_variable_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/variables/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.test_variable_id
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
                                let rsp_value: models::TestVariable =
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
        type Response = models::TestVariable;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TestVariableCreateUpdateParameters,
            pub(crate) project: String,
            pub(crate) test_variable_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/variables/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.test_variable_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                                let rsp_value: models::TestVariable =
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
            pub(crate) project: String,
            pub(crate) test_variable_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/testplan/variables/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.test_variable_id
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
}
