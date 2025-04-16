use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct ThemeConfig {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct FontConfig {
    pub family: String,
    pub size: u32,
    pub line_height: f32,
}

#[derive(Deserialize, Serialize)]
pub struct EditorConfig {
    pub padding: u32,
    pub border_radius: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub theme: ThemeConfig,
    pub font: FontConfig,
    pub editor: EditorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: ThemeConfig {
                name: "tokyo-night".to_string(),
            },
            font: FontConfig {
                family: "IBM Plex Sans Arabic".to_string(),
                size: 20,         // Increased from 16
                line_height: 2.0, // Increased from 1.8
            },
            editor: EditorConfig {
                padding: 30,       // Increased from 20
                border_radius: 16, // Increased from 12
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string("config.toml") {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write("config.toml", content)?;
        Ok(())
    }
}
