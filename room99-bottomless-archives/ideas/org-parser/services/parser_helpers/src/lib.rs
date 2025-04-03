pub const DIGIT_LIST: &str = "1234567890";

pub const fn is_spacing(c: char) -> bool {
    c == ' ' || c == '\t'
}

pub const fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}

pub fn end_position_in_str(src: &str, previous_position: Option<(usize, usize)>) -> (usize, usize) {
    src.chars().fold(
        previous_position.unwrap_or((1_usize, 1_usize)),
        |(col, line), c| {
            if is_newline(c) {
                (1, line + 1)
            } else {
                (col + 1, line)
            }
        },
    )
}
