//! File access operations.

use std::fs;
use std::path;

use super::error;
use super::unix;

/// Opens a file for read, but returns it only if its permissions and mode match
/// some seriously pedantic checks.  Otherwise, the file is immediately closed.
///
/// On success, returns the file along with some of the metadata retrieved
/// during the checks, as a useful side effect.
///
/// Analog of djb's `file_open` from `file.c`.
pub fn safe_open<P>(path: P) -> error::Result<FileOrDir>
where
    P: AsRef<path::Path>,
{
    let f = fs::File::open(path)?;
    let s = unix::fstat(&f)?;

    if (s.st_mode & 0o444) != 0o444 {
        Err(error::HttpError::NotFound(b"not ugo+r"))
    } else if (s.st_mode & 0o101) == 0o001 {
        Err(error::HttpError::NotFound(b"o+x but u-x"))
    } else if (s.st_mode & libc::S_IFMT) == libc::S_IFDIR {
        Ok(FileOrDir::Dir)
    } else if (s.st_mode & libc::S_IFMT) == libc::S_IFREG {
        Ok(FileOrDir::File(OpenFile {
            file: f,
            mtime: time::Timespec {
                sec: s.st_mtime,
                nsec: 0,
            },
            length: s.st_size as u64,
        }))
    } else {
        Err(error::HttpError::NotFound(b"not a regular file"))
    }
}

/// Used to represent the result of opening a path, which might have turned out
/// to be a directory.
pub enum FileOrDir {
    File(OpenFile),
    Dir,
}

/// Result type for `safe_open`.
pub struct OpenFile {
    /// The opened file.
    pub file: fs::File,
    /// The file's modification time in seconds since the epoch, at the last time
    /// we checked.
    pub mtime: time::Timespec,
    /// The file's length, at the last time we checked.  Note that this may change
    /// at runtime; take care.
    pub length: u64,
}
