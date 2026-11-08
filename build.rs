fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .out_dir("./src/pulse_db/rpc/v1/proto")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["./proto/v1/pulse.proto"], &["./pulse_core/rpc/v1"])?;
    Ok(())
}
