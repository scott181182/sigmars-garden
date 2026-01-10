#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::str::FromStr;

use sigmars_lib::{Board, solve_board};

fn main() {
    let filename = std::env::args().nth(1);
    if let Some(filename) = filename {
        let filedata = std::fs::read_to_string(filename).expect("Failed to read file");
        let board = Board::<6>::from_str(&filedata).expect("Failed to parse board");
        match solve_board(&board) {
            Some(solution) => {
                println!("Solution found with {} moves:", solution.len());
                for match_set in solution {
                    let msg = match_set
                        .iter()
                        .map(|c| format!("{:?}@({},{})", board.get_tile(c), c.row, c.col))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("{}", msg);
                }
            }
            None => eprintln!("No solution found"),
        }
    } else {
        eprintln!("Usage: sigmars_cli <board_file>");
    }
}
