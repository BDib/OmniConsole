#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use iced::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: ColorDef,
    pub foreground: ColorDef,
    pub cursor: ColorDef,
    pub selection: ColorDef,
    pub black: ColorDef,
    pub red: ColorDef,
    pub green: ColorDef,
    pub yellow: ColorDef,
    pub blue: ColorDef,
    pub magenta: ColorDef,
    pub cyan: ColorDef,
    pub white: ColorDef,
    pub bright_black: ColorDef,
    pub bright_red: ColorDef,
    pub bright_green: ColorDef,
    pub bright_yellow: ColorDef,
    pub bright_blue: ColorDef,
    pub bright_magenta: ColorDef,
    pub bright_cyan: ColorDef,
    pub bright_white: ColorDef,
    pub tab_bar_background: ColorDef,
    pub tab_bar_active: ColorDef,
    pub tab_bar_inactive: ColorDef,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorDef {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<ColorDef> for Color {
    fn from(def: ColorDef) -> Self {
        Color {
            r: def.r,
            g: def.g,
            b: def.b,
            a: def.a,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            name: "Default".to_string(),
            background: rgb(30, 30, 30),
            foreground: rgb(204, 204, 204),
            cursor: rgb(255, 255, 255),
            selection: rgb(38, 79, 120),
            black: rgb(0, 0, 0),
            red: rgb(205, 49, 49),
            green: rgb(13, 188, 121),
            yellow: rgb(229, 229, 16),
            blue: rgb(36, 114, 200),
            magenta: rgb(188, 63, 188),
            cyan: rgb(17, 168, 205),
            white: rgb(229, 229, 229),
            bright_black: rgb(102, 102, 102),
            bright_red: rgb(241, 76, 76),
            bright_green: rgb(22, 198, 12),
            bright_yellow: rgb(245, 245, 67),
            bright_blue: rgb(59, 142, 234),
            bright_magenta: rgb(214, 112, 214),
            bright_cyan: rgb(41, 184, 219),
            bright_white: rgb(255, 255, 255),
            tab_bar_background: rgb(45, 45, 45),
            tab_bar_active: rgb(30, 30, 30),
            tab_bar_inactive: rgb(55, 55, 55),
        }
    }

    pub fn light() -> Self {
        Theme {
            name: "Light".to_string(),
            background: rgb(255, 255, 255),
            foreground: rgb(51, 51, 51),
            cursor: rgb(0, 0, 0),
            selection: rgb(173, 214, 255),
            black: rgb(0, 0, 0),
            red: rgb(205, 49, 49),
            green: rgb(13, 188, 121),
            yellow: rgb(229, 229, 16),
            blue: rgb(36, 114, 200),
            magenta: rgb(188, 63, 188),
            cyan: rgb(17, 168, 205),
            white: rgb(229, 229, 229),
            bright_black: rgb(102, 102, 102),
            bright_red: rgb(241, 76, 76),
            bright_green: rgb(22, 198, 12),
            bright_yellow: rgb(245, 245, 67),
            bright_blue: rgb(59, 142, 234),
            bright_magenta: rgb(214, 112, 214),
            bright_cyan: rgb(41, 184, 219),
            bright_white: rgb(255, 255, 255),
            tab_bar_background: rgb(240, 240, 240),
            tab_bar_active: rgb(255, 255, 255),
            tab_bar_inactive: rgb(220, 220, 220),
        }
    }

    pub fn dracula() -> Self {
        Theme {
            name: "Dracula".to_string(),
            background: rgb(40, 42, 54),
            foreground: rgb(248, 248, 242),
            cursor: rgb(248, 248, 242),
            selection: rgb(68, 71, 90),
            black: rgb(0, 0, 0),
            red: rgb(255, 85, 85),
            green: rgb(80, 250, 123),
            yellow: rgb(241, 250, 140),
            blue: rgb(98, 114, 164),
            magenta: rgb(255, 121, 198),
            cyan: rgb(139, 233, 253),
            white: rgb(255, 255, 255),
            bright_black: rgb(68, 71, 90),
            bright_red: rgb(255, 121, 198),
            bright_green: rgb(248, 248, 242),
            bright_yellow: rgb(241, 250, 140),
            bright_blue: rgb(189, 147, 249),
            bright_magenta: rgb(255, 121, 198),
            bright_cyan: rgb(139, 233, 253),
            bright_white: rgb(255, 255, 255),
            tab_bar_background: rgb(40, 42, 54),
            tab_bar_active: rgb(68, 71, 90),
            tab_bar_inactive: rgb(50, 52, 66),
        }
    }
}

fn rgb(r: u8, g: u8, b: u8) -> ColorDef {
    ColorDef {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}
