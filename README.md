# Password Manager

This is a simple local password manager built with Rust and SQLite.

## Description

This password manager is a secure and easy-to-use tool to store and manage your passwords. Built with Rust and SQLite, it provides a local and reliable solution for password management. With this application, you no longer need to remember all your passwords, just remember one master password to access your password vault. The application uses strong encryption to ensure that your passwords are always secure.

## Features

- Secure storage: Your passwords are encrypted using a strong encryption algorithm.
  _✅ done_
- Master password: You only need to remember one password to access all your other passwords.
  _❌ not done_
- Local storage: Your passwords are stored locally on your machine, not in the cloud.
  _✅ done_
- Easy to use: The user interface is simple and intuitive.
  _❌ not done_
- Search functionality: Easily find the password you're looking for with a built-in search feature.
  _❌ not done_
- Password generator: Generate strong, random passwords to increase your security.
  _❌ not done_
- Cross-platform: Built with Rust, this password manager can be compiled and run on multiple platforms. _✅ done_

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [SQLite](https://www.sqlite.org/download.html)

### Installing

1. Clone the repository

```bash
git clone https://github.com/lavryniukk/password-manager.git
```

Navigate to the project directory

```bash
cd password-manager
```

Build the project

```bash
cargo build --release
```

Run the application

```bash
./target/release/password_manager
```

### Usage

Provide usage instructions here.

### Contributing

Contribution is unavailable yet.

### License

This project is licensed under the MIT License - see the LICENSE.md file for details
