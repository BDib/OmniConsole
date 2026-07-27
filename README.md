# OmniConsole

A fast terminal emulator with RTL (Right-to-Left) support for English/Arabic, built in Rust with the iced GUI framework.

## Features

- **RTL Support**: Full bidirectional text handling with automatic per-line RTL detection
- **Manual RTL Toggle**: Press `Ctrl+Shift+R` to toggle RTL mode per tab
- **Tab Management**: Multiple terminal tabs with easy switching
- **Themes**: Built-in dark, light, and Dracula themes
- **Settings UI**: Graphical settings editor for fonts, themes, and profiles
- **Shell Profiles**: Support for PowerShell, Command Prompt, Bash, Zsh, and custom shells
- **Fast Performance**: Native Rust implementation for optimal speed

## Building

### Prerequisites

- Rust 1.70 or later
- For Windows: Visual Studio Build Tools
- For Linux: build-essential, libfontconfig-dev
- For macOS: Xcode Command Line Tools

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run
cargo run

# Run release build
cargo run --release
```

## Usage

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close tab |
| `Ctrl+Shift+R` | Toggle RTL mode |
| `Ctrl+Shift+D` | Split horizontal |
| `Ctrl+Shift+E` | Split vertical |
| `Ctrl+Shift+P` | Command palette |
| `Ctrl+,` | Open settings |

### RTL Features

#### Automatic RTL Detection
OmniConsole automatically detects RTL text (Arabic, Hebrew, etc.) on a per-line basis and displays it with proper right-to-left ordering.

#### Manual RTL Toggle
You can manually toggle RTL mode for any tab:
1. Press `Ctrl+Shift+R` or click the RTL button in the status bar
2. RTL mode will be indicated in the tab title
3. All output in the tab will be displayed in RTL mode

### Configuration

Settings are stored in:
- Windows: `%APPDATA%\omniconsole\settings.toml`
- Linux: `~/.config/omniconsole/settings.toml`
- macOS: `~/Library/Application Support/omniconsole/settings.toml`

### Theme Customization

Create custom themes by adding TOML files to the themes directory:

```toml
name = "My Theme"
background = { r = 0.1, g = 0.1, b = 0.1, a = 1.0 }
foreground = { r = 0.9, g = 0.9, b = 0.9, a = 1.0 }
# ... more colors
```

## Architecture

```
src/
├── main.rs          # Application entry point
├── lib.rs           # Library exports
├── pty.rs           # PTY (pseudo-terminal) management
├── terminal/
│   ├── mod.rs       # Terminal types and exports
│   ├── buffer.rs    # Screen buffer management
│   ├── parser.rs    # VT100/xterm escape sequence parser
│   ├── screen.rs    # Screen state and rendering
│   └── line.rs      # Line metadata and direction
├── bidi/
│   ├── mod.rs       # Bidi algorithm integration
│   ├── reorder.rs   # Visual reordering for display
│   └── detector.rs  # RTL character detection
├── ui/
│   ├── mod.rs       # UI module exports
│   ├── app.rs       # Main application state
│   ├── tab_bar.rs   # Tab management UI
│   ├── terminal_view.rs  # Terminal rendering
│   ├── settings.rs  # Settings dialog
│   └── command_palette.rs  # Command palette
└── config/
    ├── mod.rs       # Config module exports
    ├── settings.rs  # Application settings
    ├── theme.rs     # Theme definitions
    └── profile.rs   # Shell profiles
```

## Roadmap

- [ ] Split pane support
- [ ] GPU-accelerated text rendering
- [ ] Custom key bindings
- [ ] Import/export settings
- [ ] Windows Terminal profile import
- [ ] Search functionality
- [ ] Unicode block selection
- [ ] Clickable URLs
- [ ] Notification support

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
