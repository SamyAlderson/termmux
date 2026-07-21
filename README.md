# termmux: A Rust Terminal Multiplexer

## What is it?

termmux is a terminal multiplexer written in Rust. It allows you to split your terminal into multiple windows, resize them, and manage multiple sessions.

## Features

*   Terminal splitting
*   Window resizing
*   Session management

## Dependencies

*   crossbeam
*   termion

## Usage

To use termmux, simply compile and run the binary:

```bash
cargo run
```

## Build from Source

To build termmux from source, clone the repository and run:

```bash
cargo build
```

## Project Structure

The project is structured as follows:

*   `Cargo.toml`: Rust project configuration
*   `src/main.rs`: Main entry point
*   `src/mux.rs`: Terminal multiplexer logic
*   `src/utils.rs`: Utility functions
*   `tests/main.rs`: Unit tests for main entry point
*   `tests/mux.rs`: Unit tests for terminal multiplexer
*   `Cargo.lock`: Dependency lock file

## License

termmux is licensed under the MIT License.

## Architecture

termmux uses the `crossbeam` library for concurrency and the `termion` library for terminal manipulation. The terminal multiplexer logic is implemented in `src/mux.rs`, while utility functions are implemented in `src/utils.rs`. The main entry point is in `src/main.rs`.

## Code Standards

termmux follows idiomatic Rust conventions and best practices. The code is well-structured, readable, and maintainable. Clear comments explain non-obvious decisions, and proper error handling is implemented throughout the codebase.