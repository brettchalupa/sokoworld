use macroquad::input::is_key_pressed;

use super::pause::Pause;
use super::{EScene, Scene};
use crate::audio::play_sfx;
use crate::context::Context;
use crate::input::action_pressed;
use crate::input::Action;
use crate::level::pack::PackLevel;
use crate::level::{pack::Pack, playable_level::PlayableLevel};

pub struct Gameplay {
    level: PlayableLevel,
    pack: Pack,
    level_index: usize,
    pause_subscene: Pause,
}

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.update(ctx);
        } else {
            if action_pressed(Action::Pause, &ctx.gamepads) {
                self.pause_subscene.active = true;
                play_sfx(ctx, &ctx.audio.sfx.menu_select);
            }

            self.level.update(ctx);

            if ctx.load_next_level {
                ctx.load_next_level = false;
                self.level_index += 1;
                if self.level_index >= self.pack.levels.len() {
                    ctx.switch_scene_to = Some(EScene::LevelSelect(self.pack.clone()));
                } else {
                    self.sync_to_ctx(ctx);
                    self.level = PlayableLevel::new(
                        self.pack.slug.clone(),
                        self.pack.levels.get(self.level_index).unwrap(),
                    );
                }
            }

            // reloads the current level from the pack file
            // unsure if this should be debug or not...
            if is_key_pressed(macroquad::miniquad::KeyCode::Key9) {
                play_sfx(ctx, &ctx.audio.sfx.reset);
                ctx.reload_level = true;
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.draw(ctx);
        } else {
            self.level.draw(ctx);
        }
    }
}

impl Gameplay {
    pub async fn new(ctx: &mut Context, level: PackLevel, level_index: usize, pack: Pack) -> Self {
        let level = PlayableLevel::new(pack.slug.clone(), &level);
        let pause_subscene = Pause::new(ctx, pack.clone());
        let mut gameplay = Self {
            level_index,
            level,
            pack,
            pause_subscene,
        };
        gameplay.sync_to_ctx(ctx);
        gameplay
    }

    /// sets the current pack file and level index on the Context for quickly reloading the
    /// currently played level from disk
    fn sync_to_ctx(&mut self, ctx: &mut Context) {
        ctx.current_pack_file = Some(self.pack.file.as_ref().unwrap().clone());
        ctx.current_level_index = Some(self.level_index);
    }
}
