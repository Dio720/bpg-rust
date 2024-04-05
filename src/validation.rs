use crate::commom::{RunningOpts, MAX_DSIZE, MAX_N_PUZZLES, MIN_DSIZE};
use core::fmt;
use std::{env::args, usize};

// TODO: Add the dimension type
enum ValidationError {
    InvalidDsize,
    InvalidNPuzzles,
    InvalidParam,
}

// To print the errors in a human-readable format
impl fmt::Display for ValidationError {
    fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidDsize => write!(f, "Invalid dimension size"),
            ValidationError::InvalidNPuzzles => write!(f, "Invalid number of puzzles"),
            ValidationError::InvalidParam => write!(f, "Invalid param type")
        }
    }
}

// Function to validate the rows or cols
fn validate_dsize (d_str: &str) -> Result<usize, ValidationError> {
    match d_str.parse::<usize>() {
        Ok(value) => {
            if value > MIN_DSIZE && value < MAX_DSIZE {
                Ok(value)
            } else {
                Err(ValidationError::InvalidDsize)
            }
        },
        Err(_) => Err(ValidationError::InvalidParam)
    }
}

fn validate_npuzzles (npuzzles_str : &str) -> Result<usize, ValidationError> {
    match npuzzles_str.parse::<usize>() {
        Ok(value) => {
            if value > 0 && value < MAX_N_PUZZLES {
                Ok(value)
            } else {
                Err(ValidationError::InvalidNPuzzles)
            }
        },
        Err(_) => Err(ValidationError::InvalidParam)
    }
}

fn parse_argument(arg: &str, validate: fn(&str) -> Result<usize, ValidationError>) -> usize {
    match validate(arg) {
        Ok(value) => value,
        Err(e) => {
            eprint!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_input(args: &Vec<String>) -> RunningOpts {
    let mut num_rows = None;
    let mut num_cols = None;
    let mut num_puzzles = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--rows" => {
                i += 1;
                num_rows = Some(parse_argument(&args[i], validate_dsize));
            },
            "--cols" => {
                i += 1;
                num_cols = Some(parse_argument(&args[i], validate_dsize));
            },
            _ => {
                num_puzzles = Some(parse_argument(&args[i], validate_npuzzles));
            }
        }
        i += 1;
    }
    
    RunningOpts {
        num_rows: num_rows.unwrap(),
        num_cols: num_cols.unwrap(),
        num_puzzles: num_puzzles.unwrap()
    }
}

