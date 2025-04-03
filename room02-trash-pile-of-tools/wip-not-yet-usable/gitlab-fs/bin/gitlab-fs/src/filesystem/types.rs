use libc::c_int;

// The inode type
pub type Ino = u64;
// The result type of the filesystem (should be a libc error code)
pub type FSResult<T> = Result<T, c_int>;
