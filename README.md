# OmniConsole

A fast terminal emulator with RTL (Right-to-Left) support for English/Arabic, built in Rust with the iced GUI framework.

## Features

- **Perfect Arabic & RTL Support**: Complete bidirectional text handling with automatic per-line RTL detection. Includes contextual **Arabic Letter Shaping** (e.g. joining characters) before visual bidi layout.
- **Manual RTL Toggle**: Press `Ctrl+Shift+R` or use the status bar toggle to force RTL mode per tab.
- **Visual Text Selection**: Drag with the mouse to select text on the terminal screen with live selection highlighting.
- **Right-Click Copy/Paste**:
  - Right-click when text is selected to **Copy** it to the clipboard.
  - Right-click when no text is selected to **Paste** clipboard text directly into the active shell.
- **Drag and Drop Files/Folders**: Drag any file or folder from your file manager and drop it onto OmniConsole to automatically paste its path (properly quoted if it contains spaces) at the terminal cursor.
- **Scrollback History**: Full support for scrollback history. Navigate scrollback manually with `Shift+PageUp` / `Shift+PageDown` or your Mouse Scroll Wheel. New terminal output automatically scrolls to the bottom unless you are scrolled up, in which case the scroll view locks onto historical text.
- **Settings UI**: Fully interactive graphical editor for font family, font size, active theme, default RTL mode, and shell profiles. Changes are persisted dynamically to `settings.toml`.
- **Dynamic Themes**: Built-in Dark, Light, and Dracula themes, with support for custom TOML themes scanned dynamically from the local `themes/` directory.
- **Shell Profiles**: Full support for PowerShell, Command Prompt, Bash, Zsh, and custom shells. Opening a new tab automatically names it after the active profile.
- **Fast Performance**: Native Rust implementation with optimal character cell grid mapping.

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
| `Ctrl+T` | New tab (spawns the active default shell profile) |
| `Ctrl+W` | Close tab |
| `Ctrl+Shift+R` | Toggle RTL mode |
| `Ctrl+V` | Paste from clipboard |
| `Ctrl+,` | Open settings dialog |
| `Shift+PageUp` | Scroll up scrollback buffer |
| `Shift+PageDown` | Scroll down scrollback buffer |

*OmniConsole fully supports mapping standard Control modifiers like `Ctrl+C` (SIGINT interrupt), `Ctrl+D` (EOF), `Ctrl+Z` (suspend), etc. to control shell processes.*

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
# ... more colors (cursor, selection, ansi black, red, green, etc.)
```

## Architecture

```
src/
├── main.rs          # Application entry point
├── lib.rs           # Library exports
├── pty.rs           # PTY (pseudo-terminal) management
├── tests.rs         # Unit tests for shaping, bidi reordering, and coordinates
├── terminal/
│   ├── mod.rs       # Terminal types and exports
│   ├── buffer.rs    # Screen buffer and viewport line translation
│   ├── parser.rs    # VT100/xterm escape sequence parser (CSI, SGR)
│   └── screen.rs    # Screen state and rendering
├── bidi/
│   ├── mod.rs       # Bidi algorithm integration
│   ├── reorder.rs   # Arabic shaping and visual reordering
│   └── detector.rs  # RTL character detection
├── ui/
│   ├── mod.rs       # UI module exports
│   └── app.rs       # Main application view, state, subscription, settings, mouse, and keyboard event handling
└── config/
    ├── mod.rs       # Config module exports
    ├── settings.rs  # Application settings (saving, loading, scanning custom themes)
    ├── theme.rs     # Theme definitions
    └── profile.rs   # Shell profiles
```

## License

MIT License
