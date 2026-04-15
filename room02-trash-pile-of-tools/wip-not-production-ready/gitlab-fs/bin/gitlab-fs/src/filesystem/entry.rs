use std::cmp::Ordering;
use std::path::PathBuf;
use std::time::SystemTime;

use super::types::*;

/// A folder (group of Projects) in Gitlab
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FSFolder {
    /// The path of the folder in the filesystem (E.g. `./root/folder1/folder2`)
    pub path: PathBuf,
    /// All the filesystem inodes of the childs of the folder
    pub inos: Vec<Ino>,
}
impl From<FSFolder> for FSEntry {
    fn from(folder: FSFolder) -> Self {
        Self::Folder(folder)
    }
}

/// A project in Gitlab
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FSProject {
    /// The id of the project in Gitlab
    pub external_id: u32,
    /// The path of the project in the filesystem (E.g. `./root/folder1/folder2/project`)
    pub path: PathBuf,
    /// All the filesystem inodes of the childs of the project (should only be environments)
    pub inos: Vec<Ino>,
}
impl From<FSProject> for FSEntry {
    fn from(project: FSProject) -> Self {
        Self::Project(project)
    }
}

/// An environment in a Gitlab Project
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FSFile {
    /// The path of the file in the filesystem (E.g. `./root/folder1/folder2/project/dev`)
    pub path: PathBuf,
    /// The content of the file (should be UTF-8, but it is only handled as bytes)
    pub content: Vec<u8>,
    /// The last time the file was updated
    pub last_update: SystemTime,
    /// The last time the file was accessed
    pub last_access: SystemTime,
}
impl From<FSFile> for FSEntry {
    fn from(file: FSFile) -> Self {
        Self::File(file)
    }
}

/// Any possible Gitlab entry (Folder, Project, File)
#[derive(Debug, Clone, Eq)]
pub enum FSEntry {
    Folder(FSFolder),
    Project(FSProject),
    File(FSFile),
    /// Represent the root folder that contains each of the folder (the mountpoint)
    Root {
        inos: Vec<Ino>,
    },
}

impl FSEntry {
    pub fn get_path(&self) -> Option<(&PathBuf, &FSEntry)> {
        match self {
            entry @ (FSEntry::File(FSFile { path, .. })
            | FSEntry::Folder(FSFolder { path, .. })
            | FSEntry::Project(FSProject { path, .. })) => Some((path, entry)),
            _ => None,
        }
    }
    pub fn get_children_inos(&self) -> Option<(&Vec<Ino>, &FSEntry)> {
        match self {
            entry @ (FSEntry::Folder(FSFolder { inos, .. })
            | FSEntry::Project(FSProject { inos, .. })
            | FSEntry::Root { inos }) => Some((inos, entry)),
            _ => None,
        }
    }
}

/// Allow to compare entries by their path if possible
impl PartialEq for FSEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // If both objects have a path then compare them
            (
                Self::Folder(FSFolder { path: l_path, .. })
                | Self::Project(FSProject { path: l_path, .. })
                | Self::File(FSFile { path: l_path, .. }),
                Self::Folder(FSFolder { path: r_path, .. })
                | Self::Project(FSProject { path: r_path, .. })
                | Self::File(FSFile { path: r_path, .. }),
            ) => l_path == r_path,
            // If one of the object is a root then it is not equal to any other object
            (Self::Root { .. }, Self::File { .. } | Self::Folder { .. } | Self::Project { .. })
            | (Self::File { .. } | Self::Folder { .. } | Self::Project { .. }, Self::Root { .. }) => {
                false
            }
            // If both objects are root then they are equal (it should not happen, the root is unique)
            (Self::Root { .. }, Self::Root { .. }) => {
                unreachable!("The root must be unique in the FileSystem")
            }
        }
    }
}

/// Order by entry depth if possible
impl PartialOrd for FSEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // If both objects have a path then order them by the depths of their path
            // It allows us to have the root at the top and the deepest path at the bottom
            (
                Self::Folder(FSFolder { path: l_path, .. })
                | Self::Project(FSProject { path: l_path, .. })
                | Self::File(FSFile { path: l_path, .. }),
                Self::Folder(FSFolder { path: r_path, .. })
                | Self::Project(FSProject { path: r_path, .. })
                | Self::File(FSFile { path: r_path, .. }),
            ) => l_path
                .components()
                .count()
                .partial_cmp(&r_path.components().count()),
            // If the first object is a root then it is always less than any other object (except another root)
            (Self::Root { .. }, Self::File { .. } | Self::Folder { .. } | Self::Project { .. }) => {
                Some(Ordering::Less)
            }
            // If the second object is a root then it is always greater than any other object (except another root)
            (Self::File { .. } | Self::Folder { .. } | Self::Project { .. }, Self::Root { .. }) => {
                Some(Ordering::Greater)
            }
            // If both objects are root then they are equal (it should not happen, the root is unique)
            (Self::Root { .. }, Self::Root { .. }) => {
                unreachable!("The root must be unique in the FileSystem")
            }
        }
    }
}
impl Ord for FSEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Workaround because Rust does not allow to implement a trait on a type that is not defined in the same crate
/// So we cannot implement `Vec<FSEntry>` directly and we must use a trait
pub trait FSEntries {
    /// Get a mutable reference to the folders in a `FSEntry::Root` or `FSEntry::Folder` else return an empty vector
    fn get_mut_folders(&mut self) -> Vec<&mut FSFolder>;
    /// Get a mutable reference to the projects in a `FSEntry::Folder` else return an empty vector
    fn get_mut_projects(&mut self) -> Vec<&mut FSProject>;
    /// Get a mutable reference to the files in a `FSEntry::Project` else return an empty vector
    fn get_mut_files(&mut self) -> Vec<&mut FSFile>;
    /// Get the reversed index of an entry (TLDR; the last entry has index 0)
    fn get_reversed_idx(&self, idx: usize) -> usize;
}

impl FSEntries for Vec<FSEntry> {
    fn get_mut_folders(&mut self) -> Vec<&mut FSFolder> {
        self.iter_mut()
            .filter_map(|e| match e {
                FSEntry::Folder(f) => Some(f),
                _ => None,
            })
            .collect()
    }
    fn get_mut_projects(&mut self) -> Vec<&mut FSProject> {
        self.iter_mut()
            .filter_map(|e| match e {
                FSEntry::Project(p) => Some(p),
                _ => None,
            })
            .collect()
    }
    fn get_mut_files(&mut self) -> Vec<&mut FSFile> {
        self.iter_mut()
            .filter_map(|e| match e {
                FSEntry::File(f) => Some(f),
                _ => None,
            })
            .collect()
    }
    fn get_reversed_idx(&self, idx: usize) -> usize {
        self.len() - idx - 1
    }
}
