use std::collections::VecDeque;
use std::path::{Path, PathBuf};

mod directory_container;
use directory_container::DirectoryContainer;
use directory_container::ItemState;

use crate::utils::make_repeated_char_string;

#[allow(dead_code)]
pub struct PathContainer {
    current_path: PathBuf,
    directory_container_vec_deque: VecDeque<DirectoryContainer>,
}

impl PathContainer {
    pub fn new(path: PathBuf) -> Self {
        let mut directory_container_vec_deque: VecDeque<DirectoryContainer> = VecDeque::new();
        let mut parent_path: &Path = &path;

        loop {
            let selected_directory_option: Option<PathBuf> =
                if directory_container_vec_deque.is_empty() {
                    None
                } else {
                    match directory_container_vec_deque.front_mut() {
                        Some(first_directory_container) => {
                            Some(first_directory_container.path_to_directory.clone())
                        }
                        None => None,
                    }
                };

            directory_container_vec_deque.push_front(DirectoryContainer::new(
                parent_path.to_path_buf(),
                &selected_directory_option,
            ));

            match parent_path.parent() {
                Some(p_path) => parent_path = p_path,
                None => break,
            }
        }

        if let Some(directory_container) = directory_container_vec_deque.back_mut() {
            if let Some(directory_item) = directory_container.directory_item_vec.first_mut() {
                directory_item.item_state = ItemState::Selected
            }
        }

        PathContainer {
            current_path: path,
            directory_container_vec_deque,
        }
    }

    pub fn print_path(&self) {
        for directory_container in &self.directory_container_vec_deque {
            let path_to_directory = &directory_container.path_to_directory;
            let length_of_current_path: usize = path_to_directory.to_string_lossy().chars().count();
            let line: String = make_repeated_char_string('=', length_of_current_path);

            println!("{}", line);
            println!("{}", path_to_directory.to_string_lossy());
            println!("{}", line);

            println!();
            directory_container.print_directory_container();
            println!();
        }
    }
}
