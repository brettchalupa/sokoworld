use super::{EScene, Scene};
use crate::audio::play_sfx;
use crate::consts::*;
use crate::context::Context;
use crate::input::{action_pressed, Action};
use crate::level::pack::Pack;
use crate::text::{self, draw_text};
use macroquad::color::{RED, WHITE};

pub struct LevelSelect {
    pack: Pack,
    focused_level_index: i32,
}

impl Scene for LevelSelect {
    fn update(&mut self, ctx: &mut Context) {
        if action_pressed(Action::Left, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_move);
            self.focused_level_index -= 1;
            if self.focused_level_index < 0 {
                self.focused_level_index = (self.pack.levels.len() - 1) as i32;
            }
        }
        if action_pressed(Action::Right, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_move);
            self.focused_level_index += 1;
            if self.focused_level_index > (self.pack.levels.len() - 1) as i32 {
                self.focused_level_index = 0;
            }
        }

        if action_pressed(Action::Confirm, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_select);
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
            play_sfx(ctx, &ctx.audio.sfx.menu_cancel);
            ctx.switch_scene_to = Some(EScene::MainMenu);
        }
    }
    fn draw(&mut self, ctx: &mut Context) {
        draw_text(
            ctx,
            format!("{} by {}", self.pack.title, self.pack.author).as_str(),
            X_INSET,
            TITLE_Y_INSET,
            text::Size::Large,
            WHITE,
        );
        draw_text(
            ctx,
            "Select a Level",
            X_INSET,
            TITLE_Y_INSET + 60.,
            text::Size::Medium,
            WHITE,
        );

        for (i, level) in &mut self.pack.levels.iter().enumerate() {
            let color = if self.focused_level_index == i as i32 {
                RED
            } else {
                WHITE
            };

            let title_x = (i as i32 - self.focused_level_index) as f32 * 180. + X_INSET;
            let title_y = VIRTUAL_HEIGHT / 2. - 58.;

            let title = level.title.clone();
            draw_text(
                ctx,
                title.as_str(),
                title_x,
                title_y,
                text::Size::Medium,
                color,
            );

            if ctx.save.is_level_complete(&self.pack.slug, &title) {
                draw_text(
                    ctx,
                    "complete",
                    title_x,
                    title_y + 40.,
                    text::Size::Small,
                    color,
                );
            }
        }

        draw_text(
            ctx,
            "Press Z to select level",
            VIRTUAL_WIDTH / 2. - 180.,
            VIRTUAL_HEIGHT - 120.,
            text::Size::Medium,
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
