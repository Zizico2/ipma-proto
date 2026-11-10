use prost::Message;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("../proto");
    println!("cargo:rerun-if-changed={}", schema_dir.display());
    let descriptors = protox::compile(
        [schema_dir.join("ipma/proxy/v1/proxy.proto")],
        [&schema_dir],
    )?;
    std::fs::write(
        PathBuf::from(std::env::var("OUT_DIR")?).join("ipma_descriptor.bin"),
        descriptors.encode_to_vec(),
    )?;
    tonic_prost_build::configure()
        .bytes(".ipma.proxy.v1.GetGeoDatasetResponse.geobuf")
        .compile_fds(descriptors)?;
    Ok(())
}
