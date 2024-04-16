use crate::bpg::puzzle::{Orientation, Position, Ship, ShipType};
use crate::cli::Config;
use crate::Nat;
use std::collections::HashSet;

type PositionsMatrix = Vec<Vec<Position>>;
type PositionsVector = Vec<Position>;

pub struct Generator {
    unique_boards: HashSet<String>,
    possibles_pos_matrix: PositionsMatrix,
}

type Col = usize;
type Row = usize;

impl Generator {
    fn position_iterator(
        ship_len: Nat,
        orient: Orientation,
        max_row: Nat,
        max_col: Nat,
    ) -> (Row, Col) {
        match orient {
            Orientation::H => (
                max_row.into(),
                max_col.checked_sub(ship_len).unwrap_or(0).into(),
            ),
            Orientation::V => (
                max_row.checked_sub(ship_len).unwrap_or(0).into(),
                max_col.into(),
            ),
            _ => (max_row.into(), max_col.into()),
        }
    }
    fn generate_possible_positions_vtr(ship_len: Nat) -> PositionsVector {
        todo!();

        let mut positions: PositionsVector = vec![];

        positions
    }
}
