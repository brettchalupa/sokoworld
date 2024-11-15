pub const TILE_SIZE: i32 = 64;
pub const LEVEL_CLI_ARG: &str = "-l=";
pub const PACK_CLI_ARG: &str = "-p=";
pub const VIRTUAL_WIDTH: f32 = 1280.0;
pub const VIRTUAL_HEIGHT: f32 = 720.0;
/// how far down to display the title of various scenes
pub const TITLE_Y_INSET: f32 = 120.0;
/// how far right to display text when aligned
pub const X_INSET: f32 = 200.;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
/// delay in seconds between movement when held down
pub const MOVE_HELD_DELAY: f32 = 0.2;
