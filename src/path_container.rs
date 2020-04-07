use std::collections::VecDeque;
use std::path::{Path, PathBuf};

mod directory_container;
use directory_container::DirectoryContainer;
use directory_container::ItemState;

use crate::settings::PathContainerSettings;
use crate::utils::string::make_repeated_char_string;

#[allow(dead_code)]
pub struct PathContainer {
    current_path: PathBuf,
    directory_container_vec_deque: VecDeque<DirectoryContainer>,
    terminal_dimensions: (usize, usize),
    path_container_settings: PathContainerSettings,
}

impl PathContainer {
    pub fn new(path: PathBuf, path_container_settings: PathContainerSettings) -> Self {
        let mut directory_container_vec_deque: VecDeque<DirectoryContainer> = VecDeque::new();
        let mut parent_path: &Path = &path;

        loop {
            directory_container_vec_deque.push_front(DirectoryContainer::new(
                parent_path.to_path_buf(),
                &PathContainer::get_selected_directory_option(&directory_container_vec_deque),
                path_container_settings.directory_container_settings.clone(),
            ));

            match parent_path.parent() {
                Some(p_path) => parent_path = p_path,
                None => break,
            }
        }

        PathContainer::select_first_directory_in_current_directory_container(
            &mut directory_container_vec_deque,
        );

        PathContainer {
            current_path: path,
            directory_container_vec_deque,
            terminal_dimensions: term_size::dimensions().expect("Oops"),
            path_container_settings,
        }
    }

    fn get_selected_directory_option(
        directory_container_vec_deque: &VecDeque<DirectoryContainer>,
    ) -> Option<PathBuf> {
        if directory_container_vec_deque.is_empty() {
            return None;
        }
        match directory_container_vec_deque.front() {
            Some(first_directory_container) => {
                Some(first_directory_container.path_to_directory.clone())
            }
            None => None,
        }
    }

    fn select_first_directory_in_current_directory_container(
        directory_container_vec_deque: &mut VecDeque<DirectoryContainer>,
    ) {
        if let Some(directory_container) = directory_container_vec_deque.back_mut() {
            if let Some(directory_item) = directory_container.directory_item_vec.first_mut() {
                directory_item.item_state = ItemState::Selected
            }
        }
    }

    pub fn print_path(&self) {
        let starting_index = match self
            .path_container_settings
            .number_of_directory_containers_to_print_option
        {
            Some(number_of_directory_containers_to_print) => {
                if number_of_directory_containers_to_print
                    <= self.directory_container_vec_deque.len()
                {
                    self.directory_container_vec_deque.len()
                        - number_of_directory_containers_to_print
                } else {
                    0
                }
            }
            None => 0,
        };

        let mut start_and_end_iteration_tuple: (usize, usize) =
            self.update_start_and_end_iteration_tuple((starting_index, starting_index));

        while start_and_end_iteration_tuple.0 < self.directory_container_vec_deque.len() {
            self.is_directory_container_wider_than_terminal(start_and_end_iteration_tuple.0);

            self.print_one_row_of_directory_containers(start_and_end_iteration_tuple);

            start_and_end_iteration_tuple =
                self.update_start_and_end_iteration_tuple(start_and_end_iteration_tuple);
        }
    }

    fn is_directory_container_wider_than_terminal(&self, iterator: usize) {
        if self.directory_container_vec_deque[iterator].get_total_width_of_directory_container()
            > self.terminal_dimensions.0
        {
            println!(
                "The directory '{}' is wider than the terminal width and cannot be output.",
                self.directory_container_vec_deque[iterator].get_directory_name()
            );
            std::process::exit(0);
        }
    }

