// dynamic linking
fn main() {
    println!(
        "cargo:rustc-link-search=/home/papoi/Documents/pet/rustam/homework_2/ffi/target/debug"
    );
    println!("cargo:rustc-link-lib=dylib=synt_ex_lib"); // dylib instead of static
}
