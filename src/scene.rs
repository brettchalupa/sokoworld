/// enum of various scenes that exist
// not sure if there's a better way to do this...
#[derive(Clone, Debug)]
pub enum EScene {
    Gameplay(Pack),
    MainMenu,
    // Settings,
    // Pause,
    // HowToPlay,
}

use crate::{context::Context, level::pack::Pack};

pub mod gameplay;
pub mod main_menu;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}
