use macroquad::audio::Sound;

pub struct SfxAtlas {
    pub push: Sound,
    pub level_complete: Sound,
    pub reset: Sound,
}
pub struct AudioAtlas {
    pub sfx: SfxAtlas,
}

impl AudioAtlas {
    pub async fn new() -> Self {
        Self {
            sfx: SfxAtlas {
                push: macroquad::audio::load_sound("assets/push.wav")
                    .await
                    .unwrap(),
                level_complete: macroquad::audio::load_sound("assets/level_complete.wav")
                    .await
                    .unwrap(),
                reset: macroquad::audio::load_sound("assets/reset.wav")
                    .await
                    .unwrap(),
            },
        }
    }
}
