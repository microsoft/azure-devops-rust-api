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
    pub fn behaviors_client(&self) -> behaviors::Client {
        behaviors::Client(self.clone())
    }
    pub fn controls_client(&self) -> controls::Client {
        controls::Client(self.clone())
    }
    pub fn fields_client(&self) -> fields::Client {
        fields::Client(self.clone())
    }
    pub fn groups_client(&self) -> groups::Client {
        groups::Client(self.clone())
    }
    pub fn layout_client(&self) -> layout::Client {
        layout::Client(self.clone())
    }
    pub fn lists_client(&self) -> lists::Client {
        lists::Client(self.clone())
    }
    pub fn pages_client(&self) -> pages::Client {
        pages::Client(self.clone())
    }
    pub fn processes_client(&self) -> processes::Client {
        processes::Client(self.clone())
    }
    pub fn rules_client(&self) -> rules::Client {
        rules::Client(self.clone())
    }
    pub fn states_client(&self) -> states::Client {
        states::Client(self.clone())
    }
    pub fn system_controls_client(&self) -> system_controls::Client {
        system_controls::Client(self.clone())
    }
    pub fn work_item_types_client(&self) -> work_item_types::Client {
        work_item_types::Client(self.clone())
    }
    pub fn work_item_types_behaviors_client(&self) -> work_item_types_behaviors::Client {
        work_item_types_behaviors::Client(self.clone())
    }
}
pub mod groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Moves a group to a different page and section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The updated group."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `page_id`: The ID of the page the group is in."]
        #[doc = "* `section_id`: The ID of the section the group is i.n"]
        #[doc = "* `group_id`: The ID of the group."]
        #[doc = "* `remove_from_page_id`: ID of the page to remove the group from."]
        #[doc = "* `remove_from_section_id`: ID of the section to remove the group from."]
        pub fn move_group_to_page(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Group>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            page_id: impl Into<String>,
            section_id: impl Into<String>,
            group_id: impl Into<String>,
            remove_from_page_id: impl Into<String>,
            remove_from_section_id: impl Into<String>,
        ) -> move_group_to_page::RequestBuilder {
            move_group_to_page::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                page_id: page_id.into(),
                section_id: section_id.into(),
                group_id: group_id.into(),
                remove_from_page_id: remove_from_page_id.into(),
                remove_from_section_id: remove_from_section_id.into(),
            }
        }
        #[doc = "Adds a group to the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The group."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `page_id`: The ID of the page to add the group to."]
        #[doc = "* `section_id`: The ID of the section to add the group to."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Group>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            page_id: impl Into<String>,
            section_id: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                page_id: page_id.into(),
                section_id: section_id.into(),
            }
        }
        #[doc = "Moves a group to a different section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The updated group."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `page_id`: The ID of the page the group is in."]
        #[doc = "* `section_id`: The ID of the section the group is in."]
        #[doc = "* `group_id`: The ID of the group."]
        #[doc = "* `remove_from_section_id`: ID of the section to remove the group from."]
        pub fn move_group_to_section(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Group>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            page_id: impl Into<String>,
            section_id: impl Into<String>,
            group_id: impl Into<String>,
            remove_from_section_id: impl Into<String>,
        ) -> move_group_to_section::RequestBuilder {
            move_group_to_section::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                page_id: page_id.into(),
                section_id: section_id.into(),
                group_id: group_id.into(),
                remove_from_section_id: remove_from_section_id.into(),
            }
        }
        #[doc = "Updates a group in the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The updated group."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `page_id`: The ID of the page the group is in."]
        #[doc = "* `section_id`: The ID of the section the group is in."]
        #[doc = "* `group_id`: The ID of the group."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Group>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            page_id: impl Into<String>,
            section_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                page_id: page_id.into(),
                section_id: section_id.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Removes a group from the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `page_id`: The ID of the page the group is in"]
        #[doc = "* `section_id`: The ID of the section to the group is in"]
        #[doc = "* `group_id`: The ID of the group"]
        pub fn remove_group(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            page_id: impl Into<String>,
            section_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> remove_group::RequestBuilder {
            remove_group::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                page_id: page_id.into(),
                section_id: section_id.into(),
                group_id: group_id.into(),
            }
        }
    }
    pub mod move_group_to_page {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Group> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Group = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Group,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) page_id: String,
            pub(crate) section_id: String,
            pub(crate) group_id: String,
            pub(crate) remove_from_page_id: String,
            pub(crate) remove_from_section_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages/{}/sections/{}/groups/{}?removeFromPageId={}&removeFromSectionId={}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . page_id , & this . section_id , & this . group_id , & this . remove_from_page_id , & this . remove_from_section_id)) ? ;
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
                        let remove_from_page_id = &this.remove_from_page_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("removeFromPageId", remove_from_page_id);
                        let remove_from_section_id = &this.remove_from_section_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("removeFromSectionId", remove_from_section_id);
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Group>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Group> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Group = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Group,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) page_id: String,
            pub(crate) section_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages/{}/sections/{}/groups" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . page_id , & this . section_id)) ? ;
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Group>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod move_group_to_section {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Group> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Group = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Group,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) page_id: String,
            pub(crate) section_id: String,
            pub(crate) group_id: String,
            pub(crate) remove_from_section_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages/{}/sections/{}/groups/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . page_id , & this . section_id , & this . group_id)) ? ;
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
                        let remove_from_section_id = &this.remove_from_section_id;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("removeFromSectionId", remove_from_section_id);
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Group>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Group> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Group = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Group,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) page_id: String,
            pub(crate) section_id: String,
            pub(crate) group_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages/{}/sections/{}/groups/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . page_id , & this . section_id , & this . group_id)) ? ;
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Group>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod remove_group {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) page_id: String,
            pub(crate) section_id: String,
            pub(crate) group_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages/{}/sections/{}/groups/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . page_id , & this . section_id , & this . group_id)) ? ;
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod processes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of all processes including system and inherited."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                expand: None,
            }
        }
        #[doc = "Creates a process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: CreateProcessModel."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CreateProcessModel>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Get a single process of a specified ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_type_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_type_id: process_type_id.into(),
                expand: None,
            }
        }
        #[doc = "Edit a process of a specific ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn edit_process(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdateProcessModel>,
            process_type_id: impl Into<String>,
        ) -> edit_process::RequestBuilder {
            edit_process::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_type_id: process_type_id.into(),
            }
        }
        #[doc = "Removes a process of a specific ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            process_type_id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_type_id: process_type_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessInfoList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessInfoList =
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
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes",
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessInfoList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessInfo> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessInfo = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::CreateProcessModel,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes",
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessInfo>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessInfo> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessInfo = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) process_type_id: String,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_type_id
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessInfo>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod edit_process {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessInfo> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessInfo = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::UpdateProcessModel,
            pub(crate) process_type_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_type_id
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessInfo>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_type_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_type_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod behaviors {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of all behaviors in the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                expand: None,
            }
        }
        #[doc = "Creates a single behavior in the given process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ProcessBehaviorCreateRequest>,
            process_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
            }
        }
        #[doc = "Returns a behavior of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `behavior_ref_name`: The reference name of the behavior"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            behavior_ref_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                behavior_ref_name: behavior_ref_name.into(),
                expand: None,
            }
        }
        #[doc = "Replaces a behavior in the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `behavior_ref_name`: The reference name of the behavior"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ProcessBehaviorUpdateRequest>,
            process_id: impl Into<String>,
            behavior_ref_name: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                behavior_ref_name: behavior_ref_name.into(),
            }
        }
        #[doc = "Removes a behavior in the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `behavior_ref_name`: The reference name of the behavior"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            behavior_ref_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                behavior_ref_name: behavior_ref_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessBehaviorList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessBehaviorList =
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
            pub(crate) process_id: String,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/behaviors",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessBehaviorList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessBehavior> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessBehavior =
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
            pub(crate) body: models::ProcessBehaviorCreateRequest,
            pub(crate) process_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/behaviors",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessBehavior>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessBehavior> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessBehavior =
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
            pub(crate) process_id: String,
            pub(crate) behavior_ref_name: String,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/behaviors/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.behavior_ref_name
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessBehavior>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessBehavior> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessBehavior =
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
            pub(crate) body: models::ProcessBehaviorUpdateRequest,
            pub(crate) process_id: String,
            pub(crate) behavior_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/behaviors/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.behavior_ref_name
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessBehavior>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) behavior_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/behaviors/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.behavior_ref_name
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod work_item_types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of all work item types in a process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                expand: None,
            }
        }
        #[doc = "Creates a work item type in the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process on which to create work item type."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CreateProcessWorkItemTypeRequest>,
            process_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
            }
        }
        #[doc = "Returns a single work item type in a process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                expand: None,
            }
        }
        #[doc = "Updates a work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdateProcessWorkItemTypeRequest>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Removes a work itewm type in the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessWorkItemTypeList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemTypeList = serde_json::from_slice(&bytes)
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
            pub(crate) process_id: String,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Flag to determine what properties of work item type to return"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypes",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::ProcessWorkItemTypeList>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessWorkItemType> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemType =
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
            pub(crate) body: models::CreateProcessWorkItemTypeRequest,
            pub(crate) process_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypes",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessWorkItemType>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessWorkItemType> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemType =
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Flag to determine what properties of work item type to return"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessWorkItemType>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessWorkItemType> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemType =
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
            pub(crate) body: models::UpdateProcessWorkItemTypeRequest,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessWorkItemType>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod fields {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of all fields in a work item type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Adds a field to a work item type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::AddProcessWorkItemTypeFieldRequest>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Returns a field in a work item type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `field_ref_name`: The reference name of the field."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            field_ref_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                field_ref_name: field_ref_name.into(),
                expand: None,
            }
        }
        #[doc = "Updates a field in a work item type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `field_ref_name`: The reference name of the field."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdateProcessWorkItemTypeFieldRequest>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            field_ref_name: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                field_ref_name: field_ref_name.into(),
            }
        }
        #[doc = "Removes a field from a work item type. Does not permanently delete the field."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `field_ref_name`: The reference name of the field."]
        pub fn remove_work_item_type_field(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            field_ref_name: impl Into<String>,
        ) -> remove_work_item_type_field::RequestBuilder {
            remove_work_item_type_field::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                field_ref_name: field_ref_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::ProcessWorkItemTypeFieldList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemTypeFieldList = serde_json::from_slice(&bytes)
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/fields",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::ProcessWorkItemTypeFieldList>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessWorkItemTypeField> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemTypeField = serde_json::from_slice(&bytes)
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
            pub(crate) body: models::AddProcessWorkItemTypeFieldRequest,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/fields",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::ProcessWorkItemTypeField>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessWorkItemTypeField> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemTypeField = serde_json::from_slice(&bytes)
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) field_ref_name: String,
            pub(crate) expand: Option<String>,
        }
        impl RequestBuilder {
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/fields/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.field_ref_name
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
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::ProcessWorkItemTypeField>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessWorkItemTypeField> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessWorkItemTypeField = serde_json::from_slice(&bytes)
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
            pub(crate) body: models::UpdateProcessWorkItemTypeFieldRequest,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) field_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/fields/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.field_ref_name
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::ProcessWorkItemTypeField>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod remove_work_item_type_field {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) field_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/fields/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.field_ref_name
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod layout {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the form layout."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::FormLayout> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::FormLayout = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::FormLayout>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod controls {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Creates a control in a group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The control."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `group_id`: The ID of the group to add the control to."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Control>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            group_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Moves a control to a specified group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The control."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `group_id`: The ID of the group to move the control to."]
        #[doc = "* `control_id`: The ID of the control."]
        pub fn move_control_to_group(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Control>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            group_id: impl Into<String>,
            control_id: impl Into<String>,
        ) -> move_control_to_group::RequestBuilder {
            move_control_to_group::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                group_id: group_id.into(),
                control_id: control_id.into(),
                remove_from_group_id: None,
            }
        }
        #[doc = "Updates a control on the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The updated control."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `group_id`: The ID of the group."]
        #[doc = "* `control_id`: The ID of the control."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Control>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            group_id: impl Into<String>,
            control_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                group_id: group_id.into(),
                control_id: control_id.into(),
            }
        }
        #[doc = "Removes a control from the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `group_id`: The ID of the group."]
        #[doc = "* `control_id`: The ID of the control to remove."]
        pub fn remove_control_from_group(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            group_id: impl Into<String>,
            control_id: impl Into<String>,
        ) -> remove_control_from_group::RequestBuilder {
            remove_control_from_group::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                group_id: group_id.into(),
                control_id: control_id.into(),
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Control> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Control = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Control,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) group_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/groups/{}/controls" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . group_id)) ? ;
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Control>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod move_control_to_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Control> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Control = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Control,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) group_id: String,
            pub(crate) control_id: String,
            pub(crate) remove_from_group_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The group ID to remove the control from."]
            pub fn remove_from_group_id(mut self, remove_from_group_id: impl Into<String>) -> Self {
                self.remove_from_group_id = Some(remove_from_group_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/groups/{}/controls/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . group_id , & this . control_id)) ? ;
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
                        if let Some(remove_from_group_id) = &this.remove_from_group_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("removeFromGroupId", remove_from_group_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Control>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Control> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Control = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Control,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) group_id: String,
            pub(crate) control_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/groups/{}/controls/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . group_id , & this . control_id)) ? ;
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Control>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod remove_control_from_group {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) group_id: String,
            pub(crate) control_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/groups/{}/controls/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . group_id , & this . control_id)) ? ;
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
                        Ok(Response(this.client.send(&mut req).await?))
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
        #[doc = "Adds a page to the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The page."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Page>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Updates a page on the work item form"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The page"]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Page>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Removes a page from the work item form"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `page_id`: The ID of the page"]
        pub fn remove_page(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            page_id: impl Into<String>,
        ) -> remove_page::RequestBuilder {
            remove_page::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                page_id: page_id.into(),
            }
        }
    }
    pub mod add {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Page> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Page = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Page,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Page>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Page> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Page = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Page,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Page>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod remove_page {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) page_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/pages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.page_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod system_controls {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets edited system controls for a work item type in a process. To get all system controls (base + edited) use layout API(s)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Updates/adds a system control on the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `control_id`: The ID of the control."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Control>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            control_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                control_id: control_id.into(),
            }
        }
        #[doc = "Deletes a system control modification on the work item form."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process."]
        #[doc = "* `wit_ref_name`: The reference name of the work item type."]
        #[doc = "* `control_id`: The ID of the control."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            control_id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                control_id: control_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ControlList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ControlList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/systemcontrols",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ControlList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Control> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Control = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::Control,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) control_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/systemcontrols/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . control_id)) ? ;
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::Control>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ControlList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ControlList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) control_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/_apis/work/processes/{}/workItemTypes/{}/layout/systemcontrols/{}" , this . client . endpoint () , & this . organization , & this . process_id , & this . wit_ref_name , & this . control_id)) ? ;
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ControlList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod rules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of all rules in the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Adds a rule to work item type in the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CreateProcessRuleRequest>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Returns a single rule in the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `rule_id`: The ID of the rule"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            rule_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                rule_id: rule_id.into(),
            }
        }
        #[doc = "Updates a rule in the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `rule_id`: The ID of the rule"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdateProcessRuleRequest>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            rule_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                rule_id: rule_id.into(),
            }
        }
        #[doc = "Removes a rule from the work item type in the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `rule_id`: The ID of the rule"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            rule_id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                rule_id: rule_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessRuleList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessRuleList =
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/rules",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessRuleList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessRule> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessRule = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::CreateProcessRuleRequest,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/rules",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessRule>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessRule> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessRule = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) rule_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/rules/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.rule_id
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessRule>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProcessRule> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProcessRule = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::UpdateProcessRuleRequest,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) rule_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/rules/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.rule_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::ProcessRule>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) rule_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/rules/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.rule_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod states {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of all state definitions in a work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Creates a state definition in the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemStateInputModel>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
            }
        }
        #[doc = "Returns a single state definition in a work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `state_id`: The ID of the state"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            state_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                state_id: state_id.into(),
            }
        }
        #[doc = "Hides a state definition in the work item type of the process.Only states with customizationType:System can be hidden."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `state_id`: The ID of the state"]
        pub fn hide_state_definition(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::HideStateModel>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            state_id: impl Into<String>,
        ) -> hide_state_definition::RequestBuilder {
            hide_state_definition::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                state_id: state_id.into(),
            }
        }
        #[doc = "Updates a given state definition in the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `state_id`: ID of the state"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemStateInputModel>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            state_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                state_id: state_id.into(),
            }
        }
        #[doc = "Removes a state definition in the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: ID of the process"]
        #[doc = "* `wit_ref_name`: The reference name of the work item type"]
        #[doc = "* `state_id`: ID of the state"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name: impl Into<String>,
            state_id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name: wit_ref_name.into(),
                state_id: state_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(
                self,
            ) -> azure_core::Result<models::WorkItemStateResultModelList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemStateResultModelList = serde_json::from_slice(&bytes)
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/states",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::WorkItemStateResultModelList>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemStateResultModel> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemStateResultModel = serde_json::from_slice(&bytes)
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
            pub(crate) body: models::WorkItemStateInputModel,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/states",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::WorkItemStateResultModel>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemStateResultModel> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemStateResultModel = serde_json::from_slice(&bytes)
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) state_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/states/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.state_id
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::WorkItemStateResultModel>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod hide_state_definition {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemStateResultModel> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemStateResultModel = serde_json::from_slice(&bytes)
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
            pub(crate) body: models::HideStateModel,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) state_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/states/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.state_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::WorkItemStateResultModel>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemStateResultModel> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemStateResultModel = serde_json::from_slice(&bytes)
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
            pub(crate) body: models::WorkItemStateInputModel,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) state_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/states/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.state_id
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::WorkItemStateResultModel>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name: String,
            pub(crate) state_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workItemTypes/{}/states/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name,
                            &this.state_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod work_item_types_behaviors {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of all behaviors for the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name_for_behaviors`: Work item type reference name for the behavior"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name_for_behaviors: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name_for_behaviors: wit_ref_name_for_behaviors.into(),
            }
        }
        #[doc = "Adds a behavior to the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name_for_behaviors`: Work item type reference name for the behavior"]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemTypeBehavior>,
            process_id: impl Into<String>,
            wit_ref_name_for_behaviors: impl Into<String>,
        ) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name_for_behaviors: wit_ref_name_for_behaviors.into(),
            }
        }
        #[doc = "Updates a behavior for the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name_for_behaviors`: Work item type reference name for the behavior"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemTypeBehavior>,
            process_id: impl Into<String>,
            wit_ref_name_for_behaviors: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                process_id: process_id.into(),
                wit_ref_name_for_behaviors: wit_ref_name_for_behaviors.into(),
            }
        }
        #[doc = "Returns a behavior for the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name_for_behaviors`: Work item type reference name for the behavior"]
        #[doc = "* `behavior_ref_name`: The reference name of the behavior"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name_for_behaviors: impl Into<String>,
            behavior_ref_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name_for_behaviors: wit_ref_name_for_behaviors.into(),
                behavior_ref_name: behavior_ref_name.into(),
            }
        }
        #[doc = "Removes a behavior for the work item type of the process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `process_id`: The ID of the process"]
        #[doc = "* `wit_ref_name_for_behaviors`: Work item type reference name for the behavior"]
        #[doc = "* `behavior_ref_name`: The reference name of the behavior"]
        pub fn remove_behavior_from_work_item_type(
            &self,
            organization: impl Into<String>,
            process_id: impl Into<String>,
            wit_ref_name_for_behaviors: impl Into<String>,
            behavior_ref_name: impl Into<String>,
        ) -> remove_behavior_from_work_item_type::RequestBuilder {
            remove_behavior_from_work_item_type::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                process_id: process_id.into(),
                wit_ref_name_for_behaviors: wit_ref_name_for_behaviors.into(),
                behavior_ref_name: behavior_ref_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemTypeBehaviorList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemTypeBehaviorList = serde_json::from_slice(&bytes)
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name_for_behaviors: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypesbehaviors/{}/behaviors",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name_for_behaviors
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
            ) -> futures::future::BoxFuture<
                'static,
                azure_core::Result<models::WorkItemTypeBehaviorList>,
            > {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemTypeBehavior> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemTypeBehavior =
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
            pub(crate) body: models::WorkItemTypeBehavior,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name_for_behaviors: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypesbehaviors/{}/behaviors",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name_for_behaviors
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkItemTypeBehavior>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemTypeBehavior> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemTypeBehavior =
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
            pub(crate) body: models::WorkItemTypeBehavior,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name_for_behaviors: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypesbehaviors/{}/behaviors",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name_for_behaviors
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkItemTypeBehavior>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkItemTypeBehavior> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkItemTypeBehavior =
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
            pub(crate) process_id: String,
            pub(crate) wit_ref_name_for_behaviors: String,
            pub(crate) behavior_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypesbehaviors/{}/behaviors/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name_for_behaviors,
                            &this.behavior_ref_name
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkItemTypeBehavior>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod remove_behavior_from_work_item_type {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) process_id: String,
            pub(crate) wit_ref_name_for_behaviors: String,
            pub(crate) behavior_ref_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/{}/workitemtypesbehaviors/{}/behaviors/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.process_id,
                            &this.wit_ref_name_for_behaviors,
                            &this.behavior_ref_name
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod lists {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns meta data of the picklist."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Creates a picklist."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Picklist"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PickList>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Returns a picklist."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `list_id`: The ID of the list"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            list_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                list_id: list_id.into(),
            }
        }
        #[doc = "Updates a list."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `list_id`: The ID of the list"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PickList>,
            list_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                list_id: list_id.into(),
            }
        }
        #[doc = "Removes a picklist."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `list_id`: The ID of the list"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            list_id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                list_id: list_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PickListMetadataList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PickListMetadataList =
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
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/lists",
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
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::PickListMetadataList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PickList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PickList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::PickList,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/lists",
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::PickList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PickList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PickList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) list_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/lists/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.list_id
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
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::PickList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PickList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PickList = serde_json::from_slice(&bytes).map_err(|e| {
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
            pub(crate) body: models::PickList,
            pub(crate) list_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/lists/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.list_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::PickList>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) list_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/processes/lists/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.list_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
