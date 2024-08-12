use gamepads::Gamepads;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode;

pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Reset,
}

/// just pressed, not held down
pub fn action_pressed(action: Action, gamepads: &Gamepads) -> bool {
    keyboard_pressed(&action) || gamepad_pressed(&action, gamepads)
}

fn keyboard_pressed(action: &Action) -> bool {
    match action {
        Action::Up => is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up),
        Action::Down => is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down),
        Action::Left => is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left),
        Action::Right => is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right),
        Action::Reset => is_key_pressed(KeyCode::K) || is_key_pressed(KeyCode::Z),
        Action::Confirm => is_key_pressed(KeyCode::J) || is_key_pressed(KeyCode::X),
    }
}

/// checks the action for any of the connected gamepads
fn gamepad_pressed(action: &Action, gamepads: &Gamepads) -> bool {
    match action {
        Action::Up => gamepads
            .all()
            .any(|g| g.is_just_pressed(gamepads::Button::DPadUp)),
        Action::Down => gamepads
            .all()
            .any(|g| g.is_just_pressed(gamepads::Button::DPadDown)),
        Action::Left => gamepads
            .all()
            .any(|g| g.is_just_pressed(gamepads::Button::DPadLeft)),
        Action::Right => gamepads
            .all()
            .any(|g| g.is_just_pressed(gamepads::Button::DPadRight)),
        Action::Confirm => gamepads
            .all()
            .any(|g| g.is_just_pressed(gamepads::Button::ActionDown)),
        Action::Reset => gamepads
            .all()
            .any(|g| g.is_just_pressed(gamepads::Button::ActionUp)),
    }
}
