use console::style;

pub fn make_repeated_char_string(character: char, repetitions: usize) -> String {
    std::iter::repeat(character)
        .take(repetitions)
        .collect::<String>()
}

#[test]
fn make_repeated_char_string_test() {
    assert_eq!(String::from("======="), make_repeated_char_string('=', 7));
}

pub fn add_padding_to_center_string(text: &str, required_length: usize) -> String {
    let length_of_text = text.chars().count();
    let left_padding_length = (required_length - length_of_text) / 2;
    let mut right_padding_length = left_padding_length;

    if (length_of_text + required_length) % 2 == 1 {
        right_padding_length += 1;
    }

    format!(
        "{}{}{}",
        make_repeated_char_string(' ', left_padding_length),
        text,
        make_repeated_char_string(' ', right_padding_length)
    )
}

#[test]
fn add_padding_to_center_string_even_text_even_required_length() {
    assert_eq!(
        String::from("  even  "),
        add_padding_to_center_string("even", 8)
    );
}

#[test]
fn add_padding_to_center_string_even_text_odd_required_length() {
    assert_eq!(
        String::from("  even   "),
        add_padding_to_center_string("even", 9)
    );
}

#[test]
fn add_padding_to_center_string_odd_test_even_required_length() {
    assert_eq!(
        String::from("  odd   "),
        add_padding_to_center_string("odd", 8)
    );
}

#[test]
fn add_padding_to_center_string_odd_test_odd_required_length() {
    assert_eq!(
        String::from("   odd   "),
        add_padding_to_center_string("odd", 9)
    );
}

#[allow(dead_code)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
}

pub fn print_colored_text(text: String, color: Color) {
    let styled_text = style(text);

    let styled_colored_text = match color {
        Color::Black => styled_text.black(),
        Color::Blue => styled_text.blue(),
        Color::Cyan => styled_text.cyan(),
        Color::Green => styled_text.green(),
        Color::Magenta => styled_text.magenta(),
        Color::Red => styled_text.red(),
        Color::White => styled_text.white(),
    };

    print!("{}", styled_colored_text);
}

pub fn truncate_text(
    text: String,
    mut text_length_after_truncation: usize,
    text_to_append_option: Option<(String, bool)>,
) -> String {
    if text.chars().count() > text_length_after_truncation {
        return match text_to_append_option {
            Some(text_to_append) => {
                if text_to_append.1 {
                    if text_length_after_truncation <= text_to_append.0.chars().count() {
                        return String::from(&text_to_append.0[..(text_length_after_truncation)]);
                    } else {
                        text_length_after_truncation -= text_to_append.0.chars().count()
                    }
                }

                format!(
                    "{}{}",
                    String::from(&text[..(text_length_after_truncation)]),
                    text_to_append.0
                )
            }
            None => String::from(&text[..(text_length_after_truncation)]),
        };
    }

    text
}

#[test]
fn truncate_text_shorter_file_name() {
    assert_eq!(
        truncate_text(String::from("Man"), 10, None),
        String::from("Man")
    )
}

#[test]
fn truncate_text_longer_file_name() {
    assert_eq!(
        truncate_text(String::from("Man is scary!"), 8, None),
        String::from("Man is s")
    )
}

#[test]
fn truncate_text_shorter_file_name_with_text_to_append() {
    assert_eq!(
        truncate_text(String::from("Dog"), 5, Some((String::from("..."), false))),
        String::from("Dog")
    )
}

#[test]
fn truncate_text_longer_file_name_with_text_to_append_option() {
    assert_eq!(
        truncate_text(
            String::from("Dog is super cool!"),
            5,
            Some((String::from("..."), false)),
        ),
        String::from("Dog i...")
    )
}

#[test]
fn truncate_text_shorter_file_name_include_appended_text() {
    assert_eq!(
        truncate_text(String::from("Man"), 10, None),
        String::from("Man")
    )
}

#[test]
fn truncate_text_longer_file_name_include_appended_text() {
    assert_eq!(
        truncate_text(String::from("Man is scary!"), 8, None),
        String::from("Man is s")
    )
}

#[test]
fn truncate_text_shorter_file_name_with_text_to_append_include_appended_text() {
    assert_eq!(
        truncate_text(String::from("Dog"), 5, Some((String::from("..."), true))),
        String::from("Dog")
    )
}

#[test]
fn truncate_text_longer_file_name_with_text_to_append_option_include_appended_text_1() {
    assert_eq!(
        truncate_text(
            String::from("Dog is super cool!"),
            4,
            Some((String::from("..."), true)),
        ),
        String::from("D...")
    )
}

#[test]
fn truncate_text_longer_file_name_with_text_to_append_option_include_appended_text_2() {
    assert_eq!(
        truncate_text(
            String::from("Dog is super cool!"),
            3,
            Some((String::from("..."), true)),
        ),
        String::from("...")
    )
}

#[test]
fn truncate_text_longer_file_name_with_text_to_append_option_include_appended_text_3() {
    assert_eq!(
        truncate_text(
            String::from("Dog is super cool!"),
            2,
            Some((String::from("..."), true)),
        ),
        String::from("..")
    )
}

#[test]
fn truncate_text_longer_file_name_with_text_to_append_option_include_appended_text_4() {
    assert_eq!(
        truncate_text(
            String::from("Dog is super cool!"),
            1,
            Some((String::from("..."), true)),
        ),
        String::from(".")
    )
}

#[test]
fn truncate_text_longer_file_name_with_text_to_append_option_include_appended_text_5() {
    assert_eq!(
        truncate_text(
            String::from("Dog is super cool!"),
            0,
            Some((String::from("..."), true)),
        ),
        String::from("")
    )
}
