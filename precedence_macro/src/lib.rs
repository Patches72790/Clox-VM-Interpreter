extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(PrecedenceMacro)]
pub fn precedence_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_precedence_macro(&ast)
}

fn impl_precedence_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PrecedenceMacro for #name {
            fn get_precedence() {
                println!("Hello from get precedence function from {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
