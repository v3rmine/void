pub fn slugify(input: &str) -> String {
  input
    .to_lowercase()
    .replace(' ', "-")
    .replace(&['!', '?', '.', ','][..], "")
}
