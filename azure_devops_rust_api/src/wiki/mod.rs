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
    pub fn attachments_client(&self) -> attachments::Client {
        attachments::Client(self.clone())
    }
    pub fn page_moves_client(&self) -> page_moves::Client {
        page_moves::Client(self.clone())
    }
    pub fn page_stats_client(&self) -> page_stats::Client {
        page_stats::Client(self.clone())
    }
    pub fn pages_client(&self) -> pages::Client {
        pages::Client(self.clone())
    }
    pub fn pages_batch_client(&self) -> pages_batch::Client {
        pages_batch::Client(self.clone())
    }
    pub fn wikis_client(&self) -> wikis::Client {
        wikis::Client(self.clone())
    }
}
pub mod wikis {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all wikis in a project or collection."]
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
            }
        }
        #[doc = "Creates the wiki resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Parameters for the wiki creation."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiCreateParametersV2>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Gets the wiki corresponding to the wiki ID or wiki name provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            wiki_identifier: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                wiki_identifier: wiki_identifier.into(),
                project: project.into(),
            }
        }
        #[doc = "Updates the wiki corresponding to the wiki ID or wiki name provided using the update parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Update parameters."]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiUpdateParameters>,
            wiki_identifier: impl Into<String>,
            project: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                wiki_identifier: wiki_identifier.into(),
                project: project.into(),
            }
        }
        #[doc = "Deletes the wiki corresponding to the wiki ID or wiki name provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            wiki_identifier: impl Into<String>,
            project: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                wiki_identifier: wiki_identifier.into(),
                project: project.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WikiV2List;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                            "{}/{}/{}/_apis/wiki/wikis",
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiV2List =
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
        type Response = models::WikiV2;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WikiCreateParametersV2,
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
                            "{}/{}/{}/_apis/wiki/wikis",
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
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiV2 = serde_json::from_slice(&rsp_body)
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
        type Response = models::WikiV2;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) wiki_identifier: String,
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
                            "{}/{}/{}/_apis/wiki/wikis/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                                let rsp_value: models::WikiV2 = serde_json::from_slice(&rsp_body)
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
        type Response = models::WikiV2;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WikiUpdateParameters,
            pub(crate) wiki_identifier: String,
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
                            "{}/{}/{}/_apis/wiki/wikis/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiV2 = serde_json::from_slice(&rsp_body)
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
    pub mod delete {
        use super::models;
        type Response = models::WikiV2;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) wiki_identifier: String,
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
                            "{}/{}/{}/_apis/wiki/wikis/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiV2 = serde_json::from_slice(&rsp_body)
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
pub mod attachments {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Creates an attachment in the wiki."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Stream to upload"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `name`: Wiki attachment name."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                name: name.into(),
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::WikiAttachment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) name: String,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
        }
        impl Builder {
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
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
                            "{}/{}/{}/_apis/wiki/wikis/{}/attachments",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        let name = &this.name;
                        req.url_mut().query_pairs_mut().append_pair("name", name);
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                version_descriptor_version_options,
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
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiAttachment =
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
pub mod page_moves {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Creates a page move operation that updates the path and order of the page as provided in the parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Page more operation parameters."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPageMoveParameters>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                comment: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::WikiPageMove;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WikiPageMoveParameters,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) comment: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
        }
        impl Builder {
            #[doc = "Comment that is to be associated with this page move."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
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
                            "{}/{}/{}/_apis/wiki/wikis/{}/pagemoves",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                version_descriptor_version_options,
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
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiPageMove =
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
pub mod pages {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets metadata or content of the wiki page for the provided path. Content negotiation is done based on the `Accept` header sent in the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        pub fn get_page(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
        ) -> get_page::Builder {
            get_page::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                path: None,
                recursion_level: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
                include_content: None,
            }
        }
        #[doc = "Creates or edits a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Wiki create or update operation parameters."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `path`: Wiki page path."]
        #[doc = "* `version`: Version of the page on which the change is to be made. Mandatory for `Edit` scenario. To be populated in the If-Match header of the request."]
        pub fn create_or_update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPageCreateOrUpdateParameters>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            path: impl Into<String>,
            version: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                path: path.into(),
                version: version.into(),
                comment: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
        #[doc = "Deletes a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `path`: Wiki page path."]
        pub fn delete_page(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            path: impl Into<String>,
        ) -> delete_page::Builder {
            delete_page::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                path: path.into(),
                comment: None,
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
        #[doc = "Gets metadata or content of the wiki page for the provided page id. Content negotiation is done based on the `Accept` header sent in the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name.."]
        #[doc = "* `id`: Wiki page ID."]
        pub fn get_page_by_id(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            id: i32,
        ) -> get_page_by_id::Builder {
            get_page_by_id::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                id,
                recursion_level: None,
                include_content: None,
            }
        }
        #[doc = "Edits a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Wiki update operation parameters."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `id`: Wiki page ID."]
        #[doc = "* `version`: Version of the page on which the change is to be made. Mandatory for `Edit` scenario. To be populated in the If-Match header of the request."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPageCreateOrUpdateParameters>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            id: i32,
            version: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                id,
                version: version.into(),
                comment: None,
            }
        }
        #[doc = "Deletes a wiki page."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `id`: Wiki page ID."]
        pub fn delete_page_by_id(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            id: i32,
        ) -> delete_page_by_id::Builder {
            delete_page_by_id::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                id,
                comment: None,
            }
        }
    }
    pub mod get_page {
        use super::models;
        type Response = models::WikiPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) path: Option<String>,
            pub(crate) recursion_level: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
            pub(crate) include_content: Option<bool>,
        }
        impl Builder {
            #[doc = "Wiki page path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Recursion level for subpages retrieval. Defaults to `None` (Optional)."]
            pub fn recursion_level(mut self, recursion_level: impl Into<String>) -> Self {
                self.recursion_level = Some(recursion_level.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
            pub fn version_descriptor_version_type(
                mut self,
                version_descriptor_version_type: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_type = Some(version_descriptor_version_type.into());
                self
            }
            #[doc = "Set to true to include the content of the page in the response for Json content type. Defaults to false (Optional)"]
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
                            "{}/{}/{}/_apis/wiki/wikis/{}/pages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
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
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                version_descriptor_version_options,
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
                                let rsp_value: models::WikiPage = serde_json::from_slice(&rsp_body)
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Created201(models::WikiPage),
            Ok200(models::WikiPage),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WikiPageCreateOrUpdateParameters,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) path: String,
            pub(crate) version: String,
            pub(crate) comment: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
        }
        impl Builder {
            #[doc = "Comment to be associated with the page operation."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
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
                            "{}/{}/{}/_apis/wiki/wikis/{}/pages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                        let path = &this.path;
                        req.url_mut().query_pairs_mut().append_pair("path", path);
                        req.insert_header("version", &this.version);
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                version_descriptor_version_options,
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
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiPage = serde_json::from_slice(&rsp_body)
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
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiPage = serde_json::from_slice(&rsp_body)
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
                                Ok(Response::Ok200(rsp_value))
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
    pub mod delete_page {
        use super::models;
        type Response = models::WikiPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) path: String,
            pub(crate) comment: Option<String>,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
        }
        impl Builder {
            #[doc = "Comment to be associated with this page delete."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
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
                            "{}/{}/{}/_apis/wiki/wikis/{}/pages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                        let path = &this.path;
                        req.url_mut().query_pairs_mut().append_pair("path", path);
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                version_descriptor_version_options,
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
                                let rsp_value: models::WikiPage = serde_json::from_slice(&rsp_body)
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
    pub mod get_page_by_id {
        use super::models;
        type Response = models::WikiPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) id: i32,
            pub(crate) recursion_level: Option<String>,
            pub(crate) include_content: Option<bool>,
        }
        impl Builder {
            #[doc = "Recursion level for subpages retrieval. Defaults to `None` (Optional)."]
            pub fn recursion_level(mut self, recursion_level: impl Into<String>) -> Self {
                self.recursion_level = Some(recursion_level.into());
                self
            }
            #[doc = "Set to true to include the content of the page in the response for Json content type. Defaults to false (Optional)"]
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
                            "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier,
                            &this.id
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
                        if let Some(recursion_level) = &this.recursion_level {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("recursionLevel", recursion_level);
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
                                let rsp_value: models::WikiPage = serde_json::from_slice(&rsp_body)
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
        type Response = models::WikiPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WikiPageCreateOrUpdateParameters,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) id: i32,
            pub(crate) version: String,
            pub(crate) comment: Option<String>,
        }
        impl Builder {
            #[doc = "Comment to be associated with the page operation."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier,
                            &this.id
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
                        req.insert_header("version", &this.version);
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiPage = serde_json::from_slice(&rsp_body)
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
    pub mod delete_page_by_id {
        use super::models;
        type Response = models::WikiPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) id: i32,
            pub(crate) comment: Option<String>,
        }
        impl Builder {
            #[doc = "Comment to be associated with this page delete."]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier,
                            &this.id
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
                        if let Some(comment) = &this.comment {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("comment", comment);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiPage = serde_json::from_slice(&rsp_body)
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
pub mod page_stats {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns page detail corresponding to Page ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        #[doc = "* `page_id`: Wiki page ID."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
            page_id: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                page_id,
                page_views_for_days: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::WikiPageDetail;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) page_id: i32,
            pub(crate) page_views_for_days: Option<i32>,
        }
        impl Builder {
            #[doc = "last N days from the current day for which page views is to be returned. It's inclusive of current day."]
            pub fn page_views_for_days(mut self, page_views_for_days: i32) -> Self {
                self.page_views_for_days = Some(page_views_for_days);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wiki/wikis/{}/pages/{}/stats",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier,
                            &this.page_id
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
                        if let Some(page_views_for_days) = &this.page_views_for_days {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("pageViewsForDays", &page_views_for_days.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiPageDetail =
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
pub mod pages_batch {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns pageable list of Wiki Pages"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Wiki batch page request."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `wiki_identifier`: Wiki ID or wiki name."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiPagesBatchRequest>,
            project: impl Into<String>,
            wiki_identifier: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                wiki_identifier: wiki_identifier.into(),
                version_descriptor_version: None,
                version_descriptor_version_options: None,
                version_descriptor_version_type: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::WikiPageDetailList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WikiPagesBatchRequest,
            pub(crate) project: String,
            pub(crate) wiki_identifier: String,
            pub(crate) version_descriptor_version: Option<String>,
            pub(crate) version_descriptor_version_options: Option<String>,
            pub(crate) version_descriptor_version_type: Option<String>,
        }
        impl Builder {
            #[doc = "Version string identifier (name of tag/branch, SHA1 of commit)"]
            pub fn version_descriptor_version(
                mut self,
                version_descriptor_version: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version = Some(version_descriptor_version.into());
                self
            }
            #[doc = "Version options - Specify additional modifiers to version (e.g Previous)"]
            pub fn version_descriptor_version_options(
                mut self,
                version_descriptor_version_options: impl Into<String>,
            ) -> Self {
                self.version_descriptor_version_options =
                    Some(version_descriptor_version_options.into());
                self
            }
            #[doc = "Version type (branch, tag, or commit). Determines how Id is interpreted"]
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
                            "{}/{}/{}/_apis/wiki/wikis/{}/pagesbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.wiki_identifier
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
                        if let Some(version_descriptor_version) = &this.version_descriptor_version {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.version",
                                version_descriptor_version,
                            );
                        }
                        if let Some(version_descriptor_version_options) =
                            &this.version_descriptor_version_options
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "versionDescriptor.versionOptions",
                                version_descriptor_version_options,
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
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WikiPageDetailList =
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
