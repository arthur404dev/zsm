use std::collections::{HashMap, BTreeSet};
use zellij_tile::prelude::{BareKey, KeyModifier, KeyWithModifier};

/// Actions that can be triggered by keybinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyAction {
    // Main screen actions
    MoveUp,
    MoveDown,
    Select,
    DeleteSession,
    Exit,
    ClearSearch,
    
    // New session screen actions
    Confirm,
    Cancel,
    LaunchFilepicker,
    ClearFolder,
    CorrectName,
    
    // Character input (special case)
    CharacterInput(char),
    Backspace,
}

/// Manages keybind mappings and lookups
#[derive(Debug, Clone)]
pub struct KeybindManager {
    /// List of key bindings (key, action pairs)
    bindings: Vec<(KeyWithModifier, KeyAction)>,
    /// Mapping from actions to their configured keys (for help text)
    action_to_keys: HashMap<KeyAction, Vec<KeyWithModifier>>,
}

impl KeybindManager {
    /// Create a new keybind manager with default bindings
    pub fn new() -> Self {
        let mut manager = Self {
            bindings: Vec::new(),
            action_to_keys: HashMap::new(),
        };
        
        manager.set_defaults();
        manager
    }
    
    /// Set default keybinds
    fn set_defaults(&mut self) {
        // Main screen defaults
        self.add_binding(KeyAction::MoveUp, key_from_bare(BareKey::Up));
        self.add_binding(KeyAction::MoveUp, key_with_ctrl('p'));
        self.add_binding(KeyAction::MoveDown, key_from_bare(BareKey::Down));
        self.add_binding(KeyAction::MoveDown, key_with_ctrl('n'));
        self.add_binding(KeyAction::Select, key_from_bare(BareKey::Enter));
        self.add_binding(KeyAction::DeleteSession, key_from_bare(BareKey::Delete));
        self.add_binding(KeyAction::ClearSearch, key_from_bare(BareKey::Esc));
        self.add_binding(KeyAction::Exit, key_with_ctrl('c'));
        self.add_binding(KeyAction::Backspace, key_from_bare(BareKey::Backspace));
        
        // New session screen defaults
        self.add_binding(KeyAction::Confirm, key_from_bare(BareKey::Enter));
        self.add_binding(KeyAction::Cancel, key_from_bare(BareKey::Esc));
        self.add_binding(KeyAction::LaunchFilepicker, key_with_ctrl('f'));
        self.add_binding(KeyAction::ClearFolder, key_with_ctrl('c'));
        self.add_binding(KeyAction::CorrectName, key_with_ctrl('r'));
    }
    
    /// Add a keybind mapping
    pub fn add_binding(&mut self, action: KeyAction, key: KeyWithModifier) {
        self.bindings.push((key.clone(), action));
        self.action_to_keys.entry(action).or_insert_with(Vec::new).push(key);
    }
    
    /// Clear all bindings for an action
    pub fn clear_action(&mut self, action: KeyAction) {
        // Remove from bindings list
        self.bindings.retain(|(_, a)| *a != action);
        // Remove from action_to_keys
        self.action_to_keys.remove(&action);
    }
    
    /// Set bindings for an action (replaces existing)
    pub fn set_action_keys(&mut self, action: KeyAction, keys: Vec<KeyWithModifier>) {
        self.clear_action(action);
        for key in keys {
            self.add_binding(action, key);
        }
    }
    
    /// Look up action for a key press
    pub fn get_action(&self, key: &KeyWithModifier) -> Option<KeyAction> {
        // Search through bindings for a match
        for (bound_key, action) in &self.bindings {
            if keys_equal(bound_key, key) {
                return Some(*action);
            }
        }
        
        // Handle character input specially - only if no modifiers are pressed
        if let BareKey::Char(c) = key.bare_key {
            if key.key_modifiers.is_empty() && c != '\n' {
                return Some(KeyAction::CharacterInput(c));
            }
        }
        
        None
    }
    
    /// Get all keys configured for an action
    pub fn get_keys_for_action(&self, action: KeyAction) -> Vec<KeyWithModifier> {
        self.action_to_keys.get(&action).cloned().unwrap_or_default()
    }
    
    /// Format keys for display in help text
    pub fn format_keys_for_action(&self, action: KeyAction) -> String {
        let keys = self.get_keys_for_action(action);
        if keys.is_empty() {
            return "None".to_string();
        }
        
        keys.iter()
            .map(|key| format_key_for_display(key))
            .collect::<Vec<_>>()
            .join("/")
    }
}

