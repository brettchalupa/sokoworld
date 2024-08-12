use crate::level::Level;
use crate::vec2::Vec2;
use consts::*;
use gamepads::Gamepads;
use macroquad::prelude::*;

mod consts;
mod input;
mod level;
mod vec2;

struct Entity {
    texture: Texture2D,
    pos: Vec2,
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

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Sokoworld"),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut gamepads = Gamepads::new();

    // TODO: move away from indices and just use the level names + load from asset dir or some
    // other piece of data (maybe at compile time?)
    let levels = ["level1", "level2", "level3", "level4", "level5", "level6"];
    let mut level_index = 0;
    if let Some(arg) = args.iter().find(|arg| arg.starts_with(LEVEL_CLI_ARG)) {
        level_index = arg.split(LEVEL_CLI_ARG).last().unwrap().parse().unwrap();
        level_index -= 1;
    };
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

    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Linear);

    // Setup camera for the virtual screen, that will render to 'render_target'
    let mut render_target_cam =
        Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
    render_target_cam.render_target = Some(render_target.clone());

    loop {
        gamepads.poll();

        #[cfg(debug_assertions)]
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if input::action_pressed(input::Action::Reset, &gamepads) {
            beat_level = false;
            player.pos = level.player;
            for (i, c) in crates.iter_mut().enumerate() {
                c.pos = *level.crates.get(i).unwrap();
            }
        }

        if beat_level {
            if input::action_pressed(input::Action::Confirm, &gamepads) {
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

            if input::action_pressed(input::Action::Up, &gamepads) {
                move_player.y = -1;
            } else if input::action_pressed(input::Action::Down, &gamepads) {
                move_player.y = 1;
            } else if input::action_pressed(input::Action::Left, &gamepads) {
                move_player.x = -1;
            } else if input::action_pressed(input::Action::Right, &gamepads) {
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

        // Get required scaling value
        let scale: f32 = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );

        set_camera(&render_target_cam);

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
        draw_text("K = Reset Level", 48., VIRTUAL_HEIGHT - 48., 32., WHITE);

        set_default_camera();

        clear_background(BLACK); // Will be the letterbox color

        // Draw 'render_target' to window screen, porperly scaled and letterboxed
        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5,
            (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );

        next_frame().await
    }
}
