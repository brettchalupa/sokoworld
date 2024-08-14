use serde::Deserialize;

/// a collection of levels
#[derive(Debug, Deserialize, Clone)]
pub struct Pack {
    pub title: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub year: i32,
    pub levels: Vec<PackLevel>,
    pub version: String,
}

/// a level defined in a pack file
#[derive(Debug, Deserialize, Clone)]
pub struct PackLevel {
    /// name of the level
    pub title: String,
    /// grid of the puzzle's elements
    pub data: String,
}

impl Pack {
    pub async fn load(pack_file: &str) -> Self {
        let level_pack_str = macroquad::file::load_string(pack_file)
            .await
            .expect("Unable to read file");
        let pack: Pack = toml::from_str(level_pack_str.as_str()).unwrap();
        pack
    }
}
