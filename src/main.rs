use macroquad::prelude::*;
use sokoworld::consts::*;
use sokoworld::context::Context;
use sokoworld::level::{Pack, PlayableLevel};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("SokoWorld"),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ctx = Context {
        ..Context::default().await
    };

    let args: Vec<String> = std::env::args().collect();

    let mut level_index = 0;
    if let Some(arg) = args.iter().find(|arg| arg.starts_with(LEVEL_CLI_ARG)) {
        level_index = arg.split(LEVEL_CLI_ARG).last().unwrap().parse().unwrap();
        level_index -= 1;
    };
    let mut pack_file = "assets/pack-a.toml";
    if let Some(arg) = args.iter().find(|arg| arg.starts_with(PACK_CLI_ARG)) {
        pack_file = arg.split(PACK_CLI_ARG).last().unwrap();
    };

    let level_pack_str = macroquad::file::load_string(pack_file)
        .await
        .expect("Unable to read file");
    let pack: Pack = toml::from_str(level_pack_str.as_str()).unwrap();
    let mut current_level = PlayableLevel::load(pack.levels.get(level_index).unwrap());

    loop {
        ///////// UPDATE
        #[cfg(debug_assertions)]
        if is_key_pressed(KeyCode::Escape) {
            ctx.request_quit = true;
        }

        ctx.gamepads.poll();

        current_level.update(&mut ctx);

        if ctx.load_next_level {
            ctx.load_next_level = false;
            level_index += 1;
            if level_index >= pack.levels.len() {
                level_index = 0;
            }

            current_level = PlayableLevel::load(pack.levels.get(level_index).unwrap());
        }

        ///////// DRAW

        // render target drawing
        set_camera(&ctx.render_target_cam);
        clear_background(DARKGRAY);
        current_level.draw(&mut ctx);

        // regular drawing
        set_default_camera();
        clear_background(BLACK); // Will be the letterbox color

        // draw the render target properly scaled and letterboxed
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
