use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::{
    request_builder_into_future::RequestBuilderIntoFutureCode,
    request_builder_send::RequestBuilderSendCode,
    request_builder_setter::RequestBuilderSettersCode,
    request_builder_struct::RequestBuilderStructCode, response_code::ResponseCode,
};
pub struct OperationModuleCode {
    pub module_name: Ident,
    pub response_code: ResponseCode,
    pub request_builder_struct_code: RequestBuilderStructCode,
    pub request_builder_setters_code: RequestBuilderSettersCode,
    pub request_builder_send_code: RequestBuilderSendCode,
    pub request_builder_intofuture_code: RequestBuilderIntoFutureCode,
    pub generate_blocking: bool,
}
impl ToTokens for OperationModuleCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            module_name,
            response_code,
            request_builder_struct_code,
            request_builder_setters_code,
            request_builder_send_code,
            request_builder_intofuture_code,
            generate_blocking,
        } = &self;
        // With a blocking client the request builder is generic over the send mode,
        // and the send code provides its own impl blocks.
        let request_builder_impls = if *generate_blocking {
            let setters = request_builder_setters_code.to_token_stream();
            let setters_impl = if setters.is_empty() {
                quote! {}
            } else {
                quote! {
                    impl<M> RequestBuilder<M> {
                        #setters
                    }
                }
            };
            quote! {
                #setters_impl
                #request_builder_send_code
            }
        } else {
            quote! {
                impl RequestBuilder {
                    #request_builder_setters_code
                    #request_builder_send_code
                }
            }
        };
        tokens.extend(quote! {
            pub mod #module_name {
                use super::models;
                #[cfg(target_arch = "wasm32")]
                use futures::future::LocalBoxFuture as BoxFuture;
                #[cfg(not(target_arch = "wasm32"))]
                use futures::future::BoxFuture as BoxFuture;

                #response_code

                #request_builder_struct_code

                #request_builder_impls

                #request_builder_intofuture_code
            }
        })
    }
}
