use std::fs::read_to_string;
use std::io::Error;
use serde::{Serialize, Deserialize};

pub const SETTINGS_PATH: &'static str = "./data/settings.json";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    mono_font: String,
}

impl Settings {
    pub fn load() -> Self {
        let contents = match read_to_string(SETTINGS_PATH) {
            Ok(contents) => contents,
            Err(_) => String::new(),
        };

        match serde_json::from_str(contents.as_str()) {
            Ok(settings) => settings,
            Err(_) => Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let settings = serde_json::to_string(self)?;
        std::fs::write(SETTINGS_PATH, settings)?;

        Ok(())
    }

    pub fn set_mono_font(&mut self, mono_font: String) -> &mut Settings {
        self.mono_font = mono_font;
        self
    }

    pub fn mono_font(&self) -> &String {
        &self.mono_font
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mono_font: "Monospace 18".to_string(),
        }
    }
}
