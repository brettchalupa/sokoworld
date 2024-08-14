use crate::audio;
use crate::consts::*;
use crate::texture;
use crate::tile;
use gamepads::Gamepads;
use macroquad::math::Rect;
use macroquad::miniquad::FilterMode;
use macroquad::texture::render_target;
use macroquad::texture::Texture2D;
use macroquad::{camera::Camera2D, texture::RenderTarget};

/// game-wide data and resources
pub struct Context {
    pub request_quit: bool,
    pub gamepads: Gamepads,
    pub textures: texture::TextureAtlas,
    pub audio: audio::AudioAtlas,
    pub render_target: RenderTarget,
    pub render_target_cam: Camera2D,
    pub load_next_level: bool,
    pub tileset: tile::Tileset,
}

impl Context {
    pub async fn default() -> Self {
        let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);

        // Setup camera for the virtual screen, that will render to 'render_target'
        let mut render_target_cam =
            Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
        render_target_cam.render_target = Some(render_target.clone());

        Self {
            gamepads: Gamepads::new(),
            request_quit: false,
            textures: texture::TextureAtlas::new().await,
            audio: audio::AudioAtlas::new().await,
            render_target,
            load_next_level: false,
            render_target_cam,
            tileset: tile::Tileset::Retro,
        }
    }

    pub fn current_texture(&self) -> &Texture2D {
        match self.tileset {
            tile::Tileset::Retro => &self.textures.retro,
            tile::Tileset::Kenney => &self.textures.kenney,
        }
    }
}
