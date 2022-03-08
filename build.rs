extern crate protoc_rust;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or(String::from("src/binding/protos"));
    if Path::new(&out_dir).exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }
    fs::create_dir(&out_dir).unwrap();

    protoc_rust::Codegen::new()
        .customize(protoc_rust::Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir(out_dir)
        .include("src/protos")
        .inputs(&["src/protos/rustproto.proto", "src/protos/pegged.proto"])
        .run()
        .expect("Running protoc failed.");
}
