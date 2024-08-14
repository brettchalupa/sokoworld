use macroquad::texture::{load_texture, Texture2D};

pub struct TextureAtlas {
    pub retro: Texture2D,
    pub kenney: Texture2D,
}

impl TextureAtlas {
    pub async fn new() -> Self {
        let retro = load_texture("assets/sokoworld-retro.png").await.unwrap();
        retro.set_filter(macroquad::miniquad::FilterMode::Nearest);
        let kenney = load_texture("assets/sokoworld-kenney.png").await.unwrap();
        kenney.set_filter(macroquad::miniquad::FilterMode::Linear);
        Self { retro, kenney }
    }
}
