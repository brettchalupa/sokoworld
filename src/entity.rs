use crate::vec2::Vec2;

pub struct Crate {
    /// grid position
    pub pos: Vec2,
    pub on_storage_location: bool,
}

pub struct Entity {
    /// grid position
    pub pos: Vec2,
}
