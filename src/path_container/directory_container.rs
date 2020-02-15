use std::fs::{read_dir, ReadDir};
use std::path::PathBuf;
use std::vec::Vec;

mod directory_item;
pub use directory_item::{DirectoryItem, ItemState, ItemType, NameTruncationSettings};

use crate::utils::string::{add_padding_to_center_string, make_repeated_char_string};

#[allow(dead_code)]
enum LowLevelTruncationOptions {
    None,
    Constant,
    Level,
}

#[allow(dead_code)]
enum AutomaticTruncationOptions {
    Statistical,                       // Uses ByFileNameLength
    FitAllDirectoryContainersInOneRow, // Uses Constant
}

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

        for file in read_directory_iterator {
            let mut directory_item: DirectoryItem = DirectoryItem::new(file.expect("Oops"));

            if let Some(selected_directory) = selected_directory_option {
                if selected_directory == &directory_item.directory_entry.path() {
                    directory_item.item_state = ItemState::DirectoryInPath;
                }
            }

            directory_item_vec.push(directory_item);
        }

        directory_item_vec.sort_by(|a, b| {
            a.get_file_name(false)
                .partial_cmp(&b.get_file_name(false))
                .expect("Oops")
        });

        let directory_name: String = match path.file_name() {
            Some(d_name) => d_name.to_string_lossy().to_string(),
            None => path.to_string_lossy().to_string(),
        };

        let mut directory_container = DirectoryContainer {
            directory_name,
            minimum_width: 0,
            path_to_directory: path,
            directory_item_vec,
        };

        directory_container.set_minimum_width();

        directory_container
    }

    fn set_minimum_width(&mut self) {
        let mut length_of_longest_file_name: usize = 0;

        for directory_item in &self.directory_item_vec {
            let length_of_file_name: usize = directory_item.get_file_name_length(true);

            if length_of_file_name > length_of_longest_file_name {
                length_of_longest_file_name = length_of_file_name
            }
        }

        let length_of_current_directory_name = self.directory_name.chars().count();

        self.minimum_width = if length_of_current_directory_name > length_of_longest_file_name {
            length_of_current_directory_name
        } else {
            length_of_longest_file_name
        };
    }

    fn get_name_truncation_settings(&self,
        low_level_truncation_options: LowLevelTruncationOptions,
    ) -> Option<NameTruncationSettings> {
        match low_level_truncation_options {
            LowLevelTruncationOptions::None => None,
            LowLevelTruncationOptions::Constant => None,
            LowLevelTruncationOptions::Level => {
                Some(NameTruncationSettings {
                    name_length_after_truncation:
                        self.get_truncation_value_by_level(0)
                            .expect("Oops"),
                    should_include_appended_text_in_length: false,
                })
            }
        }
    }

    fn get_truncation_value_by_level(
        &self,
        mut level: usize,
    ) -> Option<usize> {
        let file_name_length_and_position_vec =
            self.get_file_name_lengths_and_positions_vec();
        let last_element_position = file_name_length_and_position_vec.len() - 1;

        level += 1;

        if level < last_element_position {
            return Some(file_name_length_and_position_vec[last_element_position - level].0);
        }

        None
    }

    fn get_file_name_lengths_and_positions_vec(
        &self,
    ) -> Vec<(usize, usize)> {
        let mut file_name_lengths_and_positions_vec: Vec<(usize, usize)> = Vec::new();

        for (index, directory_item) in self.directory_item_vec.iter().enumerate() {
            let tuple = (directory_item.get_file_name_length(false), index);

            match file_name_lengths_and_positions_vec.binary_search(&tuple) {
                Ok(_) => {}
                Err(position) => file_name_lengths_and_positions_vec.insert(position, tuple),
            }
        }

        file_name_lengths_and_positions_vec
    }

    pub fn print_directory_container_by_row(&self, row_number: usize) {
        if row_number < self.get_total_height_of_directory_container() - 1 {
            match row_number {
                0 => print!(
                    " {} ",
                    make_repeated_char_string('-', self.minimum_width + 2)
                ),
                1 => print!(
                    "|{}|",
                    add_padding_to_center_string(&self.directory_name, self.minimum_width + 2)
                ),
                2 => print!(
                    "|{}|",
                    make_repeated_char_string('=', self.minimum_width + 2)
                ),
                _ => {
                    print!("| ");

                    let directory_item = &self.directory_item_vec[row_number - 3];
                    directory_item.print_styled_file_name(true);

                    let length_of_current_file_name: usize =
                        directory_item.get_file_name_length(true);
                    let difference: usize = self.minimum_width - length_of_current_file_name;

                    print!("{} |", make_repeated_char_string(' ', difference));
                }
            }
        } else {
            print!(
                " {} ",
                make_repeated_char_string('-', self.minimum_width + 2)
            );
        }
    }

    pub fn get_total_width_of_directory_container(&self) -> usize {
        self.minimum_width + 4
    }

    pub fn get_total_height_of_directory_container(&self) -> usize {
        self.get_number_of_directory_items() + 4
    }

    pub fn get_number_of_directory_items(&self) -> usize {
        self.directory_item_vec.len()
    }
}
