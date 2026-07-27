//! # OmniConsole
//!
//! A fast, complete terminal emulator with Right-to-Left (RTL) support for English and Arabic text.
//! Built in Rust using the `iced` GUI framework.
//!
//! ## Key Modules
//!
//! - **`pty`**: Manages the pseudo-terminal (PTY) spawn, read, and write operations.
//! - **`terminal`**: Contains the terminal buffer, parser, cell structure, and active screen viewport state.
//! - **`bidi`**: Implements RTL/Arabic character detection, Arabic letter shaping, and visual bidi reordering.
//! - **`ui`**: Defines the main graphical user interface, event subscriptions, layout, settings dialog, and scrollback.
//! - **`config`**: Handles application settings, shell profiles, and custom themes stored in TOML files.

pub mod pty;
pub mod terminal;
pub mod bidi;
pub mod ui;
pub mod config;

#[cfg(test)]
mod tests;
