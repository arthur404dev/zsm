# Unified Enter Keybind Implementation Plan

## Context

### Current Architecture

The ZSM (Zoxide Session Manager) plugin currently has two distinct workflows for session management:

1. **Switch/New functionality (Enter key)**:
   - For existing sessions: switches directly to the session
   - For directories: creates a new session with incremented naming (e.g., `webapp.2`) and opens layout selection UI

2. **Quick Create functionality (Ctrl+Enter key)**:
   - Creates sessions immediately using the default layout without UI interaction
   - Still uses incremented naming for duplicates

### Key Files and Components

#### State Management (`src/state.rs`)

- **Lines 250-256**: Main screen keybind handling for Enter and Ctrl+Enter
- **Lines 292-308**: New session screen keybind handling  
- **Lines 397-428**: `handle_item_selection()` - processes Enter key for Switch/New
- **Lines 495-550**: `handle_quick_session_creation()` - processes Ctrl+Enter for Quick Create
- **Lines 164-175**: `is_incremented_session()` - checks for session name conflicts

#### Session Management (`src/session/manager.rs`)

- **Lines 58-78**: `generate_incremented_name()` - creates incremented session names (e.g., `webapp.2`, `webapp.3`)
- **Lines 27-31**: Session switching logic
- **Lines 37-49**: Session deletion logic

#### New Session Info (`src/new_session_info.rs`)

- **Lines 142-180**: `handle_quick_session_creation()` - Quick create implementation
- **Lines 182-211**: `handle_selection()` - Switch/New implementation with layout selection
- **Lines 15-22**: `EnteringState` enum managing UI states (name entry vs layout selection)

#### UI Components (`src/ui/renderer.rs`)

- **Line 246**: Current help text showing both Enter and Ctrl+Enter options
- **Line 244**: New session screen help text

### Current Session Duplication Logic

The system currently handles session name conflicts by:

1. Checking if base session name exists (`src/session/manager.rs:60`)
2. If exists, generating incremented names with separator (`.2`, `.3`, etc.)
3. Searching up to 1000 increments before falling back to UUID suffix
4. This logic is used in both Switch/New and Quick Create workflows

### Current Workflow Analysis

**Enter Key (Switch/New)**:

1. If existing session selected → switch immediately
2. If directory selected → generate incremented name → open layout selection UI → create session

**Ctrl+Enter Key (Quick Create)**:

1. If existing session selected → switch immediately  
2. If directory selected → generate incremented name → create session with default layout

## Spec

### Desired Behavior

Implement a unified Enter key behavior that:

1. **For existing sessions**: Switch to the session immediately (no change from current behavior)
2. **For directories with no existing session**: Create new session immediately using quick create logic (no layout selection UI)
3. **For directories with existing session**: Switch to existing session instead of creating duplicates
4. **Remove Ctrl+Enter functionality**: Only Enter key will be used
5. **Remove layout selection UI**: All session creation will use default layout or no layout

### Behavioral Changes

| Current State | Current Enter Behavior | Current Ctrl+Enter | New Enter Behavior |
|---------------|----------------------|-------------------|-------------------|
| Existing session selected | Switch to session | Switch to session | Switch to session (unchanged) |
| Directory with no session | Create incremented name + layout UI | Create with default layout | Create with default layout |
| Directory with existing session | Create incremented name + layout UI | Create with default layout | **Switch to existing session** |

### Files Requiring Changes

1. **`src/state.rs`**:
   - Remove Ctrl+Enter keybind handling (lines 254-256, 298-308)
   - Modify `handle_item_selection()` to use quick create logic instead of layout selection
   - Add logic to detect existing sessions for directories and switch instead of create
   - Remove `handle_quick_session_creation()` method

2. **`src/new_session_info.rs`**:
   - Remove or simplify layout selection logic since it won't be used
   - Keep `handle_quick_session_creation()` but may need modifications

3. **`src/ui/renderer.rs`**:
   - Update help text to remove Ctrl+Enter references
   - Simplify instructions to show only Enter functionality

