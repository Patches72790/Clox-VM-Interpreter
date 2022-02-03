extern crate proc_macro;

use proc_macro::*;
use quote::{quote, ToTokens};
use syn::{self, AttributeArgs, ItemStruct, Lit, NestedMeta};

///
/// A macro for producing precedence and partial order
/// for precedence type structs in the Rox implementation
/// of the Clox VM.
#[proc_macro_attribute]
pub fn make_precedence(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as AttributeArgs);
    let struct_ast = syn::parse_macro_input!(input as ItemStruct);

    impl_make_precedence(&attr_ast, &struct_ast)
}

fn parse_precedence_literal(attr_ast: &AttributeArgs) -> Result<u8, ()> {
    let path = match &attr_ast[0] {
        NestedMeta::Lit(val) => val,
        _ => panic!("unknown literal in attribute definition"),
    };

    let num = match path {
        Lit::Int(num) => num
            .base10_parse::<u8>()
            .expect("Error parsing int literal!"),
        _ => panic!("Not an int type parsed from attribute args!"),
    };

    Ok(num)
}

fn impl_make_precedence(attr_ast: &AttributeArgs, struct_ast: &ItemStruct) -> TokenStream {
    let num = parse_precedence_literal(attr_ast).ok().unwrap();
    let struct_tokens = struct_ast.into_token_stream();
    let struct_name = &struct_ast.ident;

    let code_gen = quote! {
        #struct_tokens

        impl std::ops::Deref for #struct_name {
            type Target = u8;

            fn deref(&self) -> &Self::Target {
                &#num
            }
        }
    };

    code_gen.into()
}
