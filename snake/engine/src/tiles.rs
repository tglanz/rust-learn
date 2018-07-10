use std::vec::Vec;

#[derive(PartialEq, Clone, Copy)]
pub enum Tile {
    Empty,          // Freely move on
    Occupied,       // Die on impact
    Food,           // Get longer
}

pub type Tiles = Vec<Tile>;