use crate::Nat;
use std::ops::Add;

type CoordType = Nat;

/// Represents a cartesian coord in a matrix
pub struct Cell {
    row: CoordType,
    col: CoordType,
}
impl Add for Cell {
    type Output = Cell;
    fn add(self, rhs: Cell) -> Cell {
        Cell {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
#[derive(PartialEq)]
enum CellState {
    Water,
    Ship,
    Margin,
}
pub enum Orientation {
    H, // Horizontal
    V, // Vertical
    U, // Undefined
}
enum Direction {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    East,
    West,
}
impl Direction {
    fn offset(&self, cell: &Cell) -> Cell {
        let coord = match self {
            Direction::North => (cell.row + 1, cell.col),
            Direction::NorthWest => (cell.row + 1, cell.col.checked_sub(1).unwrap_or(0)),
            Direction::NorthEast => (cell.row + 1, cell.col + 1),
            Direction::South => (cell.row.checked_sub(1).unwrap_or(0), cell.col),
            Direction::SouthWest => (
                cell.row.checked_sub(1).unwrap_or(0),
                cell.col.checked_sub(1).unwrap_or(0),
            ),
            Direction::SouthEast => (cell.row.checked_sub(1).unwrap_or(0), cell.col + 1),
            Direction::East => (cell.row, cell.col + 1),
            Direction::West => (cell.row, cell.col.checked_sub(1).unwrap_or(0)),
        }; Cell {
            row: coord.0,
            col: coord.1,
        }
    }
}
pub enum ShipType {
    Battleship,
    Destroyer,
    Cruiser,
    Submarine,
}
pub struct Position {
    coord: Cell,
    orient: Orientation,
}
pub struct Ship {
    pub _type: ShipType,
    pos: Position,
    len: Nat,
}

type BoardType = Vec<Vec<CellState>>;

/// This struct will be the representation of a bpg board
pub struct Board {
    rows: CoordType,
    cols: CoordType,
    board: BoardType,
}

impl Board {
    fn is_within_limits(&self, cell: &Cell) -> bool {
        (0..self.rows).contains(&cell.row) && (0..self.cols).contains(&cell.col)
    }

    fn iterate_ship_cells<F: FnMut(Cell) -> Option<()>>(&self, ship: &Ship, mut f: F) {
        for index in 0..ship.len {
            let cell = match ship.pos.orient {
                Orientation::H => Cell {
                    row: ship.pos.coord.row,
                    col: ship.pos.coord.col + index,
                },
                _ => Cell {
                    row: ship.pos.coord.row + index,
                    col: ship.pos.coord.col,
                },
            };
            match f(cell) {
                None => continue,
                _ => break,
            }
        }
    }

    /// Is used to verify if a ship can be added in the add_ship method
    fn is_overlapping(&self, ship: &Ship) -> bool {
        let mut overlap = false;
        self.iterate_ship_cells(ship, |cell| {
            match self.board[cell.row as usize][cell.col as usize] {
                CellState::Water => Some(()),
                _ => {
                    overlap = true;
                    Some(())
                }
            }
        });
        overlap
    }

    fn is_within_board_limits(&self, cell: &Cell) -> bool {
        !(cell.row > self.rows && cell.col > self.cols)
    }

    fn fill_margins(&mut self, cell: Cell) {
        let directions = [
            Direction::North,
            Direction::NorthWest,
            Direction::NorthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::SouthEast,
            Direction::West,
            Direction::East,
        ];

        for dir in directions {
            let adj_cell = dir.offset(&cell);

            if self.board[adj_cell.row as usize][adj_cell.col as usize] == CellState::Water {
                self.board[adj_cell.row as usize][adj_cell.col as usize] = CellState::Margin;
            }
        }
    }

    pub fn add_ship(&mut self, ship: Ship) -> bool {
        if self.is_overlapping(&ship) {
            false
        } else {
            let mut updates = Vec::new();
            self.iterate_ship_cells(&ship, |cell| {
                updates.push(cell);
                None
            });

            for cell in updates {
                self.board[cell.row as usize][cell.col as usize] = CellState::Ship;
                self.fill_margins(cell);
            }
            true
        }
    }
}
