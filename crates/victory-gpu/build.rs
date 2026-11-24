// Exports the DirectX Agility SDK values `agility_exports!` defines, so that
// Windows loads the newer D3D12 runtime from `D3D12\` beside the program
// (see crates/victory-shader/src/agility.rs). crates/victory-gpu and
// crates/victory-shader have the same script for their GPU tests.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        for name in ["D3D12SDKVersion", "D3D12SDKPath"] {
            println!("cargo:rustc-link-arg=/EXPORT:{name},DATA");
        }
    }
}
