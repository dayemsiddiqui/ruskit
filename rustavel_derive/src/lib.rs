use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(GenerateValidationFields)]
pub fn generate_validation_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields_name = quote::format_ident!("{}Fields", name);
    
    let (fields, field_inits) = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => {
                    let filtered_fields: Vec<_> = fields.named.iter()
                        .filter(|f| {
                            // Filter out id, created_at, and updated_at fields
                            let name = f.ident.as_ref().unwrap().to_string();
                            !["id", "created_at", "updated_at", "user_id"].contains(&name.as_str())
                        })
                        .collect();

                    let fields = filtered_fields.iter().map(|f| {
                        let field_name = &f.ident;
                        let field_type = &f.ty;
                        quote! {
                            pub #field_name: Field<#field_type>
                        }
                    }).collect::<Vec<_>>();

                    let field_inits = filtered_fields.iter().map(|f| {
                        let field_name = &f.ident;
                        quote! {
                            #field_name: Field::new(stringify!(#field_name))
                        }
                    }).collect::<Vec<_>>();

                    (fields, field_inits)
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
    };

    TokenStream::from(expanded)
} 