use serde_derive;
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::Path;

#[allow(dead_code)]
#[derive(serde_derive::Deserialize, Clone)]
pub enum TruncationOptions {
    None,
    Constant {
        constant: usize,
        should_include_truncated_text_indicator_in_length: bool,
    },
    Level {
        level: usize,
        should_include_truncated_text_indicator_in_length: bool,
    },
    AverageFileNameLength {
        should_include_truncated_text_indicator_in_length: bool,
    },
    Outliers {
        should_include_truncated_text_indicator_in_length: bool, // Does this option even make sense here?
    },
    HorizontalFit, // Performs calculations and then uses Constant
}

#[derive(serde_derive::Deserialize, Clone)]
pub struct DirectoryItemSettings {
    pub item_type_indicator_directory: String,
    pub item_type_indicator_file: String,
    pub item_type_indicator_symlink: String,
    pub item_type_indicator_unknown: String,
}

#[derive(serde_derive::Deserialize, Clone)]
pub struct DirectoryContainerSettings {
    pub sort_directory_item_by_item_type_indicator: bool,
    pub truncation_options: TruncationOptions,
    pub horizontal_border_symbol: char,
    pub vertical_border_symbol: char,
    pub content_divider_symbol: char,

    pub directory_item_settings: DirectoryItemSettings,
}

#[derive(serde_derive::Deserialize)]
pub struct PathContainerSettings {
    pub number_of_directory_containers_to_print_option: Option<usize>,
    pub spacing_between_directory_containers: usize,
    pub spacing_between_directory_containers_char: char,
    pub spacing_between_directory_container_rows: usize,
    pub spacing_between_directory_container_rows_char: char,

    pub directory_container_settings: DirectoryContainerSettings,
}

#[derive(serde_derive::Deserialize)]
pub struct Settings {
    pub path_container_settings: PathContainerSettings,
}

fn get_default_settings() -> Settings {
    Settings {
        path_container_settings: PathContainerSettings {
            number_of_directory_containers_to_print_option: None,
            spacing_between_directory_containers: 1,
            spacing_between_directory_containers_char: ' ',
            spacing_between_directory_container_rows: 1,
            spacing_between_directory_container_rows_char: ' ',
            directory_container_settings: DirectoryContainerSettings {
                sort_directory_item_by_item_type_indicator: false,
                truncation_options: {
                    TruncationOptions::Outliers {
                        should_include_truncated_text_indicator_in_length: true,
                    }
                },
                horizontal_border_symbol: '-',
                vertical_border_symbol: '|',
                content_divider_symbol: '=',
                directory_item_settings: DirectoryItemSettings {
                    item_type_indicator_directory: String::from("(D)"),
                    item_type_indicator_file: String::from("(F)"),
                    item_type_indicator_symlink: String::from("(S)"),
                    item_type_indicator_unknown: String::from("(U)"),
                },
            },
        },
    }
}

pub fn get_settings_from_file() -> Settings {
    // WILL NEED TO USE A CRATE TO FIND A PLACE TO STORE THE SETTINGS FILE THAT IS CROSS COMPATIBLE
    let path_to_json_settings_file =
        Path::new("/Users/josephlyons/Programming/Rust/termifind/termifind_settings.json");

    if let Ok(json_settings_string) = read_to_string(path_to_json_settings_file) {
        if let Ok(settings) = from_str(&json_settings_string) {
            return settings;
        }
    }

    get_default_settings()
}
