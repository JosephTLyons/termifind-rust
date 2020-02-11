use std::fs::{DirEntry, Metadata};

use crate::utils::{print_colored_text, Color};

pub enum ItemState {
    Selected,
    Unselected,
    DirectoryInPath,
}

pub enum ItemType {
    Unknown,
    File,
    Directory,
    Symlink,
}

#[allow(dead_code)]
pub struct DirectoryItem {
    pub item_state: ItemState,
    pub directory_entry: DirEntry,
    item_type: ItemType,
}

impl DirectoryItem {
    pub fn new(directory_entry: DirEntry) -> Self {
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
            item_type,
        }
    }

    pub fn get_printable_file_name(&self) -> String {
        self.directory_entry
            .file_name()
            .to_string_lossy()
            .to_string()
    }

    pub fn print_colored_file_name_based_on_state(&self) {
        let file_name = self.get_printable_file_name();

        match self.item_state {
            ItemState::Selected => print_colored_text(file_name, Color::Green),
            ItemState::Unselected => match self.item_type {
                ItemType::Unknown => print_colored_text(file_name, Color::Cyan),
                ItemType::File => print_colored_text(file_name, Color::Magenta),
                ItemType::Directory => print_colored_text(file_name, Color::White),
                ItemType::Symlink => print_colored_text(file_name, Color::Red),
            },
            ItemState::DirectoryInPath => print_colored_text(file_name, Color::Blue),
        };
    }
}
