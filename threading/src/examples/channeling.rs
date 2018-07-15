use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::result::Result;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let val = String::from("Message from spawned");
        tx.send(val).unwrap();
    });

    println!("Start");
    let received = rx.recv().unwrap();
    println!("Received {}", received)
}