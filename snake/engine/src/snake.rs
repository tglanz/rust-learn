use std::collections::VecDeque;

use terrain::TerrainIndex;

pub type Snake = VecDeque<TerrainIndex>;

pub fn add_part(snake: &mut Snake, terrain_index: TerrainIndex) {
    snake.push_front(terrain_index);
}

pub fn is_occupy_terrain_index(snake: &Snake, terrain_index: &TerrainIndex) -> bool {
    snake.iter().any( |&x| x == *terrain_index)
}

pub fn is_head(snake: &Snake, terrain_index: &TerrainIndex) -> bool {
    terrain_index == snake.front().expect("Headless snakes should not exist!")
}

pub fn is_head_and_body_collide(snake: &Snake) -> bool {
    snake.iter().skip(1).any( |&x| x == *snake.front().unwrap() )
}

pub fn count(snake: &Snake) -> usize {
    snake.iter().count()
}