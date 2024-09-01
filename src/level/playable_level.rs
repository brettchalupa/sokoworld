use super::pack::PackLevel;
use super::Level;
use crate::audio::play_sfx;
use crate::color::BLUE;
use crate::input;
use crate::text::draw_text;
use crate::{
    context::Context,
    tile::{draw_tile, Tile},
    vec2::Vec2,
};
use macroquad::color::WHITE;
use macroquad::input::is_key_pressed;
use macroquad::time::get_frame_time;

use crate::consts::*;
use crate::entity::{Crate, Entity};

/// delay in seconds between rewind steps when held down
const REWIND_HELD_DELAY: f32 = 0.1;

/// direction that the player moved in
#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// for tracking player input to make it easy to rewind
#[derive(Clone, Debug)]
struct PlayerMove {
    direction: Direction,
    /// index of the crate that was moved, if any
    crate_moved_index: Option<usize>,
}

/// wraps the static level data and keeps track of player's progress
#[derive(Clone, Debug)]
pub struct PlayableLevel {
    pub complete: bool,
    pub steps: i32,
    pub pushes: i32,
    pub level: Level,
    pub player: Entity,
    pub crates: Vec<Crate>,
    move_held_delay: f32,
    rewind_held_delay: f32,
    moves: Vec<PlayerMove>,
    pack_slug: String,
}

impl PlayableLevel {
    /// creates a new playable level with properly reset data from the specified pack_level
    pub fn new(pack_slug: String, pack_level: &PackLevel) -> Self {
        let level = Level::parse(pack_level).unwrap();
        let player = Entity { pos: level.player };
        let mut crates: Vec<Crate> = vec![];

        for pos in &level.crates {
            let on_storage_location = level
                .storage_locations
                .clone()
                .into_iter()
                .any(|sl| sl == *pos);
            crates.push(Crate {
                pos: *pos,
                on_storage_location,
            });
        }

        Self {
            complete: false,
            steps: 0,
            pushes: 0,
            level,
            crates,
            player,
            move_held_delay: 0.,
            rewind_held_delay: 0.,
            moves: vec![],
            pack_slug,
        }
    }

    pub fn reset(&mut self) {
        self.player.pos = self.level.player;
        for (i, c) in self.crates.iter_mut().enumerate() {
            c.pos = *self.level.crates.get(i).unwrap();

            if self
                .level
                .storage_locations
                .clone() // idk if cloning is right here
                .into_iter()
                .any(|sl| sl == c.pos)
            {
                c.on_storage_location = true
            } else {
                c.on_storage_location = false
            }
        }
        self.steps = 0;
        self.pushes = 0;
        self.moves.clear();
        self.complete = false;
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if input::action_pressed(input::Action::Reset, &ctx.gamepads) {
            self.reset();
            play_sfx(ctx, &ctx.audio.sfx.reset);
        }

        // TODO: move to a game setting
        if is_key_pressed(macroquad::miniquad::KeyCode::Key0) {
            ctx.tileset = match ctx.tileset {
                crate::tile::Tileset::Retro => crate::tile::Tileset::Doggo,
                crate::tile::Tileset::Doggo => crate::tile::Tileset::Kenney,
                crate::tile::Tileset::Kenney => crate::tile::Tileset::Marble,
                crate::tile::Tileset::Marble => crate::tile::Tileset::Retro,
            }
        }

        if self.complete {
            if input::action_pressed(input::Action::Confirm, &ctx.gamepads) {
                ctx.load_next_level = true;
            }
            return;
        }

        if self.move_held_delay > 0.0 {
            self.move_held_delay -= get_frame_time();
        }

        if self.rewind_held_delay > 0.0 {
            self.rewind_held_delay -= get_frame_time();
        }

        let rewind = input::action_pressed(input::Action::Rewind, &ctx.gamepads)
            || (input::action_down(input::Action::Rewind, &ctx.gamepads)
                && self.rewind_held_delay <= 0.);

        if rewind {
            if !self.moves.is_empty() {
                self.rewind_held_delay = REWIND_HELD_DELAY;

                if let Some(m) = self.moves.pop() {
                    play_sfx(ctx, &ctx.audio.sfx.footstep);
                    self.steps -= 1;
                    if m.crate_moved_index.is_some() {
                        self.pushes -= 1;
                    }
                    let reverse_move = match m.direction {
                        Direction::Up => {
                            // move down
                            Vec2 { x: 0, y: 1 }
                        }
                        Direction::Down => {
                            // move up
                            Vec2 { x: 0, y: -1 }
                        }
                        Direction::Left => {
                            // move right
                            Vec2 { x: 1, y: 0 }
                        }
                        Direction::Right => {
                            // move left
                            Vec2 { x: -1, y: 0 }
                        }
                    };
                    self.player.pos.add(reverse_move);
                    self.reverse_move_crate(ctx, m.crate_moved_index, reverse_move);
                }
            } else {
                play_sfx(ctx, &ctx.audio.sfx.cant_move);
            }
        } else {
            self.handle_movement(ctx)
        }
    }

