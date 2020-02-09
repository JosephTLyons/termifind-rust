use console::style;
use std::fs::DirEntry;

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
        let styled_file_name = style(self.get_printable_file_name());

        let styled_colored_file_name = match self.item_state {
            ItemState::Selected => styled_file_name.green(),
            ItemState::Unselected => styled_file_name.white(),
            ItemState::DirectoryInPath => styled_file_name.blue(),
        };

        print!("{}", styled_colored_file_name);
    }
}
