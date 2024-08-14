use super::Scene;
use crate::consts::*;
use crate::context::Context;
use crate::level::{Pack, PlayableLevel};

pub struct Gameplay {
    current_level: PlayableLevel,
    level_index: usize,
    pack: Pack,
}

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        self.current_level.update(ctx);

        if ctx.load_next_level {
            ctx.load_next_level = false;
            self.level_index += 1;
            if self.level_index >= self.pack.levels.len() {
                self.level_index = 0;
            }

            self.current_level =
                PlayableLevel::load(self.pack.levels.get(self.level_index).unwrap());
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.current_level.draw(ctx);
    }
}

impl Gameplay {
    pub async fn new(_ctx: &mut Context) -> Self {
        let args: Vec<String> = std::env::args().collect();

        let mut level_index = 0;
        if let Some(arg) = args.iter().find(|arg| arg.starts_with(LEVEL_CLI_ARG)) {
            level_index = arg.split(LEVEL_CLI_ARG).last().unwrap().parse().unwrap();
            level_index -= 1;
        };
        let mut pack_file = "assets/pack-a.toml";
        if let Some(arg) = args.iter().find(|arg| arg.starts_with(PACK_CLI_ARG)) {
            pack_file = arg.split(PACK_CLI_ARG).last().unwrap();
        };

        let level_pack_str = macroquad::file::load_string(pack_file)
            .await
            .expect("Unable to read file");
        let pack: Pack = toml::from_str(level_pack_str.as_str()).unwrap();
        let current_level = PlayableLevel::load(pack.levels.get(level_index).unwrap());

        Self {
            current_level,
            level_index,
            pack,
        }
    }
}
