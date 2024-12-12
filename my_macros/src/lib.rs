use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2; (OPTIONAL)
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Identify)]
pub fn derive_identify(input: TokenStream) -> TokenStream {
    todo!()
}
