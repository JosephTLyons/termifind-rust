use std::fs::DirEntry;

use crate::utils::{print_colored_text, Color};

#[allow(dead_code)]
enum ItemType {
    Unknown,
    File,
    Directory,
    Symlink,
}

#[allow(dead_code)]
pub enum ItemState {
    Selected,
    Unselected,
    DirectoryInPath,
}

#[allow(dead_code)]
pub struct DirectoryItem {
    pub item_state: ItemState,
    pub directory_entry: DirEntry,
    item_type: ItemType,
}

impl DirectoryItem {
    pub fn new(directory_entry: DirEntry) -> Self {
        DirectoryItem {
            item_state: ItemState::Unselected,
            directory_entry,
            item_type: ItemType::Unknown,
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
            ItemState::Unselected => print_colored_text(file_name, Color::White),
            ItemState::DirectoryInPath => print_colored_text(file_name, Color::Blue),
        };
    }
}
