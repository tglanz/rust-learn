use std::thread;

use utils;

pub fn run() {
    let first_handle = thread::spawn(|| {
        utils::loop_and_print(10, |i| format!("thread({}): i({})", 1, i));
    });

    let second_handle = thread::spawn(|| {
        utils::loop_and_print(10, |i| format!("thread({}): i({})", 2, i));
    });

    let third_handle = thread::spawn(|| {
        utils::loop_and_print(10, |i| format!("thread({}): i({})", 3, i));
    });

    // Block the current thread until thread(1) is finished
    first_handle.join().unwrap();

    // Block the current thread until thread(2) is finished
    second_handle.join().unwrap();

    // Block the current thread until thread(3) is finished
    third_handle.join().unwrap();
}