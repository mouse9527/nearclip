use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Protocol Buffers 源文件目录
    let proto_dir = "../../protocol";

    // 生成所有协议文件
    prost_build::compile_protos(
        &[
            format!("{}/discovery.proto", proto_dir),
            format!("{}/pairing.proto", proto_dir),
            format!("{}/sync.proto", proto_dir),
            format!("{}/common.proto", proto_dir),
        ],
        &[proto_dir],
    )?;

    println!("cargo:rerun-if-changed={}", proto_dir);

    Ok(())
}