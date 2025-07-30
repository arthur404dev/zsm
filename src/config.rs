use std::collections::BTreeMap;
use crate::keybinds::{KeybindManager, KeyAction, parse_key_strings};

/// Plugin configuration loaded from Zellij layout
#[derive(Debug, Clone)]
pub struct Config {
    /// Default layout for session creation
    pub default_layout: Option<String>,
    /// Separator used in session names (default: ".")
    pub session_separator: String,
    /// Keybind configuration
    pub keybinds: KeybindManager,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_layout: None,
            session_separator: ".".to_string(),
            keybinds: KeybindManager::new(),
        }
    }
}

impl Config {
    /// Create configuration from Zellij plugin configuration
    pub fn from_zellij_config(config: &BTreeMap<String, String>) -> Self {
        let mut keybinds = KeybindManager::new();
        
        // Parse keybind overrides
        parse_keybind_config(&mut keybinds, config);
        
        // Validate essential keybinds
        let validation_errors = validate_keybind_config(&keybinds);
        for error in validation_errors {
            eprintln!("Error: {}", error);
        }
        
        Self {
            default_layout: config.get("default_layout").cloned(),
            session_separator: config
                .get("session_separator")
                .cloned()
                .unwrap_or_else(|| ".".to_string()),
            keybinds,
        }
    }
}

/// Parse keybind configuration from the config map
fn parse_keybind_config(keybinds: &mut KeybindManager, config: &BTreeMap<String, String>) {
    // Map of config keys to actions
    let action_mappings = [
        ("move_up", KeyAction::MoveUp),
        ("move_down", KeyAction::MoveDown),
        ("select", KeyAction::Select),
        ("delete_session", KeyAction::DeleteSession),
        ("exit", KeyAction::Exit),
        ("clear_search", KeyAction::ClearSearch),
        ("confirm", KeyAction::Confirm),
        ("cancel", KeyAction::Cancel),
        ("launch_filepicker", KeyAction::LaunchFilepicker),
        ("clear_folder", KeyAction::ClearFolder),
        ("correct_name", KeyAction::CorrectName),
    ];
    
    let mut validation_errors = Vec::new();
    
    for (config_key, action) in action_mappings {
        if let Some(keys_str) = config.get(config_key) {
            match parse_key_strings(keys_str) {
                Ok(keys) => {
                    // Validate that we have at least one key
                    if keys.is_empty() {
                        validation_errors.push(format!("No keys specified for action '{}'", config_key));
                        continue;
                    }
                    
                    // Check for conflicts with existing bindings
                    for key in &keys {
                        if let Some(existing_action) = keybinds.get_action(key) {
                            // Allow overriding the same action
                            if existing_action != action {
                                validation_errors.push(format!(
                                    "Key conflict: '{}' is already bound to {:?}, cannot bind to {:?}",
                                    crate::keybinds::format_key_for_display(key),
                                    existing_action,
                                    action
                                ));
                            }
                        }
                    }
                    
                    keybinds.set_action_keys(action, keys);
                }
                Err(err) => {
                    validation_errors.push(format!("Invalid keybind configuration for '{}': {}", config_key, err));
                }
            }
        }
    }
    
    // Print validation errors (in a real implementation, these might be logged differently)
    for error in validation_errors {
        eprintln!("Warning: {}", error);
    }
}

/// Validate that essential actions have keybinds
pub fn validate_keybind_config(keybinds: &KeybindManager) -> Vec<String> {
    let mut errors = Vec::new();
    
    // Essential actions that must have keybinds
    let essential_actions = [
        (KeyAction::Select, "select"),
        (KeyAction::Exit, "exit"),
    ];
    
    for (action, name) in essential_actions {
        if keybinds.get_keys_for_action(action).is_empty() {
            errors.push(format!("Essential action '{}' has no keybinds configured", name));
        }
    }
    
    errors
}