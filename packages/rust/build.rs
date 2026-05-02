use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../..");
    let proto_root = root.join("proto");
    let protos = [proto_root.join("qsp/example/v1/example.proto")];

    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    env::set_var("PROTOC", protoc);

    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }
    println!("cargo:rerun-if-changed={}", proto_root.display());

    prost_build::compile_protos(&protos, &[proto_root])?;

    Ok(())
}
