pub const DIGIT_LIST: &str = "1234567890";

pub const fn is_spacing(c: char) -> bool {
    c == ' ' || c == '\t'
}

pub const fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}