    // REFACTOR
    fn update_start_and_end_iteration_tuple(
        &self,
        mut start_and_end_iteration_tuple: (usize, usize),
    ) -> (usize, usize) {
        let mut previous_directory_containers_space_requirement = 0;

        start_and_end_iteration_tuple.0 = start_and_end_iteration_tuple.1;

        loop {
            let current_directory_container_space_requirement =
                if start_and_end_iteration_tuple.1 < self.directory_container_vec_deque.len() {
                    self.directory_container_vec_deque[start_and_end_iteration_tuple.1]
                        .get_total_width_of_directory_container()
                        + self
                            .path_container_settings
                            .spacing_between_directory_containers
                } else {
                    0
                };

            let mut all_directory_containers_space_requirement =
                previous_directory_containers_space_requirement
                    + current_directory_container_space_requirement;

            if all_directory_containers_space_requirement
                >= self
                    .path_container_settings
                    .spacing_between_directory_containers
            {
                all_directory_containers_space_requirement -= self
                    .path_container_settings
                    .spacing_between_directory_containers
            }

            let can_fit_current_directory_containers_in_row =
                all_directory_containers_space_requirement < self.terminal_dimensions.0;

            let at_end_of_directory_container_deque =
                start_and_end_iteration_tuple.1 >= self.directory_container_vec_deque.len();

            if can_fit_current_directory_containers_in_row && !at_end_of_directory_container_deque {
                previous_directory_containers_space_requirement += self
                    .directory_container_vec_deque[start_and_end_iteration_tuple.1]
                    .get_total_width_of_directory_container()
                    + self
                        .path_container_settings
                        .spacing_between_directory_containers;
                start_and_end_iteration_tuple.1 += 1;
            } else {
                break start_and_end_iteration_tuple;
            }
        }
    }

    fn print_one_row_of_directory_containers(&self, start_and_end_iteration_tuple: (usize, usize)) {
        let height_of_tallest_container =
            self.get_height_of_tallest_directory_container_in_range(start_and_end_iteration_tuple);

        for i in 0..height_of_tallest_container
            + self
                .path_container_settings
                .spacing_between_directory_container_rows
        {
            if i < height_of_tallest_container {
                for j in start_and_end_iteration_tuple.0..start_and_end_iteration_tuple.1 {
                    self.print_single_line_of_each_directory_container(
                        j,
                        i,
                        j < start_and_end_iteration_tuple.1 - 1,
                    );
                }

                println!();
            } else if start_and_end_iteration_tuple.1 < self.directory_container_vec_deque.len() - 1
            {
                println!(
                    "{}",
                    make_repeated_char_string(
                        self.path_container_settings
                            .spacing_between_directory_container_rows_char,
                        self.terminal_dimensions.0
                    )
                );
            }
        }
    }

    fn get_height_of_tallest_directory_container_in_range(
        &self,
        start_and_end_iteration_tuple: (usize, usize),
    ) -> usize {
        let mut height_of_tallest_container = 0;

        for i in start_and_end_iteration_tuple.0..start_and_end_iteration_tuple.1 {
            let directory_container = &self.directory_container_vec_deque[i];
            let container_height = directory_container.get_total_height_of_directory_container();

            if height_of_tallest_container < container_height {
                height_of_tallest_container = container_height;
            }
        }

        height_of_tallest_container
    }

    fn print_single_line_of_each_directory_container(
        &self,
        directory_container_number: usize,
        row_number: usize,
        should_print_spacing_between_directory_containers: bool,
    ) {
        if row_number
            < self.directory_container_vec_deque[directory_container_number]
                .get_total_height_of_directory_container()
        {
            self.directory_container_vec_deque[directory_container_number]
                .print_directory_container_by_row(row_number);
        } else {
            print!(
                "{}",
                make_repeated_char_string(
                    self.path_container_settings.filler_char,
                    self.directory_container_vec_deque[directory_container_number]
                        .get_total_width_of_directory_container()
                )
            );
        }
        if should_print_spacing_between_directory_containers {
            print!(
                "{}",
                make_repeated_char_string(
                    self.path_container_settings
                        .spacing_between_directory_containers_char,
                    self.path_container_settings
                        .spacing_between_directory_containers
                )
            );
        }
    }
}
