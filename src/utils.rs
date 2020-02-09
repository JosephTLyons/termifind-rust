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
