# Password Manager

A simple command-line password manager written in Rust. This project allows users to store, view, and search for password entries securely in a JSON file.

## Features

- Add new password entries
- View all stored password entries
- Search for a specific password entry by service name
- Secure storage of password entries in a JSON file

## Installation

To use this password manager, you need to have Rust installed on your machine. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/tusharpamnani/Password-Manager.git
cd Password-Manager
```

## Usage

To run the password manager, use the following command:

```sh
cargo run
```

### Menu Options

1. **Add a new password entry**: Prompts you to enter the service name, username, and password.
2. **View all password entries**: Displays all stored password entries.
3. **Search for a password entry**: Allows you to search for a password entry by service name.
4. **Exit**: Exits the application.

## Example

```plaintext
  _____               __      __         _ _   
  |  __ \              \ \    / /        | | |  
  | |__) |_ _ ___ ___   \ \  / /_ _ _   _| | |_ 
  |  ___/ _` / __/ __|   \ \/ / _` | | | | | __|
  | |  | (_| \__ \__ \    \  / (_| | |_| | | |_ 
  |_|   \__,_|___/___/     \/ \__,_|\__,_|_|\__|


Password Manager Menu:
1. Add a new password entry
2. View all password entries
3. Search for a password entry
4. Exit

Enter your choice: 1
Service: GitHub
Username: user123
Password: mypassword

Added new password entry

Password Manager Menu:
1. Add a new password entry
2. View all password entries
3. Search for a password entry
4. Exit

Enter your choice: 2

Service: GitHub
- Username: user123
- Password: mypassword
```

## File Structure

- `src/`
  - `main.rs`: The main entry point for the application.
  - `pass_entry.rs`: Contains the `ServiceInfo` struct and associated functions.
- `passwords.json`: The file where password entries are stored.

## Contribution

If you would like to contribute to this project, feel free to open a pull request or file an issue on GitHub.

## License

This project is licensed under the MIT License.
