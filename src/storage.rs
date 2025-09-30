// Handles reading from and writing to the password JSON file.

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::collections::HashMap;


use crate::models::PasswordEntry;


const FILE_PATH: &str = "passwords.json"; // File used to persist passwords.

// Loads password data from the JSON file into a HashMap.
pub fn load_passwords() -> HashMap<String, PasswordEntry> {

    let file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH);

        let mut file = match file_result {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[ERROR] Failed to open file for reading: {}", e);
            return HashMap::new();
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("[ERROR] Failed to read file: {}", e);
        return HashMap::new();
    }

    if contents.trim().is_empty() {
        return HashMap::new();
    }

    serde_json::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("[ERROR] Failed to parse JSON: {}", e);
        HashMap::new()
    })
}

// Saves the password HashMap into the JSON file.
pub fn save_passwords(data: &HashMap<String, PasswordEntry>) {

    let contents = serde_json::to_string_pretty(data).expect("Failed to serialize data");

    let file_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE_PATH);

    match file_result {
        Ok(mut file) => {
            if let Err(e) = file.write_all(contents.as_bytes()) {
                eprintln!("[ERROR] Failed to write: {}", e);
            } else {
                println!("[SUCCESS] Passwords saved.");
            }
        }
        Err(e) => {
            eprintln!("[CRITICAL] Failed to open file: {:?}", e);
        }
    }
}