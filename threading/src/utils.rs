use std::string::String;

pub fn loop_and_print<F>(count: usize, message_provider: F)
    where F: Fn(usize) -> String {

    for i in 0..count {
        let message = message_provider(i);
        println!("{}", message);
    }
}