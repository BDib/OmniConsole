use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use super::{Theme, Profile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub default_profile: String,
    pub profiles: Vec<Profile>,
    pub themes: Vec<Theme>,
    pub active_theme: String,
    pub font_family: String,
    pub font_size: u16,
    pub cursor_style: CursorStyle,
    pub cursor_blink: bool,
    pub scrollback_lines: usize,
    pub default_rtl_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Bar,
    Underline,
}

impl Default for Settings {
    fn default() -> Self {
        let default_profile = Profile {
            name: "Default".to_string(),
            command: get_default_shell(),
            args: vec![],
            env: std::collections::HashMap::new(),
            working_directory: None,
            rtl_mode: None,
        };

        Settings {
            default_profile: "Default".to_string(),
            profiles: vec![default_profile],
            themes: vec![Theme::default()],
            active_theme: "Default".to_string(),
            font_family: "Cascadia Code".to_string(),
            font_size: 14,
            cursor_style: CursorStyle::Block,
            cursor_blink: true,
            scrollback_lines: 10000,
            default_rtl_mode: false,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("omniconsole");
        
        let config_file = config_dir.join("settings.toml");
        
        let mut settings = if config_file.exists() {
            let content = std::fs::read_to_string(&config_file).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            let settings = Settings::default();
            settings.save();
            settings
        };

        // Scan themes/ directory and append custom themes
        if let Ok(entries) = std::fs::read_dir("themes") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(theme) = toml::from_str::<Theme>(&content) {
                            if !settings.themes.iter().any(|t| t.name == theme.name) {
                                settings.themes.push(theme);
                            }
                        }
                    }
                }
            }
        }

        settings
    }

    pub fn save(&self) {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("omniconsole");
        
        std::fs::create_dir_all(&config_dir).ok();
        
        let config_file = config_dir.join("settings.toml");
        let content = toml::to_string_pretty(self).unwrap_or_default();
        std::fs::write(config_file, content).ok();
    }
}

fn get_default_shell() -> String {
    if cfg!(target_os = "windows") {
        "powershell.exe".to_string()
    } else if cfg!(target_os = "macos") {
        "/bin/zsh".to_string()
    } else {
        "/bin/bash".to_string()
    }
}
