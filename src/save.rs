#[cfg(not(target_family = "wasm"))]
use crate::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(target_family = "wasm"))]
use std::path::PathBuf;

use crate::consts::VERSION;

/// game completion progress
#[derive(Debug, Deserialize, Serialize)]
pub struct Save {
    game_version: String,
    /// string key is PACKSLUG:LEVELNAME
    level_completions: HashMap<String, LevelCompletion>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LevelCompletion {
    pack: String,
    level: String,
    steps: i32,
    pushes: i32,
}

#[cfg(not(target_family = "wasm"))]
const SAVE_FILE: &str = "save.ron";

#[cfg(target_family = "wasm")]
const WASM_SAVE_KEY: &str = "save";

impl Default for Save {
    fn default() -> Self {
        Self {
            game_version: VERSION.to_string(),
            level_completions: HashMap::new(),
        }
    }
}

impl Save {
    /// loads the save file from disk; if it doesn't exist, instantiates a new one and saves it
    pub fn load() -> Self {
        #[cfg(target_family = "wasm")]
        let save = Self::load_wasm();

        #[cfg(not(target_family = "wasm"))]
        let save = Self::load_desktop();
        save.save();

        save
    }

    #[cfg(not(target_family = "wasm"))]
    fn load_desktop() -> Self {
        let save_path = Self::determine_save_path();

        if save_path.exists() {
            let toml_str = std::fs::read_to_string(save_path).expect("couldn't read save file");
            let save: Save = ron::from_str(toml_str.as_str()).unwrap();
            save
        } else {
            Self::default()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn determine_save_path() -> PathBuf {
        let project_dirs = fs::project_dirs();
        let save_dir = project_dirs.data_local_dir();
        std::fs::create_dir_all(save_dir).unwrap();
        let mut save_path = PathBuf::from(save_dir);
        save_path.push(SAVE_FILE);
        save_path
    }

    #[cfg(target_family = "wasm")]
    fn load_wasm() -> Self {
        let mut save = Self::default();
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let Some(wasm_save) = storage.get(WASM_SAVE_KEY) {
            save = ron::from_str(wasm_save.as_str()).unwrap();
        }
        save
    }

    pub fn complete_level(
        &mut self,
        pack_slug: String,
        level_title: String,
        steps: i32,
        pushes: i32,
    ) {
        self.level_completions.insert(
            Self::level_completion_key(&pack_slug, &level_title),
            LevelCompletion {
                pack: pack_slug,
                level: level_title,
                steps,
                pushes,
            },
        );
        self.save();
    }

    pub fn is_level_complete(&self, pack_slug: &String, level_title: &String) -> bool {
        self.level_completions
            .contains_key(&Self::level_completion_key(pack_slug, level_title))
    }

    fn level_completion_key(pack_slug: &String, level_title: &String) -> String {
        format!("{}:{}", pack_slug, level_title)
    }

    /// writes the save to local storage
    #[cfg(target_family = "wasm")]
    fn save(&self) {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        storage.set(WASM_SAVE_KEY, &self.to_ron_string().as_str());
    }

    #[cfg(not(target_family = "wasm"))]
    /// writes the save to disk
    fn save(&self) {
        std::fs::write(Self::determine_save_path(), self.to_ron_string())
            .expect("unable to write save file");
    }

    /// returns the save data in RON format as a pretty string
    fn to_ron_string(&self) -> String {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap()
    }
}
