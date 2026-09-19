fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../plugin.proto");
    tonic_prost_build::configure()
        .type_attribute(
            ".",
            "#[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize)]",
        )
        .compile_protos(&["../plugin.proto"], &[".."])?;
    Ok(())
}
