/// Embedded assets - automatically generated at build time from src/assets/

pub struct EmbeddedFile {
    pub path: &'static str,
    pub data: &'static [u8],
}

// Include the auto-generated assets list from build.rs
include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));

pub fn get_asset(path: &str) -> Option<&'static [u8]> {
    ASSETS.iter().find(|f| f.path == path).map(|f| f.data)
}

pub fn list_assets() -> impl Iterator<Item = &'static EmbeddedFile> {
    ASSETS.iter()
}
