use crate::{
    context::Context,
    tile::{draw_tile, Tile},
    vec2::Vec2,
};

use self::pack::PackLevel;

pub mod pack;
pub mod playable_level;

#[derive(Debug, Clone)]
pub struct Level {
    pub title: String,
    pub walls: Vec<Vec2>,
    pub crates: Vec<Vec2>,
    pub storage_locations: Vec<Vec2>,
    pub grounds: Vec<Vec2>,
    pub player: Vec2,
    pub width: usize,
    pub height: usize,
}

impl Level {
    /// Parses a level from the PackLevel data
    /// panics if there's an unexpected char in the level data
    pub fn parse(pack_level: &PackLevel) -> Result<Self, macroquad::Error> {
        let rows = pack_level.data.lines();
        let mut walls = vec![];
        let mut crates = vec![];
        let mut storage_locations = vec![];
        let mut grounds = vec![];
        let mut player = Vec2 { x: 0, y: 0 };
        let mut width = 0;
        let height = rows.clone().count();

        for (y, row) in rows.enumerate() {
            let row_width = row.chars().count();
            if row_width > width {
                width = row_width;
            }
            for (x, c) in row.chars().enumerate() {
                let pos = Vec2 {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '#' => walls.push(pos),
                    '@' => {
                        player = pos;
                        grounds.push(pos);
                    }
                    '+' => {
                        storage_locations.push(pos);
                        player = pos;
                    }
                    '$' => {
                        crates.push(pos);
                        grounds.push(pos);
                    }
                    '*' => {
                        storage_locations.push(pos);
                        crates.push(pos);
                    }
                    '.' => storage_locations.push(pos),
                    ' ' | '-' | '_' => grounds.push(pos),
                    _ => panic!("unexpected char in level at {}, {}", x, y),
                }
            }
        }

        Ok(Self {
            title: pack_level.title.clone(),
            walls,
            crates,
            storage_locations,
            grounds,
            player,
            width,
            height,
        })
    }

    /// draws the static elements of a level (everything except player and boxes)
    pub fn draw(&self, ctx: &Context, offset: &Vec2) {
        for wall in &self.walls {
            draw_tile(ctx, Tile::Wall, wall, offset);
        }
        for storage_location in &self.storage_locations {
            draw_tile(ctx, Tile::StorageLocation, storage_location, offset);
        }
        for ground in &self.grounds {
            draw_tile(ctx, Tile::Ground, ground, offset);
        }
    }
}
