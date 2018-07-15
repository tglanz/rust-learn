use std::vec::Vec;

use terrain::{TerrainSize, TerrainIndex, TerrainLocation, location_to_index};

pub struct Level {
    terrain_size: TerrainSize,
    occupied_indices: Vec<TerrainIndex>
}

impl Level {
    pub fn get_terrain_size(&self) -> &TerrainSize {
        &self.terrain_size
    }

    pub fn get_occupied_indices(&self) -> &Vec<TerrainIndex> {
        &self.occupied_indices
    }

    pub fn new(terrain_size: &TerrainSize, occupied_indices: &Vec<TerrainIndex>) -> Level {
        Level {
            terrain_size: *terrain_size,
            occupied_indices: occupied_indices.clone()
        }
    }
}

pub fn create_bound_indices(terrain_size: &TerrainSize) -> Vec<TerrainIndex> {

    let rows = terrain_size.get_rows();
    let columns = terrain_size.get_columns();

    let mut res: Vec<TerrainIndex> = Vec::new();

    // Can make it constructive but ...
    for row in 0..rows {
        for column in 0..columns {
            if row == 0 || row == rows - 1 || column == 0 || column == columns - 1 {
                let terrain_location = TerrainLocation::new(row, column);
                let terrain_index = location_to_index(&terrain_location, terrain_size);
                res.push(terrain_index);
            }
        }
    }

    res
}