use tonic_prost_build::compile_protos;

fn main() {
    println!("cargo:rerun-if-changed=proto/route_guide.proto");
    compile_protos("proto/route_guide.proto").unwrap();
}
