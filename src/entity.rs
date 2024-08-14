use crate::vec2::Vec2;

#[derive(Clone, Debug)]
pub struct Crate {
    /// grid position
    pub pos: Vec2,
    pub on_storage_location: bool,
}

#[derive(Clone, Debug)]
pub struct Entity {
    /// grid position
    pub pos: Vec2,
}
