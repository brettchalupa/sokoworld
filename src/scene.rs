/// enum of various scenes that exist
// not sure if there's a better way to do this...
#[derive(Clone, Debug)]
pub enum EScene {
    Gameplay(PackLevel, usize, Pack),
    MainMenu,
    LevelSelect(Pack),
    // HowToPlay,
    // Credits,
}

use crate::{
    context::Context,
    level::pack::{Pack, PackLevel},
};

pub mod credits;
pub mod gameplay;
pub mod level_select;
pub mod main_menu;
pub mod pause;
pub mod settings;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}
