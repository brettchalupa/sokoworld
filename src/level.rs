use crate::input;
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::text::draw_text;

use crate::consts::*;
use crate::entity::Entity;
use crate::{
    context::Context,
    tile::{draw_tile, Tile},
    vec2::Vec2,
};

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

/// wraps the static level data and keeps track of player's progress
pub struct PlayableLevel {
    pub complete: bool,
    pub steps: i32,
    pub pushes: i32,
    pub level: Level,
    pub player: Entity,
    pub crates: Vec<Entity>,
}

impl PlayableLevel {
    pub fn load(pack_level: &PackLevel) -> Self {
        let level = Level::load(pack_level).unwrap();
        let player = Entity { pos: level.player };
        let mut crates: Vec<Entity> = vec![];

        for pos in &level.crates {
            crates.push(Entity { pos: *pos });
        }

        Self {
            complete: false,
            steps: 0,
            pushes: 0,
            level,
            crates,
            player,
        }
    }

    pub fn reset(&mut self) {
        self.steps = 0;
        self.pushes = 0;
        self.complete = false;
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if input::action_pressed(input::Action::Reset, &ctx.gamepads) {
            self.reset();
            self.player.pos = self.level.player;
            for (i, c) in self.crates.iter_mut().enumerate() {
                c.pos = *self.level.crates.get(i).unwrap();
            }
            macroquad::audio::play_sound_once(&ctx.audio.sfx.reset);
        }

        if self.complete {
            if input::action_pressed(input::Action::Confirm, &ctx.gamepads) {
                ctx.load_next_level = true;
            }
            return;
        }

        let mut move_player = Vec2 { x: 0, y: 0 };

        if input::action_pressed(input::Action::Up, &ctx.gamepads) {
            move_player.y = -1;
        } else if input::action_pressed(input::Action::Down, &ctx.gamepads) {
            move_player.y = 1;
        } else if input::action_pressed(input::Action::Left, &ctx.gamepads) {
            move_player.x = -1;
        } else if input::action_pressed(input::Action::Right, &ctx.gamepads) {
            move_player.x = 1;
        }

        // TODO: move to a game setting
        if is_key_pressed(macroquad::miniquad::KeyCode::Key0) {
            ctx.tileset = match ctx.tileset {
                crate::tile::Tileset::Retro => crate::tile::Tileset::Kenney,
                crate::tile::Tileset::Kenney => crate::tile::Tileset::Retro,
            }
        }

        let new_player_pos = self.player.pos.clone().add(move_player).to_owned();
        let crate_at_new_player_pos = self.crates.iter().find(|c| c.pos == new_player_pos);
        let mut move_crate = false;

        if !move_player.is_zero() {
            match crate_at_new_player_pos {
                Some(c) => {
                    let new_crate_pos = c.pos.clone().add(move_player).to_owned();
                    let wall_at_new_crate_pos =
                        self.level.walls.iter().find(|w| *w == &new_crate_pos);
                    let other_crate_at_new_crate_pos =
                        self.crates.iter().find(|c| c.pos == new_crate_pos);

                    if wall_at_new_crate_pos.is_none() && other_crate_at_new_crate_pos.is_none() {
                        self.move_player_to(&new_player_pos);
                        move_crate = true;
                    }
                }
                None => {
                    let wall_at_new_player_pos =
                        self.level.walls.iter().find(|w| *w == &new_player_pos);
                    match wall_at_new_player_pos {
                        None => {
                            self.move_player_to(&new_player_pos);
                        }
                        Some(_) => (),
                    };
                }
            };

            // this feels bad and duplicative to get around borrow checker
            if move_crate {
                let c = self
                    .crates
                    .iter_mut()
                    .find(|c| c.pos == new_player_pos)
                    .unwrap();
                let new_crate_pos = c.pos.clone().add(move_player).to_owned();
                c.pos = new_crate_pos;
                self.pushes += 1;
                macroquad::audio::play_sound_once(&ctx.audio.sfx.push);
            }

            if self.crates.iter().all(|c| {
                self.level
                    .storage_locations
                    .clone() // idk if cloning is right here
                    .into_iter()
                    .any(|sl| sl == c.pos)
            }) {
                macroquad::audio::play_sound_once(&ctx.audio.sfx.level_complete);
                self.complete = true;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let offset = Vec2 {
            x: (VIRTUAL_WIDTH as i32 - (self.level.width as i32 * TILE_SIZE)) / 2,
            y: (VIRTUAL_HEIGHT as i32 - (self.level.height as i32 * TILE_SIZE)) / 2,
        };
        self.level.draw(ctx, &offset);
        draw_tile(ctx, Tile::Player, &self.player.pos, &offset);
        for c in &self.crates {
            draw_tile(ctx, Tile::Crate, &c.pos, &offset);
        }

        if self.complete {
            draw_text(
                "Nice job! Press J to go to next level.",
                VIRTUAL_WIDTH / 2. - 280.,
                VIRTUAL_HEIGHT - 92.,
                32.,
                WHITE,
            );
        }
        draw_text(
            "WASD = Move | K = Reset Level",
            VIRTUAL_WIDTH / 2. - 200.,
            VIRTUAL_HEIGHT - 48.,
            32.,
            WHITE,
        );
        draw_text(self.level.title.as_str(), 48., 32., 32., WHITE);
        draw_text(
            format!("Steps: {} | Pushes: {}", self.steps, self.pushes).as_str(),
            48.,
            72.,
            32.,
            WHITE,
        );
    }

    fn move_player_to(&mut self, new_pos: &Vec2) {
        self.player.pos = *new_pos;
        self.steps += 1;
    }
}

impl Level {
    /// loads a level from the specified file
    /// panics if the file can't be found
    pub fn load(pack_level: &PackLevel) -> Result<Self, macroquad::Error> {
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

use serde::Deserialize;

/// a collection of levels
#[derive(Debug, Deserialize)]
pub struct Pack {
    pub title: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub year: i32,
    pub levels: Vec<PackLevel>,
    pub version: String,
}
/// a level defined in a pack file
#[derive(Debug, Deserialize)]
pub struct PackLevel {
    /// name of the level
    pub title: String,
    /// grid of the puzzle's elements
    pub data: String,
}
