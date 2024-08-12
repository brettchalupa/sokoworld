use macroquad::prelude::*;

const TILE_SIZE: i32 = 64;

/// grid position
#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

struct Player {
    texture: Texture2D,
    pos: Pos,
}

pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Restart,
}

/// just pressed, not held down
pub fn action_pressed(action: Action) -> bool {
    match action {
        Action::Up => is_key_pressed(KeyCode::W),
        Action::Down => is_key_pressed(KeyCode::S),
        Action::Left => is_key_pressed(KeyCode::A),
        Action::Right => is_key_pressed(KeyCode::D),
        Action::Restart => is_key_pressed(KeyCode::K),
    }
}

impl Player {
    fn update(&mut self, _dt: f32) {
        if action_pressed(Action::Up) {
            self.pos.y -= 1;
        }
        if action_pressed(Action::Down) {
            self.pos.y += 1;
        }
        if action_pressed(Action::Left) {
            self.pos.x -= 1;
        }
        if action_pressed(Action::Right) {
            self.pos.x += 1;
        }
    }

    fn draw(&self) {
        draw_texture(
            &self.texture,
            (self.pos.x * TILE_SIZE) as f32,
            (self.pos.y * TILE_SIZE) as f32,
            WHITE,
        );
    }
}

#[macroquad::main("Sokoworld")]
async fn main() {
    let mut player = Player {
        texture: load_texture("assets/player.png").await.unwrap(),
        pos: Pos { x: 3, y: 3 },
    };

    loop {
        #[cfg(debug_assertions)]
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let dt = get_frame_time();

        player.update(dt);

        clear_background(DARKGRAY);
        player.draw();

        next_frame().await
    }
}
