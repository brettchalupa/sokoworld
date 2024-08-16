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
    pub async fn new() -> Self {
        Self {
            sfx: SfxAtlas {
                push: macroquad::audio::load_sound("assets/sfx/melos/dialogBlip2.wav")
                    .await
                    .unwrap(),
                level_complete: macroquad::audio::load_sound("assets/sfx/melos/get_item.wav")
                    .await
                    .unwrap(),
                reset: macroquad::audio::load_sound("assets/sfx/melos/save.wav")
                    .await
                    .unwrap(),
                footstep: macroquad::audio::load_sound("assets/sfx/melos/footstep_4.wav")
                    .await
                    .unwrap(),
                menu_cancel: macroquad::audio::load_sound("assets/sfx/melos/menuCancel.wav")
                    .await
                    .unwrap(),
                menu_select: macroquad::audio::load_sound("assets/sfx/melos/menuSelect.wav")
                    .await
                    .unwrap(),
                menu_move: macroquad::audio::load_sound("assets/sfx/melos/menuMove.wav")
                    .await
                    .unwrap(),
                crate_on_storage_location: macroquad::audio::load_sound(
                    "assets/sfx/melos/pieceSelect.wav",
                )
                .await
                .unwrap(),
                cant_move: macroquad::audio::load_sound("assets/sfx/melos/pieceCantPlace.wav")
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
