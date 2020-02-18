use std::fs::{read_dir, ReadDir};
use std::path::PathBuf;
use std::vec::Vec;

mod directory_item;
pub use directory_item::{DirectoryItem, ItemState, ItemType, NameTruncationSettings};

use crate::utils::string::{add_padding_to_center_string, make_repeated_char_string};

#[allow(dead_code)]
enum TruncationOptions {
    None,
    Constant {
        constant: usize,
        should_include_appended_text_in_length: bool,
    },
    Level {
        level: usize,
        should_include_appended_text_in_length: bool,
    },
    AverageFileNameLength {
        should_include_appended_text_in_length: bool,
    },
    Outliers,      // Performs calculations and then uses Level
    HorizontalFit, // Performs calculations and then uses Constant
}

pub struct DirectoryContainer {
    pub path_to_directory: PathBuf,
    pub directory_item_vec: Vec<DirectoryItem>,
    directory_name: String,
    minimum_width: usize,
    name_truncation_settings_option: Option<NameTruncationSettings>,
    horizontal_border_symbol: char,
    vertical_border_symbol: char,
    name_content_divider_symbol: char,
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

        let directory_name: String = match path.file_name() {
            Some(d_name) => d_name.to_string_lossy().to_string(),
            None => path.to_string_lossy().to_string(),
        };

        let mut directory_container = DirectoryContainer {
            directory_name,
            minimum_width: 0,
            path_to_directory: path,
            directory_item_vec,
            name_truncation_settings_option: None,
            horizontal_border_symbol: '-',
            vertical_border_symbol: '|',
            name_content_divider_symbol: '=',
        };

        directory_container.sort_directory_items(false);
        directory_container.apply_truncation_settings_to_directory_container(
            TruncationOptions::None,
        );

        directory_container
    }

    fn sort_directory_items(&mut self, by_file_type: bool) {
        let name_truncation_settings_option = &self.name_truncation_settings_option.clone();

        self.directory_item_vec.sort_by(|a, b| {
            a.get_file_name(by_file_type, &name_truncation_settings_option)
                .partial_cmp(&b.get_file_name(by_file_type, &name_truncation_settings_option))
                .expect("Oops")
        });
    }

    pub fn apply_truncation_settings_to_directory_container(
        &mut self,
        truncation_options: TruncationOptions,
    ) {
        self.set_truncation_settings(truncation_options);
        self.set_minimum_width();
    }

    fn set_truncation_settings(&mut self, truncation_options: TruncationOptions) {
        self.name_truncation_settings_option = match truncation_options {
            TruncationOptions::None => None,
            TruncationOptions::Constant {
                constant,
                should_include_appended_text_in_length,
            } => Some(NameTruncationSettings {
                name_length_after_truncation: constant,
                should_include_appended_text_in_length,
            }),
            TruncationOptions::Level {
                level,
                should_include_appended_text_in_length,
            } => Some(NameTruncationSettings {
                name_length_after_truncation: self.get_truncation_value_by_level(level),
                should_include_appended_text_in_length,
            }),
            TruncationOptions::AverageFileNameLength {
                should_include_appended_text_in_length,
            } => Some(NameTruncationSettings {
                name_length_after_truncation: self.get_truncated_value_by_file_name_average(),
                should_include_appended_text_in_length,
            }),
            TruncationOptions::Outliers => None, // Implement
            TruncationOptions::HorizontalFit => None, // Implement
        }
    }

    fn get_truncation_value_by_level(&self, mut level: usize) -> usize {
        let mut file_name_length_vec = self.get_file_name_lengths_vec(false);

        file_name_length_vec.sort();
        file_name_length_vec.dedup();
        file_name_length_vec.reverse();

        let vec_length = file_name_length_vec.len();

        level += 1;

        if level < vec_length {
            return file_name_length_vec[level];
        }

        file_name_length_vec[vec_length - 1]
    }

    fn get_truncated_value_by_file_name_average(&self) -> usize {
        let file_name_lengths_vec = self.get_file_name_lengths_vec(false);
        let sum_of_file_name_lengths: usize = file_name_lengths_vec.iter().sum();

        sum_of_file_name_lengths / file_name_lengths_vec.len()
    }

    fn set_minimum_width(&mut self) {
        let length_of_longest_file_name: usize =
            match self.get_file_name_lengths_vec(true).iter().max() {
                Some(x) => *x,
                None => 0,
            };
        let length_of_current_directory_name = self.directory_name.chars().count();

        self.minimum_width = if length_of_current_directory_name > length_of_longest_file_name {
            length_of_current_directory_name
        } else {
            length_of_longest_file_name
        };
    }

    fn get_file_name_lengths_vec(&self, include_type_indicator_in_length: bool) -> Vec<usize> {
        self.directory_item_vec
            .iter()
            .map(|directory_item| {
                directory_item.get_file_name_length(
                    include_type_indicator_in_length,
                    &self.name_truncation_settings_option,
                )
            })
            .collect()
    }

    pub fn print_directory_container_by_row(&self, row_number: usize) {
        if row_number < self.get_total_height_of_directory_container() - 1 {
            match row_number {
                0 => print!(
                    " {} ",
                    make_repeated_char_string(
                        self.horizontal_border_symbol,
                        self.minimum_width + 2
                    )
                ),
                1 => print!(
                    "{}{}{}",
                    self.vertical_border_symbol,
                    add_padding_to_center_string(&self.directory_name, self.minimum_width + 2),
                    self.vertical_border_symbol,
                ),
                2 => print!(
                    "{}{}{}",
                    self.vertical_border_symbol,
                    make_repeated_char_string(
                        self.name_content_divider_symbol,
                        self.minimum_width + 2
                    ),
                    self.vertical_border_symbol,
                ),
                _ => {
                    print!("{} ", self.vertical_border_symbol);

                    let directory_item = &self.directory_item_vec[row_number - 3];
                    directory_item
                        .print_styled_file_name(true, &self.name_truncation_settings_option);

                    let length_of_current_file_name: usize = directory_item
                        .get_file_name_length(true, &self.name_truncation_settings_option);
                    let difference: usize = self.minimum_width - length_of_current_file_name;

                    print!(
                        "{} {}",
                        make_repeated_char_string(' ', difference),
                        self.vertical_border_symbol
                    );
                }
            }
        } else {
            print!(
                " {} ",
                make_repeated_char_string(self.horizontal_border_symbol, self.minimum_width + 2)
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
