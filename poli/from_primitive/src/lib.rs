extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(FromPrimitive)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput); // syntax tree of the input
    let name = &input.ident; // the name of the enum
    let data = match input.data {
        // ensure it's an enum
        Data::Enum(data) => data,
        _ => panic!("FromPrimitive can only be derived for enums"),
    };
    // generate match arms for each variant, mapping its index to the variant
    let variants = data.variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident; // the name of the variant
        quote! {
            #i => Ok(#name::#ident)
        }
    });
    // generate the implementation of TryFrom<usize> for the enum
    quote! {
        impl std::convert::TryFrom<usize> for #name {
            type Error = String;

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                match value {
                    // generate match arms for each variant: first # is for the match arm, second # is for the variant name, * is for repeating the pattern for all variants
                    #(#variants,)*
                    _ => Err(format!("Invalid value for {}: {}", stringify!(#name), value)),
                }
            }
        }
    }
    .into()
}
