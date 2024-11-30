# Rust TCP Proxy

This project is a simple TCP proxy written in Rust using the `tokio` and `tokio-stream` crates. The proxy listens on a specified address and forwards traffic to another address.

## Getting Started

### Prerequisites

- Rust and Cargo: You can install Rust and Cargo from [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/rust-tcp-proxy.git
    cd rust-tcp-proxy

1. Build the project:

    ```rs
    cargo build --verbose
    ```

### Running the Proxy

To run the proxy, use the following commands:

```rs
cargo run
```

By default, the proxy listens on `127.0.0.1:8085` and forwards traffic to `127.0.0.1:8086`. You can change these addresses in the `src/main.rs` file.

### Project Structure

- `src/main.rs`: The entry point of the application. It starts the proxy.
- `src/lib.rs`: Contains the main logic for handling client connections and forwarding traffic.

### Unit Tests

To run the unit tests, use the following command:

```rs
cargo test --verbose
```

### Continuous Integration

This project uses GitHub Actions for continuous integration. The workflow file is located at `.github/workflows/ci.yml`. It runs the tests on ~~every push and pull request to the main branch~~ demand (via `workflow_dispatch`).

### Contributing

Contributions are welcome! Please open an issue or submit a pull request.

### License

This project is licensed under the GPL License. See the `LICENSE` file for details.
