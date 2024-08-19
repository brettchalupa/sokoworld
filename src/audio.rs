use std::path::Path;

use macroquad::audio::Sound;

use crate::context::Context;

pub struct SfxAtlas {
    pub push: Sound,
    pub crate_on_storage_location: Sound,
    pub level_complete: Sound,
    pub reset: Sound,
    pub footstep: Sound,
    pub menu_cancel: Sound,
    pub menu_select: Sound,
    pub menu_move: Sound,
    pub cant_move: Sound,
}
pub struct AudioAtlas {
    pub sfx: SfxAtlas,
}

impl AudioAtlas {
    pub async fn new(base_assets_path: &Path) -> Self {
        Self {
            sfx: SfxAtlas {
                push: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/dialogBlip2.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                level_complete: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/get_item.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                reset: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/save.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                footstep: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/footstep_4.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                menu_cancel: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/menuCancel.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                menu_select: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/menuSelect.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                menu_move: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/menuMove.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                crate_on_storage_location: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/pieceSelect.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
                cant_move: macroquad::audio::load_sound(
                    base_assets_path
                        .join("sfx/melos/pieceCantPlace.wav")
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap(),
            },
        }
    }
}

pub fn play_sfx(ctx: &Context, sfx: &Sound) {
    if ctx.settings.is_muted() {
        return;
    }

    macroquad::audio::play_sound_once(sfx);
}
