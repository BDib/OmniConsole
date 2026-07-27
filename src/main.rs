#![windows_subsystem = "windows"]

mod pty;
mod terminal;
mod bidi;
mod ui;
mod config;

use ui::app::OmniConsole;

fn main() -> iced::Result {
    env_logger::init();
    OmniConsole::run()
}
