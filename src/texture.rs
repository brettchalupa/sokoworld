use macroquad::texture::{load_texture, Texture2D};

pub struct TextureAtlas {
    pub player: Texture2D,
    pub krate: Texture2D,
    pub wall: Texture2D,
    pub ground: Texture2D,
    pub storage_location: Texture2D,
}

impl TextureAtlas {
    pub async fn new() -> Self {
        Self {
            player: load_texture("assets/player.png").await.unwrap(),
            krate: load_texture("assets/crate.png").await.unwrap(),
            wall: load_texture("assets/wall.png").await.unwrap(),
            ground: load_texture("assets/ground.png").await.unwrap(),
            storage_location: load_texture("assets/storage_location.png").await.unwrap(),
        }
    }
}
