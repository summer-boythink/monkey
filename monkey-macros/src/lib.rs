extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{self, Data};

/// Generate `fn express_node()` uniformly for different types of `nodes`
#[proc_macro_derive(DefaultExpressionNode)]
pub fn expression_node(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;
    let Data::Struct(_) = ast.data else{
        panic!("MyDefault derive macro must use in struct");
    };
    quote!();

    quote! {
        impl Node for #id {
            fn token_literal(&self) -> String {
                self.token.literal.to_string()
            }
        }


    impl Expression for #id {
        fn expression_node(&self) {
        }

        fn as_any(&self) -> &dyn std::any::Any {
            todo!()
        }
    }
    }
    .into()
}

#[proc_macro_derive(DefaultStatementNode)]
pub fn statement_node(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;
    let Data::Struct(_) = ast.data else{
        panic!("MyDefault derive macro must use in struct");
    };
    quote!();

    quote! {
    impl Node for #id {
            fn token_literal(&self) -> String {
                self.token.literal.to_string()
            }
        }


    impl Statement for #id {
        fn statement_node(&self) {
        }

        fn as_any(&self) -> &dyn std::any::Any {
            todo!()
        }
    }
    }
    .into()
}