    fn handle_movement(&mut self, ctx: &mut Context) {
        let mut move_player = Vec2 { x: 0, y: 0 };

        if input::action_pressed(input::Action::Up, &ctx.gamepads)
            || (input::action_down(input::Action::Up, &ctx.gamepads) && self.move_held_delay <= 0.)
        {
            move_player.y = -1;
        } else if input::action_pressed(input::Action::Down, &ctx.gamepads)
            || (input::action_down(input::Action::Down, &ctx.gamepads)
                && self.move_held_delay <= 0.)
        {
            move_player.y = 1;
        } else if input::action_pressed(input::Action::Left, &ctx.gamepads)
            || (input::action_down(input::Action::Left, &ctx.gamepads)
                && self.move_held_delay <= 0.)
        {
            move_player.x = -1;
        } else if input::action_pressed(input::Action::Right, &ctx.gamepads)
            || (input::action_down(input::Action::Right, &ctx.gamepads)
                && self.move_held_delay <= 0.)
        {
            move_player.x = 1;
        }

        let new_player_pos = self.player.pos.clone().add(move_player).to_owned();
        let crate_at_new_player_pos = self.crates.iter().find(|c| c.pos == new_player_pos);
        let mut move_crate = false;

        if !move_player.is_zero() {
            self.move_held_delay = MOVE_HELD_DELAY;

            match crate_at_new_player_pos {
                Some(c) => {
                    let new_crate_pos = c.pos.clone().add(move_player).to_owned();
                    let wall_at_new_crate_pos =
                        self.level.walls.iter().find(|w| *w == &new_crate_pos);
                    let other_crate_at_new_crate_pos =
                        self.crates.iter().find(|c| c.pos == new_crate_pos);

                    if wall_at_new_crate_pos.is_none() && other_crate_at_new_crate_pos.is_none() {
                        let crate_i = self.crates.iter().position(|ic| ic.pos == c.pos);
                        self.move_player_to(ctx, &move_player, &new_player_pos, crate_i);
                        move_crate = true;
                    } else {
                        play_sfx(ctx, &ctx.audio.sfx.cant_move);
                    }
                }
                None => {
                    let wall_at_new_player_pos =
                        self.level.walls.iter().find(|w| *w == &new_player_pos);
                    match wall_at_new_player_pos {
                        None => {
                            self.move_player_to(ctx, &move_player, &new_player_pos, None);
                        }
                        Some(_) => {
                            play_sfx(ctx, &ctx.audio.sfx.cant_move);
                        }
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
                play_sfx(ctx, &ctx.audio.sfx.push);

                Self::check_for_crate_on_storage_location(
                    ctx,
                    c,
                    self.level.storage_locations.clone(),
                );
            }

            if self.crates.iter().all(|c| {
                self.level
                    .storage_locations
                    .clone() // idk if cloning is right here
                    .into_iter()
                    .any(|sl| sl == c.pos)
            }) {
                play_sfx(ctx, &ctx.audio.sfx.level_complete);
                self.complete = true;
                ctx.save.complete_level(
                    self.pack_slug.clone(),
                    self.level.title.clone(),
                    self.steps,
                    self.pushes,
                );
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
            let t = match c.on_storage_location {
                true => Tile::CrateOnStorageLocation,
                false => Tile::Crate,
            };
            draw_tile(ctx, t, &c.pos, &offset);
        }

        draw_text(
            ctx,
            self.level.title.as_str(),
            X_INSET / 2.,
            62.,
            crate::text::Size::Large,
            WHITE,
        );
        draw_text(
            ctx,
            format!("Steps: {} | Pushes: {}", self.steps, self.pushes).as_str(),
            X_INSET / 2.,
            112.,
            crate::text::Size::Medium,
            WHITE,
        );
        if self.complete {
            draw_text(
                ctx,
                "Nice job! Press Z to go to next level.",
                X_INSET / 2.,
                VIRTUAL_HEIGHT - 92.,
                crate::text::Size::Medium,
                BLUE,
            );
        }
        draw_text(
            ctx,
            "Move = Arrow Keys | Rewind = X | Reset = C",
            X_INSET / 2.,
            VIRTUAL_HEIGHT - 48.,
            crate::text::Size::Small,
            WHITE,
        );
    }

    fn reverse_move_crate(
        &mut self,
        ctx: &Context,
        crate_moved_index: Option<usize>,
        reverse_move: Vec2,
    ) {
        if crate_moved_index.is_some() {
            let c = self.crates.get_mut(crate_moved_index.unwrap()).unwrap();
            c.pos.add(reverse_move);
            Self::check_for_crate_on_storage_location(ctx, c, self.level.storage_locations.clone())
        }
    }

    fn check_for_crate_on_storage_location(
        ctx: &Context,
        c: &mut Crate,
        storage_locations: Vec<Vec2>,
    ) {
        if storage_locations.into_iter().any(|sl| sl == c.pos) {
            c.on_storage_location = true;
            play_sfx(ctx, &ctx.audio.sfx.crate_on_storage_location);
        } else {
            c.on_storage_location = false;
        }
    }

    fn move_player_to(
        &mut self,
        ctx: &Context,
        movement: &Vec2,
        new_pos: &Vec2,
        crate_index: Option<usize>,
    ) {
        play_sfx(ctx, &ctx.audio.sfx.footstep);
        self.moves.push(PlayerMove {
            direction: Self::direction_of_movement(movement),
            crate_moved_index: crate_index,
        });
        self.player.pos = *new_pos;
        self.steps += 1;
    }

    fn direction_of_movement(movement_vec: &Vec2) -> Direction {
        if movement_vec.x > 0 {
            Direction::Right
        } else if movement_vec.x < 0 {
            Direction::Left
        } else if movement_vec.y < 0 {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}
