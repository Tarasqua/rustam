// INFO: The proc_macro crate is the compiler’s API that allows us to read and manipulate Rust code from our code.
// INFO: The syn crate parses Rust code from a string into a data structure that we can perform operations on. The quote crate turns syn data structures back into Rust code.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! { // INFO: quote! is a macro that generates Rust code from a syntax tree.
        impl HelloMacro for #name { // INFO: #name is a placeholder that will be replaced with the struct name at compile time.
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into() // INFO: .into() converts the quote! output into a TokenStream that can be returned by the proc macro.
}
