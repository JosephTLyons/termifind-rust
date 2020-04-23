use std::fs::DirEntry;

use crate::{
    settings::DirectoryItemSettings,
    utils::string::{print_colored_text, truncate_text, Color},
};

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

#[derive(Clone)]
pub struct NameTruncationSettings {
    pub name_length_after_truncation: usize,
    pub should_include_truncated_text_indicator_in_length: bool,
    pub truncated_text_indicator: String,
}

pub struct DirectoryItem {
    pub directory_entry: DirEntry,
    pub item_state: ItemState,
    item_type: ItemType,
    directory_item_settings: DirectoryItemSettings,
}

impl DirectoryItem {
    pub fn new(directory_entry: DirEntry, directory_item_settings: DirectoryItemSettings) -> Self {
        let item_type = DirectoryItem::get_item_type(&directory_entry);

        DirectoryItem {
            item_state: ItemState::Unselected,
            directory_entry,
            item_type,
            directory_item_settings,
        }
    }

    fn get_item_type(directory_entry: &DirEntry) -> ItemType {
        match directory_entry.metadata() {
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
        }
    }

    pub fn get_file_name(
        &self,
        should_include_item_type_indicator: bool,
        name_truncation_settings_option: &Option<NameTruncationSettings>,
    ) -> String {
        if should_include_item_type_indicator {
            return format!(
                "{} {}",
                self.get_item_type_indicator_string(),
                self.get_truncated_file_name(&name_truncation_settings_option)
            );
        }

        self.get_truncated_file_name(&name_truncation_settings_option)
    }

    pub fn get_file_name_length(
        &self,
        should_include_item_type_indicator_in_length: bool,
        name_truncation_settings_option: &Option<NameTruncationSettings>,
    ) -> usize {
        self.get_file_name(
            should_include_item_type_indicator_in_length,
            &name_truncation_settings_option,
        )
        .chars()
        .count()
    }

    pub fn print_styled_file_name(
        &self,
        should_include_item_type_indicator: bool,
        name_truncation_settings_option: &Option<NameTruncationSettings>,
    ) {
        let file_name = self.get_file_name(
            should_include_item_type_indicator,
            &name_truncation_settings_option,
        );

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

    fn get_item_type_indicator_string(&self) -> &str {
        match self.item_type {
            ItemType::Directory => &self.directory_item_settings.item_type_indicator_directory,
            ItemType::File => &self.directory_item_settings.item_type_indicator_file,
            ItemType::Symlink => &self.directory_item_settings.item_type_indicator_symlink,
            ItemType::Unknown => &self.directory_item_settings.item_type_indicator_unknown,
        }
    }

    fn get_truncated_file_name(
        &self,
        name_truncation_settings_option: &Option<NameTruncationSettings>,
    ) -> String {
        let file_name = self
            .directory_entry
            .file_name()
            .to_string_lossy()
            .to_string();

        if let Some(name_truncation_settings) = name_truncation_settings_option {
            return truncate_text(
                file_name,
                name_truncation_settings.name_length_after_truncation,
                Some((
                    name_truncation_settings.truncated_text_indicator.clone(),
                    name_truncation_settings.should_include_truncated_text_indicator_in_length,
                )),
            );
        }

        file_name
    }

    pub fn is_hidden_file(&self) -> bool {
        self.get_file_name(false, &None).starts_with('.')
    }
}
