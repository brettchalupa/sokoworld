use macroquad::prelude::*;
use sokoworld::consts::*;
use sokoworld::context::Context;
use sokoworld::scene::gameplay::Gameplay;
use sokoworld::scene::EScene;
use sokoworld::scene::{main_menu::MainMenu, Scene};

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
    let mut current_scene: Box<dyn Scene> = Box::new(MainMenu::new(&mut ctx).await);

    loop {
        ///////// UPDATE
        #[cfg(debug_assertions)]
        if is_key_pressed(KeyCode::Escape) {
            ctx.request_quit = true;
        }

        ctx.gamepads.poll();
        current_scene.update(&mut ctx);

        ///////// DRAW

        // render target drawing
        set_camera(&ctx.render_target_cam);
        clear_background(DARKGRAY);
        current_scene.draw(&mut ctx);

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

        if let Some(escene) = ctx.switch_scene_to {
            current_scene = match escene {
                EScene::MainMenu => Box::new(MainMenu::new(&mut ctx).await),
                EScene::Gameplay => Box::new(Gameplay::new(&mut ctx).await),
            };
            ctx.switch_scene_to = None;
        }

        next_frame().await
    }
}