impl Default for KeybindManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to compare two KeyWithModifier instances for equality
fn keys_equal(a: &KeyWithModifier, b: &KeyWithModifier) -> bool {
    a.bare_key == b.bare_key && a.key_modifiers == b.key_modifiers
}

/// Helper function to create a KeyWithModifier from a BareKey
fn key_from_bare(bare_key: BareKey) -> KeyWithModifier {
    KeyWithModifier {
        bare_key,
        key_modifiers: BTreeSet::new(),
    }
}

/// Helper function to create a KeyWithModifier with Ctrl modifier
fn key_with_ctrl(c: char) -> KeyWithModifier {
    let mut modifiers = BTreeSet::new();
    modifiers.insert(KeyModifier::Ctrl);
    KeyWithModifier {
        bare_key: BareKey::Char(c),
        key_modifiers: modifiers,
    }
}

/// Format a key combination for display
pub fn format_key_for_display(key: &KeyWithModifier) -> String {
    let mut parts = Vec::new();
    
    // Add modifiers
    for modifier in &key.key_modifiers {
        match modifier {
            KeyModifier::Ctrl => parts.push("Ctrl"),
            KeyModifier::Alt => parts.push("Alt"),
            KeyModifier::Shift => parts.push("Shift"),
            KeyModifier::Super => parts.push("Super"),
        }
    }
    
    // Add the base key
    let key_str = match &key.bare_key {
        BareKey::Char(c) => c.to_uppercase().to_string(),
        BareKey::Enter => "Enter".to_string(),
        BareKey::Esc => "Esc".to_string(),
        BareKey::Backspace => "Backspace".to_string(),
        BareKey::Delete => "Delete".to_string(),
        BareKey::Up => "↑".to_string(),
        BareKey::Down => "↓".to_string(),
        BareKey::Left => "←".to_string(),
        BareKey::Right => "→".to_string(),
        BareKey::Tab => "Tab".to_string(),

        _ => format!("{:?}", key.bare_key),
    };
    
    parts.push(&key_str);
    parts.join("+")
}

/// Parse a key string into a KeyWithModifier
/// Examples: "Ctrl+p", "Enter", "Esc", "a", "Up"
pub fn parse_key_string(key_str: &str) -> Result<KeyWithModifier, String> {
    let key_str = key_str.trim();
    
    if key_str.is_empty() {
        return Err("Empty key string".to_string());
    }
    
    // Split by '+' to separate modifiers from the base key
    let parts: Vec<&str> = key_str.split('+').collect();
    
    if parts.is_empty() {
        return Err("Invalid key string format".to_string());
    }
    
    // Last part is the base key, everything else is modifiers
    let base_key_str = parts.last().unwrap();
    let modifier_strs = &parts[..parts.len() - 1];
    
    // Parse modifiers
    let mut modifiers = BTreeSet::new();
    for modifier_str in modifier_strs {
        match modifier_str.to_lowercase().as_str() {
            "ctrl" => { modifiers.insert(KeyModifier::Ctrl); },
            "alt" => { modifiers.insert(KeyModifier::Alt); },
            "shift" => { modifiers.insert(KeyModifier::Shift); },
            _ => return Err(format!("Unknown modifier: {}", modifier_str)),
        }
    }
    
    // Parse base key
    let bare_key = match base_key_str.to_lowercase().as_str() {
        "enter" => BareKey::Enter,
        "esc" | "escape" => BareKey::Esc,
        "backspace" => BareKey::Backspace,
        "delete" | "del" => BareKey::Delete,
        "up" => BareKey::Up,
        "down" => BareKey::Down,
        "left" => BareKey::Left,
        "right" => BareKey::Right,
        "tab" => BareKey::Tab,
        "space" => BareKey::Char(' '),
        "home" => BareKey::Home,
        "end" => BareKey::End,
        "pageup" => BareKey::PageUp,
        "pagedown" => BareKey::PageDown,
        "insert" => BareKey::Insert,
        // Function keys
        "f1" => BareKey::F(1),
        "f2" => BareKey::F(2),
        "f3" => BareKey::F(3),
        "f4" => BareKey::F(4),
        "f5" => BareKey::F(5),
        "f6" => BareKey::F(6),
        "f7" => BareKey::F(7),
        "f8" => BareKey::F(8),
        "f9" => BareKey::F(9),
        "f10" => BareKey::F(10),
        "f11" => BareKey::F(11),
        "f12" => BareKey::F(12),
        // Single character
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            if c.is_ascii() {
                BareKey::Char(c.to_ascii_lowercase())
            } else {
                return Err(format!("Non-ASCII character not supported: {}", c));
            }
        }
        _ => return Err(format!("Unknown key: {}", base_key_str)),
    };
    
    Ok(KeyWithModifier { bare_key, key_modifiers: modifiers })
}

