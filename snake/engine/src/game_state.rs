use rand;

use directions::Direction;

use vec2::Vec2;

use snake;
use snake::{Snake};

use tiles::{Tile, Tiles};

use terrain;
use terrain::{TerrainLocation, TerrainSize, TerrainIndex};

use level::Level;

pub struct GameState {
    terrain_size: TerrainSize,
    snake: Snake,
    tiles: Tiles,
    is_alive: bool
}

impl GameState {
    pub fn get_is_alive(&self) -> &bool {
        &self.is_alive
    }

    pub fn get_tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_terrain_size(&self) -> &TerrainSize {
        &self.terrain_size
    }

    pub fn new(level: &Level) -> GameState {

        let terrain_size = level.get_terrain_size();
        let rows = terrain_size.get_rows();
        let columns = terrain_size.get_columns();

        let snake_terrain_location = TerrainLocation::new(rows / 2, columns / 2);
        let snake_terrain_index = terrain::location_to_index(&snake_terrain_location, &terrain_size);

        let mut snake = Snake::new();
        snake.push_front(snake_terrain_index);
        snake.push_back(snake_terrain_index - 1);

        let flat_size = TerrainSize::flat_size(&terrain_size);
        let mut tiles = vec![Tile::Empty; flat_size as usize];
        for occupied_index in level.get_occupied_indices() {
            tiles[*occupied_index as usize] = Tile::Occupied;
        }
        let food_index = find_random_food_index(&snake, &tiles);
        tiles[food_index as usize] = Tile::Food;

        GameState {
            terrain_size: *terrain_size,
            snake,
            tiles,
            is_alive: true
        }
    }

    pub fn update(game_state: &GameState, direction: Direction) -> GameState {
        let mut snake = game_state.snake.clone();
        let mut tiles = game_state.tiles.clone();
        let mut is_alive = game_state.is_alive;
        let terrain_size = game_state.terrain_size;

        let snake_head_index = snake.front().expect("Headless snakes is not good").clone();
        let snake_head_location = terrain::index_to_location(&snake_head_index, &terrain_size);

        let direction_vector = input_key_to_direction_vector(direction);
        let new_head_location = TerrainLocation::add(&snake_head_location, &direction_vector);
        let new_head_index = terrain::location_to_index(&new_head_location, &terrain_size);
        snake.push_front(new_head_index);

        match tiles[new_head_index as usize] {
            Tile::Food => {
                tiles[new_head_index as usize] = Tile::Empty;
                let new_food_index = find_random_food_index(&snake, &tiles);
                tiles[new_food_index as usize] = Tile::Food;
            }
            Tile::Empty => {
                snake.pop_back();
            },
            Tile::Occupied => {
                is_alive = false;
            },
        };

        GameState {
            terrain_size,
            tiles,
            snake,
            is_alive
        }
    }
}

fn find_random_food_index(snake: &Snake, tiles: &Tiles) -> TerrainIndex {

    let mut terrain_index: TerrainIndex;

    loop {
        terrain_index = rand::prelude::random::<i16>();

        if snake::is_occupy_terrain_index(&snake, &terrain_index) {
            continue;
        }

        if match tiles.get(terrain_index as usize) {
            Some(&Tile::Empty) => false,
            _ => true,
        } {
            continue;
        }

        break;
    }

    terrain_index
}

fn input_key_to_direction_vector(direction: Direction) -> Vec2 {
    match direction {
        Direction::Up => Vec2::new(0, -1),
        Direction::Down => Vec2::new(0, 1),
        Direction::Left => Vec2::new(-1, 0),
        Direction::Right => Vec2::new(0, 1),
    }
}