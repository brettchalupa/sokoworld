use macroquad::texture::{load_texture, Texture2D};

pub struct TextureAtlas {
    pub retro: Texture2D,
    pub doggo: Texture2D,
    pub kenney: Texture2D,
    pub marble: Texture2D,
}

impl TextureAtlas {
    pub async fn new() -> Self {
        let retro = load_texture("assets/sprites/retro.png").await.unwrap();
        retro.set_filter(macroquad::miniquad::FilterMode::Nearest);

        let doggo = load_texture("assets/sprites/doggo.png").await.unwrap();
        doggo.set_filter(macroquad::miniquad::FilterMode::Nearest);

        let kenney = load_texture("assets/sprites/kenney.png").await.unwrap();
        kenney.set_filter(macroquad::miniquad::FilterMode::Linear);

        let marble = load_texture("assets/sprites/marble.png").await.unwrap();
        marble.set_filter(macroquad::miniquad::FilterMode::Linear);

        Self {
            doggo,
            kenney,
            retro,
            marble,
        }
    }
}
