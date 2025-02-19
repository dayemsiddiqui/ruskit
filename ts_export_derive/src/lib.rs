extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Automatically derives Serialize, TS, and adds #[ts(export)] attribute
/// while also registering the type for TypeScript generation.
/// 
/// Example:
/// ```rust
/// #[auto_ts_export]
/// pub struct MyDto {
///     field: String
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_ts_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let const_name = format!("_AUTO_TS_EXPORT_{}", name);
    let const_ident = syn::Ident::new(&const_name, name.span());
    
    let expanded = quote! {
        #[derive(::serde::Serialize, ::ts_rs::TS)]
        #[ts(export)]
        #input

        #[doc(hidden)]
        const #const_ident: () = {
            ::inventory::submit!(crate::app::dtos::TsExporter {
                export_fn: <#name as ::ts_rs::TS>::export_to_string,
            });
        };
    };

    expanded.into()
} 