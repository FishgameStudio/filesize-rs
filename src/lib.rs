//! # filesize-rs
//! A lightweight library without complex dependencies.
//! Just process size of a file or a directory, and convert to
//! human-readable format.

use core::result;
use std::error::Error;
use std::fs::{metadata, read_dir};
use std::path::PathBuf;
use std::{fmt, io};

/// Bytes of 1 KiB.
/// 1 KiB equals 1024 Bytes.
pub const KIB: u64 = 1024;
/// Bytes of 1 MiB.
/// 1 MiB equals 1024 [`KiB`](KIB).
pub const MIB: u64 = KIB * 1024;
/// Bytes of 1 GiB.
/// 1 GiB equals 1024 [`MiB`](MIB).
pub const GIB: u64 = MIB * 1024;
/// Bytes of 1 TiB.
/// 1 TiB equals 1024 [`GiB`](GIB).
pub const TIB: u64 = GIB * 1024;
/// Bytes of 1 PiB.
/// 1 PiB equals 1024 [`TiB`](TIB).
pub const PIB: u64 = TIB * 1024;
/// Bytes of 1 EiB.
/// 1 EiB equals 1024 [`PiB`](PIB).
pub const EIB: u64 = PIB * 1024;

/// An enumeration of errors during getting size of a file or a directory.
#[derive(Debug)]
pub enum SizeError {
    Io(io::Error),
    InvalidPath,
}

impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO failure: {}", e),
            Self::InvalidPath => write!(f, "path is not a file or directory"),
        }
    }
}

impl Error for SizeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::InvalidPath => None,
        }
    }
}

/// Compat `?` operator to automatically cast [`io::Error`] to [`SizeError`].
impl From<io::Error> for SizeError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// Generic [`Result`](crate::Result) for this crate.
/// Throwes [`SizeError`] if returned a [`Err`] enumeration.
pub type Result<T> = result::Result<T, SizeError>;

/// Get size from a valid path. The path can be a file or a directory.
/// If the path is a directory, it will statstic all files in the directory
/// and returns a totally size.
/// # Example
/// ```
/// use filesize_rs::{get_size_bytes, Result};
/// fn main() {
///     match get_size_bytes("a.txt") {
///         Ok(size) => println!("Size: {size} bytes"),
///         Err(e) => eprintln!("Unable to get size: {e}"),
///     }
/// }
/// ```
/// # Errors
/// Returns a [`Err`] value if failure during getting metadata from a file.
/// Or the path is invalid.
pub fn get_size_bytes<P>(path: P) -> Result<u64>
where
    P: Into<PathBuf>,
{
    let path = path.into();
    if path.is_file() {
        Ok(metadata(&path)?.len())
    } else if path.is_dir() {
        let mut total: u64 = 0u64;
        let entries = read_dir(&path)?;
        for entry in entries {
            let entry = entry?;
            total += get_size_bytes(entry.path())?;
        }
        Ok(total)
    } else {
        Err(SizeError::InvalidPath)
    }
}

/// Generate a pretty string contains size informations.
/// # Example
/// ```
/// use filesize_rs::to_size_pretty;
/// fn main() {
///     let bytes = to_size_pretty(123);
///     let kib = to_size_pretty(2048);
///     assert_eq!(bytes, "123 Bytes");
///     assert_eq!(kib, "2 KiB");
/// }
/// ```
pub fn to_size_pretty(size: u64) -> String {
    match size {
        ..KIB => format!("{size} Bytes"),
        KIB..MIB => format!("{} KiB", size as f64 / KIB as f64),
        MIB..GIB => format!("{:.2} MiB", size as f64 / MIB as f64),
        GIB..TIB => format!("{:.2} GiB", size as f64 / GIB as f64),
        TIB..PIB => format!("{:.2} TiB", size as f64 / TIB as f64),
        PIB..EIB => format!("{:.2} PiB", size as f64 / PIB as f64),
        EIB.. => format!("{:.2} EiB", size as f64 / EIB as f64),
    }
}

/// Get size from a valid path, and then format it prettily.
/// This function is a wrap of [`get_size_bytes`] and [`to_size_pretty`]
pub fn get_size_pretty<P>(path: P) -> Result<String>
where
    P: Into<PathBuf>,
{
    Ok(to_size_pretty(get_size_bytes(path)?))
}

/// Unit tests
#[cfg(test)]
mod tests {
    use crate::get_size_bytes;
    use crate::get_size_pretty;
    use crate::to_size_pretty;

    #[test]
    fn test_to_string() {
        let zero = to_size_pretty(0);
        let bytes = to_size_pretty(123);
        let kib = to_size_pretty(2048);
        assert_eq!(zero, "0 Bytes");
        assert_eq!(bytes, "123 Bytes");
        assert_eq!(kib, "2 KiB");
    }

    #[test]
    fn test_get_size() {
        get_size_bytes("NONEXIST_FILE.txt").expect_err("File should be nonexist");
    }

    #[test]
    fn test_get_size_pretty() {
        get_size_pretty("NONEXIST_FILE.txt").expect_err("File should be nonexist");
    }
}