4. **`src/session/manager.rs`**:
   - May need to add method to check if session exists for a given directory
   - `generate_incremented_name()` logic may become unused

### Session Detection Logic

Need to implement logic to determine if a directory already has an associated session:

- Check if any existing session name matches the directory's generated session name
- Check if any existing session name is an incremented version of the directory's session name
- Use existing `is_incremented_session()` logic from `src/state.rs:164-175`

## Implementation

### Task 1: Modify Main Screen Enter Key Handler

**File**: `src/state.rs`
**Function**: `handle_item_selection()` (lines 397-428)

**Changes**:

- For directory items, check if session already exists before creating new one
- If session exists, switch to it instead of creating incremented version
- If no session exists, use quick create logic (default layout, no UI)
- Remove the layout selection UI transition (`self.active_screen = ActiveScreen::NewSession`)

### Task 2: Remove Ctrl+Enter Keybind Support

**File**: `src/state.rs`

**Changes**:

- Remove Ctrl+Enter handling in `handle_main_screen_key()` (lines 254-256)
- Remove Ctrl+Enter handling in `handle_new_session_key()` (lines 298-308)  
- Remove `handle_quick_session_creation()` method (lines 495-550)

### Task 3: Add Session Existence Check Method

**File**: `src/state.rs` or `src/session/manager.rs`

**Changes**:

- Create method to check if a directory already has an associated session
- Use existing `is_incremented_session()` logic to match base names with incremented versions
- Return the existing session name if found

### Task 4: Update UI Help Text

**File**: `src/ui/renderer.rs`

**Changes**:

- Line 246: Update main screen help text to remove Ctrl+Enter reference
- Change from: `"↑/↓: Navigate • Enter: Switch/New • Ctrl+Enter: Quick create • Delete: Kill • Type: Search • Esc: Exit"`
- Change to: `"↑/↓: Navigate • Enter: Switch/Create • Delete: Kill • Type: Search • Esc: Exit"`

### Task 5: Simplify New Session Info Logic

**File**: `src/new_session_info.rs`

**Changes**:

- Since layout selection UI will no longer be used, simplify or remove related code
- Keep `handle_quick_session_creation()` method as it may still be called programmatically
- Remove or deprecate layout selection state management

### Task 6: Update Documentation

**Files**: `README.md`, `plugin.kdl`

**Changes**:

- Update README.md to reflect new unified Enter behavior
- Remove references to Ctrl+Enter functionality
- Update workflow examples to show new behavior
- Update plugin.kdl comments if needed

### Task 7: Test Session Detection Logic

**Verification Steps**:

- Test that existing sessions are properly detected and switched to
- Test that new sessions are created when no existing session found
- Test that incremented session names are properly matched to base names
- Verify that current session detection works correctly

### Task 8: Clean Up Unused Code

**Files**: Various

**Changes**:

- Remove unused layout selection UI code if no longer needed
- Remove unused methods related to Ctrl+Enter functionality
- Clean up any dead code paths

### Implementation Order

1. **Task 3** (Add session existence check) - Foundation for new logic
2. **Task 1** (Modify Enter key handler) - Core behavior change  
3. **Task 2** (Remove Ctrl+Enter) - Clean up old functionality
4. **Task 4** (Update UI text) - User-facing changes
5. **Task 5** (Simplify new session logic) - Clean up unused UI
6. **Task 7** (Testing) - Verify functionality
7. **Task 6** (Documentation) - Update docs
8. **Task 8** (Code cleanup) - Final cleanup

### Risk Assessment

**Low Risk**:

- UI text changes
- Documentation updates
- Removing unused keybinds

**Medium Risk**:

- Modifying core session creation logic
- Session existence detection logic

**High Risk**:

- Changes to `handle_item_selection()` as it's central to the plugin's functionality

### Testing Strategy

1. Test with directories that have no existing sessions
2. Test with directories that have existing sessions (exact name match)
3. Test with directories that have existing sessions (incremented name match)
4. Test switching to existing sessions (should remain unchanged)
5. Test session creation with and without default layout configured
6. Test edge cases like session name validation and current session detection
