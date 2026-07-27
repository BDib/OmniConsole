use iced::{
    keyboard::{self, key::Named},
    widget::{button, column, container, row, scrollable, text},
    Element, Length, Task, Theme, Subscription,
};
use tokio::sync::mpsc;

use crate::config::Settings;
use crate::config::theme::Theme as AppTheme;
use crate::pty::{Pty, PtyEvent};
use crate::terminal::{Direction, Screen};
use crate::bidi::reorder::get_display_cells;

#[derive(Debug, Clone)]
pub enum Message {
    NewTab,
    CloseTab(usize),
    SwitchTab(usize),
    TerminalInput(String),
    ToggleRtl,
    OpenSettings,
    CloseSettings,
    UpdateSettings(Settings),
    Tick,
    KeyPressed(iced::keyboard::Event),
    MouseScrolled(iced::mouse::ScrollDelta),
    FontFamilyChanged(String),
    FontSizeChanged(String),
    ThemeChanged(String),
    DefaultProfileChanged(String),
    ClipboardPasted(Option<String>),
}

pub struct OmniConsole {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub settings: Settings,
    pub show_settings: bool,
    pub pty_receivers: Vec<Option<mpsc::Receiver<PtyEvent>>>,
    pub font_family_static: &'static str,
}

pub struct Tab {
    pub screen: Screen,
    pub pty: Option<Pty>,
    pub title: String,
    pub rtl_mode: bool,
    pub scroll_offset: usize,
}

impl Tab {
    fn new(cols: u16, rows: u16, scrollback: usize, title: String) -> Self {
        Tab {
            screen: Screen::new(cols, rows, scrollback),
            pty: None,
            title,
            rtl_mode: false,
            scroll_offset: 0,
        }
    }
}

impl OmniConsole {
    pub fn run() -> iced::Result {
        iced::application("OmniConsole", Self::update, Self::view)
            .theme(Self::theme)
            .subscription(Self::subscription)
            .run_with(Self::new)
    }

    fn get_active_theme(&self) -> AppTheme {
        match self.settings.active_theme.as_str() {
            "Default" => AppTheme::dark(),
            "Light" => AppTheme::light(),
            "Dracula" => AppTheme::dracula(),
            _ => {
                if let Some(theme) = self.settings.themes.iter().find(|t| t.name == self.settings.active_theme) {
                    theme.clone()
                } else {
                    AppTheme::dark()
                }
            }
        }
    }

    fn new() -> (Self, Task<Message>) {
        let settings = Settings::load();
        let font_family_static = Box::leak(settings.font_family.clone().into_boxed_str());
        let mut app = OmniConsole {
            tabs: vec![],
            active_tab: 0,
            settings,
            show_settings: false,
            pty_receivers: vec![],
            font_family_static,
        };

        app.create_tab("".to_string());
        
        (app, Task::none())
    }

