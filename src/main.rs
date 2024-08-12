use macroquad::prelude::*;

const TILE_SIZE: i32 = 64;

#[derive(Debug, Clone)]
struct Level {
    walls: Vec<Vec2>,
    crates: Vec<Vec2>,
    storage_locations: Vec<Vec2>,
    grounds: Vec<Vec2>,
    player: Vec2,
    ground_texture: Texture2D,
    wall_texture: Texture2D,
    storage_location_texture: Texture2D,
}

impl Level {
    /// loads a level from the specified file
    /// panics if the file can't be found
    /// TODO: handle errors better than panic
    async fn load(level_name: &str) -> Result<Self, macroquad::Error> {
        let data = std::fs::read_to_string(format!("assets/{}.txt", level_name))
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
    fn draw(&self) {
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

/// grid position
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vec2 {
    fn add(&mut self, vec: Vec2) -> &mut Vec2 {
        self.x += vec.x;
        self.y += vec.y;
        self
    }
}

struct Entity {
    texture: Texture2D,
    pos: Vec2,
}

pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Reset,
}

/// just pressed, not held down
pub fn action_pressed(action: Action) -> bool {
    match action {
        Action::Up => is_key_pressed(KeyCode::W),
        Action::Down => is_key_pressed(KeyCode::S),
        Action::Left => is_key_pressed(KeyCode::A),
        Action::Right => is_key_pressed(KeyCode::D),
        Action::Reset => is_key_pressed(KeyCode::K),
        Action::Confirm => is_key_pressed(KeyCode::J),
    }
}

impl Entity {
    fn draw(&self) {
        draw_texture(
            &self.texture,
            (self.pos.x * TILE_SIZE) as f32,
            (self.pos.y * TILE_SIZE) as f32,
            WHITE,
        );
    }
}
const LEVEL_CLI_ARG: &str = "-l=";

#[macroquad::main("Sokoworld")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // TODO: move away from indices and just use the level names + load from asset dir or some
    // other piece of data (maybe at compile time?)
    let levels = ["level1", "level2", "level3", "level4"];
    let mut level_index = 0;
    match args.iter().find(|arg| arg.starts_with(LEVEL_CLI_ARG)) {
        Some(arg) => {
            level_index = arg.split(LEVEL_CLI_ARG).last().unwrap().parse().unwrap();
            level_index -= 1;
        }
        None => (),
    };
    println!("{:?}", args);
    let texture_crate = load_texture("assets/crate.png").await.unwrap();

    let mut level = Level::load(levels[level_index]).await.unwrap();

    let mut player = Entity {
        texture: load_texture("assets/player.png").await.unwrap(),
        pos: level.player,
    };
    let mut crates: Vec<Entity> = vec![];
    let mut beat_level = false;

    for pos in &level.crates {
        crates.push(Entity {
            texture: texture_crate.clone(),
            pos: *pos,
        });
    }

    loop {
        #[cfg(debug_assertions)]
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if action_pressed(Action::Reset) {
            beat_level = false;
            player.pos = level.player;
            for (i, c) in crates.iter_mut().enumerate() {
                c.pos = *level.crates.get(i).unwrap();
            }
        }

        if beat_level {
            if action_pressed(Action::Confirm) {
                // DRY THIS THE HECK UP w/ init load
                level_index += 1;
                if level_index >= levels.len() {
                    level_index = 0;
                }
                level = Level::load(levels[level_index]).await.unwrap();

                player = Entity {
                    texture: load_texture("assets/player.png").await.unwrap(),
                    pos: level.player,
                };
                crates.clear();
                beat_level = false;

                for pos in &level.crates {
                    crates.push(Entity {
                        texture: texture_crate.clone(),
                        pos: *pos,
                    });
                }
            }
        } else {
            let mut move_player = Vec2 { x: 0, y: 0 };

            if action_pressed(Action::Up) {
                move_player.y = -1;
            } else if action_pressed(Action::Down) {
                move_player.y = 1;
            } else if action_pressed(Action::Left) {
                move_player.x = -1;
            } else if action_pressed(Action::Right) {
                move_player.x = 1;
            }

            let new_player_pos = player.pos.clone().add(move_player).to_owned();
            let crate_at_new_player_pos = crates.iter().find(|c| c.pos == new_player_pos);
            let mut move_crate = false;

            match crate_at_new_player_pos {
                Some(c) => {
                    let new_crate_pos = c.pos.clone().add(move_player).to_owned();
                    let wall_at_new_crate_pos = level.walls.iter().find(|w| *w == &new_crate_pos);
                    let other_crate_at_new_crate_pos =
                        crates.iter().find(|c| c.pos == new_crate_pos);

                    if wall_at_new_crate_pos.is_none() && other_crate_at_new_crate_pos.is_none() {
                        move_crate = true;
                    }
                }
                None => {
                    let wall_at_new_player_pos = level.walls.iter().find(|w| *w == &new_player_pos);
                    match wall_at_new_player_pos {
                        None => {
                            player.pos = new_player_pos;
                        }
                        Some(_) => (),
                    };
                }
            };

            // this feels bad and duplicative to get around borrow checker
            if move_crate {
                let c = crates.iter_mut().find(|c| c.pos == new_player_pos).unwrap();
                let new_crate_pos = c.pos.clone().add(move_player).to_owned();
                c.pos = new_crate_pos;
            }

            if crates.iter().all(|c| {
                level
                    .storage_locations
                    .clone() // idk if cloning is right here
                    .into_iter()
                    .any(|sl| sl == c.pos)
            }) {
                beat_level = true;
            }
        }

        clear_background(DARKGRAY);
        level.draw();
        player.draw();
        for c in &crates {
            c.draw();
        }

        if beat_level {
            draw_text(
                "Nice job! Press J to go to next level.",
                48.,
                48.,
                32.,
                WHITE,
            );
        }
        draw_text("K = Reset Level", 48., screen_height() - 48., 32., WHITE);

        next_frame().await
    }
}
