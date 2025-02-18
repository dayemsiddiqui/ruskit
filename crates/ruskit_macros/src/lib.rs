use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, FieldsNamed, Token};
use syn::punctuated::Punctuated;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let attrs = &input.attrs;

    // Get the original fields
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => quote! { #fields },
            Fields::Unit => quote! {},
            _ => panic!("Only named fields or unit structs are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Pass through the original Template derive and add metadata
    let expanded = quote! {
        #[derive(askama::Template)]
        #(#attrs)*
        pub struct #name {
            pub metadata: &'static Metadata,
            #fields
        }

        impl Default for #name {
            fn default() -> Self {
                Self {
                    metadata: get_global_metadata(),
                }
            }
        }

        impl View for #name {
            fn metadata(&self) -> &Metadata {
                self.metadata
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(BaseTemplate, attributes(template))]
pub fn derive_base_template(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    
    // Add metadata field to the struct
    if let Data::Struct(ref mut struct_data) = input.data {
        if let Fields::Named(ref mut fields) = struct_data.fields {
            let metadata_field = syn::parse_quote! { pub metadata: &'static Metadata };
            fields.named.push(metadata_field);
        }
    }

    // Generate implementation
    let name = &input.ident;
    let expanded = quote! {
        #[derive(askama::Template)]
        #input

        impl Default for #name {
            fn default() -> Self {
                Self {
                    metadata: get_global_metadata(),
                }
            }
        }
    };

    TokenStream::from(expanded)
} 