/// Parse multiple key strings separated by spaces
pub fn parse_key_strings(keys_str: &str) -> Result<Vec<KeyWithModifier>, String> {
    let mut keys = Vec::new();
    
    for key_str in keys_str.split_whitespace() {
        keys.push(parse_key_string(key_str)?);
    }
    
    if keys.is_empty() {
        return Err("No keys specified".to_string());
    }
    
    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_keybinds() {
        let manager = KeybindManager::new();
        
        // Test navigation
        assert_eq!(
            manager.get_action(&key_from_bare(BareKey::Up)),
            Some(KeyAction::MoveUp)
        );
        assert_eq!(
            manager.get_action(&key_with_ctrl('p')),
            Some(KeyAction::MoveUp)
        );
        assert_eq!(
            manager.get_action(&key_from_bare(BareKey::Down)),
            Some(KeyAction::MoveDown)
        );
        assert_eq!(
            manager.get_action(&key_with_ctrl('n')),
            Some(KeyAction::MoveDown)
        );
        
        // Test actions
        assert_eq!(
            manager.get_action(&key_from_bare(BareKey::Enter)),
            Some(KeyAction::Select)
        );
        assert_eq!(
            manager.get_action(&key_from_bare(BareKey::Delete)),
            Some(KeyAction::DeleteSession)
        );
    }
    
    #[test]
    fn test_character_input() {
        let manager = KeybindManager::new();
        
        let key = KeyWithModifier {
            bare_key: BareKey::Char('a'),
            modifiers: vec![],
        };
        
        assert_eq!(
            manager.get_action(&key),
            Some(KeyAction::CharacterInput('a'))
        );
    }
    
    #[test]
    fn test_key_formatting() {
        assert_eq!(
            format_key_for_display(&key_from_bare(BareKey::Up)),
            "↑"
        );
        assert_eq!(
            format_key_for_display(&key_with_ctrl('p')),
            "Ctrl+P"
        );
        assert_eq!(
            format_key_for_display(&key_from_bare(BareKey::Enter)),
            "Enter"
        );
    }
    
    #[test]
    fn test_key_parsing() {
        // Test simple keys
        assert_eq!(
            parse_key_string("Enter").unwrap(),
            key_from_bare(BareKey::Enter)
        );
        assert_eq!(
            parse_key_string("Esc").unwrap(),
            key_from_bare(BareKey::Esc)
        );
        assert_eq!(
            parse_key_string("Up").unwrap(),
            key_from_bare(BareKey::Up)
        );
        
        // Test character keys
        assert_eq!(
            parse_key_string("a").unwrap(),
            key_from_bare(BareKey::Char('a'))
        );
        assert_eq!(
            parse_key_string("A").unwrap(),
            key_from_bare(BareKey::Char('a')) // Should be lowercase
        );
        
        // Test keys with modifiers
        assert_eq!(
            parse_key_string("Ctrl+p").unwrap(),
            key_with_ctrl('p')
        );
        assert_eq!(
            parse_key_string("ctrl+P").unwrap(), // Case insensitive
            key_with_ctrl('p')
        );
        
        // Test multiple modifiers
        let key = parse_key_string("Ctrl+Alt+a").unwrap();
        assert_eq!(key.bare_key, BareKey::Char('a'));
        assert!(key.modifiers.contains(&KeyModifier::Ctrl));
        assert!(key.modifiers.contains(&KeyModifier::Alt));
        
        // Test invalid keys
        assert!(parse_key_string("").is_err());
        assert!(parse_key_string("InvalidKey").is_err());
        assert!(parse_key_string("Ctrl+InvalidKey").is_err());
    }
    
    #[test]
    fn test_multiple_key_parsing() {
        let keys = parse_key_strings("Up Ctrl+p").unwrap();
        assert_eq!(keys.len(), 2);
        assert_eq!(keys[0], key_from_bare(BareKey::Up));
        assert_eq!(keys[1], key_with_ctrl('p'));
        
        // Test empty string
        assert!(parse_key_strings("").is_err());
        assert!(parse_key_strings("   ").is_err());
    }
}