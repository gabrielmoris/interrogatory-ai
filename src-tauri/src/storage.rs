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
    slug.bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'-'))
        && !slug.is_empty()
}

/// Reads the case called `slug` out of `dir` and parses it.
///
/// `CaseNotFound` if the slug is not a plain name or nothing is there, `Io` if
/// the file cannot be read, and whatever `parse_case` says otherwise.
pub fn load_case(dir: &Path, slug: &str) -> AppResult<Case> {
    if !is_slug(slug) {
        return Err(AppError::CaseNotFound {
            slug: slug.to_string(),
        });
    }

    let path = case_path(dir, slug);

    let text = fs::read_to_string(&path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => AppError::CaseNotFound {
            slug: slug.to_string(),
        },
        _ => AppError::Io {
            path: path.display().to_string(),
            message: e.to_string(),
        },
    })?;

    parse_case(&text, &path.display().to_string())
}
