// INFO: there are 3 types of macros in Rust: declarative macros, procedural macros, and macro_rules macros
//  - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// - Attribute-like macros that define custom attributes usable on any item
// - Function-like macros that look like function calls but operate on the tokens specified as their argument

// NOTE: 1. declarative macros
#[macro_export] // indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope
macro_rules! vec {
    // uses a match pattern: If the pattern matches, the associated block of code will be emitted
    // $x:expr: we are looking for an expression (number, string, etc.) and save it to the variable $x
    // $( ),*: we are looking for zero or more expressions separated by commas (* - zero or more times)
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // for each expression, push it into the vector
            $(
                temp_vec.push($x);
            )*
            temp_vec // return the vector
        }
    };
}

// NOTE: 2. procedural macros
// NOTE: A Procedural Macro acts like a compiler plugin. Instead of simple text substitution (like macro_rules!), it is a Rust function that accepts a stream of code, manipulates it using Rust logic, and returns a new stream of code.

// use proc_macro::TokenStream; // TokenStream: The abstract representation of source code (keywords, identifiers, punctuation).
// The Crate Type: These macros must live in their own dedicated library crate with proc-macro = true in the Cargo.toml.

// #[proc_macro]
// pub fn my_macro(input: TokenStream) -> TokenStream {
//     // 1. Receive 'input' (the code where the macro was called)
//     // 2. Transform the code (add, delete, or modify parts)
//     // 3. Return the new code as a TokenStream
//     input
// }

// NOTE: 3. Custom derive Macros

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn call_pancakes() {
    Pancakes::hello_macro(); // Hello, Macro! My name is Pancakes!
}

// NOTE: 4. Attribute-Like Macros

// #[route(GET, "/")]
// fn index() {}

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// NOTE: 5. Function-Like Macros
//
// let sql = sql!(SELECT * FROM posts WHERE id=1);
//
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
