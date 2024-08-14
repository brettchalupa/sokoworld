use super::{EScene, Scene};
use crate::consts::*;
use crate::context::Context;
use crate::input::{action_pressed, Action};
use crate::level::pack::Pack;
use macroquad::color::{BLUE, WHITE};
use macroquad::text::draw_text;

pub struct LevelSelect {
    pack: Pack,
    focused_level_index: i32,
}

impl Scene for LevelSelect {
    fn update(&mut self, ctx: &mut Context) {
        if action_pressed(Action::Left, &ctx.gamepads) {
            self.focused_level_index -= 1;
            if self.focused_level_index < 0 {
                self.focused_level_index = (self.pack.levels.len() - 1) as i32;
            }
        }
        if action_pressed(Action::Right, &ctx.gamepads) {
            self.focused_level_index += 1;
            if self.focused_level_index > (self.pack.levels.len() - 1) as i32 {
                self.focused_level_index = 0;
            }
        }

        if action_pressed(Action::Confirm, &ctx.gamepads) {
            let level = self
                .pack
                .levels
                .get(self.focused_level_index as usize)
                .expect("level index not present in loaded pack");
            ctx.switch_scene_to = Some(EScene::Gameplay(
                level.to_owned(),
                self.focused_level_index as usize,
                self.pack.to_owned(),
            ));
        }

        if action_pressed(Action::Cancel, &ctx.gamepads) {
            ctx.switch_scene_to = Some(EScene::MainMenu);
        }
    }
    fn draw(&mut self, _ctx: &mut Context) {
        draw_text(
            format!("{} by {}", self.pack.title, self.pack.author).as_str(),
            VIRTUAL_WIDTH / 2. - 300.,
            60.,
            42.,
            WHITE,
        );
        draw_text("Select a Level", VIRTUAL_WIDTH / 2. - 90., 120., 32., WHITE);

        for (i, level) in &mut self.pack.levels.iter().enumerate() {
            let color = if self.focused_level_index == i as i32 {
                BLUE
            } else {
                WHITE
            };

            let title_x = i as f32 * 180.0 + 140.;
            let title_y = VIRTUAL_HEIGHT / 2. - 58.;

            draw_text(level.title.as_str(), title_x, title_y, 32., color);
        }

        draw_text(
            "Press Z to select level",
            VIRTUAL_WIDTH / 2. - 180.,
            VIRTUAL_HEIGHT - 120.,
            32.,
            WHITE,
        );
    }
}

impl LevelSelect {
    pub async fn new(_ctx: &mut Context, pack: Pack) -> Self {
        Self {
            pack,
            focused_level_index: 0,
        }
    }
}
