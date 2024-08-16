use macroquad::window::set_fullscreen;

/// user-set options to customize the experience to their liking
pub struct Settings {
    mute: bool,
    fullscreen: bool,
}

impl Settings {
    pub fn load() -> Self {
        // TODO: load these from somewhere
        Self {
            mute: false,
            fullscreen: false,
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn is_muted(&self) -> bool {
        self.mute
    }

    pub fn toggle_mute(&mut self) -> bool {
        self.mute = !self.mute;
        self.mute
    }

    pub fn toggle_fullscreen(&mut self) -> bool {
        self.fullscreen = !self.fullscreen;
        set_fullscreen(self.fullscreen);
        self.fullscreen
    }
}
