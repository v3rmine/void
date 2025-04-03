use std::path::PathBuf;

use glob::Pattern;
use indicatif::ProgressBar;
use walkdir::{DirEntry, WalkDir};

use crate::errors::StandardResult;
use crate::utils::terminal::alert;
use crate::{TEMPLATE_DIR_NAME, TEMPLATE_IGNORE_FILE};

pub fn scan_dir(template_dir: &PathBuf) -> StandardResult<(Vec<DirEntry>, Vec<DirEntry>)> {
    let mut folders = Vec::new();
    let mut files = Vec::new();
    let rules = generate_ignore_rules(template_dir);

    let walkdir_iter = WalkDir::new(template_dir.join(TEMPLATE_DIR_NAME))
        .follow_links(true)
        .into_iter()
        .filter_entry(|d| filters(d, &rules, template_dir));

    let progress = ProgressBar::new_spinner();
    progress.set_message("[1/4] Scanning files and folders in template...");

    for entry in progress.wrap_iter(walkdir_iter) {
        match entry {
            Ok(e) => {
                if e.path().is_file() {
                    files.push(e);
                } else if e.path() != template_dir.join(TEMPLATE_DIR_NAME) {
                    folders.push(e);
                }
            }
            Err(e) => return Err(e.into()),
        };
    }

    progress.finish_and_clear();
    Ok((folders, files))
}

fn filters(entry: &DirEntry, ignore_rules: &[Pattern], base_path: &PathBuf) -> bool {
    for rule in ignore_rules {
        if let Ok(stripped_path) = entry.path().strip_prefix(base_path.join(TEMPLATE_DIR_NAME)) {
            if rule.matches_path(stripped_path) {
                return false;
            }
        }
    }
    true
}

fn generate_ignore_rules(template_dir: &PathBuf) -> Vec<Pattern> {
    let mut ignore_rules = Vec::new();
    if let Ok(f) = std::fs::read_to_string(template_dir.join(TEMPLATE_IGNORE_FILE)) {
        for line in f.lines() {
            match Pattern::new(line) {
                Ok(p) => ignore_rules.push(p),
                Err(_) => alert(&format!(
                    "\"{}\" in {} is not a valid unix pattern",
                    line, TEMPLATE_IGNORE_FILE
                )),
            };
        }
    }

    ignore_rules
}
