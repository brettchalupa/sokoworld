use macroquad::color::{RED, WHITE};

use super::Scene;
use crate::audio::play_sfx;
use crate::consts::VIRTUAL_HEIGHT;
use crate::input::action_pressed;
use crate::input::Action;
use crate::text::Size;
use crate::{context::Context, text::draw_text};

/// sub-scene for displaying who worked on the game
pub struct Credits {
    pub active: bool,
}
impl Credits {
    pub fn new(_ctx: &Context) -> Self {
        Self { active: false }
    }
}

const X_ALIGN: f32 = 200.;

impl Scene for Credits {
    fn update(&mut self, ctx: &mut Context) {
        if action_pressed(Action::Cancel, &ctx.gamepads)
            || action_pressed(Action::Confirm, &ctx.gamepads)
        {
            play_sfx(ctx, &ctx.audio.sfx.menu_cancel);
            self.active = false;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw_text(ctx, "Credits", X_ALIGN, 128., Size::Large, WHITE);

        draw_text(
            ctx,
            "Art by Chrysalis, Vellidragon, kenney, and Brett Chalupa",
            X_ALIGN,
            240.,
            Size::Small,
            WHITE,
        );
        draw_text(
            ctx,
            "Sound effects by Melos Han-Tani",
            X_ALIGN,
            300.,
            Size::Small,
            WHITE,
        );
        draw_text(
            ctx,
            "Level design by Brett Chalupa and Yoshio Murase",
            X_ALIGN,
            360.,
            Size::Small,
            WHITE,
        );
        draw_text(
            ctx,
            "Programming by Brett Chalupa",
            X_ALIGN,
            420.,
            Size::Small,
            WHITE,
        );

        draw_text(
            ctx,
            "Press Z or X to return",
            X_ALIGN,
            VIRTUAL_HEIGHT - 120.,
            Size::Medium,
            RED,
        );
    }
}
