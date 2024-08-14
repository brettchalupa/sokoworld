use super::{EScene, Scene};
use crate::consts::*;
use crate::context::Context;
use crate::input::{action_pressed, Action};
use crate::level::pack::Pack;
use crate::text::{self, draw_text};
use macroquad::color::{BLUE, WHITE};

pub struct MainMenu {
    packs: Vec<Pack>,
    focused_pack_index: i32,
}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) {
        if action_pressed(Action::Left, &ctx.gamepads) {
            self.focused_pack_index -= 1;
            if self.focused_pack_index < 0 {
                self.focused_pack_index = (self.packs.len() - 1) as i32;
            }
        }
        if action_pressed(Action::Right, &ctx.gamepads) {
            self.focused_pack_index += 1;
            if self.focused_pack_index > (self.packs.len() - 1) as i32 {
                self.focused_pack_index = 0;
            }
        }

        if action_pressed(Action::Confirm, &ctx.gamepads) {
            let pack = self
                .packs
                .get(self.focused_pack_index as usize)
                .expect("pack index not present in loaded packs");
            ctx.switch_scene_to = Some(EScene::LevelSelect(pack.to_owned()));
        }
    }
    fn draw(&mut self, ctx: &mut Context) {
        draw_text(
            ctx,
            "SokoWorld",
            VIRTUAL_WIDTH / 2. - 140.,
            120.,
            text::Size::Large,
            WHITE,
        );

        for (i, pack) in &mut self.packs.iter().enumerate() {
            let color = if self.focused_pack_index == i as i32 {
                BLUE
            } else {
                WHITE
            };

            let title_x = i as f32 * 320.0 + 280.;
            let title_y = VIRTUAL_HEIGHT / 2. - 58.;

            draw_text(
                ctx,
                pack.title.as_str(),
                title_x,
                title_y,
                text::Size::Medium,
                color,
            );
            draw_text(
                ctx,
                pack.author.as_str(),
                title_x,
                title_y + 24.,
                text::Size::Small,
                color,
            );
            draw_text(
                ctx,
                format!("{} levels", pack.levels.len()).as_str(),
                title_x,
                title_y + 48.,
                text::Size::Small,
                color,
            );
        }

        draw_text(
            ctx,
            "Press Z to select level pack",
            VIRTUAL_WIDTH / 2. - 180.,
            VIRTUAL_HEIGHT - 120.,
            text::Size::Medium,
            WHITE,
        );
    }
}

impl MainMenu {
    pub async fn new(_ctx: &mut Context) -> Self {
        let packs = vec![
            Pack::load("assets/pack-a.toml").await,
            Pack::load("assets/yoshio-murase-automatic.toml").await,
        ];

        Self {
            packs,
            focused_pack_index: 0,
        }
    }
}
