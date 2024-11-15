use super::{EScene, Scene};
use crate::audio::play_sfx;
use crate::color::BLUE;
use crate::consts::*;
use crate::context::Context;
use crate::input::{action_down, action_pressed, Action};
use crate::level::pack::Pack;
use crate::text::{self, draw_text};
use macroquad::color::{RED, WHITE};
use macroquad::time::get_frame_time;

pub struct LevelSelect {
    pack: Pack,
    focused_level_index: i32,
    move_held_delay: f32,
}

impl Scene for LevelSelect {
    fn update(&mut self, ctx: &mut Context) {
        if self.move_held_delay > 0.0 {
            self.move_held_delay -= get_frame_time();
        }

        if action_pressed(Action::Left, &ctx.gamepads)
            || (action_down(Action::Left, &ctx.gamepads) && self.move_held_delay <= 0.)
        {
            self.move_held_delay = MOVE_HELD_DELAY;
            play_sfx(ctx, &ctx.audio.sfx.menu_move);
            self.focused_level_index -= 1;
            if self.focused_level_index < 0 {
                self.focused_level_index = (self.pack.levels.len() - 1) as i32;
            }
        }
        if action_pressed(Action::Right, &ctx.gamepads)
            || (action_down(Action::Right, &ctx.gamepads) && self.move_held_delay <= 0.)
        {
            self.move_held_delay = MOVE_HELD_DELAY;
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
            let title = level.title.clone();
            let is_level_complete = ctx.save.is_level_complete(&self.pack.slug, &title);

            let color = if self.focused_level_index == i as i32 {
                RED
            } else if is_level_complete {
                BLUE
            } else {
                WHITE
            };

            let title_x = (i as i32 - self.focused_level_index) as f32 * 180. + X_INSET;
            let title_y = VIRTUAL_HEIGHT / 2. - 58.;
            draw_text(
                ctx,
                title.as_str(),
                title_x,
                title_y,
                text::Size::Medium,
                color,
            );

            if is_level_complete {
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
            X_INSET,
            VIRTUAL_HEIGHT - 120.,
            text::Size::Medium,
            WHITE,
        );
    }
}

impl LevelSelect {
    pub async fn new(ctx: &mut Context, pack: Pack) -> Self {
        let mut focused_level_index: i32 = 0;

        for (i, level) in pack.levels.iter().enumerate() {
            if !ctx.save.is_level_complete(&pack.slug, &level.title.clone()) {
                focused_level_index = i as i32;
                break;
            }
        }

        Self {
            pack,
            focused_level_index,
            move_held_delay: 0.,
        }
    }
}
