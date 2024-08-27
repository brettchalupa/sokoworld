use macroquad::color::{RED, WHITE};
use macroquad::time::get_frame_time;

use super::settings::Settings;
use super::EScene;
use super::Scene;
use crate::audio::play_sfx;
use crate::consts::X_INSET;
use crate::consts::{MOVE_HELD_DELAY, TITLE_Y_INSET};
use crate::input::Action;
use crate::input::{action_down, action_pressed};
use crate::level::pack::Pack;
use crate::text::Size;
use crate::{context::Context, text::draw_text};

/// sub-scene rendered during gameplay with various options
pub struct Pause {
    pub active: bool,
    menu_options: Vec<MenuOption>,
    menu_index: usize,
    settings_subscene: Settings,
    pack: Pack,
    move_held_delay: f32,
}

enum MenuOption {
    Resume,
    Settings,
    LevelSelect,
    MainMenu,
    #[cfg(not(target_family = "wasm"))]
    Quit,
}

impl Pause {
    pub fn new(ctx: &mut Context, pack: Pack) -> Self {
        let menu_options = vec![
            MenuOption::Resume,
            MenuOption::Settings,
            MenuOption::LevelSelect,
            MenuOption::MainMenu,
            #[cfg(not(target_family = "wasm"))]
            MenuOption::Quit,
        ];

        Self {
            menu_options,
            menu_index: 0,
            active: false,
            move_held_delay: 0.,
            settings_subscene: Settings::new(ctx, false),
            pack,
        }
    }

    fn text_for_menu_option(&self, menu_option: &MenuOption) -> &str {
        match menu_option {
            MenuOption::Resume => "Resume",
            MenuOption::Settings => "Settings",
            MenuOption::LevelSelect => "Back to Level Select",
            MenuOption::MainMenu => "Return to Main Menu",
            #[cfg(not(target_family = "wasm"))]
            MenuOption::Quit => "Quit",
        }
    }
}

impl Scene for Pause {
    fn update(&mut self, ctx: &mut Context) {
        if self.settings_subscene.active {
            self.settings_subscene.update(ctx);
            return;
        }

        if self.move_held_delay > 0.0 {
            self.move_held_delay -= get_frame_time();
        }

        if action_pressed(Action::Pause, &ctx.gamepads)
            || action_pressed(Action::Cancel, &ctx.gamepads)
        {
            self.active = false;
            play_sfx(ctx, &ctx.audio.sfx.menu_cancel);
            return;
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

        if action_pressed(Action::Confirm, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_select);

            let menu_option = self
                .menu_options
                .get(self.menu_index)
                .expect("pause menu index out of bounds");
            match menu_option {
                MenuOption::Resume => {
                    self.active = false;
                }
                MenuOption::Settings => {
                    self.settings_subscene.active = true;
                }
                MenuOption::LevelSelect => {
                    ctx.switch_scene_to = Some(EScene::LevelSelect(self.pack.clone()));
                }
                MenuOption::MainMenu => {
                    ctx.switch_scene_to = Some(EScene::MainMenu);
                }
                #[cfg(not(target_family = "wasm"))]
                MenuOption::Quit => {
                    ctx.request_quit = true;
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        if self.settings_subscene.active {
            self.settings_subscene.draw(ctx);
            return;
        }

        draw_text(ctx, "Pause", X_INSET, TITLE_Y_INSET, Size::Large, WHITE);

        for (i, menu_option) in self.menu_options.iter().enumerate() {
            let color = if self.menu_index == i { RED } else { WHITE };

            draw_text(
                ctx,
                self.text_for_menu_option(menu_option),
                X_INSET,
                200. + (i as f32 * 40.),
                Size::Medium,
                color,
            );
        }
    }
}
