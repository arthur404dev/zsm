# Zoxide Session Manager (ZSM)

A powerful Zellij plugin that seamlessly integrates **zoxide** (smart directory navigation) with **Zellij sessions**, making it incredibly easy to jump between your most-used directories and create/manage development sessions.

## 🚀 What Does It Do?

ZSM bridges the gap between `zoxide` and Zellij's session management:

- **🎯 Smart Directory Listing**: Shows your most-used directories from zoxide, ranked by frequency
- **⚡ Instant Session Creation**: Create new Zellij sessions in any directory with one keystroke
- **🔍 Fuzzy Search**: Search through directories and existing sessions simultaneously
- **🧠 Intelligent Naming**: Auto-generates meaningful session names with conflict resolution
- **⚙️ Layout Support**: Choose from available layouts or use your default layout

## 📋 Requirements

- **[zoxide](https://github.com/ajeetdsouza/zoxide)** - Install and use it for a while to build up your directory database
- **Zellij** with plugin support

## 📦 Installation

### Option 1: Download Release (Recommended)
1. Download the latest `zsm.wasm` from [Releases](https://github.com/liam-mackie/zsm/releases)
2. Copy it to your Zellij plugins directory (~/.config/zellij/plugins/):
3. Add to your Zellij configuration (see [Configuration](#%EF%B8%8F-configuration))

```bash
mkdir -p ~/.config/zellij/plugins
curl -sSL -o ~/.config/zellij/plugins https://github.com/liam-mackie/zsm/releases/download/v0.1.0/zsm.wasm
```

### Option 2: Build from Source

```bash
# Add WASM target if not already added
rustup target add wasm32-wasip1

# Build the plugin
cargo build --target wasm32-wasip1 --release

# The plugin will be at: target/wasm32-wasip1/release/zsm.wasm
```

## ⚙️ Configuration

Add ZSM to your configuration file (e.g., `~/.config/zellij/config.kdl`):

### Basic Configuration - bind to a key

```kdl
keybinds clear-defaults=true {
...
    shared_except "locked" {
        bind "<your-key>" { 
            // Note: you must use the absolute path to the plugin file
            LaunchOrFocusPlugin "file:/your/home/dir/.config/zellij/plugins/zsm.wasm" {
                floating true
                move_to_focused_tab true
            }
        }
    }
...
}
```

### Advanced Configuration - with options
```kdl
keybinds clear-defaults=true {
...
    shared_except "locked" {
        bind "<your-key>" { 
            // Note: you must use the absolulte path to the plugin file
            LaunchOrFocusPlugin "file:/your/home/dir/.config/zellij/plugins/zsm.wasm" {
                floating true
                move_to_focused_tab true
                
                // Default layout for session creation
                default_layout "development"
            
                // Session name separator (default: ".")
                session_separator "_"
            }
        }
    }
...
```

### Configuration Options

| Option              | Description                               | Default | Example         |
|---------------------|-------------------------------------------|---------|-----------------|
| `default_layout`    | Layout name for session creation         | None    | `"development"` |
| `session_separator` | Character used in session names           | `"."`   | `"-"` or `"_"`  |

### Keybind Configuration

ZSM supports customizable keybinds. You can override any of the default keybinds by specifying them in your plugin configuration:

```kdl
plugin location="zsm.wasm" {
    // Custom keybinds - each action can have multiple keys
    move_up "Up Ctrl+p k"           // Navigate up: Arrow, Ctrl+P, or K
    move_down "Down Ctrl+n j"       // Navigate down: Arrow, Ctrl+N, or J
    select "Enter Space"            // Select item: Enter or Space
    delete_session "Delete x"       // Delete session: Delete or X
    exit "Esc q Ctrl+c"            // Exit plugin: Esc, Q, or Ctrl+C
}
```

#### Available Actions

| Action | Description | Default Keys |
|--------|-------------|--------------|
| `move_up` | Navigate up in list | `Up`, `Ctrl+P` |
| `move_down` | Navigate down in list | `Down`, `Ctrl+N` |
| `select` | Select item (switch/create session) | `Enter` |
| `delete_session` | Delete selected session | `Delete` |
| `exit` | Exit plugin | `Esc`, `Ctrl+C` |
| `clear_search` | Clear search input | `Esc` |
| `confirm` | Confirm action (new session screen) | `Enter` |
| `cancel` | Cancel/go back | `Esc` |
| `launch_filepicker` | Open filepicker | `Ctrl+F` |
| `clear_folder` | Clear session folder | `Ctrl+C` |
| `correct_name` | Go back to name entry | `Ctrl+R` |

#### Key Format

Keys can be specified in the following formats:
- **Simple keys**: `Enter`, `Esc`, `Space`, `Delete`, `Up`, `Down`, `Left`, `Right`
- **Character keys**: `a`, `b`, `c`, etc. (case insensitive)
- **Function keys**: `F1`, `F2`, ..., `F12`
- **Modified keys**: `Ctrl+p`, `Alt+a`, `Shift+f1`
- **Multiple keys**: `"Up Ctrl+p k"` (space-separated)

#### Vim-Style Navigation

By default, ZSM includes vim-style navigation with `Ctrl+N` (down) and `Ctrl+P` (up). You can customize this further:

```kdl
plugin location="zsm.wasm" {
    move_up "Up k Ctrl+p"
    move_down "Down j Ctrl+n"
    select "Enter l"
    exit "Esc q"
}
```

## 🎯 How It Works

### 1. Directory Display

ZSM shows your zoxide directories ranked by usage frequency:

```
~/projects/my-app        (most used)
~/work/client-project
~/personal/website
~/dotfiles              (least used)
```

### 2. Smart Session Naming

ZSM automatically generates meaningful session names:

- **Simple**: `~/projects/webapp` → `webapp`
- **Nested**: `~/projects/client/backend` → `client.backend`
- **Conflicts**: Multiple "app" directories → `client.app`, `personal.app`
- **Long names**: Intelligent abbreviation → `very-long-project-name` → `v-l-p-name`

### 3. Session Integration

- **Existing sessions** are shown with indicators: `● current` or `○ available`
- **Both sessions AND directories** are displayed for complete context
- **Smart switching**: If a session already exists for a directory, switches to it instead of creating duplicates

### 4. Quick Workflows

**Jump to existing session**

1. Open ZSM
2. Type to search for session
3. Press `Enter` → Instantly switch

**Create new session or switch to existing**

1. Open ZSM  
2. Navigate to directory
3. Press `Enter` → Creates new session with default layout, or switches to existing session if one already exists for that directory

## 🔐 Permissions

ZSM requires these Zellij permissions:

- **RunCommands**: Execute zoxide queries
- **ReadApplicationState**: Read existing sessions and layouts
- **ChangeApplicationState**: Create and switch sessions  
- **MessageAndLaunchOtherPlugins**: Launch filepicker

## 🐛 Troubleshooting

### No directories showing?

- Ensure zoxide is installed: `which zoxide`
- Build up your directory database by navigating around: `cd ~/projects && cd ~/work`
- Check zoxide database: `zoxide query -l`

### Default layout not working?

- Verify layout name matches exactly (case-sensitive)
- Check available layouts in Zellij
- Layout must exist in current session

### Filepicker issues?

- Ensure MessageAndLaunchOtherPlugins permission is granted

## 🚧 Development

### Local Development

#### Option 1: Using Zellij Plugin Layout

```bash
# Start the plugin development layout
zellij -l zellij.kdl
# Use the default alt-r keybinding to reload the plugin
# If you exit the plugin, you can re-open it with the following command:
zellij action launch-or-focus-plugin file:target/wasm32-wasip1/debug/zsm.wasm
```

#### Option 2: Using watchexec

```bash
watchexec --exts rs -- 'cargo build --target wasm-wasip1; zellij action start-or-reload-plugin file:target/wasm32-wasip1/debug/zsm.wasm'
```

## 🤝 Contributing

Contributions welcome, though my time is limited so please be patient with reviews!

---

**Made with ❤️ for the Zellij community**

