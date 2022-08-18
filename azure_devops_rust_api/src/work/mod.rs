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
    pub fn backlogconfiguration_client(&self) -> backlogconfiguration::Client {
        backlogconfiguration::Client(self.clone())
    }
    pub fn backlogs_client(&self) -> backlogs::Client {
        backlogs::Client(self.clone())
    }
    pub fn boardcolumns_client(&self) -> boardcolumns::Client {
        boardcolumns::Client(self.clone())
    }
    pub fn boardparents_client(&self) -> boardparents::Client {
        boardparents::Client(self.clone())
    }
    pub fn boardrows_client(&self) -> boardrows::Client {
        boardrows::Client(self.clone())
    }
    pub fn boards_client(&self) -> boards::Client {
        boards::Client(self.clone())
    }
    pub fn boardusersettings_client(&self) -> boardusersettings::Client {
        boardusersettings::Client(self.clone())
    }
    pub fn capacities_client(&self) -> capacities::Client {
        capacities::Client(self.clone())
    }
    pub fn cardrulesettings_client(&self) -> cardrulesettings::Client {
        cardrulesettings::Client(self.clone())
    }
    pub fn cardsettings_client(&self) -> cardsettings::Client {
        cardsettings::Client(self.clone())
    }
    pub fn chartimages_client(&self) -> chartimages::Client {
        chartimages::Client(self.clone())
    }
    pub fn charts_client(&self) -> charts::Client {
        charts::Client(self.clone())
    }
    pub fn columns_client(&self) -> columns::Client {
        columns::Client(self.clone())
    }
    pub fn deliverytimeline_client(&self) -> deliverytimeline::Client {
        deliverytimeline::Client(self.clone())
    }
    pub fn iterationcapacities_client(&self) -> iterationcapacities::Client {
        iterationcapacities::Client(self.clone())
    }
    pub fn iterations_client(&self) -> iterations::Client {
        iterations::Client(self.clone())
    }
    pub fn plans_client(&self) -> plans::Client {
        plans::Client(self.clone())
    }
    pub fn processconfiguration_client(&self) -> processconfiguration::Client {
        processconfiguration::Client(self.clone())
    }
    pub fn rows_client(&self) -> rows::Client {
        rows::Client(self.clone())
    }
    pub fn taskboard_columns_client(&self) -> taskboard_columns::Client {
        taskboard_columns::Client(self.clone())
    }
    pub fn taskboard_work_items_client(&self) -> taskboard_work_items::Client {
        taskboard_work_items::Client(self.clone())
    }
    pub fn teamdaysoff_client(&self) -> teamdaysoff::Client {
        teamdaysoff::Client(self.clone())
    }
    pub fn teamfieldvalues_client(&self) -> teamfieldvalues::Client {
        teamfieldvalues::Client(self.clone())
    }
    pub fn teamsettings_client(&self) -> teamsettings::Client {
        teamsettings::Client(self.clone())
    }
    pub fn workitemsorder_client(&self) -> workitemsorder::Client {
        workitemsorder::Client(self.clone())
    }
}
pub mod boardcolumns {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get available board columns in a project"]
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
    }
    pub mod list {
        use super::models;
        type Response = models::BoardSuggestedValueList;
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
                            "{}/{}/{}/_apis/work/boardcolumns",
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BoardSuggestedValueList =
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
pub mod boardrows {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get available board rows in a project"]
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
    }
    pub mod list {
        use super::models;
        type Response = models::BoardSuggestedValueList;
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
                            "{}/{}/{}/_apis/work/boardrows",
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BoardSuggestedValueList =
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
pub mod iterationcapacities {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get an iteration's capacity for all teams in iteration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                iteration_id: iteration_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::IterationCapacity;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/work/iterations/{}/iterationcapacities",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.iteration_id
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
                                let rsp_value: models::IterationCapacity =
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
pub mod plans {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the information for all the plans configured for the given team"]
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
        #[doc = "Add a new plan for the team"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Plan definition"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CreatePlan>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the information for the specified plan"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: Identifier of the plan"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id: id.into(),
            }
        }
        #[doc = "Update the information for the specified plan"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Plan definition to be updated"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: Identifier of the plan"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdatePlan>,
            project: impl Into<String>,
            id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                id: id.into(),
            }
        }
        #[doc = "Delete the specified plan"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: Identifier of the plan"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id: id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PlanList;
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
                            "{}/{}/{}/_apis/work/plans",
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PlanList =
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
        type Response = models::Plan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CreatePlan,
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
                            "{}/{}/{}/_apis/work/plans",
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
                                let rsp_value: models::Plan = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Plan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/work/plans/{}",
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Plan = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Plan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::UpdatePlan,
            pub(crate) project: String,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/work/plans/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
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
                                let rsp_value: models::Plan = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/work/plans/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
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
pub mod deliverytimeline {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get Delivery View Data"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: Identifier for delivery view"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id: id.into(),
                revision: None,
                start_date: None,
                end_date: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::DeliveryViewData;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) id: String,
            pub(crate) revision: Option<i32>,
            pub(crate) start_date: Option<time::OffsetDateTime>,
            pub(crate) end_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "Revision of the plan for which you want data. If the current plan is a different revision you will get an ViewRevisionMismatchException exception. If you do not supply a revision you will get data for the latest revision."]
            pub fn revision(mut self, revision: i32) -> Self {
                self.revision = Some(revision);
                self
            }
            #[doc = "The start date of timeline"]
            pub fn start_date(mut self, start_date: impl Into<time::OffsetDateTime>) -> Self {
                self.start_date = Some(start_date.into());
                self
            }
            #[doc = "The end date of timeline"]
            pub fn end_date(mut self, end_date: impl Into<time::OffsetDateTime>) -> Self {
                self.end_date = Some(end_date.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/work/plans/{}/deliverytimeline",
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
                        if let Some(revision) = &this.revision {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("revision", &revision.to_string());
                        }
                        if let Some(start_date) = &this.start_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("startDate", &start_date.to_string());
                        }
                        if let Some(end_date) = &this.end_date {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("endDate", &end_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeliveryViewData =
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
pub mod processconfiguration {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get process configuration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ProcessConfiguration;
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
                            "{}/{}/{}/_apis/work/processconfiguration",
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProcessConfiguration =
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
pub mod backlogconfiguration {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets backlog configuration for a team"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BacklogConfiguration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/backlogconfiguration",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::BacklogConfiguration =
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
pub mod backlogs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List all backlog levels"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Get a list of work items within a backlog level"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get_backlog_level_work_items(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            backlog_id: impl Into<String>,
        ) -> get_backlog_level_work_items::Builder {
            get_backlog_level_work_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                backlog_id: backlog_id.into(),
            }
        }
        #[doc = "Get a backlog level"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `id`: The id of the backlog level"]
        pub fn get_backlog(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            id: impl Into<String>,
        ) -> get_backlog::Builder {
            get_backlog::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                id: id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BacklogLevelConfigurationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/backlogs",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::BacklogLevelConfigurationList =
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
    pub mod get_backlog_level_work_items {
        use super::models;
        type Response = models::BacklogLevelWorkItems;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) backlog_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/backlogs/{}/workItems",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.backlog_id
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
                                let rsp_value: models::BacklogLevelWorkItems =
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
    pub mod get_backlog {
        use super::models;
        type Response = models::BacklogLevelConfiguration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/backlogs/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
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
                                let rsp_value: models::BacklogLevelConfiguration =
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
pub mod boards {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get boards"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Get board"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: identifier for board, either board's backlog level name (Eg:\"Stories\") or Id"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id: id.into(),
                team: team.into(),
            }
        }
        #[doc = "Update board options"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: options to updated"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: identifier for board, either category plural name (Eg:\"Stories\") or guid"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn set_board_options(
            &self,
            organization: impl Into<String>,
            body: impl Into<serde_json::Value>,
            project: impl Into<String>,
            id: impl Into<String>,
            team: impl Into<String>,
        ) -> set_board_options::Builder {
            set_board_options::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                id: id.into(),
                team: team.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BoardReferenceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::BoardReferenceList =
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
        type Response = models::Board;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
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
                                let rsp_value: models::Board = serde_json::from_slice(&rsp_body)?;
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
    pub mod set_board_options {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: serde_json::Value,
            pub(crate) project: String,
            pub(crate) id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.id
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
                                let rsp_value: serde_json::Value =
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
pub mod boardusersettings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get board user settings for a board id"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Board ID or Name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Update board user settings for the board id\n\nWe don't want stakeholders to update board settings (currently just autorefresh). The BacklogManagement feature check validates this."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<serde_json::Value>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BoardUserSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/boardusersettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardUserSettings =
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
        type Response = models::BoardUserSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: serde_json::Value,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/boardusersettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardUserSettings =
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
pub mod cardrulesettings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get board card Rule settings for the board id or board by name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Update board card Rule settings for the board id or board by name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update_board_card_rule_settings(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::BoardCardRuleSettings>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> update_board_card_rule_settings::Builder {
            update_board_card_rule_settings::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Update taskboard card Rule settings"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update_taskboard_card_rule_settings(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::BoardCardRuleSettings>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> update_taskboard_card_rule_settings::Builder {
            update_taskboard_card_rule_settings::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BoardCardRuleSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/cardrulesettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardCardRuleSettings =
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
    pub mod update_board_card_rule_settings {
        use super::models;
        type Response = models::BoardCardRuleSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::BoardCardRuleSettings,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/cardrulesettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardCardRuleSettings =
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
    pub mod update_taskboard_card_rule_settings {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::BoardCardRuleSettings,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/taskboard/cardrulesettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
pub mod cardsettings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get board card settings for the board id or board by name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Update board card settings for the board id or board by name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update_board_card_settings(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::BoardCardSettings>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> update_board_card_settings::Builder {
            update_board_card_settings::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Update taskboard card settings"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update_taskboard_card_settings(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::BoardCardSettings>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> update_taskboard_card_settings::Builder {
            update_taskboard_card_settings::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BoardCardSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/cardsettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardCardSettings =
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
    pub mod update_board_card_settings {
        use super::models;
        type Response = models::BoardCardSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::BoardCardSettings,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/cardsettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardCardSettings =
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
    pub mod update_taskboard_card_settings {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::BoardCardSettings,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/taskboard/cardsettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
pub mod chartimages {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a board chart image."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `board`: Identifier for board, either board's backlog level name (e.g. \"Issues\") or Id."]
        #[doc = "* `name`: The chart name. e.g. CumulativeFlow."]
        pub fn get_board_chart_image(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            board: impl Into<String>,
            name: impl Into<String>,
        ) -> get_board_chart_image::Builder {
            get_board_chart_image::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                board: board.into(),
                name: name.into(),
                width: None,
                height: None,
                show_details: None,
                title: None,
            }
        }
        #[doc = "Get an iteration chart image."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `iteration_id`: ID of the iteration."]
        #[doc = "* `name`: The chart name. e.g. Burndown."]
        pub fn get_iteration_chart_image(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            iteration_id: impl Into<String>,
            name: impl Into<String>,
        ) -> get_iteration_chart_image::Builder {
            get_iteration_chart_image::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                iteration_id: iteration_id.into(),
                name: name.into(),
                width: None,
                height: None,
                show_details: None,
                title: None,
            }
        }
        #[doc = "Get an iterations chart image."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `name`: The chart name. e.g. Velocity."]
        pub fn get_iterations_chart_image(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            name: impl Into<String>,
        ) -> get_iterations_chart_image::Builder {
            get_iterations_chart_image::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                name: name.into(),
                iterations_number: None,
                width: None,
                height: None,
                show_details: None,
                title: None,
            }
        }
    }
    pub mod get_board_chart_image {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) board: String,
            pub(crate) name: String,
            pub(crate) width: Option<i32>,
            pub(crate) height: Option<i32>,
            pub(crate) show_details: Option<bool>,
            pub(crate) title: Option<String>,
        }
        impl Builder {
            #[doc = "The width of the chart in pixels. Must be greater than 0."]
            pub fn width(mut self, width: i32) -> Self {
                self.width = Some(width);
                self
            }
            #[doc = "The height of the chart in pixels. Must be greater than 0."]
            pub fn height(mut self, height: i32) -> Self {
                self.height = Some(height);
                self
            }
            #[doc = "Whether or not the chart should include detailed information (e.g. axis labels, titles, trend lines, etc.)."]
            pub fn show_details(mut self, show_details: bool) -> Self {
                self.show_details = Some(show_details);
                self
            }
            #[doc = "The title of the chart. Can only be displayed if ShowLabels is true."]
            pub fn title(mut self, title: impl Into<String>) -> Self {
                self.title = Some(title.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/chartimages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board,
                            &this.name
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
                        if let Some(width) = &this.width {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("width", &width.to_string());
                        }
                        if let Some(height) = &this.height {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("height", &height.to_string());
                        }
                        if let Some(show_details) = &this.show_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("showDetails", &show_details.to_string());
                        }
                        if let Some(title) = &this.title {
                            req.url_mut().query_pairs_mut().append_pair("title", title);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_iteration_chart_image {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) iteration_id: String,
            pub(crate) name: String,
            pub(crate) width: Option<i32>,
            pub(crate) height: Option<i32>,
            pub(crate) show_details: Option<bool>,
            pub(crate) title: Option<String>,
        }
        impl Builder {
            #[doc = "The width of the chart in pixels. Must be greater than 0."]
            pub fn width(mut self, width: i32) -> Self {
                self.width = Some(width);
                self
            }
            #[doc = "The height of the chart in pixels. Must be greater than 0."]
            pub fn height(mut self, height: i32) -> Self {
                self.height = Some(height);
                self
            }
            #[doc = "Whether or not the chart should include detailed information (e.g. axis labels, titles, trend lines, etc.)"]
            pub fn show_details(mut self, show_details: bool) -> Self {
                self.show_details = Some(show_details);
                self
            }
            #[doc = "The title of the chart. Can only be displayed if ShowLabels is true."]
            pub fn title(mut self, title: impl Into<String>) -> Self {
                self.title = Some(title.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/iterations/{}/chartimages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id,
                            &this.name
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
                        if let Some(width) = &this.width {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("width", &width.to_string());
                        }
                        if let Some(height) = &this.height {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("height", &height.to_string());
                        }
                        if let Some(show_details) = &this.show_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("showDetails", &show_details.to_string());
                        }
                        if let Some(title) = &this.title {
                            req.url_mut().query_pairs_mut().append_pair("title", title);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_iterations_chart_image {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) name: String,
            pub(crate) iterations_number: Option<i32>,
            pub(crate) width: Option<i32>,
            pub(crate) height: Option<i32>,
            pub(crate) show_details: Option<bool>,
            pub(crate) title: Option<String>,
        }
        impl Builder {
            #[doc = "Number of iterations the chart is for."]
            pub fn iterations_number(mut self, iterations_number: i32) -> Self {
                self.iterations_number = Some(iterations_number);
                self
            }
            #[doc = "The width of the chart in pixels. Must be greater than 0."]
            pub fn width(mut self, width: i32) -> Self {
                self.width = Some(width);
                self
            }
            #[doc = "The height of the chart in pixels. Must be greater than 0."]
            pub fn height(mut self, height: i32) -> Self {
                self.height = Some(height);
                self
            }
            #[doc = "Whether or not the chart should include detailed information (e.g. axis labels, titles, trend lines, etc.)"]
            pub fn show_details(mut self, show_details: bool) -> Self {
                self.show_details = Some(show_details);
                self
            }
            #[doc = "The title of the chart. Can only be displayed if ShowLabels is true."]
            pub fn title(mut self, title: impl Into<String>) -> Self {
                self.title = Some(title.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/iterations/chartimages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.name
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
                        if let Some(iterations_number) = &this.iterations_number {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("iterationsNumber", &iterations_number.to_string());
                        }
                        if let Some(width) = &this.width {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("width", &width.to_string());
                        }
                        if let Some(height) = &this.height {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("height", &height.to_string());
                        }
                        if let Some(show_details) = &this.show_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("showDetails", &show_details.to_string());
                        }
                        if let Some(title) = &this.title {
                            req.url_mut().query_pairs_mut().append_pair("title", title);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
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
pub mod charts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get board charts"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Identifier for board, either board's backlog level name (Eg:\"Stories\") or Id"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Get a board chart"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Identifier for board, either board's backlog level name (Eg:\"Stories\") or Id"]
        #[doc = "* `name`: The chart name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            board: impl Into<String>,
            name: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                board: board.into(),
                name: name.into(),
                team: team.into(),
            }
        }
        #[doc = "Update a board chart"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Identifier for board, either board's backlog level name (Eg:\"Stories\") or Id"]
        #[doc = "* `name`: The chart name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::BoardChart>,
            project: impl Into<String>,
            board: impl Into<String>,
            name: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                board: board.into(),
                name: name.into(),
                team: team.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BoardChartReferenceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/charts",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardChartReferenceList =
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
        type Response = models::BoardChart;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) name: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/charts/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board,
                            &this.name
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
                                let rsp_value: models::BoardChart =
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
        type Response = models::BoardChart;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::BoardChart,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) name: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/charts/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board,
                            &this.name
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
                                let rsp_value: models::BoardChart =
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
pub mod columns {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get columns on a board"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Name or ID of the specific board"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Update columns on a board"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: List of board columns to update"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Name or ID of the specific board"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: Vec<models::BoardColumn>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BoardColumnList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/columns",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardColumnList =
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
        type Response = models::BoardColumnList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::BoardColumn>,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/columns",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardColumnList =
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
pub mod rows {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get rows on a board"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Name or ID of the specific board"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
        #[doc = "Update rows on a board"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: List of board rows to update"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `board`: Name or ID of the specific board"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: Vec<models::BoardRow>,
            project: impl Into<String>,
            board: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                board: board.into(),
                team: team.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BoardRowList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/rows",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardRowList =
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
        type Response = models::BoardRowList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::BoardRow>,
            pub(crate) project: String,
            pub(crate) board: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/{}/rows",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.board
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
                                let rsp_value: models::BoardRowList =
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
pub mod boardparents {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the list of parent field filter model for the given list of workitem ids"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            child_backlog_context_category_ref_name: impl Into<String>,
            workitem_ids: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                child_backlog_context_category_ref_name: child_backlog_context_category_ref_name
                    .into(),
                workitem_ids: workitem_ids.into(),
                team: team.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ParentChildWiMapList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) child_backlog_context_category_ref_name: String,
            pub(crate) workitem_ids: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/boards/boardparents",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                        let child_backlog_context_category_ref_name =
                            &this.child_backlog_context_category_ref_name;
                        req.url_mut().query_pairs_mut().append_pair(
                            "childBacklogContextCategoryRefName",
                            child_backlog_context_category_ref_name,
                        );
                        let workitem_ids = &this.workitem_ids;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("workitemIds", workitem_ids);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ParentChildWiMapList =
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
pub mod workitemsorder {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Reorder Sprint Backlog/Taskboard Work Items"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `iteration_id`: The id of the iteration"]
        pub fn reorder_iteration_work_items(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ReorderOperation>,
            project: impl Into<String>,
            team: impl Into<String>,
            iteration_id: impl Into<String>,
        ) -> reorder_iteration_work_items::Builder {
            reorder_iteration_work_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
                iteration_id: iteration_id.into(),
            }
        }
        #[doc = "Reorder Product Backlog/Boards Work Items"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn reorder_backlog_work_items(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ReorderOperation>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> reorder_backlog_work_items::Builder {
            reorder_backlog_work_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
    }
    pub mod reorder_iteration_work_items {
        use super::models;
        type Response = models::ReorderResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ReorderOperation,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) iteration_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/iterations/{}/workitemsorder",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id
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
                                let rsp_value: models::ReorderResultList =
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
    pub mod reorder_backlog_work_items {
        use super::models;
        type Response = models::ReorderResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ReorderOperation,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/workitemsorder",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::ReorderResultList =
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
pub mod taskboard_columns {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: Vec<models::UpdateTaskboardColumn>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TaskboardColumns;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/taskboardcolumns",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::TaskboardColumns =
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
        type Response = models::TaskboardColumns;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::UpdateTaskboardColumn>,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/taskboardcolumns",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::TaskboardColumns =
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
pub mod taskboard_work_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            iteration_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                iteration_id: iteration_id.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdateTaskboardWorkItemColumn>,
            project: impl Into<String>,
            team: impl Into<String>,
            iteration_id: impl Into<String>,
            work_item_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
                iteration_id: iteration_id.into(),
                work_item_id,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TaskboardWorkItemColumnList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) iteration_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/taskboardworkitems/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id
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
                                let rsp_value: models::TaskboardWorkItemColumnList =
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::UpdateTaskboardWorkItemColumn,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) iteration_id: String,
            pub(crate) work_item_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/taskboardworkitems/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id,
                            &this.work_item_id
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
pub mod teamsettings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a team's settings"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Update a team's settings"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: TeamSettings changes"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TeamSettingsPatch>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TeamSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::TeamSetting =
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
        type Response = models::TeamSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TeamSettingsPatch,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::TeamSetting =
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
pub mod iterations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a team's iterations using timeframe filter"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                timeframe: None,
            }
        }
        #[doc = "Add an iteration to the team"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Iteration to add"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn post_team_iteration(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TeamSettingsIteration>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> post_team_iteration::Builder {
            post_team_iteration::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Get team's iteration by iterationId"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: ID of the iteration"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id: id.into(),
                team: team.into(),
            }
        }
        #[doc = "Delete a team's iteration by iterationId"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `id`: ID of the iteration"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            id: impl Into<String>,
            team: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                id: id.into(),
                team: team.into(),
            }
        }
        #[doc = "Get work items for iteration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get_iteration_work_items(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
            team: impl Into<String>,
        ) -> get_iteration_work_items::Builder {
            get_iteration_work_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                iteration_id: iteration_id.into(),
                team: team.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TeamSettingsIterationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) timeframe: Option<String>,
        }
        impl Builder {
            #[doc = "A filter for which iterations are returned based on relative time. Only Current is supported currently."]
            pub fn timeframe(mut self, timeframe: impl Into<String>) -> Self {
                self.timeframe = Some(timeframe.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                        if let Some(timeframe) = &this.timeframe {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$timeframe", timeframe);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TeamSettingsIterationList =
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
    pub mod post_team_iteration {
        use super::models;
        type Response = models::TeamSettingsIteration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TeamSettingsIteration,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::TeamSettingsIteration =
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
        type Response = models::TeamSettingsIteration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
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
                                let rsp_value: models::TeamSettingsIteration =
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
            pub(crate) id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.id
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
    pub mod get_iteration_work_items {
        use super::models;
        type Response = models::IterationWorkItems;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}/workitems",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id
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
                                let rsp_value: models::IterationWorkItems =
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
pub mod capacities {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a team's capacity including total capacity and days off"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get_capacities_with_identity_ref_and_totals(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
            team: impl Into<String>,
        ) -> get_capacities_with_identity_ref_and_totals::Builder {
            get_capacities_with_identity_ref_and_totals::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                iteration_id: iteration_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Replace a team's capacity"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Team capacity to replace"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn replace_capacities_with_identity_ref(
            &self,
            organization: impl Into<String>,
            body: Vec<models::TeamMemberCapacityIdentityRef>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
            team: impl Into<String>,
        ) -> replace_capacities_with_identity_ref::Builder {
            replace_capacities_with_identity_ref::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                project: project.into(),
                iteration_id: iteration_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Get a team member's capacity"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        #[doc = "* `team_member_id`: ID of the team member"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get_capacity_with_identity_ref(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
            team_member_id: impl Into<String>,
            team: impl Into<String>,
        ) -> get_capacity_with_identity_ref::Builder {
            get_capacity_with_identity_ref::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                iteration_id: iteration_id.into(),
                team_member_id: team_member_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Update a team member's capacity"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Updated capacity"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        #[doc = "* `team_member_id`: ID of the team member"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CapacityPatch>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
            team_member_id: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                iteration_id: iteration_id.into(),
                team_member_id: team_member_id.into(),
                team: team.into(),
            }
        }
    }
    pub mod get_capacities_with_identity_ref_and_totals {
        use super::models;
        type Response = models::TeamCapacity;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}/capacities",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id
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
                                let rsp_value: models::TeamCapacity =
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
    pub mod replace_capacities_with_identity_ref {
        use super::models;
        type Response = models::TeamMemberCapacityIdentityRefList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::TeamMemberCapacityIdentityRef>,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}/capacities",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id
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
                                let rsp_value: models::TeamMemberCapacityIdentityRefList =
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
    pub mod get_capacity_with_identity_ref {
        use super::models;
        type Response = models::TeamMemberCapacityIdentityRef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
            pub(crate) team_member_id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}/capacities/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id,
                            &this.team_member_id
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
                                let rsp_value: models::TeamMemberCapacityIdentityRef =
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
        type Response = models::TeamMemberCapacityIdentityRef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CapacityPatch,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
            pub(crate) team_member_id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}/capacities/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id,
                            &this.team_member_id
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
                                let rsp_value: models::TeamMemberCapacityIdentityRef =
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
pub mod teamdaysoff {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get team's days off for an iteration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                iteration_id: iteration_id.into(),
                team: team.into(),
            }
        }
        #[doc = "Set a team's days off for an iteration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Team's days off patch containing a list of start and end dates"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `iteration_id`: ID of the iteration"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TeamSettingsDaysOffPatch>,
            project: impl Into<String>,
            iteration_id: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                iteration_id: iteration_id.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TeamSettingsDaysOff;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}/teamdaysoff",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id
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
                                let rsp_value: models::TeamSettingsDaysOff =
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
        type Response = models::TeamSettingsDaysOff;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TeamSettingsDaysOffPatch,
            pub(crate) project: String,
            pub(crate) iteration_id: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/iterations/{}/teamdaysoff",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.iteration_id
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
                                let rsp_value: models::TeamSettingsDaysOff =
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
pub mod teamfieldvalues {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a collection of team field values"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Update team field values"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TeamFieldValuesPatch>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TeamFieldValues;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/teamfieldvalues",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::TeamFieldValues =
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
        type Response = models::TeamFieldValues;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TeamFieldValuesPatch,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/work/teamsettings/teamfieldvalues",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
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
                                let rsp_value: models::TeamFieldValues =
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
