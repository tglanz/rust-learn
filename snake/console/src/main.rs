extern crate console;
extern crate engine;

use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::mpsc;

use std::vec::Vec;
use std::io::{self,Read};

mod rendering;
mod levels;

use engine::directions;
use engine::snake;

enum Input {
    Direction(directions::Direction),
    Exit,
}

fn main() {

    let update_delta_time = Duration::from_millis(80);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {

        let term = console::Term::stdout();

        loop {
            match term.read_key() {
                Ok(console::Key::Char('w')) => {
                    tx.send(Input::Direction(directions::Direction::Up)).unwrap();
                },
                Ok(console::Key::Char('d')) => {
                    tx.send(Input::Direction(directions::Direction::Right)).unwrap();
                },
                Ok(console::Key::Char('s')) => {
                    tx.send(Input::Direction(directions::Direction::Down)).unwrap();
                },
                Ok(console::Key::Char('a')) => {
                    tx.send(Input::Direction(directions::Direction::Left)).unwrap();
                },
                Ok(console::Key::Char('\u{1b}')) => {
                    tx.send(Input::Exit).unwrap();
                }
                Ok(_) => {
                },
                Err(_) => {
                },
            }

            thread::sleep(
                Duration::from_millis(20)
            );
        }
    });


    let mut last_update_time = SystemTime::now();    
    let level = levels::gen_level(&20, &60);
    let mut game_state = engine::game_state::GameState::new(&level);
    let mut current_direction = directions::Direction::Right;

    let term = console::Term::stdout();

    while *game_state.get_is_alive() {
        if last_update_time.elapsed().unwrap() > update_delta_time {
            last_update_time = SystemTime::now();

            match rx.try_recv() {
                Ok(Input::Direction(direction)) => {
                    if !directions::Direction::are_oposites(current_direction, direction) {
                        current_direction = direction;
                    }
                },
                Ok(Input::Exit) => {
                    break;
                }
                _ => {},
            }

            game_state = engine::game_state::GameState::update(&game_state, current_direction);
            rendering::render(&game_state);
        }
    }

    println!("\nOh no! youre dead..\nYour score is {}\n", snake::count(game_state.get_snake()));
}
