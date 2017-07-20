extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_derive(Custom)]
pub fn custom(input: TokenStream) -> TokenStream {
	let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
	TokenStream::from_str("").unwrap()
}