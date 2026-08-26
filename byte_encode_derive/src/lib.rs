// Code mostly made by gemini somehow

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ByteEncode)]
pub fn derive_byte_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // We only support structs with named fields for this example
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // --- 1. Generate from_le_bytes parsing logic ---
    let mut current_offset = quote! { 0 };
    let from_le_bytes_initializers = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        // Calculate start and end bounds for the current field slice
        let start = quote! { #current_offset };
        let end = quote! { #start + <#field_type>::SIZE };
        
        // Update the offset tracking for the next iteration loop
        current_offset = quote! { #end };

        quote! {
            #field_name: <#field_type as ByteEncode<{<#field_type>::SIZE}>>::from_le_bytes(
                buffer[#start..#end].try_into().unwrap()
            )
        }
    });
	let mut current_offset = quote! { 0 };
    let from_be_bytes_initializers = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        // Calculate start and end bounds for the current field slice
        let start = quote! { #current_offset };
        let end = quote! { #start + <#field_type>::SIZE };
        
        // Update the offset tracking for the next iteration loop
        current_offset = quote! { #end };

        quote! {
            #field_name: <#field_type as ByteEncode<{<#field_type>::SIZE}>>::from_be_bytes(
                &mut buffer[#start..#end].try_into().unwrap()
            )
        }
    });

	
    // --- 2. Generate to_le_bytes serialization logic ---
    let mut current_offset = quote! { 0 };
    let to_le_bytes_writers = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        let start = quote! { #current_offset };
        let end = quote! { #start + <#field_type>::SIZE };
        
        current_offset = quote! { #end };

        quote! {
            res[#start..#end].copy_from_slice(&self.#field_name.to_le_bytes());
        }
    });
	let mut current_offset = quote! { 0 };
    let to_be_bytes_writers = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        let start = quote! { #current_offset };
        let end = quote! { #start + <#field_type>::SIZE };
        
        current_offset = quote! { #end };

        quote! {
            res[#start..#end].copy_from_slice(&self.#field_name.to_be_bytes());
        }
    });
	

    // --- 3. Construct final token stream ---
    // This dynamically sums up sizes like: 0 + Field1::SIZE + Field2::SIZE ...
    let total_size_expr = fields.iter().fold(quote! { 0 }, |acc, f| {
        let field_type = &f.ty;
        quote! { #acc + <#field_type>::SIZE }
    });

    let expanded = quote! {
        impl ByteEncode<{ #total_size_expr }> for #name {
            fn from_le_bytes(buffer: &[u8; { #total_size_expr }]) -> Self {
                Self {
                    #( #from_le_bytes_initializers, )*
                }
            }

            fn to_le_bytes(&self) -> [u8; { #total_size_expr }] {
                let mut res = [0u8; { #total_size_expr }];
                #( #to_le_bytes_writers )*
                res
            }

			
            fn from_be_bytes(buffer: &mut [u8; { #total_size_expr }]) -> Self {
                Self {
                    #( #from_be_bytes_initializers, )*
                }
            }

            fn to_be_bytes(&self) -> [u8; { #total_size_expr }] {
                let mut res = [0u8; { #total_size_expr }];
                #( #to_be_bytes_writers )*
                res
            }
        }
    };

    TokenStream::from(expanded)
}
