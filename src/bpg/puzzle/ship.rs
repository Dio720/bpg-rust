use core::fmt;
use std::fmt::Formatter;

use crate::Nat;

pub type CoordType = Nat;

pub struct Cell {
    pub(super) row: CoordType,
    pub(super) col: CoordType,
}

impl Cell {
    pub fn new(row: CoordType, col: CoordType) -> Cell {
        Cell { row, col }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.row, self.col)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ShipType {
    Battleship,
    Destroyer,
    Cruiser,
    Submarine,
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    H, // Horizontal
    V, // Vertical
    U, // Undefined
}

pub struct Position {
    pub coord: Cell,
    pub orient: Orientation,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.coord, self.orient)
    }
}

impl Position {
    pub fn new(coord: Cell, orient: Orientation) -> Self {
        Position { coord, orient }
    }
}

pub struct Ship {
    pub type_: ShipType,
    pub pos: Position,
    pub len: Nat,
}

impl Ship {
    pub fn new(type_: ShipType) -> Ship {
        let ship_lens = [4, 3, 2, 1];
        Ship {
            type_,
            pos: Position::new(Cell::new(0, 0), Orientation::U),
            len: ship_lens[type_ as usize],
        }
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ship_codes = ['B', 'D', 'C', 'S'];
        write!(f, "{} {}", ship_codes[self.type_ as usize], self.pos)
    }
}

