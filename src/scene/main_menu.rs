use super::credits::Credits;
use super::settings::Settings;
use super::{EScene, Scene};
use crate::assets_path::determine_asset_path;
use crate::audio::play_sfx;
use crate::consts::*;
use crate::context::Context;
use crate::input::{action_down, action_pressed, Action};
use crate::level::pack::Pack;
use crate::text::{self, draw_text};
use macroquad::color::{RED, WHITE};
use macroquad::time::get_frame_time;

pub struct MainMenu {
    packs: Vec<Pack>,
    focused_pack_index: i32,
    menu_options: Vec<MenuOption>,
    menu_index: usize,
    settings_subscene: Settings,
    credits_subscene: Credits,
    move_held_delay: f32,
    packs_complete_count: Vec<i32>,
}

enum MenuOption {
    PackSelect,
    Settings,
    Credits,
    #[cfg(not(target_family = "wasm"))]
    Quit,
}

impl MainMenu {
    pub async fn new(ctx: &mut Context) -> Self {
        let base_assets_path = determine_asset_path();

        let menu_options = vec![
            MenuOption::PackSelect,
            MenuOption::Settings,
            MenuOption::Credits,
            #[cfg(not(target_family = "wasm"))]
            MenuOption::Quit,
        ];

        let packs = vec![
            Pack::load(
                ctx,
                base_assets_path.join("packs/pack-a.toml").to_str().unwrap(),
            )
            .await,
            Pack::load(
                ctx,
                base_assets_path
                    .join("packs/yoshio-murase-automatic.toml")
                    .to_str()
                    .unwrap(),
            )
            .await,
        ];

        let mut packs_complete_count = vec![];
        for pack in &packs {
            packs_complete_count.push(
                pack.levels
                    .iter()
                    .map(|l| {
                        if ctx.save.is_level_complete(&pack.slug, &l.title.clone()) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum(),
            )
        }

        Self {
            menu_options,
            menu_index: 0,
            packs,
            focused_pack_index: 0,
            settings_subscene: Settings::new(ctx, false),
            credits_subscene: Credits::new(ctx),
            packs_complete_count,
            move_held_delay: 0.,
        }
    }

    fn text_for_menu_option(&self, menu_option: &MenuOption) -> &str {
        match menu_option {
            MenuOption::PackSelect => "",
            MenuOption::Settings => "Settings",
            MenuOption::Credits => "Credits",
            #[cfg(not(target_family = "wasm"))]
            MenuOption::Quit => "Quit",
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) {
        if self.settings_subscene.active {
            self.settings_subscene.update(ctx);
            return;
        }

        if self.credits_subscene.active {
            self.credits_subscene.update(ctx);
            return;
        }

        if self.move_held_delay > 0.0 {
            self.move_held_delay -= get_frame_time();
        }

        let menu_option = self
            .menu_options
            .get(self.menu_index)
            .expect("pause menu index out of bounds");

        if matches!(menu_option, MenuOption::PackSelect) {
            if action_pressed(Action::Left, &ctx.gamepads)
                || (action_down(Action::Left, &ctx.gamepads) && self.move_held_delay <= 0.)
            {
                self.move_held_delay = MOVE_HELD_DELAY;
                play_sfx(ctx, &ctx.audio.sfx.menu_move);
                self.focused_pack_index -= 1;
                if self.focused_pack_index < 0 {
                    self.focused_pack_index = (self.packs.len() - 1) as i32;
                }
            }
            if action_pressed(Action::Right, &ctx.gamepads)
                || (action_down(Action::Right, &ctx.gamepads) && self.move_held_delay <= 0.)
            {
                self.move_held_delay = MOVE_HELD_DELAY;
                play_sfx(ctx, &ctx.audio.sfx.menu_move);
                self.focused_pack_index += 1;

                if self.focused_pack_index > (self.packs.len() - 1) as i32 {
                    self.focused_pack_index = 0;
                }
            }
        }

        if action_pressed(Action::Confirm, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_select);

            match menu_option {
                MenuOption::PackSelect => {
                    let pack = self
                        .packs
                        .get(self.focused_pack_index as usize)
                        .expect("pack index not present in loaded packs");
                    ctx.switch_scene_to = Some(EScene::LevelSelect(pack.to_owned()));
                }
                MenuOption::Settings => {
                    self.settings_subscene.active = true;
                }
                MenuOption::Credits => {
                    self.credits_subscene.active = true;
                }
                #[cfg(not(target_family = "wasm"))]
                MenuOption::Quit => {
                    ctx.request_quit = true;
                }
            }
        }

        if action_pressed(Action::Up, &ctx.gamepads)
            || (action_down(Action::Up, &ctx.gamepads) && self.move_held_delay <= 0.)
        {
            self.move_held_delay = MOVE_HELD_DELAY;
            play_sfx(ctx, &ctx.audio.sfx.menu_move);

            if self.menu_index == 0 {
                self.menu_index = self.menu_options.len() - 1;
            } else {
                self.menu_index -= 1;
            }
        }
        if action_pressed(Action::Down, &ctx.gamepads)
            || (action_down(Action::Down, &ctx.gamepads) && self.move_held_delay <= 0.)
        {
            self.move_held_delay = MOVE_HELD_DELAY;
            play_sfx(ctx, &ctx.audio.sfx.menu_move);

            if self.menu_index == self.menu_options.len() - 1 {
                self.menu_index = 0;
            } else {
                self.menu_index += 1;
            }
        }
    }
    fn draw(&mut self, ctx: &mut Context) {
        if self.settings_subscene.active {
            self.settings_subscene.draw(ctx);
            return;
        }

        if self.credits_subscene.active {
            self.credits_subscene.draw(ctx);
            return;
        }

        let menu_option = self
            .menu_options
            .get(self.menu_index)
            .expect("pause menu index out of bounds");

        draw_text(
            ctx,
            "SokoWorld",
            X_INSET,
            TITLE_Y_INSET,
            text::Size::Large,
            WHITE,
        );

        for (i, pack) in &mut self.packs.iter().enumerate() {
            let color = if (self.focused_pack_index == i as i32)
                && matches!(menu_option, MenuOption::PackSelect)
            {
                RED
            } else {
                WHITE
            };

            let title_x = (i as i32 - self.focused_pack_index) as f32 * 320. + X_INSET;
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
                &format!("{} • {}", pack.author, pack.difficulty),
                title_x,
                title_y + 24.,
                text::Size::Small,
                color,
            );
            draw_text(
                ctx,
                format!(
                    "{} levels ({} complete)",
                    pack.levels.len(),
                    self.packs_complete_count.get(i).unwrap(),
                )
                .as_str(),
                title_x,
                title_y + 48.,
                text::Size::Small,
                color,
            );
        }

        for (i, menu_option) in self.menu_options.iter().enumerate() {
            let color = if self.menu_index == i { RED } else { WHITE };

            draw_text(
                ctx,
                self.text_for_menu_option(menu_option),
                X_INSET,
                400. + (i as f32 * 40.),
                text::Size::Medium,
                color,
            );
        }

        draw_text(
            ctx,
            "Change Select = Arrow Keys | Confirm = Z",
            X_INSET,
            VIRTUAL_HEIGHT - 40.,
            text::Size::Small,
            WHITE,
        );

        draw_text(
            ctx,
            format!("v{}", VERSION).as_str(),
            40.,
            VIRTUAL_HEIGHT - 40.,
            text::Size::Small,
            WHITE,
        );
    }
}
