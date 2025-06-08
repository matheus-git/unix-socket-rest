# Unix Socket REST

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

A small Rust project that sets up a simple client-server communication using Unix Domain Sockets (UDS). It lets you send and receive data about people (like name and age) using a lightweight, REST-like message format.

## Screenshot

![screenshot](assets/screenshot.png)

## Features

- In-memory data storage for Person entries (name & age)
- Request types: GET, POST, DELETE
- MessagePack-based serialization via rmp-serde
- Simple CLI interface on the client

## Usage

###  Start server:
    cargo run --bin server
    
### Start client:
    cargo run --bin client

## 📝 License

This project is open-source under the MIT License.
