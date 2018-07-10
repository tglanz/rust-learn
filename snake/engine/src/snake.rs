use std::collections::VecDeque;

use terrain::TerrainIndex;

pub type Snake = VecDeque<TerrainIndex>;

pub fn add_part(snake: &mut Snake, terrain_index: TerrainIndex) {
    snake.push_front(terrain_index);
}

pub fn is_occupy_terrain_index(snake: &Snake, terrain_index: &TerrainIndex) -> bool {
    return snake.iter().any( |&x| x == *terrain_index )
}