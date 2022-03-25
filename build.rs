use std::fs;
use std::path::Path;

fn main() {
    let out_dir = String::from("src/binding/protos");
    if Path::new(&out_dir).exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }
    fs::create_dir(&out_dir).unwrap();

    let mut config = prost_build::Config::new();
    config.
    out_dir(out_dir).
    btree_map(&["."]).
    compile_protos(&["src/protos/rustproto.proto", "src/protos/pegged.proto"],
        &["src/protos"]).unwrap();
}
