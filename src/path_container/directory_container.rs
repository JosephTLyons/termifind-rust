use std::fs::{read_dir, ReadDir};
use std::path::PathBuf;
use std::vec::Vec;

mod directory_item;
pub use directory_item::{DirectoryItem, ItemState};

use crate::utils::{add_padding_to_center_string, make_repeated_char_string};

#[allow(dead_code)]
pub struct DirectoryContainer {
    pub directory_name: String,
    pub minimum_width: usize,
    pub path_to_directory: PathBuf,
    pub directory_item_vec: Vec<DirectoryItem>,
}

impl DirectoryContainer {
    pub fn new(path: PathBuf, selected_directory_option: &Option<PathBuf>) -> Self {
        let mut directory_item_vec: Vec<DirectoryItem> = Vec::new();
        let read_directory_iterator: ReadDir = read_dir(&path).expect("Oops");
        let mut length_of_longest_file_name: usize = 0;

        for file in read_directory_iterator {
            let mut directory_item: DirectoryItem = DirectoryItem::new(file.expect("Oops"));

            let length_of_file_name: usize =
                directory_item.get_printable_file_name().chars().count();

            if length_of_file_name > length_of_longest_file_name {
                length_of_longest_file_name = length_of_file_name
            }

            if let Some(selected_directory) = selected_directory_option {
                if selected_directory == &directory_item.directory_entry.path() {
                    directory_item.item_state = ItemState::DirectoryInPath;
                }
            }

            directory_item_vec.push(directory_item);
        }

        directory_item_vec.sort_by(|a, b| {
            a.get_printable_file_name()
                .partial_cmp(&b.get_printable_file_name()).expect("Oops")
        });

        let directory_name: String = match path.file_name() {
            Some(d_name) => d_name.to_string_lossy().to_string(),
            None => String::from("root"),
        };

        let length_of_current_directory_name = directory_name.chars().count();

        let minimum_width = if length_of_current_directory_name > length_of_longest_file_name {
            length_of_current_directory_name
        } else {
            length_of_longest_file_name
        };

        DirectoryContainer {
            directory_name,
            minimum_width,
            path_to_directory: path,
            directory_item_vec,
        }
    }

    pub fn print_directory_container(&self) {
        println!(
            " {} ",
            make_repeated_char_string('-', self.minimum_width + 2)
        );
        println!(
            "|{}|",
            add_padding_to_center_string(&self.directory_name, self.minimum_width + 2),
        );
        println!(
            "|{}|",
            make_repeated_char_string('=', self.minimum_width + 2)
        );

        for directory_item in &self.directory_item_vec {
            print!("| ");
            directory_item.print_colored_file_name_based_on_state();

            let length_of_current_file_name: usize =
                directory_item.get_printable_file_name().chars().count();
            let difference: usize = self.minimum_width - length_of_current_file_name;
            let space_padding: String = make_repeated_char_string(' ', difference);

            print!("{}", space_padding);
            println!(" |");
        }

        let horizontal_line = make_repeated_char_string('-', self.minimum_width + 2);

        println!(" {} ", horizontal_line);
    }
}
