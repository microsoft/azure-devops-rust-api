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
    pub fn branches_client(&self) -> branches::Client {
        branches::Client(self.clone())
    }
    pub fn changesets_client(&self) -> changesets::Client {
        changesets::Client(self.clone())
    }
    pub fn items_client(&self) -> items::Client {
        items::Client(self.clone())
    }
    pub fn labels_client(&self) -> labels::Client {
        labels::Client(self.clone())
    }
    pub fn shelvesets_client(&self) -> shelvesets::Client {
        shelvesets::Client(self.clone())
    }
}
pub mod shelvesets {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return a collection of shallow shelveset references."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                request_data_include_details: None,
                request_data_include_links: None,
                request_data_include_work_items: None,
                request_data_max_change_count: None,
                request_data_max_comment_length: None,
                request_data_name: None,
                request_data_owner: None,
                top: None,
                skip: None,
            }
        }
        #[doc = "Get a single deep shelveset."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `shelveset_id`: Shelveset's unique ID"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            shelveset_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                shelveset_id: shelveset_id.into(),
                request_data_include_details: None,
                request_data_include_links: None,
                request_data_include_work_items: None,
                request_data_max_change_count: None,
                request_data_max_comment_length: None,
                request_data_name: None,
                request_data_owner: None,
            }
        }
        #[doc = "Get changes included in a shelveset."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `shelveset_id`: Shelveset's unique ID"]
        pub fn get_shelveset_changes(
            &self,
            organization: impl Into<String>,
            shelveset_id: impl Into<String>,
        ) -> get_shelveset_changes::Builder {
            get_shelveset_changes::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                shelveset_id: shelveset_id.into(),
                top: None,
                skip: None,
            }
        }
        #[doc = "Get work items associated with a shelveset."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `shelveset_id`: Shelveset's unique ID"]
        pub fn get_shelveset_work_items(
            &self,
            organization: impl Into<String>,
            shelveset_id: impl Into<String>,
        ) -> get_shelveset_work_items::Builder {
            get_shelveset_work_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                shelveset_id: shelveset_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TfvcShelvesetRefList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) request_data_include_details: Option<bool>,
            pub(crate) request_data_include_links: Option<bool>,
            pub(crate) request_data_include_work_items: Option<bool>,
            pub(crate) request_data_max_change_count: Option<i32>,
            pub(crate) request_data_max_comment_length: Option<i32>,
            pub(crate) request_data_name: Option<String>,
            pub(crate) request_data_owner: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
        }
        impl Builder {
            #[doc = "Whether to include policyOverride and notes Only applies when requesting a single deep shelveset"]
            pub fn request_data_include_details(
                mut self,
                request_data_include_details: bool,
            ) -> Self {
                self.request_data_include_details = Some(request_data_include_details);
                self
            }
            #[doc = "Whether to include the _links field on the shallow references. Does not apply when requesting a single deep shelveset object. Links will always be included in the deep shelveset."]
            pub fn request_data_include_links(mut self, request_data_include_links: bool) -> Self {
                self.request_data_include_links = Some(request_data_include_links);
                self
            }
            #[doc = "Whether to include workItems"]
            pub fn request_data_include_work_items(
                mut self,
                request_data_include_work_items: bool,
            ) -> Self {
                self.request_data_include_work_items = Some(request_data_include_work_items);
                self
            }
            #[doc = "Max number of changes to include"]
            pub fn request_data_max_change_count(
                mut self,
                request_data_max_change_count: i32,
            ) -> Self {
                self.request_data_max_change_count = Some(request_data_max_change_count);
                self
            }
            #[doc = "Max length of comment"]
            pub fn request_data_max_comment_length(
                mut self,
                request_data_max_comment_length: i32,
            ) -> Self {
                self.request_data_max_comment_length = Some(request_data_max_comment_length);
                self
            }
            #[doc = "Shelveset name"]
            pub fn request_data_name(mut self, request_data_name: impl Into<String>) -> Self {
                self.request_data_name = Some(request_data_name.into());
                self
            }
            #[doc = "Owner's ID. Could be a name or a guid."]
            pub fn request_data_owner(mut self, request_data_owner: impl Into<String>) -> Self {
                self.request_data_owner = Some(request_data_owner.into());
                self
            }
            #[doc = "Max number of shelvesets to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Number of shelvesets to skip"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/tfvc/shelvesets?",
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
                        if let Some(request_data_include_details) =
                            &this.request_data_include_details
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeDetails",
                                &request_data_include_details.to_string(),
                            );
                        }
                        if let Some(request_data_include_links) = &this.request_data_include_links {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeLinks",
                                &request_data_include_links.to_string(),
                            );
                        }
                        if let Some(request_data_include_work_items) =
                            &this.request_data_include_work_items
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeWorkItems",
                                &request_data_include_work_items.to_string(),
                            );
                        }
                        if let Some(request_data_max_change_count) =
                            &this.request_data_max_change_count
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.maxChangeCount",
                                &request_data_max_change_count.to_string(),
                            );
                        }
                        if let Some(request_data_max_comment_length) =
                            &this.request_data_max_comment_length
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.maxCommentLength",
                                &request_data_max_comment_length.to_string(),
                            );
                        }
                        if let Some(request_data_name) = &this.request_data_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.name", request_data_name);
                        }
                        if let Some(request_data_owner) = &this.request_data_owner {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.owner", request_data_owner);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcShelvesetRefList =
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
        type Response = models::TfvcShelveset;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) shelveset_id: String,
            pub(crate) request_data_include_details: Option<bool>,
            pub(crate) request_data_include_links: Option<bool>,
            pub(crate) request_data_include_work_items: Option<bool>,
            pub(crate) request_data_max_change_count: Option<i32>,
            pub(crate) request_data_max_comment_length: Option<i32>,
            pub(crate) request_data_name: Option<String>,
            pub(crate) request_data_owner: Option<String>,
        }
        impl Builder {
            #[doc = "Whether to include policyOverride and notes Only applies when requesting a single deep shelveset"]
            pub fn request_data_include_details(
                mut self,
                request_data_include_details: bool,
            ) -> Self {
                self.request_data_include_details = Some(request_data_include_details);
                self
            }
            #[doc = "Whether to include the _links field on the shallow references. Does not apply when requesting a single deep shelveset object. Links will always be included in the deep shelveset."]
            pub fn request_data_include_links(mut self, request_data_include_links: bool) -> Self {
                self.request_data_include_links = Some(request_data_include_links);
                self
            }
            #[doc = "Whether to include workItems"]
            pub fn request_data_include_work_items(
                mut self,
                request_data_include_work_items: bool,
            ) -> Self {
                self.request_data_include_work_items = Some(request_data_include_work_items);
                self
            }
            #[doc = "Max number of changes to include"]
            pub fn request_data_max_change_count(
                mut self,
                request_data_max_change_count: i32,
            ) -> Self {
                self.request_data_max_change_count = Some(request_data_max_change_count);
                self
            }
            #[doc = "Max length of comment"]
            pub fn request_data_max_comment_length(
                mut self,
                request_data_max_comment_length: i32,
            ) -> Self {
                self.request_data_max_comment_length = Some(request_data_max_comment_length);
                self
            }
            #[doc = "Shelveset name"]
            pub fn request_data_name(mut self, request_data_name: impl Into<String>) -> Self {
                self.request_data_name = Some(request_data_name.into());
                self
            }
            #[doc = "Owner's ID. Could be a name or a guid."]
            pub fn request_data_owner(mut self, request_data_owner: impl Into<String>) -> Self {
                self.request_data_owner = Some(request_data_owner.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/tfvc/shelvesets",
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
                        let shelveset_id = &this.shelveset_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("shelvesetId", shelveset_id);
                        if let Some(request_data_include_details) =
                            &this.request_data_include_details
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeDetails",
                                &request_data_include_details.to_string(),
                            );
                        }
                        if let Some(request_data_include_links) = &this.request_data_include_links {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeLinks",
                                &request_data_include_links.to_string(),
                            );
                        }
                        if let Some(request_data_include_work_items) =
                            &this.request_data_include_work_items
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeWorkItems",
                                &request_data_include_work_items.to_string(),
                            );
                        }
                        if let Some(request_data_max_change_count) =
                            &this.request_data_max_change_count
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.maxChangeCount",
                                &request_data_max_change_count.to_string(),
                            );
                        }
                        if let Some(request_data_max_comment_length) =
                            &this.request_data_max_comment_length
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.maxCommentLength",
                                &request_data_max_comment_length.to_string(),
                            );
                        }
                        if let Some(request_data_name) = &this.request_data_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.name", request_data_name);
                        }
                        if let Some(request_data_owner) = &this.request_data_owner {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.owner", request_data_owner);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcShelveset =
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
    pub mod get_shelveset_changes {
        use super::models;
        type Response = models::TfvcChangeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) shelveset_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
        }
        impl Builder {
            #[doc = "Max number of changes to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Number of changes to skip"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/tfvc/shelvesets/changes",
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
                        let shelveset_id = &this.shelveset_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("shelvesetId", shelveset_id);
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcChangeList =
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
    pub mod get_shelveset_work_items {
        use super::models;
        type Response = models::AssociatedWorkItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) shelveset_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/tfvc/shelvesets/workitems",
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
                        let shelveset_id = &this.shelveset_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("shelvesetId", shelveset_id);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssociatedWorkItemList =
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
pub mod branches {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a single branch hierarchy at the given path with parents or children as specified."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `path`: Full path to the branch.  Default: $/ Examples: $/, $/MyProject, $/MyProject/SomeFolder."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            path: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                path: path.into(),
                project: project.into(),
                include_parent: None,
                include_children: None,
            }
        }
        #[doc = "Get a collection of branch roots -- first-level children, branches with no parents."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_branches(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_branches::Builder {
            get_branches::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                include_parent: None,
                include_children: None,
                include_deleted: None,
                include_links: None,
            }
        }
        #[doc = "Get branch hierarchies below the specified scopePath"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `scope_path`: Full path to the branch.  Default: $/ Examples: $/, $/MyProject, $/MyProject/SomeFolder."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_branch_refs(
            &self,
            organization: impl Into<String>,
            scope_path: impl Into<String>,
            project: impl Into<String>,
        ) -> get_branch_refs::Builder {
            get_branch_refs::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                scope_path: scope_path.into(),
                project: project.into(),
                include_deleted: None,
                include_links: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TfvcBranch;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) path: String,
            pub(crate) project: String,
            pub(crate) include_parent: Option<bool>,
            pub(crate) include_children: Option<bool>,
        }
        impl Builder {
            #[doc = "Return the parent branch, if there is one. Default: False"]
            pub fn include_parent(mut self, include_parent: bool) -> Self {
                self.include_parent = Some(include_parent);
                self
            }
            #[doc = "Return child branches, if there are any. Default: False"]
            pub fn include_children(mut self, include_children: bool) -> Self {
                self.include_children = Some(include_children);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/branches?path={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.path
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
                        let path = &this.path;
                        req.url_mut().query_pairs_mut().append_pair("path", path);
                        if let Some(include_parent) = &this.include_parent {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeParent", &include_parent.to_string());
                        }
                        if let Some(include_children) = &this.include_children {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeChildren", &include_children.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcBranch =
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
    pub mod get_branches {
        use super::models;
        type Response = models::TfvcBranchList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) include_parent: Option<bool>,
            pub(crate) include_children: Option<bool>,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) include_links: Option<bool>,
        }
        impl Builder {
            #[doc = "Return the parent branch, if there is one. Default: False"]
            pub fn include_parent(mut self, include_parent: bool) -> Self {
                self.include_parent = Some(include_parent);
                self
            }
            #[doc = "Return the child branches for each root branch. Default: False"]
            pub fn include_children(mut self, include_children: bool) -> Self {
                self.include_children = Some(include_children);
                self
            }
            #[doc = "Return deleted branches. Default: False"]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "Return links. Default: False"]
            pub fn include_links(mut self, include_links: bool) -> Self {
                self.include_links = Some(include_links);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/branches?",
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
                        if let Some(include_parent) = &this.include_parent {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeParent", &include_parent.to_string());
                        }
                        if let Some(include_children) = &this.include_children {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeChildren", &include_children.to_string());
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(include_links) = &this.include_links {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeLinks", &include_links.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcBranchList =
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
    pub mod get_branch_refs {
        use super::models;
        type Response = models::TfvcBranchRefList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) scope_path: String,
            pub(crate) project: String,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) include_links: Option<bool>,
        }
        impl Builder {
            #[doc = "Return deleted branches. Default: False"]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "Return links. Default: False"]
            pub fn include_links(mut self, include_links: bool) -> Self {
                self.include_links = Some(include_links);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/branches",
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
                        let scope_path = &this.scope_path;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("scopePath", scope_path);
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(include_links) = &this.include_links {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeLinks", &include_links.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcBranchRefList =
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
pub mod items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get Item Metadata and/or Content for a single item. The download parameter is to indicate whether the content should be available as a download or just sent as a stream in the response. Doesn't apply to zipped content which is always returned as a download."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `path`: Version control path of an individual item to return."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            path: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                path: path.into(),
                project: project.into(),
                file_name: None,
                download: None,
                scope_path: None,
                recursion_level: None,
                version_descriptor_version: None,
                version_descriptor_version_option: None,
                version_descriptor_version_type: None,
                include_content: None,
            }
        }
        #[doc = "Post for retrieving a set of items given a list of paths or a long path. Allows for specifying the recursionLevel and version descriptors for each path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_items_batch(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TfvcItemRequestData>,
            project: impl Into<String>,
        ) -> get_items_batch::Builder {
            get_items_batch::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get a list of Tfvc items"]
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
                scope_path: None,
                recursion_level: None,
                include_links: None,
                version_descriptor_version: None,
                version_descriptor_version_option: None,
                version_descriptor_version_type: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TfvcItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) path: String,
            pub(crate) project: String,
            pub(crate) file_name: Option<String>,
            pub(crate) download: Option<bool>,
            pub(crate) scope_path: Option<String>,
            pub(crate) recursion_level: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_option: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
            pub(crate) include_content: Option<bool>,
        }
        impl Builder {
            #[doc = "File name of item returned."]
            pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
                self.file_name = Some(file_name.into());
                self
            }
            #[doc = "If true, create a downloadable attachment."]
            pub fn download(mut self, download: bool) -> Self {
                self.download = Some(download);
                self
            }
            #[doc = "Version control path of a folder to return multiple items."]
            pub fn scope_path(mut self, scope_path: impl Into<String>) -> Self {
                self.scope_path = Some(scope_path.into());
                self
            }
            #[doc = "None (just the item), or OneLevel (contents of a folder)."]
            pub fn recursion_level(mut self, recursion_level: impl Into<String>) -> Self {
                self.recursion_level = Some(recursion_level.into());
                self
            }
            #[doc = "Version object."]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version descriptor.  Default is null."]
            pub fn version_descriptor_version_option(
                mut self,
                version_descriptor_version_option: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_option =
                    Some(version_descriptor_version_option.into());
                self
            }
            #[doc = "Version descriptor.  Default is null."]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
                self
            }
            #[doc = "Set to true to include item content when requesting json.  Default is false."]
            pub fn include_content(mut self, include_content: bool) -> Self {
                self.include_content = Some(include_content);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/items?path={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.path
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
                        let path = &this.path;
                        req.url_mut().query_pairs_mut().append_pair("path", path);
                        if let Some(file_name) = &this.file_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fileName", file_name);
                        }
                        if let Some(download) = &this.download {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("download", &download.to_string());
                        }
                        if let Some(scope_path) = &this.scope_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("scopePath", scope_path);
                        }
                        if let Some(recursion_level) = &this.recursion_level {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("recursionLevel", recursion_level);
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_option) =
                            &this.version_descriptor_version_option
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOption",
                                version_descriptor_version_option,
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                version_descriptor_version_type,
                            );
                        }
                        if let Some(include_content) = &this.include_content {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeContent", &include_content.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcItem =
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
    pub mod get_items_batch {
        use super::models;
        type Response = Vec<String>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TfvcItemRequestData,
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
                            "{}/{}/{}/_apis/tfvc/itembatch",
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
                                let rsp_value: Vec<String> = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::TfvcItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) scope_path: Option<String>,
            pub(crate) recursion_level: Option<String>,
            pub(crate) include_links: Option<bool>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_option: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
        }
        impl Builder {
            #[doc = "Version control path of a folder to return multiple items."]
            pub fn scope_path(mut self, scope_path: impl Into<String>) -> Self {
                self.scope_path = Some(scope_path.into());
                self
            }
            #[doc = "None (just the item), or OneLevel (contents of a folder)."]
            pub fn recursion_level(mut self, recursion_level: impl Into<String>) -> Self {
                self.recursion_level = Some(recursion_level.into());
                self
            }
            #[doc = "Set to true to include links."]
            pub fn include_links(mut self, include_links: bool) -> Self {
                self.include_links = Some(include_links);
                self
            }
            #[doc = "Version object."]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            pub fn version_descriptor_version_option(
                mut self,
                version_descriptor_version_option: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_option =
                    Some(version_descriptor_version_option.into());
                self
            }
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/items",
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
                        if let Some(scope_path) = &this.scope_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("scopePath", scope_path);
                        }
                        if let Some(recursion_level) = &this.recursion_level {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("recursionLevel", recursion_level);
                        }
                        if let Some(include_links) = &this.include_links {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeLinks", &include_links.to_string());
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_option) =
                            &this.version_descriptor_version_option
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOption",
                                version_descriptor_version_option,
                            );
                        }
                        if let Some(version_descriptor_version_type) =
                            &this.version_descriptor_version_type
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionType",
                                version_descriptor_version_type,
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcItemList =
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
pub mod changesets {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieve Tfvc changes for a given changeset."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: ID of the changeset. Default: null"]
        pub fn get_changeset_changes(
            &self,
            organization: impl Into<String>,
            id: i32,
        ) -> get_changeset_changes::Builder {
            get_changeset_changes::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                skip: None,
                top: None,
                continuation_token: None,
            }
        }
        #[doc = "Retrieves the work items associated with a particular changeset."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: ID of the changeset."]
        pub fn get_changeset_work_items(
            &self,
            organization: impl Into<String>,
            id: i32,
        ) -> get_changeset_work_items::Builder {
            get_changeset_work_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
            }
        }
        #[doc = "Returns changesets for a given list of changeset Ids."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: List of changeset IDs."]
        pub fn get_batched_changesets(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TfvcChangesetsRequestData>,
        ) -> get_batched_changesets::Builder {
            get_batched_changesets::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Retrieve Tfvc Changesets\n\nNote: This is a new version of the GetChangesets API that doesn't expose the unneeded queryParams\npresent in the 1.0 version of the API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_changesets(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_changesets::Builder {
            get_changesets::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                max_comment_length: None,
                skip: None,
                top: None,
                orderby: None,
                search_criteria_author: None,
                search_criteria_follow_renames: None,
                search_criteria_from_date: None,
                search_criteria_from_id: None,
                search_criteria_include_links: None,
                search_criteria_item_path: None,
                search_criteria_mappings: Vec::new(),
                search_criteria_to_date: None,
                search_criteria_to_id: None,
            }
        }
        #[doc = "Create a new changeset.\n\nAccepts TfvcChangeset as JSON body"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TfvcChangeset>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Retrieve a Tfvc Changeset"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: Changeset Id to retrieve."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            id: i32,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                project: project.into(),
                max_change_count: None,
                include_details: None,
                include_work_items: None,
                max_comment_length: None,
                include_source_rename: None,
                skip: None,
                top: None,
                orderby: None,
                search_criteria_author: None,
                search_criteria_follow_renames: None,
                search_criteria_from_date: None,
                search_criteria_from_id: None,
                search_criteria_include_links: None,
                search_criteria_item_path: None,
                search_criteria_mappings: Vec::new(),
                search_criteria_to_date: None,
                search_criteria_to_id: None,
            }
        }
    }
    pub mod get_changeset_changes {
        use super::models;
        type Response = models::TfvcChangeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) skip: Option<i32>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<String>,
        }
        impl Builder {
            #[doc = "Number of results to skip. Default: null"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "The maximum number of results to return. Default: null"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Return the next page of results. Default: null"]
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
                            "{}/{}/_apis/tfvc/changesets/{}/changes",
                            this.client.endpoint(),
                            &this.organization,
                            &this.id
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
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
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
                                let rsp_value: models::TfvcChangeList =
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
    pub mod get_changeset_work_items {
        use super::models;
        type Response = models::AssociatedWorkItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/tfvc/changesets/{}/workItems",
                            this.client.endpoint(),
                            &this.organization,
                            &this.id
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
                                let rsp_value: models::AssociatedWorkItemList =
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
    pub mod get_batched_changesets {
        use super::models;
        type Response = models::TfvcChangesetRefList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TfvcChangesetsRequestData,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/tfvc/changesetsbatch",
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
                                let rsp_value: models::TfvcChangesetRefList =
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
    pub mod get_changesets {
        use super::models;
        type Response = models::TfvcChangesetRefList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) max_comment_length: Option<i32>,
            pub(crate) skip: Option<i32>,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) search_criteria_author: Option<String>,
            pub(crate) search_criteria_follow_renames: Option<bool>,
            pub(crate) search_criteria_from_date: Option<String>,
            pub(crate) search_criteria_from_id: Option<i32>,
            pub(crate) search_criteria_include_links: Option<bool>,
            pub(crate) search_criteria_item_path: Option<String>,
            pub(crate) search_criteria_mappings: Vec<models::TfvcMappingFilter>,
            pub(crate) search_criteria_to_date: Option<String>,
            pub(crate) search_criteria_to_id: Option<i32>,
        }
        impl Builder {
            #[doc = "Include details about associated work items in the response. Default: null"]
            pub fn max_comment_length(mut self, max_comment_length: i32) -> Self {
                self.max_comment_length = Some(max_comment_length);
                self
            }
            #[doc = "Number of results to skip. Default: null"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "The maximum number of results to return. Default: null"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Results are sorted by ID in descending order by default. Use id asc to sort by ID in ascending order."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Alias or display name of user who made the changes."]
            pub fn search_criteria_author(
                mut self,
                search_criteria_author: impl Into<String>,
            ) -> Self {
                self.search_criteria_author = Some(search_criteria_author.into());
                self
            }
            #[doc = "Whether or not to follow renames for the given item being queried."]
            pub fn search_criteria_follow_renames(
                mut self,
                search_criteria_follow_renames: bool,
            ) -> Self {
                self.search_criteria_follow_renames = Some(search_criteria_follow_renames);
                self
            }
            #[doc = "If provided, only include changesets created after this date (string)."]
            pub fn search_criteria_from_date(
                mut self,
                search_criteria_from_date: impl Into<String>,
            ) -> Self {
                self.search_criteria_from_date = Some(search_criteria_from_date.into());
                self
            }
            #[doc = "If provided, only include changesets after this changesetID."]
            pub fn search_criteria_from_id(mut self, search_criteria_from_id: i32) -> Self {
                self.search_criteria_from_id = Some(search_criteria_from_id);
                self
            }
            #[doc = "Whether to include the _links field on the shallow references."]
            pub fn search_criteria_include_links(
                mut self,
                search_criteria_include_links: bool,
            ) -> Self {
                self.search_criteria_include_links = Some(search_criteria_include_links);
                self
            }
            #[doc = "Path of item to search under."]
            pub fn search_criteria_item_path(
                mut self,
                search_criteria_item_path: impl Into<String>,
            ) -> Self {
                self.search_criteria_item_path = Some(search_criteria_item_path.into());
                self
            }
            #[doc = "Following criteria available (.itemPath, .version, .versionType, .versionOption, .author, .fromId, .toId, .fromDate, .toDate) Default: null"]
            pub fn search_criteria_mappings(
                mut self,
                search_criteria_mappings: Vec<models::TfvcMappingFilter>,
            ) -> Self {
                self.search_criteria_mappings = search_criteria_mappings;
                self
            }
            #[doc = "If provided, only include changesets created before this date (string)."]
            pub fn search_criteria_to_date(
                mut self,
                search_criteria_to_date: impl Into<String>,
            ) -> Self {
                self.search_criteria_to_date = Some(search_criteria_to_date.into());
                self
            }
            #[doc = "If provided, a version descriptor for the latest change list to include."]
            pub fn search_criteria_to_id(mut self, search_criteria_to_id: i32) -> Self {
                self.search_criteria_to_id = Some(search_criteria_to_id);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/changesets",
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
                        if let Some(max_comment_length) = &this.max_comment_length {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("maxCommentLength", &max_comment_length.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(orderby) = &this.orderby {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$orderby", orderby);
                        }
                        if let Some(search_criteria_author) = &this.search_criteria_author {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.author", search_criteria_author);
                        }
                        if let Some(search_criteria_follow_renames) =
                            &this.search_criteria_follow_renames
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.followRenames",
                                &search_criteria_follow_renames.to_string(),
                            );
                        }
                        if let Some(search_criteria_from_date) = &this.search_criteria_from_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.fromDate", search_criteria_from_date);
                        }
                        if let Some(search_criteria_from_id) = &this.search_criteria_from_id {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.fromId",
                                &search_criteria_from_id.to_string(),
                            );
                        }
                        if let Some(search_criteria_include_links) =
                            &this.search_criteria_include_links
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.includeLinks",
                                &search_criteria_include_links.to_string(),
                            );
                        }
                        if let Some(search_criteria_item_path) = &this.search_criteria_item_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.itemPath", search_criteria_item_path);
                        }
                        if let Some(search_criteria_to_date) = &this.search_criteria_to_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.toDate", search_criteria_to_date);
                        }
                        if let Some(search_criteria_to_id) = &this.search_criteria_to_id {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.toId",
                                &search_criteria_to_id.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcChangesetRefList =
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
        type Response = models::TfvcChangesetRef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TfvcChangeset,
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
                            "{}/{}/{}/_apis/tfvc/changesets",
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
                                let rsp_value: models::TfvcChangesetRef =
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
        type Response = models::TfvcChangeset;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) project: String,
            pub(crate) max_change_count: Option<i32>,
            pub(crate) include_details: Option<bool>,
            pub(crate) include_work_items: Option<bool>,
            pub(crate) max_comment_length: Option<i32>,
            pub(crate) include_source_rename: Option<bool>,
            pub(crate) skip: Option<i32>,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) search_criteria_author: Option<String>,
            pub(crate) search_criteria_follow_renames: Option<bool>,
            pub(crate) search_criteria_from_date: Option<String>,
            pub(crate) search_criteria_from_id: Option<i32>,
            pub(crate) search_criteria_include_links: Option<bool>,
            pub(crate) search_criteria_item_path: Option<String>,
            pub(crate) search_criteria_mappings: Vec<models::TfvcMappingFilter>,
            pub(crate) search_criteria_to_date: Option<String>,
            pub(crate) search_criteria_to_id: Option<i32>,
        }
        impl Builder {
            #[doc = "Number of changes to return (maximum 100 changes) Default: 0"]
            pub fn max_change_count(mut self, max_change_count: i32) -> Self {
                self.max_change_count = Some(max_change_count);
                self
            }
            #[doc = "Include policy details and check-in notes in the response. Default: false"]
            pub fn include_details(mut self, include_details: bool) -> Self {
                self.include_details = Some(include_details);
                self
            }
            #[doc = "Include workitems. Default: false"]
            pub fn include_work_items(mut self, include_work_items: bool) -> Self {
                self.include_work_items = Some(include_work_items);
                self
            }
            #[doc = "Include details about associated work items in the response. Default: null"]
            pub fn max_comment_length(mut self, max_comment_length: i32) -> Self {
                self.max_comment_length = Some(max_comment_length);
                self
            }
            #[doc = "Include renames.  Default: false"]
            pub fn include_source_rename(mut self, include_source_rename: bool) -> Self {
                self.include_source_rename = Some(include_source_rename);
                self
            }
            #[doc = "Number of results to skip. Default: null"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "The maximum number of results to return. Default: null"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Results are sorted by ID in descending order by default. Use id asc to sort by ID in ascending order."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Alias or display name of user who made the changes."]
            pub fn search_criteria_author(
                mut self,
                search_criteria_author: impl Into<String>,
            ) -> Self {
                self.search_criteria_author = Some(search_criteria_author.into());
                self
            }
            #[doc = "Whether or not to follow renames for the given item being queried."]
            pub fn search_criteria_follow_renames(
                mut self,
                search_criteria_follow_renames: bool,
            ) -> Self {
                self.search_criteria_follow_renames = Some(search_criteria_follow_renames);
                self
            }
            #[doc = "If provided, only include changesets created after this date (string)."]
            pub fn search_criteria_from_date(
                mut self,
                search_criteria_from_date: impl Into<String>,
            ) -> Self {
                self.search_criteria_from_date = Some(search_criteria_from_date.into());
                self
            }
            #[doc = "If provided, only include changesets after this changesetID."]
            pub fn search_criteria_from_id(mut self, search_criteria_from_id: i32) -> Self {
                self.search_criteria_from_id = Some(search_criteria_from_id);
                self
            }
            #[doc = "Whether to include the _links field on the shallow references."]
            pub fn search_criteria_include_links(
                mut self,
                search_criteria_include_links: bool,
            ) -> Self {
                self.search_criteria_include_links = Some(search_criteria_include_links);
                self
            }
            #[doc = "Path of item to search under."]
            pub fn search_criteria_item_path(
                mut self,
                search_criteria_item_path: impl Into<String>,
            ) -> Self {
                self.search_criteria_item_path = Some(search_criteria_item_path.into());
                self
            }
            #[doc = "Following criteria available (.itemPath, .version, .versionType, .versionOption, .author, .fromId, .toId, .fromDate, .toDate) Default: null"]
            pub fn search_criteria_mappings(
                mut self,
                search_criteria_mappings: Vec<models::TfvcMappingFilter>,
            ) -> Self {
                self.search_criteria_mappings = search_criteria_mappings;
                self
            }
            #[doc = "If provided, only include changesets created before this date (string)."]
            pub fn search_criteria_to_date(
                mut self,
                search_criteria_to_date: impl Into<String>,
            ) -> Self {
                self.search_criteria_to_date = Some(search_criteria_to_date.into());
                self
            }
            #[doc = "If provided, a version descriptor for the latest change list to include."]
            pub fn search_criteria_to_id(mut self, search_criteria_to_id: i32) -> Self {
                self.search_criteria_to_id = Some(search_criteria_to_id);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/changesets/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
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
                        if let Some(max_change_count) = &this.max_change_count {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("maxChangeCount", &max_change_count.to_string());
                        }
                        if let Some(include_details) = &this.include_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDetails", &include_details.to_string());
                        }
                        if let Some(include_work_items) = &this.include_work_items {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeWorkItems", &include_work_items.to_string());
                        }
                        if let Some(max_comment_length) = &this.max_comment_length {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("maxCommentLength", &max_comment_length.to_string());
                        }
                        if let Some(include_source_rename) = &this.include_source_rename {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeSourceRename",
                                &include_source_rename.to_string(),
                            );
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(orderby) = &this.orderby {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$orderby", orderby);
                        }
                        if let Some(search_criteria_author) = &this.search_criteria_author {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.author", search_criteria_author);
                        }
                        if let Some(search_criteria_follow_renames) =
                            &this.search_criteria_follow_renames
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.followRenames",
                                &search_criteria_follow_renames.to_string(),
                            );
                        }
                        if let Some(search_criteria_from_date) = &this.search_criteria_from_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.fromDate", search_criteria_from_date);
                        }
                        if let Some(search_criteria_from_id) = &this.search_criteria_from_id {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.fromId",
                                &search_criteria_from_id.to_string(),
                            );
                        }
                        if let Some(search_criteria_include_links) =
                            &this.search_criteria_include_links
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.includeLinks",
                                &search_criteria_include_links.to_string(),
                            );
                        }
                        if let Some(search_criteria_item_path) = &this.search_criteria_item_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.itemPath", search_criteria_item_path);
                        }
                        if let Some(search_criteria_to_date) = &this.search_criteria_to_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("searchCriteria.toDate", search_criteria_to_date);
                        }
                        if let Some(search_criteria_to_id) = &this.search_criteria_to_id {
                            req.url_mut().query_pairs_mut().append_pair(
                                "searchCriteria.toId",
                                &search_criteria_to_id.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcChangeset =
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
pub mod labels {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get items under a label."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `label_id`: Unique identifier of label"]
        pub fn get_label_items(
            &self,
            organization: impl Into<String>,
            label_id: impl Into<String>,
        ) -> get_label_items::Builder {
            get_label_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                label_id: label_id.into(),
                top: None,
                skip: None,
            }
        }
        #[doc = "Get a collection of shallow label references."]
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
                request_data_include_links: None,
                request_data_item_label_filter: None,
                request_data_label_scope: None,
                request_data_max_item_count: None,
                request_data_name: None,
                request_data_owner: None,
                top: None,
                skip: None,
            }
        }
        #[doc = "Get a single deep label."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `label_id`: Unique identifier of label"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            label_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                label_id: label_id.into(),
                project: project.into(),
                request_data_include_links: None,
                request_data_item_label_filter: None,
                request_data_label_scope: None,
                request_data_max_item_count: None,
                request_data_name: None,
                request_data_owner: None,
            }
        }
    }
    pub mod get_label_items {
        use super::models;
        type Response = models::TfvcItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) label_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
        }
        impl Builder {
            #[doc = "Max number of items to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Number of items to skip"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/tfvc/labels/{}/items",
                            this.client.endpoint(),
                            &this.organization,
                            &this.label_id
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
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcItemList =
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
        type Response = models::TfvcLabelRefList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) request_data_include_links: Option<bool>,
            pub(crate) request_data_item_label_filter: Option<String>,
            pub(crate) request_data_label_scope: Option<String>,
            pub(crate) request_data_max_item_count: Option<i32>,
            pub(crate) request_data_name: Option<String>,
            pub(crate) request_data_owner: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
        }
        impl Builder {
            #[doc = "Whether to include the _links field on the shallow references"]
            pub fn request_data_include_links(mut self, request_data_include_links: bool) -> Self {
                self.request_data_include_links = Some(request_data_include_links);
                self
            }
            #[doc = "labelScope, name, owner, and itemLabelFilter"]
            pub fn request_data_item_label_filter(
                mut self,
                request_data_item_label_filter: impl Into<String>,
            ) -> Self {
                self.request_data_item_label_filter = Some(request_data_item_label_filter.into());
                self
            }
            #[doc = "labelScope, name, owner, and itemLabelFilter"]
            pub fn request_data_label_scope(
                mut self,
                request_data_label_scope: impl Into<String>,
            ) -> Self {
                self.request_data_label_scope = Some(request_data_label_scope.into());
                self
            }
            #[doc = "labelScope, name, owner, and itemLabelFilter"]
            pub fn request_data_max_item_count(mut self, request_data_max_item_count: i32) -> Self {
                self.request_data_max_item_count = Some(request_data_max_item_count);
                self
            }
            #[doc = "labelScope, name, owner, and itemLabelFilter"]
            pub fn request_data_name(mut self, request_data_name: impl Into<String>) -> Self {
                self.request_data_name = Some(request_data_name.into());
                self
            }
            #[doc = "labelScope, name, owner, and itemLabelFilter"]
            pub fn request_data_owner(mut self, request_data_owner: impl Into<String>) -> Self {
                self.request_data_owner = Some(request_data_owner.into());
                self
            }
            #[doc = "Max number of labels to return, defaults to 100 when undefined"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Number of labels to skip"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/labels",
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
                        if let Some(request_data_include_links) = &this.request_data_include_links {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeLinks",
                                &request_data_include_links.to_string(),
                            );
                        }
                        if let Some(request_data_item_label_filter) =
                            &this.request_data_item_label_filter
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.itemLabelFilter",
                                request_data_item_label_filter,
                            );
                        }
                        if let Some(request_data_label_scope) = &this.request_data_label_scope {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.labelScope", request_data_label_scope);
                        }
                        if let Some(request_data_max_item_count) = &this.request_data_max_item_count
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.maxItemCount",
                                &request_data_max_item_count.to_string(),
                            );
                        }
                        if let Some(request_data_name) = &this.request_data_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.name", request_data_name);
                        }
                        if let Some(request_data_owner) = &this.request_data_owner {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.owner", request_data_owner);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcLabelRefList =
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
        type Response = models::TfvcLabel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) label_id: String,
            pub(crate) project: String,
            pub(crate) request_data_include_links: Option<bool>,
            pub(crate) request_data_item_label_filter: Option<String>,
            pub(crate) request_data_label_scope: Option<String>,
            pub(crate) request_data_max_item_count: Option<i32>,
            pub(crate) request_data_name: Option<String>,
            pub(crate) request_data_owner: Option<String>,
        }
        impl Builder {
            #[doc = "Whether to include the _links field on the shallow references"]
            pub fn request_data_include_links(mut self, request_data_include_links: bool) -> Self {
                self.request_data_include_links = Some(request_data_include_links);
                self
            }
            #[doc = "maxItemCount"]
            pub fn request_data_item_label_filter(
                mut self,
                request_data_item_label_filter: impl Into<String>,
            ) -> Self {
                self.request_data_item_label_filter = Some(request_data_item_label_filter.into());
                self
            }
            #[doc = "maxItemCount"]
            pub fn request_data_label_scope(
                mut self,
                request_data_label_scope: impl Into<String>,
            ) -> Self {
                self.request_data_label_scope = Some(request_data_label_scope.into());
                self
            }
            #[doc = "maxItemCount"]
            pub fn request_data_max_item_count(mut self, request_data_max_item_count: i32) -> Self {
                self.request_data_max_item_count = Some(request_data_max_item_count);
                self
            }
            #[doc = "maxItemCount"]
            pub fn request_data_name(mut self, request_data_name: impl Into<String>) -> Self {
                self.request_data_name = Some(request_data_name.into());
                self
            }
            #[doc = "maxItemCount"]
            pub fn request_data_owner(mut self, request_data_owner: impl Into<String>) -> Self {
                self.request_data_owner = Some(request_data_owner.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/tfvc/labels/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.label_id
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
                        if let Some(request_data_include_links) = &this.request_data_include_links {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.includeLinks",
                                &request_data_include_links.to_string(),
                            );
                        }
                        if let Some(request_data_item_label_filter) =
                            &this.request_data_item_label_filter
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.itemLabelFilter",
                                request_data_item_label_filter,
                            );
                        }
                        if let Some(request_data_label_scope) = &this.request_data_label_scope {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.labelScope", request_data_label_scope);
                        }
                        if let Some(request_data_max_item_count) = &this.request_data_max_item_count
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "requestData.maxItemCount",
                                &request_data_max_item_count.to_string(),
                            );
                        }
                        if let Some(request_data_name) = &this.request_data_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.name", request_data_name);
                        }
                        if let Some(request_data_owner) = &this.request_data_owner {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("requestData.owner", request_data_owner);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TfvcLabel =
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
