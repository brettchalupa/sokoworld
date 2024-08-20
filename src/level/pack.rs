use serde::Deserialize;
use std::fmt;

use crate::context::Context;

#[derive(Debug, Deserialize, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Difficulty::Easy => "Easy".to_string(),
            Difficulty::Medium => "Medium".to_string(),
            Difficulty::Hard => "Hard".to_string(),
        };

        write!(f, "{}", name)
    }
}

/// a collection of levels
#[derive(Debug, Deserialize, Clone)]
pub struct Pack {
    /// name of the pack
    pub title: String,
    /// id used for save data; NOTE: changing this will break saves, don't change
    pub slug: String,
    pub description: String,
    /// who compiled the levels in the pack, often just the designer
    pub author: String,
    pub license: String,
    /// initial year the pack was releaesd
    pub year: i32,
    pub levels: Vec<PackLevel>,
    pub version: String,
    /// file path of where the Pack is located
    pub file: Option<String>,
    /// the general difficulty level of the pack
    pub difficulty: Difficulty,
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
    pub async fn load(_ctx: &mut Context, pack_file: &str) -> Self {
        let level_pack_str = macroquad::file::load_string(pack_file)
            .await
            .expect("Unable to read file");
        Pack {
            file: Some(pack_file.to_string()),
            ..toml::from_str(level_pack_str.as_str()).unwrap()
        }
    }
}
