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

#[proc_macro_derive(TypeScriptType)]
pub fn derive_typescript_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Get the fields from the struct
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("TypeScriptType can only be derived for structs with named fields"),
        },
        _ => panic!("TypeScriptType can only be derived for structs"),
    };
    
    // Generate TypeScript field definitions
    let ts_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let ts_type = rust_type_to_ts(field_type);
        quote! {
            #field_name: #ts_type
        }
    });
    
    // Generate the TypeScript interface
    let ts_interface = format!("export interface {} {{\n    {}\n}}", 
        name.to_string(),
        ts_fields.map(|f| f.to_string()).collect::<Vec<_>>().join("\n    ")
    );
    
    let name_str = name.to_string();
    
    // Generate the Rust code that will register this type
    let expanded = quote! {
        impl crate::framework::typescript::TypeScriptDefinition for #name {
            fn register() {
                inventory::submit!(crate::framework::typescript::TypeScriptType {
                    name: #name_str,
                    definition: #ts_interface,
                });
            }
        }
    };
    
    TokenStream::from(expanded)
}

fn rust_type_to_ts(ty: &syn::Type) -> proc_macro2::TokenStream {
    match ty {
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            let ident = &segment.ident;
            match ident.to_string().as_str() {
                "String" => quote!("string"),
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" => quote!("number"),
                "bool" => quote!("boolean"),
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(type_arg)) = args.args.first() {
                            let inner_type = rust_type_to_ts(type_arg);
                            quote!( concat!(#inner_type, "[]") )
                        } else {
                            quote!("any[]")
                        }
                    } else {
                        quote!("any[]")
                    }
                },
                "Option" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(type_arg)) = args.args.first() {
                            let inner_type = rust_type_to_ts(type_arg);
                            quote!( concat!(#inner_type, " | null") )
                        } else {
                            quote!("any | null")
                        }
                    } else {
                        quote!("any | null")
                    }
                },
                _ => quote!("any"),
            }
        },
        _ => quote!("any"),
    }
} 