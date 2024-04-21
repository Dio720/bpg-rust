use crate::bpg::puzzle::board::Board;
use crate::bpg::puzzle::ship::{Orientation, Position, Ship, ShipType, Cell};
use crate::cli::Config;

use crate::Nat;
use std::collections::HashSet;

type PositionsMatrix = Vec<Vec<Position>>;
type PositionsVector = Vec<Position>;
pub struct Generator {
    unique_boards: HashSet<String>,
    possibles_pos_matrix: PositionsMatrix,
}

type Col = Nat;
type Row = Nat;

fn calculate_it_limits(o: Orientation, ship: &Ship, config: &Config) -> (Row, Col) {
    match o {
        Orientation::H => (config.rows, config.cols - ship.len),
        Orientation::V => (config.rows - ship.len, config.cols),
        _ => (config.rows, config.cols),
    }
}

fn generate_positions_vector(ship: &Ship, config: &Config) -> PositionsVector {
    let mut positions: PositionsVector = vec![];
    let orientations = match ship.type_ {
        ShipType::Submarine => vec![Orientation::U],
        _ => vec![Orientation::H, Orientation::V],
    };

    for o in orientations {
        let iteration_limit = calculate_it_limits(o, ship, config);
        for row in 0..iteration_limit.0 {
            for col in 0..iteration_limit.1 {
                positions.push(Position::new(Cell::new(row, col), o));
            }
        }
    }

    positions
}

impl Generator {
    pub fn new() -> Self {
        Self {
            unique_boards: HashSet::new(),
            possibles_pos_matrix: vec![],
        }
    }
    fn generate_positions_matrix(&mut self, config: &Config) {
        let ships = [
            Ship::new(ShipType::Battleship),
            Ship::new(ShipType::Destroyer),
            Ship::new(ShipType::Cruiser),
            Ship::new(ShipType::Submarine),
        ];

        for ship in ships {
            self.possibles_pos_matrix
                .push(generate_positions_vector(&ship, config));
        }
    }
    // fn add_unique_board(&mut self, board: &Board) -> bool {
    //     let board_str = board.to_string_armada();
    //     self.unique_boards.insert(board_str.to_string()) 
    // }
    // pub fn generate(&mut self, config: Config) -> Vec<Board> {
    //     let mut boards_list: Vec<Board> = vec![];
    //     let mut counter = 0;
    //     while counter < config.num_puzzles {
    //         let board =
    //     }
    //     todo!()
    // }
}
