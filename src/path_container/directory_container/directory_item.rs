use std::fs::DirEntry;

use crate::utils::{print_colored_text, truncate_text, Color};

pub enum ItemState {
    DirectoryInPath,
    Selected,
    Unselected,
}

pub enum ItemType {
    Directory,
    File,
    Symlink,
    Unknown,
}

pub struct DirectoryItem {
    pub item_state: ItemState,
    pub directory_entry: DirEntry,
    name_truncation_settings_option: Option<(usize, bool)>,
    item_type: ItemType,
}

impl DirectoryItem {
    pub fn new(
        directory_entry: DirEntry,
        name_truncation_settings_option: Option<(usize, bool)>,
    ) -> Self {
        let item_type = match directory_entry.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    ItemType::Directory
                } else if metadata.is_file() {
                    ItemType::File
                } else {
                    ItemType::Symlink
                }
            }
            Err(_) => ItemType::Unknown,
        };

        DirectoryItem {
            item_state: ItemState::Unselected,
            directory_entry,
            name_truncation_settings_option,
            item_type,
        }
    }

    pub fn print_colored_file_name_based_on_state(&self) {
        let file_name = self.get_printable_file_name();

        match self.item_state {
            ItemState::DirectoryInPath => print_colored_text(file_name, Color::Blue),
            ItemState::Selected => print_colored_text(file_name, Color::Green),
            ItemState::Unselected => match self.item_type {
                ItemType::Directory => print_colored_text(file_name, Color::White),
                ItemType::File => print_colored_text(file_name, Color::Magenta),
                ItemType::Symlink => print_colored_text(file_name, Color::Red),
                ItemType::Unknown => print_colored_text(file_name, Color::Cyan),
            },
        };
    }

    pub fn get_printable_file_name(&self) -> String {
        format!(
            "{} {}",
            self.get_file_type_indicator_string(),
            self.get_truncated_file_name()
        )
    }

    fn get_file_type_indicator_string(&self) -> &str {
        match self.item_type {
            ItemType::Unknown => "(U)",
            ItemType::File => "(F)",
            ItemType::Directory => "(D)",
            ItemType::Symlink => "(S)",
        }
    }

    fn get_truncated_file_name(&self) -> String {
        let file_name = self
            .directory_entry
            .file_name()
            .to_string_lossy()
            .to_string();

        if let Some(file_name_length_after_truncation) = self.name_truncation_settings_option {
            return truncate_text(
                file_name,
                file_name_length_after_truncation.0,
                Some((String::from("..."), file_name_length_after_truncation.1)),
            );
        }

        file_name
    }
}
