use gamepads::Gamepads;
use macroquad::prelude::*;
use sokoworld::audio;
use sokoworld::consts::*;
use sokoworld::draw;
use sokoworld::input;
use sokoworld::level::PlayableLevel;
use sokoworld::texture;
use sokoworld::vec2::Vec2;

struct Entity {
    pos: Vec2,
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

/// game-wide data and resources
struct Context {
    request_quit: bool,
    gamepads: Gamepads,
    textures: texture::TextureAtlas,
    audio: audio::AudioAtlas,
    render_target: RenderTarget,
    render_target_cam: Camera2D,
}

impl Context {
    async fn default() -> Self {
        let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Linear);

        // Setup camera for the virtual screen, that will render to 'render_target'
        let mut render_target_cam =
            Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
        render_target_cam.render_target = Some(render_target.clone());

        Self {
            gamepads: Gamepads::new(),
            request_quit: false,
            textures: texture::TextureAtlas::new().await,
            audio: audio::AudioAtlas::new().await,
            render_target,
            render_target_cam,
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ctx = Context {
        ..Context::default().await
    };

    let args: Vec<String> = std::env::args().collect();

    // TODO: move away from indices and just use the level names + load from asset dir or some
    // other piece of data (maybe at compile time?)
    let levels = ["level1", "level2", "level3", "level4", "level5", "level6"];
    let mut level_index = 0;
    if let Some(arg) = args.iter().find(|arg| arg.starts_with(LEVEL_CLI_ARG)) {
        level_index = arg.split(LEVEL_CLI_ARG).last().unwrap().parse().unwrap();
        level_index -= 1;
    };

    // TODO: move these into ctx
    let mut current_level = PlayableLevel::load(levels[level_index]).await;
    let mut player = Entity {
        pos: current_level.level.player,
    };
    let mut crates: Vec<Entity> = vec![];

    for pos in &current_level.level.crates {
        crates.push(Entity { pos: *pos });
    }

    loop {
        ///////// UPDATE
        ctx.gamepads.poll();

        #[cfg(debug_assertions)]
        if is_key_pressed(KeyCode::Escape) {
            ctx.request_quit = true;
        }

        if input::action_pressed(input::Action::Reset, &ctx.gamepads) {
            current_level.reset();
            player.pos = current_level.level.player;
            for (i, c) in crates.iter_mut().enumerate() {
                c.pos = *current_level.level.crates.get(i).unwrap();
            }
            macroquad::audio::play_sound_once(&ctx.audio.sfx.reset);
        }

        if current_level.complete {
            if input::action_pressed(input::Action::Confirm, &ctx.gamepads) {
                // DRY THIS THE HECK UP w/ init load
                level_index += 1;
                if level_index >= levels.len() {
                    level_index = 0;
                }
                current_level = PlayableLevel::load(levels[level_index]).await;

                player = Entity {
                    pos: current_level.level.player,
                };
                crates.clear();

                for pos in &current_level.level.crates {
                    crates.push(Entity { pos: *pos });
                }
            }
        } else {
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

            let new_player_pos = player.pos.clone().add(move_player).to_owned();
            let crate_at_new_player_pos = crates.iter().find(|c| c.pos == new_player_pos);
            let mut move_crate = false;

            if !move_player.is_zero() {
                match crate_at_new_player_pos {
                    Some(c) => {
                        let new_crate_pos = c.pos.clone().add(move_player).to_owned();
                        let wall_at_new_crate_pos = current_level
                            .level
                            .walls
                            .iter()
                            .find(|w| *w == &new_crate_pos);
                        let other_crate_at_new_crate_pos =
                            crates.iter().find(|c| c.pos == new_crate_pos);

                        if wall_at_new_crate_pos.is_none() && other_crate_at_new_crate_pos.is_none()
                        {
                            move_crate = true;
                        }
                    }
                    None => {
                        let wall_at_new_player_pos = current_level
                            .level
                            .walls
                            .iter()
                            .find(|w| *w == &new_player_pos);
                        match wall_at_new_player_pos {
                            None => {
                                player.pos = new_player_pos;
                                current_level.steps += 1;
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
                    current_level.pushes += 1;
                    macroquad::audio::play_sound_once(&ctx.audio.sfx.push);
                }

                if crates.iter().all(|c| {
                    current_level
                        .level
                        .storage_locations
                        .clone() // idk if cloning is right here
                        .into_iter()
                        .any(|sl| sl == c.pos)
                }) {
                    macroquad::audio::play_sound_once(&ctx.audio.sfx.level_complete);
                    current_level.complete = true;
                }
            }
        }

        ///////// DRAW

        set_camera(&ctx.render_target_cam);

        clear_background(DARKGRAY);
        current_level.level.draw(&ctx.textures);
        draw::draw_tile(&ctx.textures.player, &player.pos);
        for c in &crates {
            draw::draw_tile(&ctx.textures.krate, &c.pos);
        }

        if current_level.complete {
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
        draw_text(current_level.level.name.as_str(), 48., 32., 32., WHITE);
        draw_text(
            format!(
                "Steps: {} | Pushes: {}",
                current_level.steps, current_level.pushes
            )
            .as_str(),
            48.,
            72.,
            32.,
            WHITE,
        );

        set_default_camera();

        clear_background(BLACK); // Will be the letterbox color

        // Draw 'render_target' to window screen, porperly scaled and letterboxed
        let scale: f32 = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );
        draw_texture_ex(
            &ctx.render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5,
            (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );

        if ctx.request_quit {
            break;
        }

        next_frame().await
    }
}
