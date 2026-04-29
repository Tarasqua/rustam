// linking static library
fn main() {
    println!(
        "cargo:rustc-link-search=/home/papoi/Documents/pet/rustam/homework_2/ffi/target/debug"
    );
    println!("cargo:rustc-link-lib=static=synt_ex_lib");
}
