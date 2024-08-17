use crate::vec2::Vec2;
use crate::{consts::*, context::Context};
use macroquad::{
    color::WHITE,
    math::Rect,
    texture::{draw_texture_ex, DrawTextureParams},
};

pub enum Tileset {
    Doggo,
    Retro,
    Kenney,
    Marble,
}

pub enum Tile {
    Crate,
    CrateOnStorageLocation,
    Ground,
    Player,
    StorageLocation,
    Wall,
}

/// draws the specified tile at the specified grid position
pub fn draw_tile(ctx: &Context, t: Tile, pos: &Vec2, offset: &Vec2) {
    draw_texture_ex(
        ctx.current_texture(),
        (pos.x * TILE_SIZE + offset.x) as f32,
        (pos.y * TILE_SIZE + offset.y) as f32,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                tilesheet_i_for_tile(t) as f32 * TILE_SIZE as f32,
                0.,
                TILE_SIZE as f32,
                TILE_SIZE as f32,
            )),
            dest_size: Some(macroquad::math::Vec2::new(
                TILE_SIZE as f32,
                TILE_SIZE as f32,
            )),
            ..Default::default()
        },
    );
}

fn tilesheet_i_for_tile(t: Tile) -> usize {
    match t {
        Tile::Crate => 0,
        Tile::CrateOnStorageLocation => 1,
        Tile::StorageLocation => 2,
        Tile::Player => 3,
        Tile::Ground => 4,
        Tile::Wall => 5,
    }
}
