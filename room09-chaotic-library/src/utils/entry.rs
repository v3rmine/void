use walkdir::DirEntry;

/// Convert the file path to a string and strip the root directory prefix
/// If the resulting path is empty, use "/" instead
/// This creates a clean relative path for the blog entry
pub fn direntry_to_string(entry: &DirEntry, root: Option<&str>) -> String {
  entry
    .path()
    .to_string_lossy()
    .strip_prefix(root.unwrap_or(""))
    .map(|s| String::from(if s.len() > 0 { s } else { "/" }))
    .unwrap()
}

/// Extract the name and parent path from a given path string
/// 1. Split the path by '/' to get path components and filter out empty segments
/// 2. Use split_last() to separate the last component (name) from the rest (parent path)
/// 3. Format the parent path with a leading '/'
/// 4. Return an empty tuple if the path has no components
pub fn path_to_name_and_parent(path: &str) -> (String, String) {
  path
    .split('/')
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .split_last()
    .map(|(last, rest)| {
      let rest = format!("/{}", rest.join("/"));

      (last.to_string(), rest)
    })
    .unwrap_or_else(|| ("".to_string(), "".to_string()))
}
