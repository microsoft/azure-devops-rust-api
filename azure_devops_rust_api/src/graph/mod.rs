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
pub const DEFAULT_ENDPOINT: &str = "https://vssps.dev.azure.com";
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
    pub fn avatars_client(&self) -> avatars::Client {
        avatars::Client(self.clone())
    }
    pub fn descriptors_client(&self) -> descriptors::Client {
        descriptors::Client(self.clone())
    }
    pub fn groups_client(&self) -> groups::Client {
        groups::Client(self.clone())
    }
    pub fn identity_translation_client(&self) -> identity_translation::Client {
        identity_translation::Client(self.clone())
    }
    pub fn membership_states_client(&self) -> membership_states::Client {
        membership_states::Client(self.clone())
    }
    pub fn memberships_client(&self) -> memberships::Client {
        memberships::Client(self.clone())
    }
    pub fn provider_info_client(&self) -> provider_info::Client {
        provider_info::Client(self.clone())
    }
    pub fn request_access_client(&self) -> request_access::Client {
        request_access::Client(self.clone())
    }
    pub fn storage_keys_client(&self) -> storage_keys::Client {
        storage_keys::Client(self.clone())
    }
    pub fn subject_lookup_client(&self) -> subject_lookup::Client {
        subject_lookup::Client(self.clone())
    }
    pub fn subject_query_client(&self) -> subject_query::Client {
        subject_query::Client(self.clone())
    }
    pub fn users_client(&self) -> users::Client {
        users::Client(self.clone())
    }
}
pub mod descriptors {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Resolve a storage key to a descriptor"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_key`: Storage key of the subject (user, group, scope, etc.) to resolve"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            storage_key: impl Into<String>,
            organization: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                storage_key: storage_key.into(),
                organization: organization.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::GraphDescriptorResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) storage_key: String,
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
                            "{}/{}/_apis/graph/descriptors/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.storage_key
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
                                let rsp_value: models::GraphDescriptorResult =
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
pub mod groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of all groups in the current scope (usually organization or account).\n\nThe optional parameters are used to filter down the returned results. Returned results are in no guaranteed order.\n\n Since the list of groups may be large, results are returned in pages of groups.  If there are more results\n than can be returned in a single page, the result set will contain a continuation token for retrieval of the\n next set of results."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                scope_descriptor: None,
                subject_types: None,
                continuation_token: None,
            }
        }
        #[doc = "Create a new Azure DevOps group or materialize an existing AAD group.\n\nThe body of the request must be a derived type of GraphGroupCreationContext:\n  * GraphGroupVstsCreationContext - Create a new Azure DevOps group that is not backed by an external provider.\n  * GraphGroupMailAddressCreationContext - Create a new group using the mail address as a reference to an existing group from an external AD or AAD backed provider.\n  * GraphGroupOriginIdCreationContext - Create a new group using the OriginID as a reference to a group from an external AD or AAD backed provider.\n\n Optionally, you can add the newly created group as a member of an existing Azure DevOps group and/or specify a custom storage key for the group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The subset of the full graph group used to uniquely find the graph subject in an external provider."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::GraphGroupCreationContext>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                scope_descriptor: None,
                group_descriptors: None,
            }
        }
        #[doc = "Get a group by its descriptor.\n\nThe group will be returned even if it has been deleted from the account or has had all its memberships\ndeleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `group_descriptor`: The descriptor of the desired graph group."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            group_descriptor: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                group_descriptor: group_descriptor.into(),
            }
        }
        #[doc = "Update the properties of an Azure DevOps group.\n\nCurrently limited to only changing the description and account name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `group_descriptor`: The descriptor of the group to modify."]
        #[doc = "* `body`: The JSON+Patch document containing the fields to alter."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            group_descriptor: impl Into<String>,
            body: impl Into<models::JsonPatchDocument>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                group_descriptor: group_descriptor.into(),
                body: body.into(),
            }
        }
        #[doc = "Removes an Azure DevOps group from all of its parent groups.\n\nThe group will still be visible, but membership\n checks for the group, and all descendants which derive membership through it, will return false.”"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `group_descriptor`: The descriptor of the group to delete."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            group_descriptor: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                group_descriptor: group_descriptor.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::GraphGroupList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) scope_descriptor: Option<String>,
            pub(crate) subject_types: Option<String>,
            pub(crate) continuation_token: Option<String>,
        }
        impl Builder {
            #[doc = "Specify a non-default scope (collection, project) to search for groups."]
            pub fn scope_descriptor(mut self, scope_descriptor: impl Into<String>) -> Self {
                self.scope_descriptor = Some(scope_descriptor.into());
                self
            }
            #[doc = "A comma separated list of user subject subtypes to reduce the retrieved results, e.g. Microsoft.IdentityModel.Claims.ClaimsIdentity"]
            pub fn subject_types(mut self, subject_types: impl Into<String>) -> Self {
                self.subject_types = Some(subject_types.into());
                self
            }
            #[doc = "An opaque data blob that allows the next page of data to resume immediately after where the previous page ended. The only reliable way to know if there is more data left is the presence of a continuation token."]
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
                            "{}/{}/_apis/graph/groups",
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
                        if let Some(scope_descriptor) = &this.scope_descriptor {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("scopeDescriptor", scope_descriptor);
                        }
                        if let Some(subject_types) = &this.subject_types {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("subjectTypes", subject_types);
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
                                let rsp_value: models::GraphGroupList =
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
        type Response = models::GraphGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::GraphGroupCreationContext,
            pub(crate) scope_descriptor: Option<String>,
            pub(crate) group_descriptors: Option<String>,
        }
        impl Builder {
            #[doc = "A descriptor referencing the scope (collection, project) in which the group should be created. If omitted, will be created in the scope of the enclosing account or organization. Valid only for VSTS groups."]
            pub fn scope_descriptor(mut self, scope_descriptor: impl Into<String>) -> Self {
                self.scope_descriptor = Some(scope_descriptor.into());
                self
            }
            #[doc = "A comma separated list of descriptors referencing groups you want the graph group to join"]
            pub fn group_descriptors(mut self, group_descriptors: impl Into<String>) -> Self {
                self.group_descriptors = Some(group_descriptors.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/groups",
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
                        if let Some(scope_descriptor) = &this.scope_descriptor {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("scopeDescriptor", scope_descriptor);
                        }
                        if let Some(group_descriptors) = &this.group_descriptors {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("groupDescriptors", group_descriptors);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GraphGroup =
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
        type Response = models::GraphGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) group_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/groups/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.group_descriptor
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
                                let rsp_value: models::GraphGroup =
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
        type Response = models::GraphGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) group_descriptor: String,
            pub(crate) body: models::JsonPatchDocument,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/groups/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.group_descriptor
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
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GraphGroup =
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
            pub(crate) group_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/groups/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.group_descriptor
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
pub mod identity_translation {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn translate(&self, organization: impl Into<String>) -> translate::Builder {
            translate::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                master_id: None,
                local_id: None,
            }
        }
    }
    pub mod translate {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) master_id: Option<String>,
            pub(crate) local_id: Option<String>,
        }
        impl Builder {
            pub fn master_id(mut self, master_id: impl Into<String>) -> Self {
                self.master_id = Some(master_id.into());
                self
            }
            pub fn local_id(mut self, local_id: impl Into<String>) -> Self {
                self.local_id = Some(local_id.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/identitytranslation",
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
                        if let Some(master_id) = &this.master_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("masterId", master_id);
                        }
                        if let Some(local_id) = &this.local_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("localId", local_id);
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
pub mod memberships {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all the memberships where this descriptor is a member in the relationship.\n\nThe default value for direction is 'up' meaning return all memberships where the subject is a member (e.g. all groups the subject is a member of).\n Alternatively, passing the direction as 'down' will return all memberships where the subject is a container (e.g. all members of the subject group)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subject_descriptor`: Fetch all direct memberships of this descriptor."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            subject_descriptor: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_descriptor: subject_descriptor.into(),
                direction: None,
                depth: None,
            }
        }
        #[doc = "Get a membership relationship between a container and subject."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subject_descriptor`: A descriptor to the child subject in the relationship."]
        #[doc = "* `container_descriptor`: A descriptor to the container in the relationship."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            subject_descriptor: impl Into<String>,
            container_descriptor: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_descriptor: subject_descriptor.into(),
                container_descriptor: container_descriptor.into(),
            }
        }
        #[doc = "Create a new membership between a container and subject."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subject_descriptor`: A descriptor to a group or user that can be the child subject in the relationship."]
        #[doc = "* `container_descriptor`: A descriptor to a group that can be the container in the relationship."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            subject_descriptor: impl Into<String>,
            container_descriptor: impl Into<String>,
        ) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_descriptor: subject_descriptor.into(),
                container_descriptor: container_descriptor.into(),
            }
        }
        #[doc = "Deletes a membership between a container and subject."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subject_descriptor`: A descriptor to a group or user that is the child subject in the relationship."]
        #[doc = "* `container_descriptor`: A descriptor to a group that is the container in the relationship."]
        pub fn remove_membership(
            &self,
            organization: impl Into<String>,
            subject_descriptor: impl Into<String>,
            container_descriptor: impl Into<String>,
        ) -> remove_membership::Builder {
            remove_membership::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_descriptor: subject_descriptor.into(),
                container_descriptor: container_descriptor.into(),
            }
        }
        #[doc = "Check to see if a membership relationship between a container and subject exists."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subject_descriptor`: The group or user that is a child subject of the relationship."]
        #[doc = "* `container_descriptor`: The group that is the container in the relationship."]
        pub fn check_membership_existence(
            &self,
            organization: impl Into<String>,
            subject_descriptor: impl Into<String>,
            container_descriptor: impl Into<String>,
        ) -> check_membership_existence::Builder {
            check_membership_existence::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_descriptor: subject_descriptor.into(),
                container_descriptor: container_descriptor.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::GraphMembershipList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_descriptor: String,
            pub(crate) direction: Option<String>,
            pub(crate) depth: Option<i32>,
        }
        impl Builder {
            #[doc = "Defaults to Up."]
            pub fn direction(mut self, direction: impl Into<String>) -> Self {
                self.direction = Some(direction.into());
                self
            }
            #[doc = "The maximum number of edges to traverse up or down the membership tree. Currently the only supported value is '1'."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/Memberships/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor
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
                        if let Some(direction) = &this.direction {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("direction", direction);
                        }
                        if let Some(depth) = &this.depth {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("depth", &depth.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GraphMembershipList =
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
        type Response = models::GraphMembership;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_descriptor: String,
            pub(crate) container_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/memberships/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor,
                            &this.container_descriptor
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
                                let rsp_value: models::GraphMembership =
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
        type Response = models::GraphMembership;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_descriptor: String,
            pub(crate) container_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/memberships/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor,
                            &this.container_descriptor
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GraphMembership =
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
    pub mod remove_membership {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_descriptor: String,
            pub(crate) container_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/memberships/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor,
                            &this.container_descriptor
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
    pub mod check_membership_existence {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_descriptor: String,
            pub(crate) container_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/memberships/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor,
                            &this.container_descriptor
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
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
pub mod membership_states {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Check whether a subject is active or inactive."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `subject_descriptor`: Descriptor of the subject (user, group, scope, etc.) to check state of"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            subject_descriptor: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_descriptor: subject_descriptor.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::GraphMembershipState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/membershipstates/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor
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
                                let rsp_value: models::GraphMembershipState =
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
pub mod request_access {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn request_access(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::JToken>,
        ) -> request_access::Builder {
            request_access::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod request_access {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::JToken,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/requestaccess",
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
pub mod storage_keys {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Resolve a descriptor to a storage key."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            subject_descriptor: impl Into<String>,
            organization: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subject_descriptor: subject_descriptor.into(),
                organization: organization.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::GraphStorageKeyResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subject_descriptor: String,
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
                            "{}/{}/_apis/graph/storagekeys/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor
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
                                let rsp_value: models::GraphStorageKeyResult =
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
pub mod subject_lookup {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Resolve descriptors to users, groups or scopes (Subjects) in a batch."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A list of descriptors that specifies a subset of subjects to retrieve. Each descriptor uniquely identifies the subject across all instance scopes, but only at a single point in time."]
        pub fn lookup_subjects(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::GraphSubjectLookup>,
        ) -> lookup_subjects::Builder {
            lookup_subjects::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod lookup_subjects {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::GraphSubjectLookup,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/subjectlookup",
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
pub mod subject_query {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Search for Azure Devops users, or/and groups. Results will be returned in a batch with no more than 100 graph subjects."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The query that we'll be using to search includes the following: Query: the search term. The search will be prefix matching only. SubjectKind: \"User\" or \"Group\" can be specified, both or either ScopeDescriptor: Non-default scope can be specified, i.e. project scope descriptor"]
        pub fn query(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::GraphSubjectQuery>,
        ) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod query {
        use super::models;
        type Response = models::GraphSubjectList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::GraphSubjectQuery,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/subjectquery",
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
                                let rsp_value: models::GraphSubjectList =
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
pub mod avatars {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            subject_descriptor: impl Into<String>,
            organization: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subject_descriptor: subject_descriptor.into(),
                organization: organization.into(),
                size: None,
                format: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn set_avatar(
            &self,
            body: impl Into<models::Avatar>,
            subject_descriptor: impl Into<String>,
            organization: impl Into<String>,
        ) -> set_avatar::Builder {
            set_avatar::Builder {
                client: self.0.clone(),
                body: body.into(),
                subject_descriptor: subject_descriptor.into(),
                organization: organization.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn delete(
            &self,
            subject_descriptor: impl Into<String>,
            organization: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subject_descriptor: subject_descriptor.into(),
                organization: organization.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Avatar;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subject_descriptor: String,
            pub(crate) organization: String,
            pub(crate) size: Option<String>,
            pub(crate) format: Option<String>,
        }
        impl Builder {
            pub fn size(mut self, size: impl Into<String>) -> Self {
                self.size = Some(size.into());
                self
            }
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/Subjects/{}/avatars",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor
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
                        if let Some(size) = &this.size {
                            req.url_mut().query_pairs_mut().append_pair("size", size);
                        }
                        if let Some(format) = &this.format {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("format", format);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Avatar = serde_json::from_slice(&rsp_body)?;
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
    pub mod set_avatar {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::Avatar,
            pub(crate) subject_descriptor: String,
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
                            "{}/{}/_apis/graph/Subjects/{}/avatars",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subject_descriptor: String,
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
                            "{}/{}/_apis/graph/Subjects/{}/avatars",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor
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
pub mod users {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of all users in a given scope.\n\nSince the list of users may be large, results are returned in pages of users.  If there are more results\n than can be returned in a single page, the result set will contain a continuation token for retrieval of the\n next set of results."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_types: None,
                continuation_token: None,
                scope_descriptor: None,
            }
        }
        #[doc = "Materialize an existing AAD or MSA user into the VSTS account.\n\nNOTE: Created users are not active in an account unless they have been explicitly assigned a parent group at creation time or have signed in\n  and been autolicensed through AAD group memberships.\n\n Adding a user to an account is required before the user can be added to VSTS groups or assigned an asset.\n\n The body of the request must be a derived type of GraphUserCreationContext:\n  * GraphUserMailAddressCreationContext - Create a new user using the mail address as a reference to an existing user from an external AD or AAD backed provider.\n  * GraphUserOriginIdCreationContext - Create a new user using the OriginID as a reference to an existing user from an external AD or AAD backed provider.\n  * GraphUserPrincipalNameCreationContext - Create a new user using the principal name as a reference to an existing user from an external AD or AAD backed provider.\n\n If the user to be added corresponds to a user that was previously deleted, then that user will be restored.\n\n Optionally, you can add the newly created user as a member of an existing VSTS group and/or specify a custom storage key for the user."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The subset of the full graph user used to uniquely find the graph subject in an external provider."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::GraphUserCreationContext>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                group_descriptors: None,
            }
        }
        #[doc = "Get a user by its descriptor."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `user_descriptor`: The descriptor of the desired user."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            user_descriptor: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                user_descriptor: user_descriptor.into(),
            }
        }
        #[doc = "Map an existing user to a different identity"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The subset of the full graph user used to uniquely find the graph subject in an external provider."]
        #[doc = "* `user_descriptor`: the descriptor of the user to update"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::GraphUserUpdateContext>,
            user_descriptor: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                user_descriptor: user_descriptor.into(),
            }
        }
        #[doc = "Disables a user.\n\nThe user will still be visible, but membership checks for the user will return false.”"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `user_descriptor`: The descriptor of the user to delete."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            user_descriptor: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                user_descriptor: user_descriptor.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::GraphUserList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_types: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) scope_descriptor: Option<String>,
        }
        impl Builder {
            #[doc = "A comma separated list of user subject subtypes to reduce the retrieved results, e.g. msa’, ‘aad’, ‘svc’ (service identity), ‘imp’ (imported identity), etc."]
            pub fn subject_types(mut self, subject_types: impl Into<String>) -> Self {
                self.subject_types = Some(subject_types.into());
                self
            }
            #[doc = "An opaque data blob that allows the next page of data to resume immediately after where the previous page ended. The only reliable way to know if there is more data left is the presence of a continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Specify a non-default scope (collection, project) to search for users."]
            pub fn scope_descriptor(mut self, scope_descriptor: impl Into<String>) -> Self {
                self.scope_descriptor = Some(scope_descriptor.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/users",
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
                        if let Some(subject_types) = &this.subject_types {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("subjectTypes", subject_types);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(scope_descriptor) = &this.scope_descriptor {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("scopeDescriptor", scope_descriptor);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GraphUserList =
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
        type Response = models::GraphUser;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::GraphUserCreationContext,
            pub(crate) group_descriptors: Option<String>,
        }
        impl Builder {
            #[doc = "A comma separated list of descriptors of groups you want the graph user to join"]
            pub fn group_descriptors(mut self, group_descriptors: impl Into<String>) -> Self {
                self.group_descriptors = Some(group_descriptors.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/users",
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
                        if let Some(group_descriptors) = &this.group_descriptors {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("groupDescriptors", group_descriptors);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GraphUser =
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
        type Response = models::GraphUser;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) user_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/users/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.user_descriptor
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
                                let rsp_value: models::GraphUser =
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
        type Response = models::GraphUser;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::GraphUserUpdateContext,
            pub(crate) user_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/users/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.user_descriptor
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
                                let rsp_value: models::GraphUser =
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
            pub(crate) user_descriptor: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/graph/users/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.user_descriptor
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
pub mod provider_info {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            user_descriptor: impl Into<String>,
            organization: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                user_descriptor: user_descriptor.into(),
                organization: organization.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::GraphProviderInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) user_descriptor: String,
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
                            "{}/{}/_apis/graph/Users/{}/providerinfo",
                            this.client.endpoint(),
                            &this.organization,
                            &this.user_descriptor
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
                                let rsp_value: models::GraphProviderInfo =
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
