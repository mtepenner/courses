# Rule 110 Cellular Automaton

- **Author:** Matthew Penner

## Description

This program is a simple command-line implementation of the [Rule 110 elementary cellular automaton](https://en.wikipedia.org/wiki/Rule_110) written in Rust.

A cellular automaton consists of a grid of cells, each in a finite number of states. For each cell, a new state is computed based on the states of its neighbors. This program simulates an 8-cell wide, one-dimensional automaton where the state of a cell in the next generation is determined by its own state and the state of its left and right neighbors. It uses "wrap-around" boundary conditions, meaning the grid is treated as a circle.

The program prints 10 generations of the automaton to the console, starting from an initial configuration.

## How to Build and Run

You must have the Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).

1.  **Clone or download the project and navigate into the directory.**

2.  **Format and check the code (optional but recommended):**
    ```sh
    cargo fmt
    cargo clippy
    ```

3.  **Run the tests:**
    ```sh
    cargo test
    ```

4.  **Build the project:**
    For a development build:
    ```sh
    cargo build
    ```
    For an optimized release build:
    ```sh
    cargo build --release
    ```

5.  **Run the program:**
    To run with the default starting row (`*.*..*..`):
    ```sh
    cargo run
    ```
    To run with a custom 8-character starting row:
    ```sh
    # Example with a "glider" pattern
    cargo run -- "..*....."
    ```

## Issues and Comments

The implementation was straightforward. I chose to use a fixed-size array `[bool; 8]` to represent a row. This is efficient as the data is stored on the stack and the size is known at compile time.

Error handling for command-line arguments is included. The program will exit gracefully with a descriptive error message if the user provides an input string that is not 8 characters long or contains characters other than `*` and `.`.

The code is fully documented with Rustdoc comments and includes a comprehensive suite of unit tests to verify the correctness of the core logic.
