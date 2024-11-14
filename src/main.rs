#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use miniquad::conf::Icon;
use sokoworld::consts::*;
use sokoworld::context::Context;
use sokoworld::level::pack::Pack;
use sokoworld::scene::gameplay::Gameplay;
use sokoworld::scene::level_select::LevelSelect;
use sokoworld::scene::EScene;
use sokoworld::scene::{main_menu::MainMenu, Scene};
use sokoworld::text::{draw_text, Size};

fn window_conf() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: true,
        icon: Some(Icon {
            small: include_bytes!("../icons/16x16.rgba").to_owned(),
            medium: include_bytes!("../icons/32x32.rgba").to_owned(),
            big: include_bytes!("../icons/64x64.rgba").to_owned(),
        }),
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        window_height: 720,
        window_resizable: true,
        window_title: String::from("SokoWorld"),
        window_width: 1280,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ctx = Context {
        ..Context::default().await
    };

    let mut current_scene: Box<dyn Scene>;

    // load pack & level from arg for quick testing, otherwise boot to main menu
    let args: Vec<String> = std::env::args().collect();
    if let Some(arg) = args.iter().find(|arg| arg.starts_with(PACK_CLI_ARG)) {
        let pack_file = arg.split(PACK_CLI_ARG).last().unwrap();
        let pack = Pack::load(&mut ctx, pack_file).await;

        let mut level_index = 0;
        if let Some(arg) = args.iter().find(|arg| arg.starts_with(LEVEL_CLI_ARG)) {
            level_index = arg.split(LEVEL_CLI_ARG).last().unwrap().parse().unwrap();
            level_index -= 1;
        };
        let level = pack.levels.get(level_index).unwrap();
        current_scene = Box::new(Gameplay::new(&mut ctx, level.clone(), level_index, pack).await);
    } else {
        current_scene = Box::new(MainMenu::new(&mut ctx).await);
    };

    loop {
        ///////// UPDATE
        #[cfg(debug_assertions)]
        if (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
            && is_key_down(KeyCode::Escape)
        {
            ctx.request_quit = true;
        }

        ctx.gamepads.poll();
        current_scene.update(&mut ctx);

        ///////// DRAW

        // render target drawing
        set_camera(&ctx.render_target_cam);
        clear_background(sokoworld::color::DARKGRAY);
        current_scene.draw(&mut ctx);

        // regular drawing
        set_default_camera();
        clear_background(sokoworld::color::DARKGRAY); // Will be the letterbox color

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

        if ctx.settings.show_fps() {
            draw_text(
                &mut ctx,
                format!("{}", (1. / get_frame_time()).round() as i32).as_str(),
                24.,
                36.,
                Size::Small,
                WHITE,
            );
        }

        // reloads the the pack from disk, useful for designing levels
        // this happens in main with a flag on Context because loading the file is async, and
        // making all update functions async becomes a hassle
        // it may make sense to make this only be present for debug builds, but also it's kind of
        // nice if people want to design and test their own levels
        if ctx.reload_level {
            ctx.reload_level = false;
            if let Some(current_pack_file) = ctx.current_pack_file.clone() {
                if let Some(current_level_index) = ctx.current_level_index {
                    let pack = Pack::load(&mut ctx, current_pack_file.as_str()).await;
                    let level = pack.levels.get(current_level_index).unwrap();
                    current_scene = Box::new(
                        Gameplay::new(&mut ctx, level.clone(), current_level_index, pack).await,
                    );
                }
            }
        }

        if let Some(escene) = ctx.switch_scene_to.clone() {
            current_scene = match escene {
                EScene::MainMenu => Box::new(MainMenu::new(&mut ctx).await),
                EScene::LevelSelect(pack) => {
                    Box::new(LevelSelect::new(&mut ctx, pack.clone()).await)
                }
                EScene::Gameplay(level, level_index, pack) => Box::new(
                    Gameplay::new(&mut ctx, level.clone(), level_index, pack.clone()).await,
                ),
            };
            ctx.switch_scene_to = None;
        }

        next_frame().await;

        if ctx.request_quit {
            break;
        }
    }
}
