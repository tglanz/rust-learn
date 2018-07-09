use vec2::Vec2;

pub type TerrainIndex = i16;

#[derive(Debug, PartialEq, Clone)]
pub struct TerrainLocation {
    row: i16,
    column: i16,
}

#[derive(Debug, PartialEq)]
pub struct TerrainSize {
    rows: i16,
    columns: i16,
}

impl TerrainLocation {
    pub fn new(row: i16, column: i16) -> TerrainLocation {
        TerrainLocation {
            row, column,
        }
    }

    pub fn add_vec(terrain_location: &TerrainLocation, vec: &Vec2) -> TerrainLocation {
        TerrainLocation {
            row: terrain_location.row + *vec.y(),
            column: terrain_location.column + *vec.x()
        }
    }
}

impl TerrainSize {
    pub fn new(rows: i16, columns: i16) -> TerrainSize {
        TerrainSize {
            rows, columns
        }
    }
}

pub fn index_to_location(terrain_index: TerrainIndex, terrain_size: TerrainSize) -> TerrainLocation {
    TerrainLocation::new (
        terrain_index / terrain_size.columns, // column
        terrain_index % terrain_size.columns, // row
    )
}

pub fn location_to_index(terrain_location: TerrainLocation, terrain_size: TerrainSize) -> TerrainIndex {
    (terrain_location.row * terrain_size.columns) + terrain_location.column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_index_to_location() {
        assert_eq!(
            index_to_location(
                2,
                TerrainSize::new(2, 2)
            ),
            TerrainLocation::new(1, 0)
        )
    }

    #[test]
    fn terrain_location_to_index() {
        assert_eq!(
            location_to_index(
                TerrainLocation::new(1, 0),
                TerrainSize::new(2, 2)
            ),
            2
        )
    }
}
