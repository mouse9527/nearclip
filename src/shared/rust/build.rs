fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 重新构建条件
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=../protobuf/proto");

    // 编译 Protocol Buffers 文件
    prost_build::compile_protos(
        &[
            "../protobuf/proto/nearclip.proto",
            "../protobuf/proto/device.proto",
            "../protobuf/proto/sync.proto"
        ],
        &["../protobuf/proto"]
    )?;

    Ok(())
}