use crate::{draw::draw_tile, texture::TextureAtlas, vec2::Vec2};

#[derive(Debug, Clone)]
pub struct Level {
    pub name: String,
    pub walls: Vec<Vec2>,
    pub crates: Vec<Vec2>,
    pub storage_locations: Vec<Vec2>,
    pub grounds: Vec<Vec2>,
    pub player: Vec2,
}

/// wraps the static level data and keeps track of player's progress
pub struct PlayableLevel {
    pub complete: bool,
    pub steps: i32,
    pub pushes: i32,
    pub level: Level,
}

impl PlayableLevel {
    pub async fn load(name: &str) -> Self {
        Self {
            complete: false,
            steps: 0,
            pushes: 0,
            level: Level::load(name).await.unwrap(),
        }
    }

    pub fn reset(&mut self) {
        self.steps = 0;
        self.pushes = 0;
        self.complete = false;
    }
}

impl Level {
    /// loads a level from the specified file
    /// panics if the file can't be found
    pub async fn load(level_name: &str) -> Result<Self, macroquad::Error> {
        let data = macroquad::file::load_string(format!("assets/{}.txt", level_name).as_str())
            .await
            .expect("Unable to read file");
        let rows = data.lines();
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
                    ' ' | '-' | '_' => grounds.push(pos),
                    _ => panic!("unexpected char in level at {}, {}", x, y),
                }
            }
        }

        Ok(Self {
            name: level_name.to_owned(),
            walls,
            crates,
            storage_locations,
            grounds,
            player,
        })
    }

    /// draws the static elements of a level (everything except player and boxes)
    pub fn draw(&self, texture_atlas: &TextureAtlas) {
        for wall in &self.walls {
            draw_tile(&texture_atlas.wall, wall);
        }
        for storage_location in &self.storage_locations {
            draw_tile(&texture_atlas.storage_location, storage_location);
        }
        for ground in &self.grounds {
            draw_tile(&texture_atlas.ground, ground);
        }
    }
}
