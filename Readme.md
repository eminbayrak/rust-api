# Project Name

A simple RESTful API application built with Rust.

## Description

This project aims to provide CRUD operations for users in a PostgreSQL database. It includes a migration script that creates 100 dummy users when the application is compiled. The passwords are stored hashed in the database. Additional endpoints may be added in the future as needed.

## Dependencies

- actix-web = "4.0"
- argon2 = "0.6.0-pre.0"
- chrono = { version = "0.4", features = ["serde"] }
- dotenv = "0.15.0"
- env_logger = "0.9"
- log = "0.4"
- serde_json = "1.0"
- serde = { version = "1.0", features = ["derive"] }
- sqlx = { version = "0.5", default-features = false, features = ["postgres", "runtime-actix-native-tls", "chrono"] }
- tokio = { version = "1.0", features = ["full"] }

## Installation

1. Clone the repository.
2. Install Rust and Cargo.
3. Run `cargo build` to build the project.
4. Run `cargo run` to start the application.

## Usage

- To add a new user, send a POST request to the `/api/adduser` endpoint with the following form data in the request body:
  {
  "first_name": "Max",
  "last_name": "Doe",
  "username": "max_doe",
  "email": "maxdoe@email.com",
  "password": "p@ssw0rd!"
  }

- GET request to `/api/users`

## Contributing

Contributions are welcome! Please follow the [contribution guidelines](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE).
