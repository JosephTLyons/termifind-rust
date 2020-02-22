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
        should_include_appended_text_in_length: bool,
    },
    Level {
        level: usize,
        should_include_appended_text_in_length: bool,
    },
    AverageFileNameLength {
        should_include_appended_text_in_length: bool,
    },
    Outliers {
        should_include_appended_text_in_length: bool, // Does this option even make sense here?
    },
    HorizontalFit, // Performs calculations and then uses Constant
}

#[derive(serde_derive::Deserialize, Clone)]
pub struct DirectoryContainerSettings {
    pub sort_directory_item_by_type: bool,
    pub truncation_options: TruncationOptions,
    pub horizontal_border_symbol: char,
    pub vertical_border_symbol: char,
    pub content_divider_symbol: char,
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
    let directory_container_settings = DirectoryContainerSettings {
        sort_directory_item_by_type: false,
        truncation_options: {
            TruncationOptions::Outliers {
                should_include_appended_text_in_length: true,
            }
        },
        horizontal_border_symbol: '-',
        vertical_border_symbol: '|',
        content_divider_symbol: '=',
    };

    let path_container_settings = PathContainerSettings {
        number_of_directory_containers_to_print_option: None,
        spacing_between_directory_containers: 2,
        spacing_between_directory_containers_char: ' ',
        spacing_between_directory_container_rows: 1,
        spacing_between_directory_container_rows_char: ' ',
        directory_container_settings,
    };

    Settings {
        path_container_settings,
    }
}

pub fn get_settings_from_file() -> Settings {
    // WILL NEED TO USE A CRATE TO FIND A PLACE TO STORE THE SETTINGS FILE THAT IS CROSS COMPATIBLE
    let path_to_json_settings_file =
        Path::new("/Users/josephlyons/Programming/Rust/termifind/termifind_settings.json");

    match read_to_string(path_to_json_settings_file) {
        Ok(json_settings_string) => match from_str(&json_settings_string) {
            Ok(settings) => settings,
            Err(_) => get_default_settings(),
        },
        Err(_) => get_default_settings(),
    }
}
