// Data structure representing a password entry.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub site: String,
    pub username: String,
    pub password_hash: String,
}