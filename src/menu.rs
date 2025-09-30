// User interface logic and actions for the CLI password manager.

use std::collections::HashMap;
use std::io;

use sha2::{Sha256, Digest};

use crate::models::PasswordEntry;
use crate::storage::{load_passwords, save_passwords};

// Entry function that starts the menu loop.
pub fn start() {
    let mut passwords = load_passwords();

    if !passwords.is_empty() {
        println!("\nPreviously saved entries:");
        list_entries(&passwords);
    }

    loop {
        println!("\nPassword Manager");
        println!("1. Add password");
        println!("2. Retrieve password");
        println!("3. List entries");
        println!("4. Delete entry");
        println!("5. Exit");

        let choice = prompt("Choose an option: ");
        match choice.trim() {
            "1" => add_password(&mut passwords),
            "2" => get_password(&passwords),
            "3" => list_entries(&passwords),
            "4" => delete_entry(&mut passwords),
            "5" => {
                println!("Exiting Password Manager");
                break;
            },
            _ => println!("Invalid option."),
        }
    }
}

// Prompts user and reads a line of input.
fn prompt(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    let _ = io::Write::flush(&mut io::stdout());
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Hashes the password using SHA-256.
fn hash_password(pw: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pw.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Adds a new password entry and saves it.
fn add_password(passwords: &mut HashMap<String, PasswordEntry>) {
    let site = prompt("Site: ");
    let username = prompt("Username: ");
    let password = prompt("Password: ");
    let password_hash = hash_password(&password);

    let entry = PasswordEntry {
        site: site.clone(),
        username,
        password_hash,
    };

    passwords.insert(site, entry);
    save_passwords(passwords);
    println!("Password saved.");
}

// Retrieves and displays a password entry by site name.
fn get_password(passwords: &HashMap<String, PasswordEntry>) {
    let site = prompt("Site: ");
    if let Some(entry) = passwords.get(&site) {
        println!("Username: {}", entry.username);
        println!("(Hashed) Password: {}", entry.password_hash);
    } else {
        println!("No entry found.");
    }
}

// Lists all entries (site and username)
fn list_entries(passwords: &HashMap<String, PasswordEntry>) {
    for (site, entry) in passwords {
        println!("Site: {}, Username: {}", site, entry.username);
    }
}

// Deletes a password entry and saves the updated file.
fn delete_entry(passwords: &mut HashMap<String, PasswordEntry>) {
    let site = prompt("Site to delete: ");
    if passwords.remove(&site).is_some() {
        save_passwords(passwords);
        println!("Entry deleted.");
    } else {
        println!("No entry found.");
    }
}
