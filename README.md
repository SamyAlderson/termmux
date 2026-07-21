# termmux
Terminal multiplexer for the modern terminal experience

termmux is a terminal multiplexer written in Rust. It allows you to split your terminal into multiple windows, manage sessions, and resize windows on the fly. I built this because I got tired of using the built-in `tmux` and wanted something with a simpler API.

## Installation

To install termmux, add the following to your `Cargo.toml`:

```toml
[dependencies]
termmux = "0.1.0"
```

Then, run the following command in your terminal:

```bash
cargo install termmux
```

## Usage

To start termmux, simply run the following command:

```bash
termmux
```

This will open a new terminal window with a termmux prompt. You can then split windows, create new sessions, and resize windows using the built-in termmux commands.

## Building from Source

To build termmux from source, simply clone this repository and run the following command:

```bash
cargo build
```

This will build the termmux executable.

## Running Tests

To run the tests for termmux, simply run the following command:

```bash
cargo test
```

This will run all the tests in the `tests` directory.

## Project Structure

Here is a high-level overview of the project structure:

* `src/main.rs`: The main entry point for termmux.
* `src/parser.rs`: Handles parsing of user input.
* `src/window.rs`: Handles window management.
* `src/session.rs`: Handles session management.
* `tests`: Contains all the test cases for termmux.

## License

Copyright (c) 2026 SamyAlderson

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.