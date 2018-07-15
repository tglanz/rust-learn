use std::thread;

use utils;

pub fn run() {
    thread::spawn(|| {
        utils::loop_and_print(10, |i| format!("thread({}): i({})", 1, i));
    });

    thread::spawn(|| {
        utils::loop_and_print(10, |i| format!("thread({}): i({})", 2, i));
    });

    thread::spawn(|| {
        utils::loop_and_print(10, |i| format!("thread({}): i({})", 3, i));
    });
}