use macroquad::window::set_fullscreen;

/// user-set options to customize the experience to their liking
pub struct Settings {
    /// whether or not audio should play
    mute: bool,
    /// whether or not the window should take up the entire screen
    fullscreen: bool,
}

const FULLSCREEN: &str = "fullscreen";
const MUTE: &str = "mute";

impl Settings {
    pub fn load() -> Self {
        let mut fullscreen = false;
        let mut mute = false;

        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let Some(storage_fullscreen) = storage.get(FULLSCREEN) {
            fullscreen = storage_fullscreen == "true";
        }
        if let Some(storage_mute) = storage.get(MUTE) {
            mute = storage_mute == "true";
        }

        if fullscreen {
            set_fullscreen(fullscreen);
        }

        Self { mute, fullscreen }
    }

    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn is_muted(&self) -> bool {
        self.mute
    }

    pub fn toggle_mute(&mut self) -> bool {
        self.mute = !self.mute;
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        storage.set(MUTE, self.mute.to_string().as_str());
        self.mute
    }

    pub fn toggle_fullscreen(&mut self) -> bool {
        self.fullscreen = !self.fullscreen;
        set_fullscreen(self.fullscreen);
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        storage.set(FULLSCREEN, self.mute.to_string().as_str());
        self.fullscreen
    }
}
