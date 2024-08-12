use macroquad::prelude::*;

const TILE_SIZE: i32 = 64;

/// grid position
#[derive(Debug, Clone, Copy)]
struct Vec {
    x: i32,
    y: i32,
}

impl PartialEq for Vec {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vec {
    fn add(&mut self, vec: Vec) -> &mut Vec {
        self.x += vec.x;
        self.y += vec.y;
        self
    }
}

struct Entity {
    texture: Texture2D,
    pos: Vec,
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

#[macroquad::main("Sokoworld")]
async fn main() {
    let mut player = Entity {
        texture: load_texture("assets/player.png").await.unwrap(),
        pos: Vec { x: 3, y: 3 },
    };

    let mut krate = Entity {
        texture: load_texture("assets/crate.png").await.unwrap(),
        pos: Vec { x: 5, y: 5 },
    };

    loop {
        #[cfg(debug_assertions)]
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // let dt = get_frame_time();

        let mut move_player = Vec { x: 0, y: 0 };

        if action_pressed(Action::Up) {
            move_player.y = -1;
        } else if action_pressed(Action::Down) {
            move_player.y = 1;
        } else if action_pressed(Action::Left) {
            move_player.x = -1;
        } else if action_pressed(Action::Right) {
            move_player.x = 1;
        }

        let new_pos = player.pos.clone().add(move_player).to_owned();
        if new_pos == krate.pos {
            krate.pos.add(move_player);
        } else {
            player.pos = new_pos;
        }

        clear_background(DARKGRAY);
        player.draw();
        krate.draw();

        next_frame().await
    }
}
