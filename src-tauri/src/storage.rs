use crate::case::Case;
use crate::case_file::parse_case;
use crate::error::{AppError, AppResult};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Where the case called `slug` would live inside `dir`. Says nothing about
/// whether anything is there.
pub fn case_path(dir: &Path, slug: &str) -> PathBuf {
    dir.join(format!("{slug}.toml"))
}

/// A case slug is a plain name: ASCII lowercase letters, digits and `-`.
fn is_slug(slug: &str) -> bool {
    todo!()
}

/// Reads the case called `slug` out of `dir` and parses it.
///
/// `CaseNotFound` if the slug is not a plain name or nothing is there, `Io` if
/// the file cannot be read, and whatever `parse_case` says otherwise.
pub fn load_case(dir: &Path, slug: &str) -> AppResult<Case> {
    todo!()
}
