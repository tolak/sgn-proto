extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/binding/protos")
        .include("src/protos")
        .inputs(&["src/protos/pegged.proto"])
        .run()
        .expect("Running protoc failed.");
}
