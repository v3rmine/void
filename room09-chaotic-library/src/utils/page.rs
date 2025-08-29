use serde::Serialize;

use crate::common::AppSettings;

#[derive(Debug, Clone, Serialize)]
pub struct PageMetadata {
  pub is_dir: bool,
  pub file_name: String,
  pub path: String,
  pub parent: String,
}

pub fn list_pages(settings: &AppSettings) -> Vec<PageMetadata> {
  let root = format!("{}/{}", settings.views_dir, settings.views_blog_subdir);
  let entries = walkdir::WalkDir::new(&root)
    .into_iter()
    // Safely handle any filesystem errors by skipping them
    .filter_map(|e| e.ok())
    .collect::<Vec<_>>();

  let page_entries = entries
    .iter()
    .map(|entry| {
      let path = super::direntry_to_string(entry, Some(&root));
      let (name, parent) = super::path_to_name_and_parent(&path);

      PageMetadata {
        is_dir: entry.file_type().is_dir(),
        file_name: name,
        path,
        parent,
      }
    })
    .filter(|metadata| !metadata.is_dir)
    .collect::<Vec<_>>();

  page_entries
}
