use std::{
    collections::{HashMap, HashSet},
    fs,
};

use loco_rs::prelude::*;
use serde::Serialize;

use super::parse_frontmatter;
use crate::{common::AppSettings, utils};

#[derive(Debug, Clone, Serialize)]
pub struct ContentMetadata {
    pub is_dir: bool,
    pub file_name: String,
    pub path: String,
    pub lang: String,
    pub parent: String,
    pub basename: String,
    pub slug: String,
    pub unique_name: String,
    pub route_path: String,
    pub rendered_content: Option<String>,
    pub data: serde_json::Value,
}

pub fn list_content(settings: &AppSettings) -> Vec<ContentMetadata> {
    let root = &settings.contents_dir;
    let entries = walkdir::WalkDir::new(root)
        .into_iter()
        // Safely handle any filesystem errors by skipping them
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    let mut parent_contexts: HashMap<String, serde_json::Value> = HashMap::new();

    let mut content_entries = entries
        .iter()
        .filter_map(|entry| {
            let path = super::direntry_to_string(entry, Some(root));
            if path.contains("/_") {
                return None;
            }

            let (file_name, parent) = super::path_to_name_and_parent(&path);
            let is_dir = entry.file_type().is_dir();

            // Read the folder metadata if it exists
            let folder_data = if is_dir {
                let data_path = entry.path().join("_data.toml");
                if data_path.exists() {
                    fs::read_to_string(&data_path)
                        .ok()
                        .and_then(|content| toml::from_str::<serde_json::Value>(&content).ok())
                } else {
                    None // No _data.toml file exists
                }
            } else {
                None // Not a directory
            };

            let empty_data = data!({});
            let parent_data = parent_contexts.get(&parent).unwrap_or_else(|| &empty_data);

            // Merge parent inherited metadata with the folder metadata (if any)
            let data = if is_dir {
                let folder_data = folder_data.as_ref().unwrap_or_else(|| &empty_data);

                // If folder_data has basename return it else get it from parent_data
                let folder_data = if folder_data.get("basename").is_some() {
                    folder_data.clone()
                } else {
                    // Define the basename for the childrens by joining the parent's basename with the folder's name
                    let augmented_basename = [
                        parent_data
                            .get("basename")
                            .and_then(|v| v.as_str())
                            // if the parent's basename is "/" then set it to an empty string (join will add the /)
                            .map(|v| if v == "/" { "" } else { v })
                            .unwrap_or_else(|| ""),
                        // if the folder's name is "/" then set it to an empty string (join will add the /)
                        if file_name == "/" { "" } else { &file_name },
                    ]
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join("/");

                    super::merge_json_values(
                        folder_data,
                        &data!({ "basename": augmented_basename }),
                    )
                };

                let merged_data = super::merge_json_values(parent_data, &folder_data);

                // Add all the inherited metadata to the context
                parent_contexts.insert(path.clone(), merged_data.clone());

                merged_data
            } else {
                if file_name.ends_with(".md") {
                    let content = fs::read_to_string(&entry.path()).unwrap();
                    let frontmatter = parse_frontmatter::<tera::Value>(&content).unwrap();

                    super::merge_json_values(parent_data, &frontmatter)
                } else {
                    parent_data.clone()
                }
            };

            if file_name == "_data.toml" {
                return None;
            };

            let file_name_parts = (&file_name).split('.').collect::<Vec<_>>();
            let name = file_name_parts[0];
            let lang = if file_name_parts.len() >= 3 {
                file_name_parts[1].to_string()
            } else {
                settings.default_language.clone()
            };

            let slug = super::slugify(name);
            let basename = data
                .get("basename")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            // Special case for index: map to root path "/" instead of "/index"
            let route_path = format!("{}/{}", basename, slug);

            // Add language prefix to route path if not default language
            let route_path = if lang != settings.default_language {
                format!("/{}{}", lang, route_path)
            } else {
                route_path
            };

            Some(ContentMetadata {
                is_dir,
                slug,
                lang,
                file_name,
                path,
                parent,
                route_path,
                unique_name: "toreplace".to_string(),
                basename,
                rendered_content: None,
                data,
            })
        })
        .filter(|metadata| !metadata.is_dir)
        .collect::<Vec<_>>();

    let mut unique_names: HashSet<&str> = HashSet::new();
    for entry in content_entries.iter_mut() {
        let slug = entry.slug.clone();
        let mut name = if entry.lang == settings.default_language {
            slug.clone()
        } else {
            format!("{slug}[{}]", entry.lang)
        };
        let mut inc = 1;

        while unique_names.contains(name.as_str()) {
            name = format!("{slug}-{inc}");
            inc += 1;
        }

        entry.unique_name = name.to_owned();
        unique_names.insert(&entry.unique_name);
    }

    content_entries
}

pub fn render_contents(
    settings: &AppSettings,
    contents: Vec<ContentMetadata>,
) -> Vec<ContentMetadata> {
    let root = &settings.contents_dir;
    let markdown_parser = utils::new_markdown_parser();

    contents
        .into_iter()
        .map(|mut entry| {
            if !entry.is_dir && entry.file_name.ends_with(".md") {
                let Ok(content) = fs::read_to_string(&format!("{root}/{}", entry.path)) else {
                    return entry;
                };
                let content_without_frontmatter =
                    content.splitn(3, "---").last().unwrap_or_default();
                let rendered_content = markdown_parser.parse(&content_without_frontmatter).render();
                entry.rendered_content = Some(rendered_content);
            }

            entry
        })
        .collect::<Vec<_>>()
}
