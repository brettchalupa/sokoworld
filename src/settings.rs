#[cfg(not(target_family = "wasm"))]
use crate::fs;
use macroquad::window::set_fullscreen;
use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use std::path::PathBuf;

/// user-set options to customize the experience to their liking
#[derive(Deserialize, Serialize)]
pub struct Settings {
    /// whether or not audio should play
    mute: Option<bool>,
    /// whether or not the window should take up the entire screen
    fullscreen: Option<bool>,
    /// whether or not to show the frames per second in the upper left area of the screen
    show_fps: Option<bool>,
}

#[cfg(target_family = "wasm")]
const FULLSCREEN: &str = "fullscreen";
#[cfg(target_family = "wasm")]
const MUTE: &str = "mute";
#[cfg(target_family = "wasm")]
const SHOW_FPS: &str = "show_fps";
#[cfg(not(target_family = "wasm"))]
const SETTINGS_FILE: &str = "settings.toml";

impl Settings {
    fn default() -> Self {
        Settings {
            fullscreen: Some(false),
            mute: Some(false),
            show_fps: Some(false),
        }
    }

    pub fn load() -> Self {
        #[cfg(target_family = "wasm")]
        let settings = Self::load_wasm();

        #[cfg(not(target_family = "wasm"))]
        let settings = Self::load_desktop();

        if settings.is_fullscreen() {
            set_fullscreen(settings.is_fullscreen());
        }

        settings
    }

    #[cfg(target_family = "wasm")]
    fn load_wasm() -> Self {
        let mut settings = Self::default();
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let Some(storage_fullscreen) = storage.get(FULLSCREEN) {
            settings.fullscreen = Some(storage_fullscreen == "true");
        }
        if let Some(storage_mute) = storage.get(MUTE) {
            settings.mute = Some(storage_mute == "true");
        }
        if let Some(storage_show_fps) = storage.get(SHOW_FPS) {
            settings.mute = Some(storage_show_fps == "true");
        }
        settings
    }

    #[cfg(not(target_family = "wasm"))]
    fn load_desktop() -> Self {
        let settings_path = Self::determine_settings_path();

        if settings_path.exists() {
            let toml_str =
                std::fs::read_to_string(settings_path).expect("couldn't read settings file");
            let settings: Settings = toml::from_str(toml_str.as_str()).unwrap();
            settings
        } else {
            Self::default()
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
            .unwrap_or(Self::default().fullscreen.unwrap())
    }

    pub fn is_muted(&self) -> bool {
        self.mute.unwrap_or(Self::default().mute.unwrap())
    }

    pub fn show_fps(&self) -> bool {
        self.show_fps.unwrap_or(Self::default().show_fps.unwrap())
    }

    pub fn toggle_mute(&mut self) -> bool {
        self.mute = Some(!self.is_muted());

        self.save_settings();
        self.mute.unwrap()
    }

    pub fn toggle_fullscreen(&mut self) -> bool {
        self.fullscreen = Some(!self.is_fullscreen());
        set_fullscreen(self.is_fullscreen());
        self.save_settings();
        self.is_fullscreen()
    }

    pub fn toggle_show_fps(&mut self) -> bool {
        self.show_fps = Some(!self.show_fps());

        self.save_settings();
        self.show_fps()
    }

    #[cfg(target_family = "wasm")]
    fn save_settings(&self) {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        storage.set(MUTE, self.is_muted().to_string().as_str());
        storage.set(FULLSCREEN, self.is_fullscreen().to_string().as_str());
        storage.set(SHOW_FPS, self.show_fps().to_string().as_str());
    }

    #[cfg(not(target_family = "wasm"))]
    fn save_settings(&self) {
        let toml = toml::to_string(self).unwrap();
        std::fs::write(Self::determine_settings_path(), toml)
            .expect("unable to write settings file");
    }

    #[cfg(not(target_family = "wasm"))]
    fn determine_settings_path() -> PathBuf {
        let project_dirs = fs::project_dirs();
        let settings_dir = project_dirs.config_local_dir();
        std::fs::create_dir_all(settings_dir).unwrap();
        let mut settings_path = PathBuf::from(settings_dir);
        settings_path.push(SETTINGS_FILE);
        settings_path
    }
}
