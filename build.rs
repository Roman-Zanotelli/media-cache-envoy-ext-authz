fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    tonic_build::compile_protos("proto/ext_authz.proto")?;
    tonic_build::compile_protos("proto/media_lts.proto")?;
    print!("Generated Rust files at: {}", out_dir);
    Ok(())
 }