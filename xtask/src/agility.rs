//! `cargo xtask agility`: fetch the DirectX Agility SDK and put its DLLs in
//! `D3D12\` beside every build, where the exports of `victory.exe` and the
//! GPU test programs point (see crates/victory-shader/src/agility.rs).
//!
//! The package comes from nuget.org and is checked against a pinned hash
//! before anything is unpacked. It is kept under `target\agility\`, so
//! nothing of it is ever committed.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail, ensure};

/// Must match `victory_shader::agility::SDK_VERSION` (the middle number).
const VERSION: &str = "1.620.2";
/// BLAKE3 of the `.nupkg`.
const BLAKE3: &str = "b5811248412481b3fa9d77fbe3cc1c7703f6b961beca45c4a956f3792fcb0d8b";
/// The x64 runtime and its debug layer (`VICTORY_D3D12_DEBUG`).
const DLLS: [&str; 2] = ["D3D12Core.dll", "d3d12SDKLayers.dll"];
/// Build folders that get a copy (each also for its `deps\` test programs).
const PROFILES: [&str; 3] = ["debug", "release", "diag"];

pub fn run(root: &Path, to: Option<PathBuf>) -> Result<()> {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"));
    let cache = target.join("agility").join(VERSION);
    if !DLLS.iter().all(|d| cache.join(d).is_file()) {
        fetch(&cache)?;
    }
    let dirs: Vec<PathBuf> = match to {
        Some(dir) => vec![dir.join("D3D12")],
        None => PROFILES
            .iter()
            .flat_map(|p| {
                let p = target.join(p);
                [p.join("D3D12"), p.join("deps").join("D3D12")]
            })
            .collect(),
    };
    for dir in &dirs {
        fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        for dll in DLLS {
            fs::copy(cache.join(dll), dir.join(dll))
                .with_context(|| format!("copying {dll} to {}", dir.display()))?;
        }
        println!("Agility SDK {VERSION} -> {}", dir.display());
    }
    Ok(())
}

fn fetch(cache: &Path) -> Result<()> {
    fs::create_dir_all(cache)?;
    let package = cache.join("package.nupkg");
    let url = format!(
        "https://api.nuget.org/v3-flatcontainer/microsoft.direct3d.d3d12/{VERSION}/microsoft.direct3d.d3d12.{VERSION}.nupkg"
    );
    println!("downloading {url}");
    // curl.exe and tar.exe ship with Windows 10 (1803 and later). The
    // system tar is named in full: a GNU tar on PATH (Git's) reads no zip.
    let status = Command::new("curl")
        .args(["-fsSL", "--retry", "2", "-o"])
        .arg(&package)
        .arg(&url)
        .status()
        .context("running curl")?;
    ensure!(status.success(), "the download failed");
    let hash = blake3::hash(&fs::read(&package)?).to_hex().to_string();
    if hash != BLAKE3 {
        fs::remove_file(&package).ok();
        bail!("the package's BLAKE3 is {hash}, expected {BLAKE3}; not unpacked");
    }
    let system = std::env::var_os("SystemRoot").context("SystemRoot is not set")?;
    let tar = Path::new(&system).join("System32").join("tar.exe");
    let inside = |dll: &str| format!("build/native/bin/x64/{dll}");
    let status = Command::new(&tar)
        .arg("-xf")
        .arg(&package)
        .arg("-C")
        .arg(cache)
        .args(DLLS.map(inside))
        .status()
        .with_context(|| format!("running {}", tar.display()))?;
    ensure!(status.success(), "unpacking the package failed");
    for dll in DLLS {
        fs::rename(cache.join(inside(dll)), cache.join(dll))?;
    }
    fs::remove_dir_all(cache.join("build")).ok();
    fs::remove_file(&package).ok();
    Ok(())
}
