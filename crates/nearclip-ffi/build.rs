//! Build script for nearclip-ffi
//!
//! Generates uniffi scaffolding from the UDL file.

fn main() {
    uniffi::generate_scaffolding("src/nearclip.udl").expect("Failed to generate uniffi scaffolding");
}
