use std::collections::VecDeque;

use terrain::TerrainLocation;
use vec2::Vec2;

pub type Snake = VecDeque<TerrainLocation>;

pub fn advance(snake: &Snake, direction: &Vec2) -> Snake {
    let mut cloned = snake.clone();
    cloned.pop_back();
    return advance_head(&cloned, direction);
}

pub fn advance_to_food(snake: &Snake, direction: &Vec2) -> Snake {
    return advance_head(snake, direction);    
}


fn advance_head(snake: &Snake, direction: &Vec2) -> Snake {
    let new_head_location = match snake.front() {
        Some(head_location) => TerrainLocation::add_vec(
            head_location,
            direction
        ),
        None => panic!("Empty snake is invalid"),
    };

    let mut cloned = snake.clone();
    cloned.push_front(new_head_location);
    cloned
}