use super::{EScene, Scene};
use crate::context::Context;
use crate::level::pack::PackLevel;
use crate::level::{pack::Pack, PlayableLevel};

pub struct Gameplay {
    level: PlayableLevel,
    pack: Pack,
    level_index: usize,
}

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        self.level.update(ctx);

        if ctx.load_next_level {
            ctx.load_next_level = false;
            self.level_index += 1;
            if self.level_index >= self.pack.levels.len() {
                ctx.switch_scene_to = Some(EScene::LevelSelect(self.pack.clone()));
            } else {
                self.level = PlayableLevel::load(self.pack.levels.get(self.level_index).unwrap());
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.level.draw(ctx);
    }
}

impl Gameplay {
    // TODO: determine level_index dynamically based on where level is in the pack
    pub async fn new(_ctx: &mut Context, level: PackLevel, level_index: usize, pack: Pack) -> Self {
        let level = PlayableLevel::load(&level);
        Self {
            level_index,
            level,
            pack,
        }
    }
}
