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
pub const DEFAULT_ENDPOINT: &str = "https://extmgmt.dev.azure.com";
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
    pub fn installed_extensions_client(&self) -> installed_extensions::Client {
        installed_extensions::Client(self.clone())
    }
}
pub mod installed_extensions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the installed extensions in the account / project collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                include_disabled_extensions: None,
                include_errors: None,
                asset_types: None,
                include_installation_issues: None,
            }
        }
        #[doc = "Update an installed extension. Typically this API is used to enable or disable an extension."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::InstalledExtension>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Get an installed extension by its publisher and extension name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `publisher_name`: Name of the publisher. Example: \"fabrikam\"."]
        #[doc = "* `extension_name`: Name of the extension. Example: \"ops-tools\"."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            publisher_name: impl Into<String>,
            extension_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_name: publisher_name.into(),
                extension_name: extension_name.into(),
                asset_types: None,
            }
        }
        #[doc = "Uninstall the specified extension from the account / project collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `publisher_name`: Name of the publisher. Example: \"fabrikam\"."]
        #[doc = "* `extension_name`: Name of the extension. Example: \"ops-tools\"."]
        pub fn uninstall_extension_by_name(
            &self,
            organization: impl Into<String>,
            publisher_name: impl Into<String>,
            extension_name: impl Into<String>,
        ) -> uninstall_extension_by_name::RequestBuilder {
            uninstall_extension_by_name::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_name: publisher_name.into(),
                extension_name: extension_name.into(),
                reason: None,
                reason_code: None,
            }
        }
        #[doc = "Install the specified extension into the account / project collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `publisher_name`: Name of the publisher. Example: \"fabrikam\"."]
        #[doc = "* `extension_name`: Name of the extension. Example: \"ops-tools\"."]
        pub fn install_extension_by_name(
            &self,
            organization: impl Into<String>,
            publisher_name: impl Into<String>,
            extension_name: impl Into<String>,
            version: impl Into<String>,
        ) -> install_extension_by_name::RequestBuilder {
            install_extension_by_name::RequestBuilder {
                client: self.0.clone(),
                organization: organization.into(),
                publisher_name: publisher_name.into(),
                extension_name: extension_name.into(),
                version: version.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::InstalledExtensionList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::InstalledExtensionList =
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
            pub(crate) include_disabled_extensions: Option<bool>,
            pub(crate) include_errors: Option<bool>,
            pub(crate) asset_types: Option<String>,
            pub(crate) include_installation_issues: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "If true (the default), include disabled extensions in the results."]
            pub fn include_disabled_extensions(
                mut self,
                include_disabled_extensions: bool,
            ) -> Self {
                self.include_disabled_extensions = Some(include_disabled_extensions);
                self
            }
            #[doc = "If true, include installed extensions with errors."]
            pub fn include_errors(mut self, include_errors: bool) -> Self {
                self.include_errors = Some(include_errors);
                self
            }
            #[doc = "Determines which files are returned in the files array.  Provide the wildcard '*' to return all files, or a colon separated list to retrieve files with specific asset types."]
            pub fn asset_types(mut self, asset_types: impl Into<String>) -> Self {
                self.asset_types = Some(asset_types.into());
                self
            }
            pub fn include_installation_issues(
                mut self,
                include_installation_issues: bool,
            ) -> Self {
                self.include_installation_issues = Some(include_installation_issues);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/extensionmanagement/installedextensions",
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
                        if let Some(include_disabled_extensions) = &this.include_disabled_extensions
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeDisabledExtensions",
                                &include_disabled_extensions.to_string(),
                            );
                        }
                        if let Some(include_errors) = &this.include_errors {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeErrors", &include_errors.to_string());
                        }
                        if let Some(asset_types) = &this.asset_types {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("assetTypes", asset_types);
                        }
                        if let Some(include_installation_issues) = &this.include_installation_issues
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeInstallationIssues",
                                &include_installation_issues.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::InstalledExtensionList>;
            type IntoFuture = futures::future::BoxFuture<
                'static,
                azure_core::Result<models::InstalledExtensionList>,
            >;
            #[doc = "Send the request and return the response body."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::InstalledExtension> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::InstalledExtension =
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
            pub(crate) body: models::InstalledExtension,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/extensionmanagement/installedextensions",
                            this.client.endpoint(),
                            &this.organization
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
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::InstalledExtension>;
            type IntoFuture =
                futures::future::BoxFuture<'static, azure_core::Result<models::InstalledExtension>>;
            #[doc = "Send the request and return the response body."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::InstalledExtension> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::InstalledExtension =
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
            pub(crate) publisher_name: String,
            pub(crate) extension_name: String,
            pub(crate) asset_types: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Determines which files are returned in the files array.  Provide the wildcard '*' to return all files, or a colon separated list to retrieve files with specific asset types."]
            pub fn asset_types(mut self, asset_types: impl Into<String>) -> Self {
                self.asset_types = Some(asset_types.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/extensionmanagement/installedextensionsbyname/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.publisher_name,
                            &this.extension_name
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
                        if let Some(asset_types) = &this.asset_types {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("assetTypes", asset_types);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::InstalledExtension>;
            type IntoFuture =
                futures::future::BoxFuture<'static, azure_core::Result<models::InstalledExtension>>;
            #[doc = "Send the request and return the response body."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod uninstall_extension_by_name {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) publisher_name: String,
            pub(crate) extension_name: String,
            pub(crate) reason: Option<String>,
            pub(crate) reason_code: Option<String>,
        }
        impl RequestBuilder {
            pub fn reason(mut self, reason: impl Into<String>) -> Self {
                self.reason = Some(reason.into());
                self
            }
            pub fn reason_code(mut self, reason_code: impl Into<String>) -> Self {
                self.reason_code = Some(reason_code.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/extensionmanagement/installedextensionsbyname/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.publisher_name,
                            &this.extension_name
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
                        if let Some(reason) = &this.reason {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("reason", reason);
                        }
                        if let Some(reason_code) = &this.reason_code {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("reasonCode", reason_code);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod install_extension_by_name {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::InstalledExtension> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::InstalledExtension =
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
            pub(crate) publisher_name: String,
            pub(crate) extension_name: String,
            pub(crate) version: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/extensionmanagement/installedextensionsbyname/{}/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.publisher_name,
                            &this.extension_name,
                            &this.version
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::InstalledExtension>;
            type IntoFuture =
                futures::future::BoxFuture<'static, azure_core::Result<models::InstalledExtension>>;
            #[doc = "Send the request and return the response body."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
