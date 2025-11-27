fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../plugin.proto");
    tonic_prost_build::compile_protos("../plugin.proto")?;
    Ok(())
}
