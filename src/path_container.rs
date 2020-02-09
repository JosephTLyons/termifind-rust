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
        let mut parent_path_result: Option<&Path>;
        let mut counter: usize = 0;

        loop {
            directory_container_vec_deque
                .push_front(DirectoryContainer::new(parent_path.to_path_buf()));

            if counter > 0 {
                let selected_directory_path =
                    directory_container_vec_deque[1].path_to_directory.clone();

                for directory_item in &mut directory_container_vec_deque[0].directory_item_vec {
                    if directory_item.directory_entry.path() == selected_directory_path {
                        directory_item.item_state = ItemState::DirectoryInPath;
                    }
                }
            }

            parent_path_result = parent_path.parent();

            if parent_path_result.is_none() {
                break;
            }

            parent_path = parent_path_result.unwrap();
            counter += 1;
        }

        directory_container_vec_deque[counter].directory_item_vec[0].item_state =
            ItemState::Selected;

        PathContainer {
            current_path: path,
            directory_container_vec_deque,
        }
    }

    pub fn print_path(&self) {
        for directory_container in &self.directory_container_vec_deque {
            // let path_to_directory = &directory_container.path_to_directory;
            // let length_of_current_path: usize = path_to_directory.to_string_lossy().chars().count();
            // let line: String = make_repeated_char_string('=', length_of_current_path);

            // println!("{}", line);
            // println!("{}", path_to_directory.to_string_lossy());
            // println!("{}", line);

            // println!();
            directory_container.print_directory_container();
            println!();
        }
    }
}
