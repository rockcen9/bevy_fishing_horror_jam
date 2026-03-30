//! Scans the `assets/` directory and generates `assets/asset_manifest.json`.
//!
//! Run from the project root:
//!   cargo run -p asset_scanner

use serde::Serialize;
use std::path::Path;
use walkdir::WalkDir;

const ASSETS_DIR: &str = "assets";
const OUTPUT_FILE: &str = "assets/asset_manifest.json";

/// Files to skip during scanning.
const SKIP_NAMES: &[&str] = &[".DS_Store", "asset_manifest.json", ".gitkeep", ".gitignore"];

#[derive(Serialize, Default)]
struct AssetManifest {
    images: Vec<String>,
    audio: Vec<String>,
    fonts: Vec<String>,
    shaders: Vec<String>,
    data: Vec<String>,
    other: Vec<String>,
}

fn main() {
    let assets_path = Path::new(ASSETS_DIR);
    if !assets_path.exists() {
        eprintln!("ERROR: '{}' directory not found. Run from the project root.", ASSETS_DIR);
        std::process::exit(1);
    }

    let mut manifest = AssetManifest::default();

    for entry in WalkDir::new(assets_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let full_path = entry.path();
        let file_name = full_path.file_name().unwrap_or_default().to_string_lossy();

        if SKIP_NAMES.iter().any(|s| *s == file_name.as_ref()) {
            continue;
        }

        // Path relative to assets/, using forward slashes.
        let relative = full_path
            .strip_prefix(assets_path)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");

        let ext = full_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        match ext.as_ref() {
            "png" | "jpg" | "jpeg" | "webp" | "bmp" | "tga" => manifest.images.push(relative),
            "ogg" | "mp3" | "wav" | "flac" => manifest.audio.push(relative),
            "ttf" | "otf" => manifest.fonts.push(relative),
            "wgsl" | "glsl" | "spv" => manifest.shaders.push(relative),
            "json" | "csv" | "ron" | "toml" => manifest.data.push(relative),
            _ => manifest.other.push(relative),
        }
    }

    manifest.images.sort();
    manifest.audio.sort();
    manifest.fonts.sort();
    manifest.shaders.sort();
    manifest.data.sort();
    manifest.other.sort();

    let json = serde_json::to_string_pretty(&manifest).expect("Failed to serialize manifest");
    std::fs::write(OUTPUT_FILE, &json).expect("Failed to write asset_manifest.json");

    println!("Written to {OUTPUT_FILE}");
    println!("  images:  {}", manifest.images.len());
    println!("  audio:   {}", manifest.audio.len());
    println!("  fonts:   {}", manifest.fonts.len());
    println!("  shaders: {}", manifest.shaders.len());
    println!("  data:    {}", manifest.data.len());
    println!("  other:   {}", manifest.other.len());
}
