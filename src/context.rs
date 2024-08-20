use crate::assets_path::determine_asset_path;
use crate::audio;
use crate::consts::*;
use crate::font;
use crate::save::Save;
use crate::scene::EScene;
use crate::settings::Settings;
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
    pub fonts: font::FontAtlas,
    pub audio: audio::AudioAtlas,
    pub render_target: RenderTarget,
    pub render_target_cam: Camera2D,
    pub load_next_level: bool,
    pub tileset: tile::Tileset,
    pub switch_scene_to: Option<EScene>,
    /// whether or not to reload the level from disk at the end of the current game loop
    pub reload_level: bool,
    /// what pack is currently being played, if any. needed for reloading from disk
    pub current_pack_file: Option<String>,
    /// what level is currently being played, if any. needed for reloading from disk
    pub current_level_index: Option<usize>,
    pub settings: Settings,
    pub save: Save,
}

impl Context {
    pub async fn default() -> Self {
        let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);

        // Setup camera for the virtual screen, that will render to 'render_target'
        let mut render_target_cam =
            Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
        render_target_cam.render_target = Some(render_target.clone());

        let base_assets_path = determine_asset_path();

        Self {
            gamepads: Gamepads::new(),
            request_quit: false,
            textures: texture::TextureAtlas::new(&base_assets_path).await,
            audio: audio::AudioAtlas::new(&base_assets_path).await,
            fonts: font::FontAtlas::new(&base_assets_path).await,
            render_target,
            load_next_level: false,
            reload_level: false,
            render_target_cam,
            tileset: tile::Tileset::Doggo,
            switch_scene_to: None,
            current_pack_file: None,
            current_level_index: None,
            settings: Settings::load(),
            save: Save::load(),
        }
    }

    pub fn current_texture(&self) -> &Texture2D {
        match self.tileset {
            tile::Tileset::Retro => &self.textures.retro,
            tile::Tileset::Kenney => &self.textures.kenney,
            tile::Tileset::Doggo => &self.textures.doggo,
            tile::Tileset::Marble => &self.textures.marble,
        }
    }
}
