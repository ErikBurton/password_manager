# Overview

# 🔐 Password Manager CLI (Rust)

As a software engineer, I am always looking to expand my understanding of new languages and improve my ability to write safe, efficient, and maintainable code. This project gave me the opportunity to explore the 🦀 **Rust** programming language and gain hands-on experience with its unique features like ownership, borrowing, and type safety.

To demonstrate my understanding, I built a simple command-line **Password Manager** that allows users to securely store, retrieve, and delete passwords. The program uses a `HashMap` to manage data in memory and saves entries to a JSON file on disk. Passwords are hashed using SHA256 before being saved, providing basic encryption for security.

My primary goal was to strengthen my understanding of core Rust concepts such as file I/O, data structures, and memory safety. Through this project, I was also able to explore modular Rust development and work with external crates for serialization and cryptography.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

- macOS Ventura 13.x
- Visual Studio Code (with Rust Analyzer extension)
- Cargo (Rust package manager)

**Programming Language & Libraries:**
- Rust (Stable)
- [serde](https://crates.io/crates/serde) – for data serialization
- [serde_json](https://crates.io/crates/serde_json) – to save/load JSON data
- [sha2](https://crates.io/crates/sha2) – for password hashing (SHA256)

# Useful Websites

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Serde Docs](https://serde.rs/)
- [Crates.io](https://crates.io/)
- [How to Hash a Password in Rust](https://blog.logrocket.com/hashing-passwords-efficiently-rust/)

# Future Work

- 🗂 Add support for password categories or tags
- 🔑 Encrypt data instead of just hashing (AES or other)
- 🗃️ Implement search and filter functionality for large datasets
- 🖥 Add unit tests and error-handling improvements
- 🌐 Build a simple GUI or web-based frontend in future modules
