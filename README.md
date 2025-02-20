# Screenshot Taker & Saver 

A simple Rust program that listens for the "PrintScreen" key and captures screenshots when pressed. The screenshots are saved with timestamps in a specified directory.

## Features

- Captures screenshots of all connected screens.
- Saves each screenshot with a timestamp in the specified directory.
- Allows for custom directory specification via command-line arguments.
- Default directory is `screens`.

## Requirements

- Rust programming language (use [rustup](https://rustup.rs/) to install Rust).
- External dependencies:
  - `chrono` - for working with dates and times.
  - `rdev` - for capturing keyboard events.
  - `screenshots` - for capturing the screen.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/vihrom/screenshot-saver.git
    ```

2. Navigate to the project directory:

    ```bash
    cd screenshot-saver
    ```

3. Build the project using Cargo:

    ```bash
    cargo build --release
    ```

## Usage

### Run the program:

After building the project, you can run the compiled executable from the `target/release` directory.


You can also run the program without building it by using the following command:

```bash
cargo run
```

Example usage with a custom directory:
    
```bash
cargo run -- "custom_screenshots"
```

## This project is licensed under the MIT License - see the LICENSE file for details.

