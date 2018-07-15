use std::thread;

#[derive(Debug)]
struct Container {
    value: i32,
}

pub fn run() {
    let container = Container { value: 2 };

    /*
        The move keyword is mandatory for this to work.
        The reason is that 'container' can be cleaned up after the run() function,
        but there is no gaurantee that the spawned thread will still be alive,
        and will us the burrowed container
    */
    let handle = thread::spawn(move || {
        println!("Spawned Thread: {:?}", container);
    });

    handle.join().unwrap();
}