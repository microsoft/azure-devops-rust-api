use crate::{doc_comment::DocCommentCode, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::{
    function_params::{FunctionCallParamsCode, FunctionParam, FunctionParams},
    web_operation_gen::WebOperationGen,
};
#[derive(Clone)]
struct BlockingClientFunction {
    client_type: TokenStream,
    async_client: String,
}

/// Create the client function that produces the request builder instance.
#[derive(Clone)]
pub(crate) struct ClientFunctionCode {
    summary: Option<String>,
    description: Option<String>,
    fname: Ident,
    parameters: FunctionParams,
    in_operation_group: bool,
    /// Set for a blocking client function (see `with_blocking_client`).
    blocking: Option<BlockingClientFunction>,
}

impl ClientFunctionCode {
    pub fn new(
        operation: &WebOperationGen,
        parameters: &FunctionParams,
        in_operation_group: bool,
    ) -> Result<Self> {
        let fname = operation.function_name()?;
        let summary = operation.0.summary.clone();
        let description = operation.0.description.clone();
        Ok(Self {
            summary,
            description,
            fname,
            parameters: parameters.clone(),
            in_operation_group,
            blocking: None,
        })
    }

    /// Returns a copy of this function for a blocking client.
    ///
    /// `client_type` is the blocking client type, and `async_client` is the path of the
    /// corresponding async client, whose documentation is linked to rather than repeated.
    pub fn with_blocking_client(&self, client_type: TokenStream, async_client: String) -> Self {
        Self {
            blocking: Some(BlockingClientFunction {
                client_type,
                async_client,
            }),
            ..self.clone()
        }
    }
}

impl ToTokens for ClientFunctionCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params: Vec<TokenStream> = Vec::new();
        if self.in_operation_group {
            params.push(quote! { client: self.0.clone() });
        } else {
            params.push(quote! { client: self.clone() });
        }
        for param in self.parameters.required_params() {
            let FunctionParam {
                variable_name,
                type_name,
                ..
            } = param;
            let mut type_name = type_name.clone();
            let is_vec = type_name.is_vec();
            type_name.impl_into(!is_vec);
            if type_name.has_impl_into() {
                params.push(quote! { #variable_name: #variable_name.into() });
            } else {
                params.push(quote! { #variable_name });
            }
        }
        for param in self.parameters.optional_params() {
            let FunctionParam {
                variable_name,
                type_name,
                ..
            } = param;
            if type_name.is_vec() {
                params.push(quote! { #variable_name: Vec::new() });
            } else {
                params.push(quote! { #variable_name: None });
            }
        }

        let summary = DocCommentCode::new(self.summary.clone());
        let description = DocCommentCode::new(self.description.clone());

        let mut param_descriptions: Vec<TokenStream> = Vec::new();
        if self
            .parameters
            .required_params()
            .into_iter()
            .any(|param| param.description.is_some())
        {
            // Add a blank link before the arguments if there is a summary or description.
            if !summary.is_empty() || !description.is_empty() {
                param_descriptions.push(quote! { #[doc = ""] });
            }
            param_descriptions.push(quote! { #[doc = "Arguments:"] });
            for required_param in self.parameters.required_params().iter() {
                if let Some(desc) = &required_param.description {
                    if !desc.is_empty() {
                        let doc_comment = format!("* `{}`: {desc}", required_param.variable_name);
                        param_descriptions.push(quote! { #[doc = #doc_comment] });
                    }
                }
            }
        };

        let fname = &self.fname;
        let parameters = FunctionCallParamsCode(self.parameters.clone());
        let generics = match &self.blocking {
            Some(BlockingClientFunction { client_type, .. }) => quote! { <#client_type> },
            None => quote! {},
        };
        // Blocking client functions link to the async function's documentation,
        // rather than repeating the argument descriptions.
        let param_descriptions = match &self.blocking {
            Some(BlockingClientFunction { async_client, .. }) => {
                let doc = format!(
                    "See [`{fname}`]({async_client}::{fname}) for details of the arguments."
                );
                let blank = if summary.is_empty() && description.is_empty() {
                    quote! {}
                } else {
                    quote! { #[doc = ""] }
                };
                vec![quote! { #blank #[doc = #doc] }]
            }
            None => param_descriptions,
        };
        tokens.extend(quote! {
            #summary
            #description
            #(#param_descriptions)*
            pub fn #fname(#parameters) -> #fname::RequestBuilder #generics {
                #fname::RequestBuilder {
                    #(#params),*
                }
            }
        });
    }
}
