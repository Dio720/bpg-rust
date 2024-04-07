use super::puzzle::Cell;


pub enum Orientation {
    H, // Horizontal 
    V, // Vertical
    U, // Undefined
}

pub enum ShipType {
    Battleship,
    Destroyer,
    Cruiser,
    Submarine,
}


pub struct Ship {
    coordinate: Cell,
    orientation: Orientation,
    ship_type: ShipType,
}
