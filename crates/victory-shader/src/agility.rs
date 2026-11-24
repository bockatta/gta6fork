//! The DirectX Agility SDK: a newer D3D12 runtime shipped beside the program.
//!
//! Translated shaders need shader model 6.6 (and the world needs mesh
//! shaders). The D3D12 runtime built into Windows 10 stops at 6.5 even on a
//! card that does more, so on Windows 10 every device reports too low a model
//! and the world does not draw. A program that exports `D3D12SDKVersion` and
//! `D3D12SDKPath` makes Windows load `D3D12Core.dll` from that path instead,
//! when the file is there and newer than the built-in runtime; otherwise
//! Windows keeps its own, so the exports never hurt.
//!
//! [`agility_exports!`](crate::agility_exports) defines the two values; a
//! program's build script exports them (`/EXPORT:<name>,DATA`), and
//! `cargo xtask agility` puts the DLLs in `D3D12\` beside the builds.

/// The Agility SDK release (`1.<SDK_VERSION>.x`) `cargo xtask agility`
/// fetches; Windows requires the DLL to be this version.
pub const SDK_VERSION: u32 = 620;

/// Where the DLLs go, relative to the program's folder (NUL-terminated).
pub const SDK_PATH: &[u8; 9] = b".\\D3D12\\\0";

/// Defines `D3D12SDKVersion` and `D3D12SDKPath` in the program that invokes
/// it, at most once per program. The program's build script must also export
/// them (see `apps/victory/build.rs`).
#[macro_export]
macro_rules! agility_exports {
    () => {
        #[allow(non_upper_case_globals)]
        #[unsafe(no_mangle)]
        #[used]
        pub static D3D12SDKVersion: u32 = $crate::agility::SDK_VERSION;

        #[allow(non_upper_case_globals)]
        #[unsafe(no_mangle)]
        #[used]
        pub static D3D12SDKPath: &[u8; 9] = $crate::agility::SDK_PATH;
    };
}
