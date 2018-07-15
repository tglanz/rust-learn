extern crate engine;

use std::sync::Mutex;
use std::thread;
use std::io;

mod rendering;
mod levels;

use engine::directions::Direction;


fn main() {

    let level = levels::gen_level(&20, &60);

    let mut game_state = engine::game_state::GameState::new(&level);

    while *game_state.get_is_alive() {
        game_state = engine::game_state::GameState::update(&game_state, Direction::Down);
        rendering::render(&game_state);
    }
}
