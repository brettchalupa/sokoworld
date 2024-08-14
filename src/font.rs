use macroquad::text::{load_ttf_font, Font};

pub struct FontAtlas {
    pub regular: Font,
}

impl FontAtlas {
    pub async fn new() -> Self {
        let mut regular = load_ttf_font("assets/Atkinson-Hyperlegible-Regular-102.ttf")
            .await
            .unwrap();
        regular.set_filter(macroquad::miniquad::FilterMode::Linear);
        Self { regular }
    }
}
