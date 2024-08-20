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
            // this bool is used to prevent space characters before the first wall in a row being
            // rendered as ground
            let mut found_first_wall = false;
            for (x, c) in row.chars().enumerate() {
                let pos = Vec2 {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '#' => {
                        if !found_first_wall {
                            found_first_wall = true;
                        }
                        walls.push(pos)
                    }
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
                    ' ' | '-' | '_' => {
                        // first row and column can never be ground
                        if found_first_wall && y != 0 && x != 0 {
                            grounds.push(pos)
                        }
                    }
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

    /// whether or not the level data meets all of the criteria to be considered a valid Sokoban
    /// level:
    ///
    /// 1. enclosed by walls (TODO: not part of this check yet)
    /// 2. only one player
    /// 3. at least one crate
    /// 4. a storage location for each crate
    ///
    /// does not factor in whether or not the level can be completed
    ///
    /// in the future this could be nice companion to a `validate()` function that returns the
    ///     various issues.
    pub fn is_valid(&self) -> bool {
        // player can't be placed at 0, 0 and that's the default, so that means the player is
        // missing
        !self.player.is_zero()
            && !self.crates.is_empty()
            && self.storage_locations.len() == self.crates.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::level::{pack::PackLevel, Level};

    #[test]
    fn test_level_is_valid() {
        let pack_level = PackLevel {
            title: "test level".to_string(),
            data: r#"
            #####
            #@$.#
            #####
            "#
            .to_string(),
        };
        let level = Level::parse(&pack_level).unwrap();
        assert!(level.is_valid());
    }

    #[test]
    fn test_level_without_player_is_invalid() {
        let pack_level = PackLevel {
            title: "test level".to_string(),
            data: r#"
            ###
            # #
            ###
            "#
            .to_string(),
        };
        let level = Level::parse(&pack_level).unwrap();
        assert!(!level.is_valid());
    }

    #[test]
    fn test_level_without_crate_is_invalid() {
        let pack_level = PackLevel {
            title: "test level".to_string(),
            data: r#"
            #####
            #@  #
            #####
            "#
            .to_string(),
        };
        let level = Level::parse(&pack_level).unwrap();
        assert!(!level.is_valid());
    }

    #[test]
    fn test_level_with_mismatch_crates_and_storage_locations_is_invalid() {
        let pack_level = PackLevel {
            title: "test level".to_string(),
            data: r#"
            #######
            #@$.. #
            #######
            "#
            .to_string(),
        };
        let level = Level::parse(&pack_level).unwrap();
        assert!(!level.is_valid());
    }
}
