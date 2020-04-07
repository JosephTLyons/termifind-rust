use std::fs::{read_dir, ReadDir};
use std::path::PathBuf;
use std::vec::Vec;

mod directory_item;
pub use directory_item::{DirectoryItem, ItemState, ItemType, NameTruncationSettings};

use crate::settings::{DirectoryContainerSettings, TruncationOptions};
use crate::utils::maths::{get_average, get_outliers};
use crate::utils::string::{add_padding_to_center_string, make_repeated_char_string};

pub struct DirectoryContainer {
    pub path_to_directory: PathBuf,
    pub directory_item_vec: Vec<DirectoryItem>,
    directory_name: String,
    minimum_width: usize,
    name_truncation_settings_option: Option<NameTruncationSettings>,
    directory_container_settings: DirectoryContainerSettings,
}

impl DirectoryContainer {
    pub fn new(
        path: PathBuf,
        selected_directory_option: &Option<PathBuf>,
        directory_container_settings: DirectoryContainerSettings,
    ) -> Self {
        let mut directory_item_vec: Vec<DirectoryItem> = Vec::new();
        let read_directory_iterator: ReadDir = read_dir(&path).expect("Oops");

        for file in read_directory_iterator {
            let mut directory_item: DirectoryItem = DirectoryItem::new(
                file.expect("Oops"),
                directory_container_settings.directory_item_settings.clone(),
            );

            if !directory_container_settings.should_display_hidden_files
                && directory_item.is_hidden_file()
            {
                continue;
            }

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
            directory_container_settings,
        };

        directory_container.sort_directory_items(
            directory_container
                .directory_container_settings
                .sort_directory_item_by_item_type_indicator,
        );
        directory_container.apply_truncation_settings_to_directory_container(
            directory_container
                .directory_container_settings
                .truncation_options
                .clone(),
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

    // REFACTOR
    fn set_truncation_settings(&mut self, truncation_options: TruncationOptions) {
        self.name_truncation_settings_option = match truncation_options {
            TruncationOptions::None => None,
            TruncationOptions::Constant {
                constant,
                should_include_truncated_text_indicator_in_length,
            } => Some(NameTruncationSettings {
                name_length_after_truncation: constant,
                should_include_truncated_text_indicator_in_length,
                truncated_text_indicator: self
                    .directory_container_settings
                    .truncated_text_indicator
                    .clone(),
            }),
            TruncationOptions::Level {
                level,
                should_include_truncated_text_indicator_in_length,
            } => match level {
                0 => None,
                _ => Some(NameTruncationSettings {
                    name_length_after_truncation: self.get_truncation_value_by_level(level, true),
                    should_include_truncated_text_indicator_in_length,
                    truncated_text_indicator: self
                        .directory_container_settings
                        .truncated_text_indicator
                        .clone(),
                }),
            },
            TruncationOptions::AverageFileNameLength {
                should_include_truncated_text_indicator_in_length,
            } => Some(NameTruncationSettings {
                name_length_after_truncation: self.get_truncated_value_by_file_name_average(),
                should_include_truncated_text_indicator_in_length,
                truncated_text_indicator: self
                    .directory_container_settings
                    .truncated_text_indicator
                    .clone(),
            }),
            TruncationOptions::Outliers {
                should_include_truncated_text_indicator_in_length,
            } => {
                let outliers_vec_option = get_outliers(self.get_file_name_lengths_vec(false), true);

                match outliers_vec_option {
                    None => None,
                    Some(outliers_vec) => Some(NameTruncationSettings {
                        name_length_after_truncation: self
                            .get_truncation_value_by_level(outliers_vec.1.len(), false),
                        should_include_truncated_text_indicator_in_length,
                        truncated_text_indicator: self
                            .directory_container_settings
                            .truncated_text_indicator
                            .clone(),
                    }),
                }
            }
            TruncationOptions::HorizontalFit => None, // Implement
        }
    }

    fn get_truncation_value_by_level(&self, level: usize, should_remove_duplicates: bool) -> usize {
        let mut file_name_length_vec = self.get_file_name_lengths_vec(false);

        if should_remove_duplicates {
            file_name_length_vec.dedup();
        }

        file_name_length_vec.reverse();

        let vec_length = file_name_length_vec.len();

        if level < vec_length {
            return file_name_length_vec[level];
        }

        file_name_length_vec[vec_length - 1]
    }

    fn get_truncated_value_by_file_name_average(&self) -> usize {
        get_average(&self.get_file_name_lengths_vec(false)) as usize
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
        let mut file_name_vec: Vec<usize> = self
            .directory_item_vec
            .iter()
            .map(|directory_item| {
                directory_item.get_file_name_length(
                    include_type_indicator_in_length,
                    &self.name_truncation_settings_option,
                )
            })
            .collect();

        file_name_vec.sort();

        file_name_vec
    }

    pub fn print_directory_container_by_row(&self, row_number: usize) {
        let beginning_directory_item_row = 3;
        let ending_directory_item_row = self.get_total_height_of_directory_container() - 1;

        match row_number {
            1 => self.print_directory_container_file_name_row(),
            2 => self.print_content_divider_row(),
            x if (beginning_directory_item_row..ending_directory_item_row).contains(&x) => {
                self.print_directory_item_row(row_number - beginning_directory_item_row);
            }
            _ => self.print_horizontal_directory_container_line_row(),
        }
    }

    fn print_horizontal_directory_container_line_row(&self) {
        print!(
            " {} ",
            make_repeated_char_string(
                self.directory_container_settings.horizontal_border_symbol,
                self.minimum_width + 2
            )
        );
    }

    fn print_directory_container_file_name_row(&self) {
        print!(
            "{}{}{}",
            self.directory_container_settings.vertical_border_symbol,
            add_padding_to_center_string(&self.directory_name, self.minimum_width + 2),
            self.directory_container_settings.vertical_border_symbol,
        );
    }

    fn print_content_divider_row(&self) {
        print!(
            "{}{}{}",
            self.directory_container_settings.vertical_border_symbol,
            make_repeated_char_string(
                self.directory_container_settings.content_divider_symbol,
                self.minimum_width + 2
            ),
            self.directory_container_settings.vertical_border_symbol,
        );
    }

    fn print_directory_item_row(&self, row_number: usize) {
        print!(
            "{} ",
            self.directory_container_settings.vertical_border_symbol
        );

        let directory_item = &self.directory_item_vec[row_number];
        directory_item.print_styled_file_name(true, &self.name_truncation_settings_option);

        let length_of_current_file_name: usize =
            directory_item.get_file_name_length(true, &self.name_truncation_settings_option);
        let difference: usize = self.minimum_width - length_of_current_file_name;

        print!(
            "{} {}",
            make_repeated_char_string(' ', difference),
            self.directory_container_settings.vertical_border_symbol
        );
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

    pub fn get_directory_name(&self) -> &String {
        &self.directory_name
    }
}
