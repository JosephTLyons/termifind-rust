use std::env;
use std::path::PathBuf;

mod path_container;
use path_container::PathContainer;

mod settings;
mod utils;

#[allow(dead_code)]
enum ArrowKeys {
    Up,
    Down,
    Left,
    Right,
}

fn event_loop() {
    let settings = settings::get_settings_from_file();
    let current_directory: PathBuf = env::current_dir().expect("Oops");
    let path_container: PathContainer =
        PathContainer::new(current_directory, settings.path_container_settings);
    path_container.print_path();

    // loop {

    // }
}

fn main() {
    event_loop();

    // Return string to console here somehow
    // String::from("HELLO");
}
