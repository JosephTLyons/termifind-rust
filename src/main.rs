use std::env;
use std::path::PathBuf;

mod path_container;
use path_container::PathContainer;

mod utils;

#[allow(dead_code)]
enum ArrowKeys {
    Up,
    Down,
    Left,
    Right,
}

fn event_loop() {
    let current_directory: PathBuf = env::current_dir().expect("Oops");
    let path_containter: PathContainer = PathContainer::new(current_directory);
    path_containter.print_path();

    // loop {

    // }
}

fn main() {
    event_loop();

    // Return string to console here somehow
    // String::from("HELLO");
}
