fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["proto/proxy.proto", "proto/report.proto"], &["proto/"])?;
    Ok(())
}
