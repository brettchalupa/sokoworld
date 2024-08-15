use macroquad::color::{RED, WHITE};

use super::Scene;
use crate::audio::play_sfx;
use crate::input::action_pressed;
use crate::input::Action;
use crate::text::Size;
use crate::{context::Context, text::draw_text};

/// sub-scene for setting user preferences, at times a subscene
pub struct Settings {
    pub active: bool,
    menu_options: Vec<MenuOption>,
    menu_index: usize,
}

enum MenuOption {
    Fullscreen(bool),
    Mute(bool),
    Back,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Settings {
    pub fn new(active: bool) -> Self {
        let menu_options = vec![
            MenuOption::Fullscreen(false), // TODO: set from config
            MenuOption::Mute(false),       // TODO: set from config
            MenuOption::Back,
        ];

        Self {
            menu_options,
            menu_index: 0,
            active,
        }
    }

    fn text_for_menu_option(&self, menu_option: &MenuOption) -> &str {
        match menu_option {
            MenuOption::Back => "Back",
            MenuOption::Fullscreen(_enabled) => "Fullscreen",
            MenuOption::Mute(_enabled) => "Mute",
        }
    }
}

const X_ALIGN: f32 = 200.;

impl Scene for Settings {
    fn update(&mut self, ctx: &mut Context) {
        if action_pressed(Action::Cancel, &ctx.gamepads) {
            self.active = false;
            play_sfx(ctx, &ctx.audio.sfx.menu_cancel);
            return;
        }

        if action_pressed(Action::Up, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_move);

            if self.menu_index == 0 {
                self.menu_index = self.menu_options.len() - 1;
            } else {
                self.menu_index -= 1;
            }
        }
        if action_pressed(Action::Down, &ctx.gamepads) {
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
                MenuOption::Back => {
                    self.active = false;
                }
                MenuOption::Fullscreen(_enabled) => {
                    println!("Fullscreen not yet implemented");
                }
                MenuOption::Mute(_enabled) => {
                    println!("Mute not yet implemented");
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw_text(ctx, "Settings", X_ALIGN, 128., Size::Large, WHITE);

        for (i, menu_option) in self.menu_options.iter().enumerate() {
            let color = if self.menu_index == i { RED } else { WHITE };

            draw_text(
                ctx,
                self.text_for_menu_option(menu_option),
                X_ALIGN,
                200. + (i as f32 * 40.),
                Size::Medium,
                color,
            );
        }
    }
}
