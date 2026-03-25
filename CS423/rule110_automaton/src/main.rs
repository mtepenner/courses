//! # Rule 110 Elementary Cellular Automaton
//!
//! This program simulates and renders the Rule 110 cellular automaton.
//! It starts with an initial row of 8 cells and generates subsequent rows
//! based on the Rule 110 ruleset.
//!
//! The program can be run with an optional command-line argument to specify
//! the starting row. If no argument is provided, it uses a default pattern.
//! Gemini 2.5 Pro was used to write this code

use std::env;
use std::process;

/// The width of the cellular automaton grid.
const WIDTH: usize = 8;
/// The number of generations to simulate and print.
const NUM_GENERATIONS: u32 = 10;
/// The default starting pattern if none is provided via command-line arguments.
const DEFAULT_START: &str = "*.*..*..";

/// The main entry point of the program.
///
/// It parses command-line arguments for a starting row, then simulates
/// and prints the specified number of generations for the automaton.
fn main() {
    // Use the first command-line argument if present, otherwise use the default.
    let start_str = env::args().nth(1).unwrap_or_else(|| DEFAULT_START.to_string());

    // Parse the string into our internal boolean array representation.
    let mut current_row = match parse_row(&start_str) {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Error parsing starting row: {}", e);
            process::exit(1);
        }
    };

    println!("Starting with Rule 110 for {} generations:", NUM_GENERATIONS);

    // Main simulation loop.
    for _ in 0..NUM_GENERATIONS {
        println!("{}", format_row(&current_row));
        current_row = compute_next_generation(&current_row);
    }
}

/// Computes the next generation of the cellular automaton.
///
/// It iterates through each cell of the `current_row`, determines its
/// 3-cell neighborhood (with wrap-around logic), and applies `rule110`
/// to determine the cell's state in the next generation.
///
/// # Arguments
/// * `current_row` - A reference to the boolean array representing the current state.
///
/// # Returns
/// A new boolean array representing the next state.
fn compute_next_generation(current_row: &[bool; WIDTH]) -> [bool; WIDTH] {
    let mut next_row = [false; WIDTH];
    for i in 0..WIDTH {
        // Determine indices of the left, center, and right neighbors with wrap-around.
        // Adding WIDTH before subtracting 1 prevents underflow when i is 0.
        let left_index = (i + WIDTH - 1) % WIDTH;
        let center_index = i;
        let right_index = (i + 1) % WIDTH;

        let neighborhood = [
            current_row[left_index],
            current_row[center_index],
            current_row[right_index],
        ];

        next_row[i] = rule110(neighborhood);
    }
    next_row
}

/// Applies Rule 110 to a 3-bit neighborhood to determine the next state.
///
/// Rule 110 is defined by the following mapping (1=true, 0=false):
/// 111->0, 110->1, 101->1, 100->0, 011->1, 010->1, 001->1, 000->0
///
/// # Arguments
/// * `neighborhood` - A 3-element boolean array `[left, center, right]`.
///
/// # Returns
/// The new state (`bool`) for the center cell in the next generation.
fn rule110(neighborhood: [bool; 3]) -> bool {
    match neighborhood {
        [true, true, true] => false,
        [true, true, false] => true,
        [true, false, true] => true,
        [true, false, false] => false,
        [false, true, true] => true,
        [false, true, false] => true,
        [false, false, true] => true,
        [false, false, false] => false,
    }
}

/// Formats a row of booleans into a user-friendly string representation.
///
/// `true` is converted to '*' and `false` is converted to '.'.
///
/// # Arguments
/// * `row` - A reference to the boolean array to format.
///
/// # Returns
/// A `String` representing the row.
fn format_row(row: &[bool; WIDTH]) -> String {
    row.iter().map(|&cell| if cell { '*' } else { '.' }).collect()
}

/// Parses a string representation of a row into a boolean array.
///
/// Accepts strings containing '*' (true) and '.' (false).
///
/// # Arguments
/// * `s` - The string slice to parse.
///
/// # Returns
/// A `Result` containing the parsed boolean array or a static string error message.
fn parse_row(s: &str) -> Result<[bool; WIDTH], &'static str> {
    if s.len() != WIDTH {
        return Err("Input string must be 8 characters long.");
    }

    let mut row = [false; WIDTH];
    for (i, char) in s.chars().enumerate() {
        match char {
            '*' => row[i] = true,
            '.' => row[i] = false,
            _ => return Err("Input string can only contain '*' or '.'."),
        }
    }
    Ok(row)
}

// Unit tests for the automaton logic.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule110_logic() {
        assert_eq!(rule110([true, true, true]), false);
        assert_eq!(rule110([true, true, false]), true);
        assert_eq!(rule110([true, false, true]), true);
        assert_eq!(rule110([true, false, false]), false);
        assert_eq!(rule110([false, true, true]), true);
        assert_eq!(rule110([false, true, false]), true);
        assert_eq!(rule110([false, false, true]), true);
        assert_eq!(rule110([false, false, false]), false);
    }

    #[test]
    fn test_next_generation_from_prompt() {
        let start_row = parse_row("*.*..*..").unwrap();
        let next_row = compute_next_generation(&start_row);
        let expected_row = parse_row("***.**.*").unwrap();
        assert_eq!(next_row, expected_row);
    }
    
    #[test]
    fn test_all_dead_cells_stay_dead() {
        let start_row = parse_row("........").unwrap();
        let next_row = compute_next_generation(&start_row);
        assert_eq!(next_row, start_row);
    }

    #[test]
    fn test_format_row() {
        let row = [true, false, true, false, false, true, false, false];
        assert_eq!(format_row(&row), "*.*..*..");
    }

    #[test]
    fn test_parse_row_ok() {
        let expected = [true, false, true, false, false, true, false, false];
        assert_eq!(parse_row("*.*..*..").unwrap(), expected);
    }

    #[test]
    fn test_parse_row_wrong_length() {
        assert!(parse_row("*.*").is_err());
    }

    #[test]
    fn test_parse_row_invalid_char() {
        assert!(parse_row("*.*..*..A").is_err());
    }
}
