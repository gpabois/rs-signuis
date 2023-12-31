use std::{path::PathBuf, env};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
    .file_descriptor_set_path(out_dir.join("signuis_descriptor.bin"))
    .compile(&["../proto/signuis.proto"], &["../proto"])?;

    Ok(())
}