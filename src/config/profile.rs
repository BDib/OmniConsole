#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    pub working_directory: Option<PathBuf>,
    pub rtl_mode: Option<bool>,
}

impl Profile {
    pub fn powershell() -> Self {
        Profile {
            name: "PowerShell".to_string(),
            command: "powershell.exe".to_string(),
            args: vec![],
            env: HashMap::new(),
            working_directory: None,
            rtl_mode: None,
        }
    }

    pub fn cmd() -> Self {
        Profile {
            name: "Command Prompt".to_string(),
            command: "cmd.exe".to_string(),
            args: vec![],
            env: HashMap::new(),
            working_directory: None,
            rtl_mode: None,
        }
    }

    pub fn bash() -> Self {
        Profile {
            name: "Bash".to_string(),
            command: "/bin/bash".to_string(),
            args: vec![],
            env: HashMap::new(),
            working_directory: None,
            rtl_mode: None,
        }
    }

    pub fn zsh() -> Self {
        Profile {
            name: "Zsh".to_string(),
            command: "/bin/zsh".to_string(),
            args: vec![],
            env: HashMap::new(),
            working_directory: None,
            rtl_mode: None,
        }
    }
}
