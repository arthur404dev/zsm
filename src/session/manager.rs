use zellij_tile::prelude::{SessionInfo, kill_sessions, switch_session};
use crate::session::types::SessionAction;

/// Manages session operations and state
#[derive(Debug, Default)]
pub struct SessionManager {
    /// Currently known sessions from Zellij
    sessions: Vec<SessionInfo>,
    /// Session name pending deletion confirmation
    pending_deletion: Option<String>,
}

impl SessionManager {
    /// Update the session list with new session information
    pub fn update_sessions(&mut self, sessions: Vec<SessionInfo>) {
        self.sessions = sessions;
    }

    /// Get all sessions
    pub fn sessions(&self) -> &[SessionInfo] {
        &self.sessions
    }

    /// Execute a session action
    pub fn execute_action(&mut self, action: SessionAction) {
        match action {
            SessionAction::Switch(name) => {
                switch_session(Some(&name));
            }
            SessionAction::Kill(name) => {
                kill_sessions(&[&name]);
            }
        }
    }

    /// Start session deletion confirmation
    pub fn start_deletion(&mut self, session_name: String) {
        self.pending_deletion = Some(session_name);
    }

    /// Confirm session deletion
    pub fn confirm_deletion(&mut self) {
        if let Some(session_name) = self.pending_deletion.take() {
            self.execute_action(SessionAction::Kill(session_name));
        }
    }

    /// Cancel session deletion
    pub fn cancel_deletion(&mut self) {
        self.pending_deletion = None;
    }

    /// Get session pending deletion
    pub fn pending_deletion(&self) -> Option<&str> {
        self.pending_deletion.as_deref()
    }

    /// Check if a session exists for a given directory session name
    /// Returns the existing session name if found (could be base name or incremented version)
    pub fn find_existing_session_for_directory(&self, base_name: &str, separator: &str) -> Option<String> {
        // First check for exact match
        if self.sessions.iter().any(|s| s.name == base_name) {
            return Some(base_name.to_string());
        }
        
        // Then check for incremented versions
        for session in &self.sessions {
            if self.is_incremented_session(&session.name, base_name, separator) {
                return Some(session.name.clone());
            }
        }
        
        None
    }
    
    /// Check if session name is an incremented version of base name
    fn is_incremented_session(&self, session_name: &str, base_name: &str, separator: &str) -> bool {
        if session_name.len() <= base_name.len() || !session_name.starts_with(base_name) {
            return false;
        }
        
        let remainder = &session_name[base_name.len()..];
        if !remainder.starts_with(separator) {
            return false;
        }
        
        let number_part = &remainder[separator.len()..];
        number_part.parse::<u32>().is_ok() && !number_part.is_empty()
    }

    /// Generate incremented session name for a base name
    pub fn generate_incremented_name(&self, base_name: &str, separator: &str) -> String {
        let base_exists = self.sessions.iter().any(|s| s.name == base_name);
        
        if !base_exists {
            return base_name.to_string();
        }
        
        // Find the next available increment
        for counter in 2..=1000 {
            let candidate = format!("{}{}{}", base_name, separator, counter);
            let exists = self.sessions.iter().any(|s| s.name == candidate);
            
            if !exists {
                return candidate;
            }
        }
        
        // Fallback with UUID if too many increments
        format!("{}{}{}", base_name, separator, uuid::Uuid::new_v4().to_string()[..8].to_string())
    }
}