use super::{EScene, Scene};
use crate::consts::*;
use crate::context::Context;
use crate::input::action_pressed;
use macroquad::color::WHITE;
use macroquad::text::draw_text;

pub struct MainMenu {}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) {
        if action_pressed(crate::input::Action::Confirm, &ctx.gamepads) {
            ctx.switch_scene_to = Some(EScene::Gameplay);
        }
    }
    fn draw(&mut self, _ctx: &mut Context) {
        draw_text(
            "SokoWorld",
            VIRTUAL_WIDTH / 2. - 140.,
            VIRTUAL_HEIGHT / 2.,
            64.,
            WHITE,
        );

        draw_text(
            "Press [Confirm] to start",
            VIRTUAL_WIDTH / 2. - 140.,
            VIRTUAL_HEIGHT / 2. + 58.,
            32.,
            WHITE,
        );
    }
}

impl MainMenu {
    pub async fn new(_ctx: &mut Context) -> Self {
        Self {}
    }
}
