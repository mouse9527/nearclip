fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use relative paths from the crate root
    let proto_files = vec![
        "../protocol/device_discovery.proto",
        "../protocol/data_sync.proto",
        "../protocol/error_handling.proto",
    ];

    // Create src/generated directory if it doesn't exist
    std::fs::create_dir_all("src/generated")?;

    prost_build::compile_protos(
        &proto_files,
        &["../protocol/"],
    )?;

    println!("cargo:rerun-if-changed=../protocol/");

    Ok(())
}