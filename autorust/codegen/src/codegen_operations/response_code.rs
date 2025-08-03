use autorust_openapi::Response;
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::spec::get_type_name_for_schema_ref;
use crate::status_codes::get_status_code_ident;
use crate::{codegen::TypeNameCode, CodeGen};
use crate::{content_type, Result};

use super::response_headers::{HeaderCode, HeadersCode};
use super::web_operation_gen::{Pageable, WebOperationGen};
/// The response for an operation.
/// An operation may have multiple valid status codes.
#[derive(Clone)]
pub struct ResponseCode {
    pub status_responses: Vec<StatusResponseCode>,
    pub pageable: Option<Pageable>,
    produces: String,
    headers: HeadersCode,
}

/// A single status code response of an operation.
#[derive(Clone)]
pub struct StatusResponseCode {
    pub status_code_name: Ident,
    response_type: Option<TypeNameCode>,
}

impl ResponseCode {
    pub fn new(cg: &CodeGen, operation: &WebOperationGen, produces: String) -> Result<Self> {
        let success_responses = operation.success_responses();
        let status_responses = success_responses
            .iter()
            .map(|(status_code, rsp)| {
                Ok(StatusResponseCode {
                    status_code_name: get_status_code_ident(status_code)?,
                    response_type: create_response_type(cg, rsp)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let headers = success_responses
            .iter()
            .flat_map(|(_, rsp)| &rsp.headers)
            .filter_map(|(name, header)| match header {
                autorust_openapi::ReferenceOr::Item(header) => {
                    Some((name.clone(), HeaderCode::new(name.clone(), header)))
                }
                _ => None,
            })
            .collect::<IndexMap<_, _>>()
            .into_values()
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            status_responses,
            pageable: operation.pageable(),
            produces,
            headers: HeadersCode::new(headers),
        })
    }

    /// Get the response type for the HTTP response body
    pub fn response_type(&self) -> Option<TypeNameCode> {
        let responses = &self.status_responses;
        if responses.is_empty() {
            return None;
        }
        self.fix_response_type(responses[0].response_type.as_ref())
    }

    fn fix_response_type(&self, response_type: Option<&TypeNameCode>) -> Option<TypeNameCode> {
        if let Some(tp) = response_type {
            let mut tp = tp.clone();
            if tp.is_value() && self.produces_xml() {
                tp.set_as_bytes();
            }
            return Some(tp);
        }
        None
    }

    fn produces_xml(&self) -> bool {
        self.produces == content_type::APPLICATION_XML
    }
}

impl ToTokens for ResponseCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (response_type_tokens, azure_core_response_tokens) = match self.response_type() {
            Some(response_type) => {
                if response_type.is_bytes() {
                    (
                        quote! { Bytes },
                        quote! { azure_core::http::Response<Bytes, azure_core::http::NoFormat> },
                    )
                } else if self.produces_xml() {
                    (
                        quote! { #response_type },
                        quote! { azure_core::http::Response<#response_type, azure_core::http::XmlFormat> },
                    )
                } else {
                    (
                        quote! { #response_type },
                        quote! { azure_core::http::Response<#response_type, azure_core::http::JsonFormat> },
                    )
                }
            }
            None => (
                quote! { () },
                quote! { azure_core::http::Response<(), azure_core::http::NoFormat> },
            ),
        };

        tokens.extend(quote! {
            #[derive(Debug)]
            pub struct Response(#azure_core_response_tokens);
        });

        let body_fn = if self.response_type().is_some() {
            quote! {
                pub async fn into_body(self) -> azure_core::Result<#response_type_tokens> {
                    self.0.into_body().await
                }
            }
        } else {
            quote! {}
        };

        let raw_response_fn = quote! {
            pub fn into_raw_response(self) -> azure_core::http::RawResponse {
                self.0.into()
            }
        };

        let headers_fn = if self.headers.has_headers() {
            quote! { pub fn headers(&self) -> Headers { Headers(self.0.headers()) } }
        } else {
            quote! {}
        };

        tokens.extend(quote! {
            impl Response {
                #body_fn
                #raw_response_fn
                #headers_fn
            }
        });
        tokens.extend(self.headers.to_token_stream());
    }
}

fn create_response_type(cg: &CodeGen, rsp: &Response) -> Result<Option<TypeNameCode>> {
    if let Some(schema) = &rsp.schema {
        let mut type_name = TypeNameCode::new(&get_type_name_for_schema_ref(schema)?)?;
        type_name.qualify_models(true);
        cg.set_if_union_type(&mut type_name);
        Ok(Some(type_name))
    } else {
        Ok(None)
    }
}