    pub fn create_tab(&mut self, mut title: String) {
        let cols = 80;
        let rows = 24;
        let scrollback = self.settings.scrollback_lines;
        
        let profile = self.settings.profiles.iter()
            .find(|p| p.name == self.settings.default_profile)
            .or_else(|| self.settings.profiles.first())
            .unwrap()
            .clone();

        if title.is_empty() || title.starts_with("Terminal") || title == "PowerShell" {
            title = profile.name.clone();
        }

        let mut tab = Tab::new(cols, rows, scrollback, title);
        tab.rtl_mode = self.settings.default_rtl_mode;
        
        match Pty::new(&profile.command, &profile.args, cols, rows) {
            Ok(pty) => {
                let (rx, pty) = pty.spawn_reader();
                tab.pty = Some(pty);
                self.pty_receivers.push(Some(rx));
            }
            Err(e) => {
                eprintln!("Failed to spawn PTY: {}", e);
                self.pty_receivers.push(None);
            }
        }
        
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::Tick),
            iced::event::listen_with(|event, _status, _id| {
                match event {
                    iced::Event::Keyboard(keyboard_event) => {
                        Some(Message::KeyPressed(keyboard_event))
                    }
                    iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta }) => {
                        Some(Message::MouseScrolled(delta))
                    }
                    _ => None,
                }
            }),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NewTab => {
                let tab_num = self.tabs.len() + 1;
                self.create_tab(format!("Terminal {}", tab_num));
            }
            Message::CloseTab(index) => {
                if self.tabs.len() > 1 {
                    self.tabs.remove(index);
                    self.pty_receivers.remove(index);
                    if self.active_tab >= self.tabs.len() {
                        self.active_tab = self.tabs.len() - 1;
                    }
                }
            }
            Message::SwitchTab(index) => {
                if index < self.tabs.len() {
                    self.active_tab = index;
                }
            }
            Message::TerminalInput(input) => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    if let Some(pty) = &mut tab.pty {
                        let _ = pty.write_str(&input);
                    }
                }
            }
            Message::ToggleRtl => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    tab.rtl_mode = !tab.rtl_mode;
                }
            }
            Message::OpenSettings => {
                self.show_settings = true;
            }
            Message::CloseSettings => {
                self.show_settings = false;
            }
            Message::UpdateSettings(settings) => {
                self.font_family_static = Box::leak(settings.font_family.clone().into_boxed_str());
                self.settings = settings;
                self.settings.save();
                self.show_settings = false;
            }
            Message::FontFamilyChanged(family) => {
                self.font_family_static = Box::leak(family.clone().into_boxed_str());
                self.settings.font_family = family;
                self.settings.save();
            }
            Message::FontSizeChanged(size_str) => {
                if let Ok(size) = size_str.parse::<u16>() {
                    self.settings.font_size = size;
                    self.settings.save();
                }
            }
            Message::ThemeChanged(theme_name) => {
                self.settings.active_theme = theme_name;
                self.settings.save();
            }
            Message::DefaultProfileChanged(profile_name) => {
                self.settings.default_profile = profile_name;
                self.settings.save();
            }
            Message::ClipboardPasted(Some(text)) => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    if let Some(pty) = &mut tab.pty {
                        let _ = pty.write_str(&text);
                    }
                }
            }
            Message::ClipboardPasted(None) => {}
            Message::KeyPressed(event) => {
                if self.show_settings {
                    return Task::none();
                }

                if let iced::keyboard::Event::KeyPressed { key, text, modifiers, .. } = event {
                    // Ctrl+V: Paste from clipboard
                    if key == keyboard::Key::Character("v".into()) && modifiers.control() {
                        return iced::clipboard::read().map(Message::ClipboardPasted);
                    }

                    // Ctrl+T: New tab
                    if key == keyboard::Key::Character("t".into()) && modifiers.control() {
                        return Task::perform(async {}, |_| Message::NewTab);
                    }

                    // Ctrl+W: Close tab
                    if key == keyboard::Key::Character("w".into()) && modifiers.control() {
                        let idx = self.active_tab;
                        return Task::perform(async move {}, move |_| Message::CloseTab(idx));
                    }

                    // Ctrl+Shift+R: Toggle RTL
                    if key == keyboard::Key::Character("r".into()) && modifiers.control() && modifiers.shift() {
                        return Task::perform(async {}, |_| Message::ToggleRtl);
                    }

                    // Ctrl+,: Settings
                    if key == keyboard::Key::Character(",".into()) && modifiers.control() {
                        return Task::perform(async {}, |_| Message::OpenSettings);
                    }

                    // Shift+PageUp / Shift+PageDown: Scroll
                    if key == keyboard::Key::Named(Named::PageUp) && modifiers.shift() {
                        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                            let max_offset = tab.screen.buffer.lines.len().saturating_sub(tab.screen.buffer.rows as usize);
                            tab.scroll_offset = (tab.scroll_offset + 10).min(max_offset);
                            return Task::none();
                        }
                    }
                    if key == keyboard::Key::Named(Named::PageDown) && modifiers.shift() {
                        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                            tab.scroll_offset = tab.scroll_offset.saturating_sub(10);
                            return Task::none();
                        }
                    }

                    // Ctrl + [a-z]: control characters
                    if modifiers.control() && !modifiers.alt() && !modifiers.logo() {
                        if let keyboard::Key::Character(ch_str) = &key {
                            if let Some(ch) = ch_str.chars().next() {
                                let ch_lower = ch.to_ascii_lowercase();
                                if ('a'..='z').contains(&ch_lower) {
                                    let ascii_val = (ch_lower as u8 - b'a' + 1) as char;
                                    return Task::perform(async move { ascii_val.to_string() }, Message::TerminalInput);
                                }
                            }
                        }
                    }

                    // Handle regular typing
                    if let Some(text) = &text {
                        let s: &str = text;
                        if !s.is_empty() && !modifiers.control() && !modifiers.alt() && !modifiers.logo() {
                            let input = s.to_string();
                            return Task::perform(async move { input }, Message::TerminalInput);
                        }
                    }

                    // Handle special keys
                    let input = match key {
                        keyboard::Key::Named(Named::Enter) => "\r".to_string(),
                        keyboard::Key::Named(Named::Backspace) => "\x08".to_string(),
                        keyboard::Key::Named(Named::Tab) => "\t".to_string(),
                        keyboard::Key::Named(Named::Escape) => "\x1b".to_string(),
                        keyboard::Key::Named(Named::ArrowUp) => "\x1b[A".to_string(),
                        keyboard::Key::Named(Named::ArrowDown) => "\x1b[B".to_string(),
                        keyboard::Key::Named(Named::ArrowRight) => "\x1b[C".to_string(),
                        keyboard::Key::Named(Named::ArrowLeft) => "\x1b[D".to_string(),
                        keyboard::Key::Named(Named::Home) => "\x1b[H".to_string(),
                        keyboard::Key::Named(Named::End) => "\x1b[F".to_string(),
                        keyboard::Key::Named(Named::PageUp) => "\x1b[5~".to_string(),
                        keyboard::Key::Named(Named::PageDown) => "\x1b[6~".to_string(),
                        keyboard::Key::Named(Named::Delete) => "\x1b[3~".to_string(),
                        _ => return Task::none(),
                    };

                    return Task::perform(async move { input }, Message::TerminalInput);
                }
            }
            Message::MouseScrolled(delta) => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    let lines = match delta {
                        iced::mouse::ScrollDelta::Lines { y, .. } => y,
                        iced::mouse::ScrollDelta::Pixels { y, .. } => {
                            if y > 0.0 { 1.0 } else if y < 0.0 { -1.0 } else { 0.0 }
                        }
                    };

                    let scroll_amount = lines.round() as i32;
                    if scroll_amount > 0 {
                        let max_offset = tab.screen.buffer.lines.len().saturating_sub(tab.screen.buffer.rows as usize);
                        tab.scroll_offset = (tab.scroll_offset + scroll_amount as usize).min(max_offset);
                    } else if scroll_amount < 0 {
                        tab.scroll_offset = tab.scroll_offset.saturating_sub(-scroll_amount as usize);
                    }
                }
            }
            Message::Tick => {
                for (i, rx) in self.pty_receivers.iter_mut().enumerate() {
                    if let Some(rx) = rx {
                        while let Ok(event) = rx.try_recv() {
                            match event {
                                PtyEvent::Output(output) => {
                                    if let Some(tab) = self.tabs.get_mut(i) {
                                        let old_len = tab.screen.buffer.lines.len();
                                        tab.screen.process_input(&output);
                                        let new_len = tab.screen.buffer.lines.len();
                                        if tab.scroll_offset > 0 && new_len > old_len {
                                            tab.scroll_offset += new_len - old_len;
                                            let max_offset = tab.screen.buffer.lines.len().saturating_sub(tab.screen.buffer.rows as usize);
                                            tab.scroll_offset = tab.scroll_offset.min(max_offset);
                                        }
                                    }
                                }
                                PtyEvent::Exited => {
                                    if let Some(tab) = self.tabs.get_mut(i) {
                                        tab.pty = None;
                                        tab.title = format!("{} [Exited]", tab.title);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        if self.show_settings {
            return self.settings_view();
        }
        self.main_view()
    }

    fn settings_view(&self) -> Element<'_, Message> {
        let title = text("Settings").size(24);
        
        let mut available_themes = vec!["Default".to_string(), "Light".to_string(), "Dracula".to_string()];
        for theme in &self.settings.themes {
            if !available_themes.contains(&theme.name) {
                available_themes.push(theme.name.clone());
            }
        }

        let mut theme_buttons = row![].spacing(8);
        for theme_name in &available_themes {
            let is_active = theme_name == &self.settings.active_theme;
            theme_buttons = theme_buttons.push(
                button(text(theme_name.clone()))
                    .on_press(Message::ThemeChanged(theme_name.clone()))
                    .style(if is_active {
                        iced::widget::button::primary
                    } else {
                        iced::widget::button::secondary
                    })
            );
        }

        let mut profile_buttons = row![].spacing(8);
        for profile in &self.settings.profiles {
            let is_default = profile.name == self.settings.default_profile;
            profile_buttons = profile_buttons.push(
                button(text(profile.name.clone()))
                    .on_press(Message::DefaultProfileChanged(profile.name.clone()))
                    .style(if is_default {
                        iced::widget::button::primary
                    } else {
                        iced::widget::button::secondary
                    })
            );
        }

        let general_section = column![
            text("General").size(18),
            row![
                text("Font Family:"),
                iced::widget::text_input("Cascadia Code", &self.settings.font_family)
                    .on_input(Message::FontFamilyChanged),
            ].spacing(8).align_y(iced::Alignment::Center),
            row![
                text("Font Size:"),
                iced::widget::text_input("14", &self.settings.font_size.to_string())
                    .on_input(Message::FontSizeChanged),
            ].spacing(8).align_y(iced::Alignment::Center),
            row![
                text("Default RTL Mode:"),
                button(text(if self.settings.default_rtl_mode { "ON" } else { "OFF" }))
                    .on_press({
                        let mut s = self.settings.clone();
                        s.default_rtl_mode = !s.default_rtl_mode;
                        Message::UpdateSettings(s)
                    }),
            ].spacing(8).align_y(iced::Alignment::Center),
            column![
                text("Theme:"),
                theme_buttons,
            ].spacing(8),
            column![
                text("Default Shell Profile:"),
                profile_buttons,
            ].spacing(8),
        ].spacing(8);
        
        let buttons = row![
            button(text("Close"))
                .on_press(Message::CloseSettings)
                .style(iced::widget::button::primary),
        ].spacing(8);
        
        let content = column![
            title,
            iced::widget::vertical_rule(1),
            general_section,
            iced::widget::vertical_rule(1),
            buttons,
        ]
        .spacing(16)
        .max_width(600);
        
        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Color::from_rgb(0.15, 0.15, 0.15).into()),
                ..Default::default()
            })
            .into()
    }

    fn main_view(&self) -> Element<'_, Message> {
        let mut tab_row = row![].spacing(2);
        for (i, tab) in self.tabs.iter().enumerate() {
            let is_active = i == self.active_tab;
            let tab_title = if tab.rtl_mode {
                format!("{} [RTL]", tab.title)
            } else {
                tab.title.clone()
            };

            let close_btn = button(text("x"))
                .on_press(Message::CloseTab(i))
                .padding(2);

            let tab_element = row![
                button(text(tab_title))
                    .on_press(Message::SwitchTab(i))
                    .style(if is_active {
                        iced::widget::button::primary
                    } else {
                        iced::widget::button::secondary
                    }),
                close_btn,
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center);

            tab_row = tab_row.push(tab_element);
        }

        let new_tab_btn = button(text("+"))
            .on_press(Message::NewTab)
            .padding([4, 8]);

        let tab_bar = row![
            tab_row,
            iced::widget::horizontal_space(),
            new_tab_btn,
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let tab_bar_element = container(tab_bar)
            .width(Length::Fill)
            .padding([4, 8])
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Color::from_rgb(0.25, 0.25, 0.25).into()),
                ..Default::default()
            });

        let theme = self.get_active_theme();
        let font_size = self.settings.font_size;
        let theme_bg: iced::Color = theme.background.into();

        let font_family_static = self.font_family_static;
        let terminal_font = iced::Font {
            family: iced::font::Family::Name(font_family_static),
            ..Default::default()
        };

        let terminal: Element<'_, Message> = if let Some(tab) = self.tabs.get(self.active_tab) {
            let visible_rows = tab.screen.buffer.rows as usize;
            let rtl_mode = tab.rtl_mode;
            let cursor_x = tab.screen.cursor.x as usize;
            let cursor_y = tab.screen.cursor.y as usize;

            let total_lines = tab.screen.buffer.lines.len();
            let viewport_top = total_lines.saturating_sub(visible_rows);
            let scrolled_top = viewport_top.saturating_sub(tab.scroll_offset);

            let mut lines_vec: Vec<Element<'_, Message>> = Vec::new();

            for y in 0..visible_rows {
                let line_idx = scrolled_top + y;
                let line = match tab.screen.buffer.get_line(line_idx) {
                    Some(line) => line,
                    None => continue,
                };

                let direction = if rtl_mode {
                    Direction::RTL
                } else {
                    line.direction
                };

                let display_cells = get_display_cells(&line.cells, direction);

                let mut line_content = row![].spacing(0);

                for (x, display_cell) in display_cells.iter().enumerate() {
                    let ch = if display_cell.cell.ch == '\0' || display_cell.cell.ch == '\x20' {
                        ' '
                    } else {
                        display_cell.cell.ch
                    };

                    let cell = &display_cell.cell;
                    let (fg_color, _bg_color) = self.cell_colors(cell, &theme);

                    let is_cursor = tab.scroll_offset == 0 && y == cursor_y && x == cursor_x && tab.screen.cursor.visible;

                    let rendered: Element<'_, Message> = if is_cursor {
                        // Cursor: white block with dark character
                        let cursor_char = if ch == ' ' { ' ' } else { ch };
                        container(text(cursor_char.to_string()).size(font_size).font(terminal_font).color(iced::Color::from_rgb(0.0, 0.0, 0.0)))
                            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                                background: Some(iced::Color::from_rgb(0.9, 0.9, 0.9).into()),
                                ..Default::default()
                            })
                            .into()
                    } else {
                        text(ch.to_string())
                            .size(font_size)
                            .font(terminal_font)
                            .color(fg_color)
                            .into()
                    };

                    line_content = line_content.push(rendered);
                }

                lines_vec.push(line_content.into());
            }

            let lines = column(lines_vec).spacing(0);

            let rtl_label = if rtl_mode { "ON" } else { "OFF" };
            let rtl_toggle = button(text(format!("RTL: {}", rtl_label)))
                .on_press(Message::ToggleRtl)
                .padding([4, 8]);

            let status_bar = row![
                rtl_toggle,
                iced::widget::horizontal_space(),
                text(format!("Ln {}, Col {}", cursor_y + 1, cursor_x + 1)).size(12),
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center);

            let terminal_content = column![
                scrollable(lines).height(Length::Fill),
                status_bar,
            ];

            container(terminal_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(8)
                .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(theme_bg.into()),
                    ..Default::default()
                })
                .into()
        } else {
            text("No terminal").into()
        };

        column![
            tab_bar_element,
            terminal,
        ]
        .into()
    }

    fn cell_colors(&self, cell: &crate::terminal::Cell, theme: &AppTheme) -> (iced::Color, iced::Color) {
        let ansi_colors = [
            theme.black,       // 0
            theme.red,         // 1
            theme.green,       // 2
            theme.yellow,      // 3
            theme.blue,        // 4
            theme.magenta,     // 5
            theme.cyan,        // 6
            theme.white,       // 7
            theme.bright_black,  // 8
            theme.bright_red,    // 9
            theme.bright_green,  // 10
            theme.bright_yellow, // 11
            theme.bright_blue,   // 12
            theme.bright_magenta,// 13
            theme.bright_cyan,   // 14
            theme.bright_white,  // 15
        ];

        let fg_idx = cell.fg.min(15) as usize;
        let bg_idx = cell.bg.min(15) as usize;

        let mut fg: iced::Color = ansi_colors[fg_idx].into();
        let mut bg: iced::Color = ansi_colors[bg_idx].into();

        if cell.reverse {
            std::mem::swap(&mut fg, &mut bg);
        }

        if cell.dim {
            fg = iced::Color {
                r: fg.r * 0.5,
                g: fg.g * 0.5,
                b: fg.b * 0.5,
                a: fg.a,
            };
        }

        (fg, bg)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
