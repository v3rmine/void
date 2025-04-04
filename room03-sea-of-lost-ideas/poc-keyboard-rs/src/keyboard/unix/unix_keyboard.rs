use crate::Result;

pub fn cleanup_key(name: &str) -> Result<(&str,bool)> {
    let name = name.strip_prefix("+");
    let is_keypad =
}