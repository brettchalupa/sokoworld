use macroquad::{
    color::WHITE,
    texture::{draw_texture, load_texture, Texture2D},
};

use crate::{consts::TILE_SIZE, vec2::Vec2};

#[derive(Debug, Clone)]
pub struct Level {
    pub walls: Vec<Vec2>,
    pub crates: Vec<Vec2>,
    pub storage_locations: Vec<Vec2>,
    pub grounds: Vec<Vec2>,
    pub player: Vec2,
    ground_texture: Texture2D,
    wall_texture: Texture2D,
    storage_location_texture: Texture2D,
}

impl Level {
    /// loads a level from the specified file
    /// panics if the file can't be found
    /// TODO: handle errors better than panic
    pub async fn load(level_name: &str) -> Result<Self, macroquad::Error> {
        let data = macroquad::file::load_string(format!("assets/{}.txt", level_name).as_str())
            .await
            .expect("Unable to read file");
        let rows = data.split('\n'); // TODO: maybe split on more OS-friendl format
        let mut walls = vec![];
        let mut crates = vec![];
        let mut storage_locations = vec![];
        let mut grounds = vec![];
        let mut player = Vec2 { x: 0, y: 0 };

        for (y, row) in rows.enumerate() {
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
                    ' ' => grounds.push(pos),
                    _ => panic!("unexpected char in level at {}, {}", x, y),
                }
            }
        }

        Ok(Self {
            walls,
            crates,
            storage_locations,
            grounds,
            player,
            wall_texture: load_texture("assets/wall.png").await.unwrap(),
            ground_texture: load_texture("assets/ground.png").await.unwrap(),
            storage_location_texture: load_texture("assets/storage_location.png").await.unwrap(),
        })
    }

    /// draws the static elements of a level (everything except player and boxes)
    pub fn draw(&self) {
        // TOOD: draw vec fn
        // TODO: draw vec2 fn
        for wall in &self.walls {
            draw_texture(
                &self.wall_texture,
                (wall.x * TILE_SIZE) as f32,
                (wall.y * TILE_SIZE) as f32,
                WHITE,
            );
        }
        for storage_location in &self.storage_locations {
            draw_texture(
                &self.storage_location_texture,
                (storage_location.x * TILE_SIZE) as f32,
                (storage_location.y * TILE_SIZE) as f32,
                WHITE,
            );
        }
        for ground in &self.grounds {
            draw_texture(
                &self.ground_texture,
                (ground.x * TILE_SIZE) as f32,
                (ground.y * TILE_SIZE) as f32,
                WHITE,
            );
        }
    }
}
