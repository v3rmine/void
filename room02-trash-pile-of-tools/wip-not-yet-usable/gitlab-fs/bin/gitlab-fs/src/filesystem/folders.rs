use std::path::PathBuf;

use log_utils::{info, trace, tracing};

use super::{
    entry::{FSEntries, FSFolder, FSProject},
    FSEntry, GitlabFS,
};

/// Generate the folders from flat project list of Gitlab
#[tracing::instrument(level = "debug")]
pub fn generate_folders_from_projects(projects: &[FSProject]) -> Vec<FSFolder> {
    // Get the list of parents of the projects so we only have folders
    let projects_parents = projects.iter().filter_map(|p| p.path.parent());
    // Get all the parents of the parents (ancestors) to recreate the arborecence
    let mut expanded_parents = projects_parents
        .flat_map(|parent| parent.ancestors().into_iter().map(|a| a.as_os_str()))
        .collect::<Vec<_>>();

    // Sort and remove duplicates
    expanded_parents.sort();
    expanded_parents.dedup();
    // Remove the root folder
    expanded_parents.remove(0);

    info!("Expanded to {} parents", expanded_parents.len());
    trace!(parents =? expanded_parents, "Generated parents folders list");

    // Create all the folders
    expanded_parents
        .into_iter()
        .map(|p| FSFolder {
            path: PathBuf::from(p),
            inos: Vec::new(),
        })
        .collect::<Vec<_>>()
}

/// Fill all entries children inodes
pub fn generate_ino_from_fs_entries(fs_entries: &[FSEntry]) -> Vec<FSEntry> {
    let mut fs_entries = fs_entries.to_vec();
    fs_entries.sort();
    fs_entries.reverse();
    // Here we have the root at the end of the vector and the longest path at the beginning
    let mut results = fs_entries.clone();

    for (idx, fs_entry) in fs_entries.iter().enumerate() {
        // We convert the index to an inode
        let fs_entry_ino = GitlabFS::idx_to_ino(fs_entries.get_reversed_idx(idx));

        // If the entry is a project or a folder then we add the inode to the root or the parent folder
        if let FSEntry::Project(FSProject { path, .. }) | FSEntry::Folder(FSFolder { path, .. }) =
            fs_entry
        {
            trace!(path =? path, "Computing childrens of projects and folders");

            // If the path has only one component (E.g "/") then it is the root
            if path.components().count() == 1 {
                // We know that the root is the last element of the vector
                // So we can add the inode to the root childrens
                match results.get_mut(fs_entries.len() - 1) {
                    Some(FSEntry::Root {
                        inos: ref mut root_inos,
                    }) => {
                        root_inos.push(fs_entry_ino);
                    }
                    Some(_) => {
                        unreachable!("The root must be the last element of the filesystem entries")
                    }
                    None => unreachable!("No root entry found"),
                }
            } else {
                // Else we add the inode to the parent folder
                for folder in results.get_mut_folders() {
                    if folder.path.eq(path.parent().unwrap()) {
                        folder.inos.push(fs_entry_ino);
                        break;
                    }
                }
            }
        }
    }

    // We need to reverse the vector again to have the root at the beginning
    results.reverse();
    results
}
