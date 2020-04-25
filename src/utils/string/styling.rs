use console::style;

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
