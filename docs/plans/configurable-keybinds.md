# Configurable Keybinds Implementation Plan

## Context

### Current Keybind Architecture

The ZSM plugin currently has hardcoded keybinds throughout the codebase:

#### Main Screen Keybinds (`src/state.rs:240-285`)
- **Up Arrow**: Navigate up in the list
- **Down Arrow**: Navigate down in the list  
- **Enter**: Switch to existing session or create new one
- **Delete**: Kill selected session
- **Esc**: Clear search or exit plugin
- **Ctrl+C**: Exit plugin
- **Character input**: Search/filter
- **Backspace**: Remove character from search

#### New Session Screen Keybinds (`src/state.rs:286-320`)
- **Enter**: Create session
- **Esc**: Go back to main screen
- **Ctrl+F**: Launch filepicker
- **Ctrl+C**: Clear session folder
- **Other keys**: Delegated to NewSessionInfo

#### NewSessionInfo Keybinds (`src/new_session_info.rs:96-139`)
- **Backspace**: Delete character
- **Ctrl+C**: Clear input
- **Ctrl+R**: Correct session name (go back from layout selection)
- **Esc**: Navigate back in UI flow
- **Character input**: Add to session name
- **Up/Down**: Navigate layout list

### Current Configuration System

The plugin configuration is loaded from Zellij's plugin configuration in `src/config.rs`:

1. Configuration is passed as `BTreeMap<String, String>` to the plugin
2. `Config::from_zellij_config()` parses the configuration
3. Currently supports:
   - `default_layout`: Optional string for default layout name
   - `session_separator`: String for session name separator (default: ".")

### Key Representation in Zellij

Zellij uses the following types for key handling:
- `KeyWithModifier`: Contains `bare_key: BareKey` and modifiers
- `BareKey`: Enum with variants like `Char(char)`, `Enter`, `Backspace`, `Up`, `Down`, etc.
- `KeyModifier`: Enum with `Ctrl`, `Alt`, `Shift`, etc.

## Spec

### Desired Behavior

1. **Add vim-style navigation**: Support Ctrl+N/Ctrl+P alongside arrow keys for navigation
2. **Make keybinds configurable**: Allow users to override default keybinds via plugin configuration
3. **Maintain sensible defaults**: Keep current keybinds as defaults for backward compatibility
4. **Support multiple bindings**: Allow multiple keys to trigger the same action

### Proposed Configuration Structure

```kdl
plugin location="zsm.wasm" {
    // Existing configuration
    default_layout "default"
    session_separator "."
    
    // New keybinds configuration
    keybinds {
        // Navigation
        move_up "Up" "Ctrl+p"
        move_down "Down" "Ctrl+n"
        
        // Actions
        select "Enter"
        delete_session "Delete"
        exit "Esc" "Ctrl+c"
        
        // Search
        clear_search "Esc"
        
        // New session screen
        launch_filepicker "Ctrl+f"
        clear_folder "Ctrl+c"
        correct_name "Ctrl+r"
    }
}
```

### Keybind Configuration Format

Each keybind action can accept:
- Single key: `"Enter"`
- Key with modifier: `"Ctrl+p"`
- Multiple bindings: `"Up" "Ctrl+p"`

### Actions to Support

#### Main Screen Actions
- `move_up`: Navigate up in list
- `move_down`: Navigate down in list
- `select`: Select item (switch/create session)
- `delete_session`: Delete selected session
- `exit`: Exit plugin
- `clear_search`: Clear search input

#### New Session Screen Actions
- `confirm`: Create session
- `cancel`: Go back to main screen
- `launch_filepicker`: Open filepicker
- `clear_folder`: Clear session folder
- `correct_name`: Go back to name entry

### Default Keybinds

| Action | Default Keys |
|--------|-------------|
| move_up | Up, Ctrl+P |
| move_down | Down, Ctrl+N |
| select | Enter |
| delete_session | Delete |
| exit | Esc, Ctrl+C |
| clear_search | Esc |
| confirm | Enter |
| cancel | Esc |
| launch_filepicker | Ctrl+F |
| clear_folder | Ctrl+C |
| correct_name | Ctrl+R |

## Implementation

### Task 1: Create Keybind Configuration Structure

**File**: `src/config.rs`

**Changes**:
- Add `KeybindConfig` struct to hold keybind mappings
- Add parsing logic to convert string representations to Zellij key types
- Define default keybinds
- Integrate with existing `Config` struct

### Task 2: Implement Key Parsing

**File**: `src/config.rs` or new `src/keybinds.rs`

**Changes**:
- Create parser for key strings (e.g., "Ctrl+p" → `KeyWithModifier`)
- Handle special keys: "Enter", "Esc", "Delete", "Up", "Down", etc.
- Handle modifiers: "Ctrl", "Alt", "Shift"
- Support single character keys: "a", "b", etc.

### Task 3: Create Keybind Action Enum

**File**: `src/keybinds.rs` (new file)

**Changes**:
- Define `KeyAction` enum with all supported actions
- Create mapping from `KeyWithModifier` to `KeyAction`
- Implement lookup method for finding action from key press

### Task 4: Refactor Main Screen Key Handler

**File**: `src/state.rs`

**Changes**:
- Replace hardcoded key matching with keybind configuration lookup
- Maintain same behavior but use configurable keys
- Add Ctrl+N/Ctrl+P to default navigation bindings

### Task 5: Refactor New Session Screen Key Handler

**File**: `src/state.rs` and `src/new_session_info.rs`

**Changes**:
- Update key handling to use configuration
- Ensure consistent behavior with new keybind system

### Task 6: Update Help Text

**File**: `src/ui/renderer.rs`

**Changes**:
- Make help text dynamic based on configured keybinds
- Show actual configured keys instead of hardcoded strings

### Task 7: Add Configuration Validation

**File**: `src/config.rs`

**Changes**:
- Validate keybind configuration on load
- Warn about invalid key specifications
- Ensure no conflicts in keybind mappings

### Task 8: Update Documentation

**Files**: `README.md`, `plugin.kdl`

**Changes**:
- Document new keybind configuration options
- Provide examples of custom keybind configurations
- Explain key string format

### Implementation Order

1. **Task 3** - Create KeyAction enum (foundation)
2. **Task 2** - Implement key parsing (needed for configuration)
3. **Task 1** - Create keybind configuration structure
4. **Task 4** - Refactor main screen handler (core functionality)
5. **Task 5** - Refactor new session screen handler
6. **Task 7** - Add configuration validation
7. **Task 6** - Update help text (user-facing improvement)
8. **Task 8** - Update documentation

### Risk Assessment

**Low Risk**:
- Adding new configuration options
- Documentation updates
- Adding vim-style defaults

**Medium Risk**:
- Key parsing implementation
- Configuration validation
- Dynamic help text

**High Risk**:
- Refactoring core key handling logic
- Ensuring backward compatibility
- Handling edge cases in key parsing

### Testing Strategy

1. Test default keybinds work as before
2. Test vim-style navigation (Ctrl+N/Ctrl+P)
3. Test custom keybind configuration
4. Test invalid configuration handling
5. Test multiple keys for same action
6. Test help text updates correctly
7. Test all actions are accessible via configured keys

### Configuration Examples

#### Vim-style Navigation Only
```kdl
keybinds {
    move_up "Ctrl+p" "k"
    move_down "Ctrl+n" "j"
}
```

#### Custom Exit Key
```kdl
keybinds {
    exit "q" "Ctrl+q"
}
```

#### Minimal Configuration
```kdl
keybinds {
    // Only override specific keys, others use defaults
    select "Space"
    delete_session "x"
}
```
