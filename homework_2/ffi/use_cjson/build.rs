use std::env;

fn main() {
    let bindgen = bindgen::Builder::default()
        .header("third/cJSON.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for cJSON");

    // check on ./target/debug/build/use_cjson-.../out/bindings.rs
    let out_path = env::var("OUT_DIR").unwrap();
    bindgen
        .write_to_file(format!("{}/bindings.rs", out_path))
        .expect("Failed to write bindings");

    cc::Build::new().file("third/cJSON.c").compile("cjson"); // compile the cJSON source file and link it with the Rust binary
}
