extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

// NOTE: proc_macro: This crate provides the necessary macros and utilities for defining procedural macros.
// NOTE: quote: This crate helps in generating Rust code as a string from syntax trees.
// NOTE: syn: This crate is used to parse Rust code into a syntax tree, which we can then manipulate.

#[proc_macro_derive(Count)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);
    let name = &macro_input.ident; // The name of the type (e.g. enum) we're deriving for

    // Check if the input is an enum and generate code to count its variants
    let count_ = match &macro_input.data {
        Data::Enum(data) => data.variants.len(),
        _ => panic!("Count can only be derived for enums"),
    };

    // Generate the implementation of the count method for the enum
    quote!(
        impl #name {
            pub fn count() -> usize {
                #count_
            }
        }
    )
    .into() // Convert the generated code into a TokenStream to be returned
}
