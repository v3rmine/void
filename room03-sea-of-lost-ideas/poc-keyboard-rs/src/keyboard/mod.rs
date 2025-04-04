//#[cfg(target_os = "linux")]
pub mod unix;
pub mod canonical_names;

#[derive(Debug)]
pub struct KeyNotEmpty;
impl std::error::Error for KeyNotEmpty {}
impl std::fmt::Display for KeyNotEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Registered key cannot be empty")
    }
}