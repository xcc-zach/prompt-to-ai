use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_assets.rs");
    let mut file = File::create(&dest_path).unwrap();

    let assets_dir = Path::new("src/assets");

    println!("cargo:rerun-if-changed=src/assets");

    let mut entries = Vec::new();
    if assets_dir.exists() {
        collect_files(assets_dir, assets_dir, &mut entries);
    }

    writeln!(file, "pub static ASSETS: &[EmbeddedFile] = &[").unwrap();
    for (relative_path, full_path) in &entries {
        println!("cargo:rerun-if-changed={}", full_path);
        // Use forward slashes for consistent paths
        let normalized_path = relative_path.replace('\\', "/");
        writeln!(
            file,
            r#"    EmbeddedFile {{ path: "{}", data: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/{}")) }},"#,
            normalized_path,
            full_path.replace('\\', "/")
        )
        .unwrap();
    }
    writeln!(file, "];").unwrap();
}

fn collect_files(base: &Path, dir: &Path, entries: &mut Vec<(String, String)>) {
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(base, &path, entries);
            } else if path.is_file() {
                let relative_to_base = path.strip_prefix(base).unwrap();
                let relative_path = relative_to_base.to_string_lossy().to_string();
                let full_path = path.to_string_lossy().to_string();
                entries.push((relative_path, full_path));
            }
        }
    }
}
