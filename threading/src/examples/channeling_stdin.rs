extern crate console;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug)]
enum Command {
    Up,
    Down,
    Left,
    Right,
    Exit
}

pub fn run() {
    let sleep_duration = Duration::from_millis(50);
    let (tx, rx) = mpsc::channel();
    let term = console::Term::stdout();

    thread::spawn(move || {
        loop {
            match term.read_key() {
                Ok(console::Key::Char('w')) => {
                    tx.send(Command::Up).unwrap();
                },
                Ok(console::Key::Char('d')) => {
                    tx.send(Command::Right).unwrap();
                },
                Ok(console::Key::Char('s')) => {
                    tx.send(Command::Down).unwrap();
                },
                Ok(console::Key::Char('a')) => {
                    tx.send(Command::Left).unwrap();
                },
                Ok(console::Key::Char('\u{1b}')) => {
                    tx.send(Command::Exit).unwrap();
                }
                Ok(other) => {
                },
                Err(error) => {
                },
            }

            thread::sleep(sleep_duration);
        }
    });

    println!("Start");
    loop {
        match rx.try_recv() {
            Ok(Command::Exit) => {
                println!("Exit requested");
                break;
            },
            Ok(command) => {
                println!("Command {:?}", command);
            }
            Err(_) => {},
        }

        thread::sleep(sleep_duration);
    }
    println!("Done");
}