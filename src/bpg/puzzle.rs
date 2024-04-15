use crate::Nat;

type CoordType = Nat;

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
enum CellState {
    Water,
    Ship,
    Margin,
}

enum Orientation {
    H, // Horizontal
    V, // Vertical
    U, // Undefined
}
enum ShipType {
    Battleship,
    Destroyer,
    Cruiser,
    Submarine,
}
struct Position {
    coord: Cell,
    orient: Orientation,
}
pub struct Ship {
    ship_t: ShipType,
    pos: Position,
    len: Nat,
}

type BoardType = Vec<Vec<CellState>>;

/// This struct will be the representation of a bpg board
pub struct Board {
    rows: CoordType,
    cols: CoordType,
    pub board: BoardType,
}

impl Board {
    /*TODO: docs*/
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
                Some(_) => continue,
                _ => break,
            }
        }
    }

    /*TODO: docs  */
    /// Is used to verify if a ship can be added in the add_ship method
    fn is_overlapping(&self, ship: &Ship) -> bool {
        let mut overlap = false;
        self.iterate_ship_cells(ship, |cell| {
            match self.board[cell.row as usize][cell.col as usize] {
                CellState::Water => Some(()),
                _ => {
                    overlap = true;
                    None
                }
            }
        });
        overlap
    }
}
