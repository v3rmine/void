// Source: https://github.com/wfraser/fuse-mt/blob/master/example/src/passthrough.rs

use std::{
    ffi::OsStr,
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use fuser::{FileAttr, FileType, Filesystem};
use gitlab::Gitlab;
use if_chain::if_chain;
use log_utils::{debug, info, trace, tracing, warn};
use parking_lot::RwLock;

use crate::project::Project;

mod constants;
mod entry;
mod folders;
mod types;

use constants::*;
use entry::*;
use types::*;

#[derive(Debug)]
pub struct GitlabFS {
    /// The Gitlab client
    client: Arc<Gitlab>,
    /// The virtual filesystem state
    vfs: Arc<RwLock<Vec<FSEntry>>>,
    /// The Gitlab query to filter projects
    query: String,
}

impl GitlabFS {
    /// Create a new GitlabFS filesystem
    pub fn new(host: String, token: String, query: String) -> Self {
        Self {
            client: Arc::new(Gitlab::new(host, token).expect("Failed to initialize Gitlab client")),
            vfs: Arc::new(RwLock::new(Vec::default())),
            query,
        }
    }

    /// Convert an index in the filesystem to an inode
    fn idx_to_ino(idx: usize) -> Ino {
        (idx + 1) as Ino
    }

    /// Convert an inode to an index in the filesystem
    fn ino_to_idx(ino: Ino) -> usize {
        (ino - 1) as usize
    }

    /// Convert a filesystem entry to a FUSE FileAttr response
    fn entry_as_fileattr(request: &fuser::Request<'_>, entry: &FSEntry, ino: u64) -> FileAttr {
        // It can only be a folder or a file
        match entry {
            FSEntry::Folder { .. } | FSEntry::Project { .. } | FSEntry::Root { .. } => FileAttr {
                ino,
                // A folder standard size is 4096 bytes but it does not matter
                size: 4096,
                blocks: 0,
                // We set the time to UNIX_EPOCH because it is not used
                atime: UNIX_EPOCH,
                ctime: UNIX_EPOCH,
                crtime: UNIX_EPOCH,
                mtime: UNIX_EPOCH,
                kind: FileType::Directory,
                perm: 0o755,
                nlink: 0,
                // We copy the user id and group so he always have access
                uid: request.uid(),
                gid: request.gid(),
                // NOTE: Not used, only for special files
                rdev: 0,
                // NOTE: Only applicable on MacOS see <https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chflags.2.html>
                flags: 0,
                blksize: 8,
            },
            FSEntry::File(FSFile {
                content,
                last_update,
                last_access,
                ..
            }) => {
                let file_len = content.len() as u64;
                FileAttr {
                    ino,
                    size: file_len,
                    blocks: (file_len - file_len % 8) / 8 + (if file_len % 8 > 1 { 1 } else { 0 }),
                    atime: *last_access,
                    ctime: *last_update,
                    crtime: UNIX_EPOCH,
                    mtime: *last_update,
                    kind: FileType::RegularFile,
                    perm: 0o644,
                    nlink: 0,
                    // We copy the user id and group so he always have access
                    uid: request.uid(),
                    gid: request.gid(),
                    // NOTE: Not used, only for special files
                    rdev: 0,
                    // NOTE: Only applicable on MacOS see <https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chflags.2.html>
                    flags: 0,
                    blksize: 8,
                }
            }
        }
    }

    /// Generate a vector of filesystem entries from the list of projects
    fn generate_fs_entries_from_projects(projects: Vec<Project>) -> FSResult<Vec<FSEntry>> {
        // We generate a list of projects and their parent folder
        let projects = projects
            .iter()
            .map(|p| {
                let path = PathBuf::from(&p.full_path);
                let parent = path.parent().map(|p| p.to_path_buf());
                (
                    FSProject {
                        external_id: p.id,
                        path,
                        inos: Vec::new(),
                    },
                    parent,
                )
            })
            .collect::<Vec<_>>();
        trace!("Generated project list of {} projects", projects.len());

        // We split the projects and their folders
        let projects = projects.into_iter().map(|(p, _)| p).collect::<Vec<_>>();
        let folders = folders::generate_folders_from_projects(&projects);
        info!(
            "Generated project parent folders list of {} folder",
            folders.len()
        );
        let root = FSEntry::Root { inos: Vec::new() };

        let results = [
            vec![root],
            folders.into_iter().map(FSEntry::from).collect::<Vec<_>>(),
            projects.into_iter().map(FSEntry::from).collect::<Vec<_>>(),
        ]
        .concat();

        let results = folders::generate_ino_from_fs_entries(&results);
        debug!(filesystem_state =? results, "Filesystem generated from projects");

        // REVIEW: Env variables are loaded on access to each project directory

        Ok(results)
    }

    // WIP: Add entry to the filesystem
    /*fn add_entry(&mut self, entry: FSEntry) -> Ino {
        let mut vfs = self.vfs.write();
        let idx = vfs.len();
        vfs.push(entry);
        return Self::idx_to_ino(idx);
    }*/
}

impl Filesystem for GitlabFS {
    /// Initialise the filesystem to its default state
    #[tracing::instrument(skip(self, req, config))]
    fn init(&mut self, req: &fuser::Request<'_>, config: &mut fuser::KernelConfig) -> FSResult<()> {
        trace!(request =? req, config =? config, "init FS");

        let projects =
            crate::project::get_projects(&self.client, &self.query).map_err(|_err| EAGAIN)?;
        let fs_entries = Self::generate_fs_entries_from_projects(projects)?;

        let mut vfs = self.vfs.write();
        *vfs = fs_entries;

        Ok(())
    }

    /// The "getattr" syscall in FUSE (Filesystem in Userspace)
    /// is used to retrieve the attributes of a specific file or directory
    /// in a FUSE VFS.
    /// It takes the file path as an input and returns information
    /// such as file size, permissions, timestamps,
    /// and other metadata associated with the file.
    /// This syscall is essential for obtaining information about files and directories.
    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn getattr(&mut self, req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        trace!(request =? req, "getattr of ino {ino}");
        if let Some(fs_entry) = { self.vfs.read().get(Self::ino_to_idx(ino)) } {
            reply.attr(&TTL, &Self::entry_as_fileattr(req, fs_entry, ino));
        } else {
            reply.error(ENOENT);
        }
    }

    /// The "readdir" syscall in FUSE (Filesystem in Userspace)
    /// is used to retrieve a list of directory entries
    /// within a specific directory in a FUSE VFS.
    /// It takes the file handle and an offset as inputs
    /// and returns a list of directory entries along with their attributes.
    /// This syscall is crucial for listing and traversing directories.
    #[allow(unused)]
    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn readdir(
        &mut self,
        req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        trace!(request =? req, "readdir of ino {ino}");
        let mut err = None;
        let mut self_idx = Self::ino_to_idx(ino);

        {
            // Here we borrow vfs as write till the end of the block
            let mut vfs = self.vfs.write();
            // TODO: Add a method on the filesystem to add new files to it
            let mut next_idx: usize = vfs.len();
            let mut file_buffer = Vec::new();
            let mut self_entry = {
                if let Some(entry) = vfs.get_mut(self_idx) {
                    entry
                } else {
                    debug!("Entry not found {ino}");
                    reply.error(ENOENT);
                    return;
                }
            };

            if_chain! {
                // Only if the entry is a project
                if let FSEntry::Project(FSProject {
                    external_id, path, inos,
                }) = self_entry;
                if inos.is_empty(); // If there is no inodes, we havent got the env variables of the project
                then {
                    let path = path.clone();
                    info!(external_id, ?path, "Listing project");
                    let project_variables = crate::project_env::variables_to_env(
                        crate::project_env::get_project_env(&self.client, *external_id),
                    );
                    for (env, env_file) in project_variables {
                        file_buffer.push(FSEntry::File(FSFile {
                            path: path.join(env),
                            content: env_file.as_bytes().to_vec(),
                            last_update: SystemTime::now(),
                            last_access: SystemTime::now(),
                        }));
                        // TODO: Move this to a "add" method on the filesystem
                        inos.push(Self::idx_to_ino(next_idx));
                        next_idx += 1;
                    }
                }
            }

            for file in file_buffer {
                vfs.push(file);
            }

            // Here the writable borrow to vfs is dropped
        }

        // Here we borrow vfs as read till the end of the function
        let vfs = self.vfs.read();
        // Only if the entry is a Folder, a Project or the Root
        if let Some((inos, self_entry)) = { vfs.get(self_idx).and_then(|e| e.get_children_inos()) }
        {
            // We iterate on the children
            for (idx, child_ino) in inos.iter().enumerate().skip(offset as usize) {
                if let Some((path, entry)) = {
                    vfs.get(Self::ino_to_idx(*child_ino))
                        .and_then(|e| e.get_path())
                } {
                    let attrs = Self::entry_as_fileattr(req, entry, *child_ino);
                    if_chain! {
                        if let Some(name) = path.file_name();
                        let _ = trace!(name =? name, "Child found");
                        if reply.add(*child_ino, (idx + 1) as i64, attrs.kind, name);
                        then {
                            break;
                        }
                    }
                } else {
                    trace!(ino = child_ino, parent = ino, "Child not found");
                }
            }

            if let Some(err) = err {
                reply.error(err);
            } else {
                reply.ok();
            }
        }
    }

    /// The "lookup" syscall in FUSE (Filesystem in Userspace) is used to find
    /// and retrieve information about a specific file or directory
    /// in a FUSE VFS.
    /// It takes the name of the file or directory as an input and
    /// returns the attributes and file handle associated with it.
    /// This syscall is essential for navigating and accessing files.
    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn lookup(
        &mut self,
        req: &fuser::Request<'_>,
        parent_ino: u64,
        name: &OsStr,
        reply: fuser::ReplyEntry,
    ) {
        trace!(request =? req, parent = parent_ino, "loopup of name {:?}", name);

        let vfs = self.vfs.read();

        if_chain! {
            if let Some(target_file_name) = name.to_str();
            if let Some((parent_inos, _))
                = { vfs.get(Self::ino_to_idx(parent_ino)).and_then(|e| e.get_children_inos()) };
            then {
                for child_ino in parent_inos.iter() {
                    if_chain! {
                        if let Some(
                            (path, fs_entry),
                        ) = { vfs.get(Self::ino_to_idx(*child_ino)).and_then(|e| e.get_path()) };
                        if let Some(found_file_name) = path.file_name();
                        if found_file_name == target_file_name;
                        then {
                            let attrs = Self::entry_as_fileattr(req, fs_entry, *child_ino);
                            debug!(folder =? attrs, "Lookup folder");
                            reply.entry(&TTL,  &attrs, 0);
                            return;
                        }
                    }
                }
            }
        }

        reply.error(ENOENT);
    }

    /// The read function in a FUSE (Filesystem in Userspace) implementation\
    /// is a callback function responsible for handling read operations
    /// on files. It is invoked when a process attempts to read data
    /// from a file. The read function takes parameters such as the file path,
    /// buffer for storing the read data,
    /// maximum size to read, offset within the file,
    /// and additional file information.
    /// Its implementation retrieves the requested data from the underlying storage
    /// or data source associated with the file and returns it to the caller.
    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn read(
        &mut self,
        req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,                 // The file handle identify and track open files
        offset: i64,             // Where to start reading in the file
        size: u32,               // How much data we want to read
        flags: i32, // Integer value that can be used to indicate various file access modes and behavior
        lock_owner: Option<u64>, // Lock owner id. Available in locking operations and flush
        reply: fuser::ReplyData,
    ) {
        trace!(request =? req, "read of file at ino {ino}");

        let mut vfs = self.vfs.write();
        if let Some(FSEntry::File(FSFile {
            content,
            last_access,
            ..
        })) = vfs.get_mut(Self::ino_to_idx(ino))
        {
            *last_access = SystemTime::now();
            if let Some(end_offset) = offset.checked_add_unsigned(size as u64) {
                info!(content =? String::from_utf8(content.clone()), "Reading from {offset} to {end_offset}");
                if (end_offset as usize) > content.len() {
                    reply.data(&content[offset as usize..]);
                } else {
                    reply.data(&content[offset as usize..(end_offset as usize)]);
                }
            }
        } else {
            reply.error(ENOENT);
        }
    }

    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn open(&mut self, req: &fuser::Request<'_>, ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
        trace!(request =? req, "open file at ino {ino}");
        info!("Open do nothing because the file access is not handled manually");

        let vfs = self.vfs.read();
        if let Some(FSEntry::File(FSFile { .. })) = vfs.get(Self::ino_to_idx(ino)) {
            reply.opened(0, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn write(
        &mut self,
        req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,     // The file handle identify and track open files
        offset: i64, // Where to start writing in the file
        data: &[u8],
        write_flags: u32,        // ? Same as flags
        flags: i32,              // ? Same as write_flags
        lock_owner: Option<u64>, // Lock owner id. Available in locking operations and flush
        reply: fuser::ReplyWrite,
    ) {
        trace!(request =? req, "write of {} bytes in file at ino {ino}", data.len());

        let mut vfs = self.vfs.write();
        if let Some(FSEntry::File(FSFile {
            content,
            last_update,
            ..
        })) = vfs.get_mut(Self::ino_to_idx(ino))
        {
            info!(content =? content.clone(), "Writing {} bytes at {offset}", data.len());

            let max_len = (offset as usize) + data.len();
            if max_len > content.len() {
                content.resize(max_len, 0);
            }
            content[(offset as usize)..max_len].copy_from_slice(data);

            *last_update = SystemTime::now();
            reply.written(data.len() as u32);
        } else {
            reply.error(ENOENT);
        }
    }

    #[tracing::instrument(level = "warn", skip(self, req, reply))]
    fn create(
        &mut self,
        req: &fuser::Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        warn!(request =? req, "create file under parent ino {parent}");

        let mut vfs = self.vfs.write();
        let child_ino = Self::idx_to_ino(vfs.len());
        let mut child_path = PathBuf::new();
        if let Some(FSEntry::Project(FSProject { inos, path, .. })) =
            vfs.get_mut(Self::ino_to_idx(parent))
        {
            child_path.push(path);
            inos.push(child_ino);
        } else {
            reply.error(ENOENT);
            return;
        }


        child_path.push(name);
        let entry = FSEntry::File(FSFile {
            path: child_path,
            content: Vec::new(),
            last_update: SystemTime::now(),
            last_access: SystemTime::now(),
        });
        vfs.push(entry.clone());
        reply.created(
            &TTL,
            &Self::entry_as_fileattr(req, &entry, child_ino),
            child_ino,
            0,
            0,
        );
    }

    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn fsync(
        &mut self,
        req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        _datasync: bool,
        reply: fuser::ReplyEmpty,
    ) {
        trace!(request =? req, "fsync file at ino {ino}");

        let vfs = self.vfs.read();
        if let Some(FSEntry::File(FSFile { .. })) = vfs.get(Self::ino_to_idx(ino)) {
            // TODO: Time to write content online
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }

    #[tracing::instrument(level = "debug", skip(self, req, reply))]
    fn setattr(
            &mut self,
            req: &fuser::Request<'_>,
            ino: u64,
            _mode: Option<u32>,
            _uid: Option<u32>,
            _gid: Option<u32>,
            _size: Option<u64>,
            atime: Option<fuser::TimeOrNow>,
            mtime: Option<fuser::TimeOrNow>,
            _ctime: Option<SystemTime>,
            _fh: Option<u64>,
            _crtime: Option<SystemTime>,
            _chgtime: Option<SystemTime>,
            _bkuptime: Option<SystemTime>,
            _flags: Option<u32>,
            reply: fuser::ReplyAttr,
        ) {
        trace!(request =? req, "setattr file at ino {ino}");

        let mut vfs = self.vfs.write();
        if let Some(FSEntry::File(FSFile { last_update, last_access, .. })) = vfs.get_mut(Self::ino_to_idx(ino)) {
            match atime {
                Some(fuser::TimeOrNow::Now) => {
                    *last_access = SystemTime::now();
                },
                Some(fuser::TimeOrNow::SpecificTime(time)) => {
                    *last_access = time;
                },
                None => ()
            }

            match mtime {
                Some(fuser::TimeOrNow::Now) => {
                    *last_update = SystemTime::now();
                },
                Some(fuser::TimeOrNow::SpecificTime(time)) => {
                    *last_update = time;
                },
                None => ()
            }
        } else {
            reply.error(ENOENT);
            return;
        }

        reply.attr(&TTL, &Self::entry_as_fileattr(&req, vfs.get(Self::ino_to_idx(ino)).unwrap(), ino))
    }
}
