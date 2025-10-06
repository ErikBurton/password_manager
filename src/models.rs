// Data structure representing a password entry.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub site: String,
    pub username: String,
    pub password_hash: String,
}

// Implementing methods using 'impl' block
impl PasswordEntry {
    // Display entry summary
    pub fn summary(&self) -> String {
        format!("Site: {}, Username: {}", self.site, self.username)
    }

    // Verify if a username matches
    pub fn is_for_user(&self, username: &str) -> bool {
        self.username == username
    }
}