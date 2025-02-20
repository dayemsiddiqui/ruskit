use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(GenerateValidationFields)]
pub fn generate_validation_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields_name = quote::format_ident!("{}Fields", name);
    
    let fields = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => {
                    fields.named.iter().map(|f| {
                        let field_name = &f.ident;
                        let field_type = &f.ty;
                        quote! {
                            pub #field_name: Field<#field_type>
                        }
                    }).collect::<Vec<_>>()
                },
                _ => panic!("Only named fields are supported")
            }
        },
        _ => panic!("Only structs are supported")
    };

    let field_inits = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => {
                    fields.named.iter().map(|f| {
                        let field_name = &f.ident;
                        quote! {
                            #field_name: Field::new(stringify!(#field_name))
                        }
                    }).collect::<Vec<_>>()
                },
                _ => panic!("Only named fields are supported")
            }
        },
        _ => panic!("Only structs are supported")
    };

    let expanded = quote! {
        pub struct #fields_name {
            #(#fields,)*
        }

        impl #fields_name {
            pub fn new() -> Self {
                Self {
                    #(#field_inits,)*
                }
            }
        }

        impl ModelValidation for #name {
            type Fields = #fields_name;

            fn fields() -> Self::Fields {
                #fields_name::new()
            }
        }
    };

    TokenStream::from(expanded)
} 