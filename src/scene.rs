/// enum of various scenes that exist
// not sure if there's a better way to do this...
#[derive(Clone, Copy, Debug)]
pub enum EScene {
    Gameplay,
    MainMenu,
    // Settings,
    // Pause,
    // HowToPlay,
}

use crate::context::Context;

pub mod gameplay;
pub mod main_menu;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}
