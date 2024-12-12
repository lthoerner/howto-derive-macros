use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2; (OPTIONAL)
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Identify, attributes(friendly_name))]
pub fn derive_identify(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name, ..
    } = parse_macro_input!(input);
    let type_name_string = type_name.clone().to_string();

    quote! {
        impl crate::Identify for #type_name {
            fn type_name(&self) -> &'static str {
                #type_name_string
            }
        }
    }
    .into()
}
