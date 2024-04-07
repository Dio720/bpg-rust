use std::u8;
use std::ops::Add;

type CoordType = u8;

/// Represents a cartesian coord in a matrix
pub struct Cell {
    row: CoordType,
    col: CoordType,
}

impl Cell {
    /// overload of (+) operator
    pub fn add(&self, rhs: &Cell) -> Self {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

pub enum CellState {
    Water,
    Ship,
    Margin
}

type BoardType = Vec<Vec<CellState>>;

pub struct Puzzle {
    rows: CoordType,
    cols: CoordType,
    pub board: BoardType,
}

impl Puzzle {
    /*TODO: docs*/
    fn is_within_limits(&self, cell: Cell) -> bool {
        (0..self.rows).contains(&cell.row) && (0..self.cols).contains(&cell.col)
    } 

}
