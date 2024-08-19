use std::path::Path;

use macroquad::texture::{load_texture, Texture2D};

pub struct TextureAtlas {
    pub retro: Texture2D,
    pub doggo: Texture2D,
    pub kenney: Texture2D,
    pub marble: Texture2D,
}

impl TextureAtlas {
    pub async fn new(base_assets_path: &Path) -> Self {
        let retro = load_texture(base_assets_path.join("sprites/retro.png").to_str().unwrap())
            .await
            .unwrap();
        retro.set_filter(macroquad::miniquad::FilterMode::Nearest);

        let doggo = load_texture(base_assets_path.join("sprites/doggo.png").to_str().unwrap())
            .await
            .unwrap();
        doggo.set_filter(macroquad::miniquad::FilterMode::Nearest);

        let kenney = load_texture(
            base_assets_path
                .join("sprites/kenney.png")
                .to_str()
                .unwrap(),
        )
        .await
        .unwrap();
        kenney.set_filter(macroquad::miniquad::FilterMode::Linear);

        let marble = load_texture(
            base_assets_path
                .join("sprites/marble.png")
                .to_str()
                .unwrap(),
        )
        .await
        .unwrap();
        marble.set_filter(macroquad::miniquad::FilterMode::Linear);

        Self {
            doggo,
            kenney,
            retro,
            marble,
        }
    }
}
