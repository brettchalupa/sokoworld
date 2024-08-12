use crate::consts::*;
use crate::vec2::Vec2;
use macroquad::{
    color::WHITE,
    texture::{draw_texture, Texture2D},
};

/// draws the specified tile at the specified grid position
pub fn draw_tile(texture: &Texture2D, pos: &Vec2) {
    draw_texture(
        texture,
        (pos.x * TILE_SIZE + GRID_OFFSET) as f32,
        (pos.y * TILE_SIZE + GRID_OFFSET) as f32,
        WHITE,
    );
}
