use super::ship::{Cell, CoordType, Orientation, Ship};

#[derive(PartialEq)]
enum CellState {
    Water,
    Ship,
    Margin,
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
            Direction::North     => (cell.row + 1, cell.col),
            Direction::NorthWest => (cell.row + 1, cell.col - 1),
            Direction::NorthEast => (cell.row + 1, cell.col + 1),
            Direction::South     => (cell.row - 1, cell.col),
            Direction::SouthWest => (cell.row - 1, cell.col - 1),
            Direction::SouthEast => (cell.row - 1, cell.col + 1),
            Direction::East      => (cell.row, cell.col + 1),
            Direction::West      => (cell.row, cell.col - 1),
        };
        Cell::new(cell.row, cell.col)
    }
}

type BoardType = Vec<Vec<CellState>>;

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
                Orientation::H => Cell::new(ship.pos.coord.row, ship.pos.coord.col + index),
                _ => Cell::new(ship.pos.coord.row + index, ship.pos.coord.col),
            };
            match f(cell) {
                None => continue,
                _ => break,
            }
        }
    }

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
