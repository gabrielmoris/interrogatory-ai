//! Stage 8 — the case comes off the disk
//!
//! Run with:  cargo test --test storage      (from src-tauri/)
//!
//! Everything up to here worked on text somebody else had already fetched:
//! `parse_case(text, path)` takes a `path` that is only a label. This file is
//! where the label becomes a real location and the text comes from a real file.
//!
//! That makes `storage.rs` the first *shell* module — the first place in the
//! crate allowed to touch the filesystem. Two consequences the tests below are
//! about: the outside world fails in ways of its own (a missing file, a file
//! that is not text), and the argument naming the case arrives from outside and
//! is not to be trusted with a path.

use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::storage::{case_path, load_case};
use std::fs;
use std::path::{Path, PathBuf};

const THE_LEDGER: &str = include_str!("cases/the-ledger.toml");

/// `src-tauri/tests/cases` — the two real case files, wherever this repo sits.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("cases")
}

/// A fresh empty directory of our own, for the files we want to be wrong.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("interrogator-stage-08")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

#[test]
fn case_path_is_the_slug_with_a_toml_extension() {
    let path = case_path(Path::new("cases"), "the-ledger");

    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("the-ledger.toml")
    );
    assert_eq!(path.parent(), Some(Path::new("cases")));
}

#[test]
fn a_real_case_file_comes_off_the_disk() {
    let case =
        load_case(&fixtures(), "the-ledger").expect("the-ledger.toml is there, and is a case");

    assert_eq!(case.title, "The Ledger");
    assert_eq!(case.suspect_count(), 2);
    assert_eq!(case.fact_count(), 4);
}

#[test]
fn the_other_case_file_loads_too() {
    let case = load_case(&fixtures(), "the-lighthouse").expect("the-lighthouse.toml is a case");

    assert_eq!(case.title, "The Lighthouse");
    assert_eq!(case.suspect_count(), 3);
    assert_eq!(case.fact_count(), 5);
}

#[test]
fn a_slug_with_no_file_behind_it_is_case_not_found() {
    // Not an `Io` failure. "There is no such case" is a thing the app knows how
    // to say, and React branches on it — Stage 5's `CaseNotFound`.
    assert_eq!(
        load_case(&fixtures(), "the-missing-hour"),
        Err(AppError::CaseNotFound {
            slug: "the-missing-hour".to_string()
        })
    );
}

#[test]
fn a_file_that_is_not_text_is_an_io_failure() {
    let dir = scratch("not-text");
    fs::write(dir.join("garbled.toml"), [0xff, 0xfe, 0x00, 0x9f]).expect("a file of junk bytes");

    // The file is there, so this is not `CaseNotFound`. It cannot be read as
    // text, so it never reaches the parser either.
    match load_case(&dir, "garbled") {
        Err(AppError::Io { path, message }) => {
            assert!(
                path.contains("garbled.toml"),
                "the error names the file: {path}"
            );
            assert!(
                !message.is_empty(),
                "the operating system's own complaint is kept"
            );
        }
        other => panic!("expected AppError::Io, got {other:?}"),
    }
}

#[test]
fn a_file_that_is_not_toml_is_a_parse_failure_naming_the_file() {
    let dir = scratch("not-toml");
    fs::write(
        dir.join("broken.toml"),
        "title = \"The Ledger\"\nbriefing =\n[[suspects\n",
    )
    .expect("a file of broken TOML");

    // Stage 6d already builds this error. What is new is that the path in it is
    // now a real one, not a label the caller made up.
    match load_case(&dir, "broken") {
        Err(AppError::Parse { path, .. }) => {
            assert!(
                path.contains("broken.toml"),
                "the error names the file: {path}"
            );
        }
        other => panic!("expected AppError::Parse, got {other:?}"),
    }
}

#[test]
fn a_structural_failure_arrives_unchanged() {
    const KNOWN_BY_A_STRANGER: &str = r#"
title = "The Stranger"
briefing = "Somebody who is not in this file knows something."

[[suspects]]
id = 1
name = "Marta Reyes"

[[facts]]
id = 1
statement = "The safe was opened at 21:40."
known_by = [1, 7]
"#;

    let dir = scratch("stranger");
    fs::write(dir.join("stranger.toml"), KNOWN_BY_A_STRANGER).expect("a file");

    // Reading the file adds a step in front of Stage 6d's road. It does not
    // add a step to the road: what comes back out is still the error the
    // conversion raised.
    assert_eq!(
        load_case(&dir, "stranger"),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(7)
        })
    );
}

#[test]
fn a_slug_cannot_climb_out_of_the_case_directory() {
    let root = scratch("escape");
    let cases = root.join("cases");
    fs::create_dir_all(&cases).expect("a cases subdirectory");
    fs::write(root.join("outside.toml"), THE_LEDGER).expect("a case file one level up");

    // The file is real, and loads perfectly well when it is in the directory
    // we were asked to look in.
    assert!(load_case(&root, "outside").is_ok());

    // Same file, reached by walking up out of the directory we were asked to
    // look in. In Stage 9a this slug arrives from the front end.
    assert_eq!(
        load_case(&cases, "../outside"),
        Err(AppError::CaseNotFound {
            slug: "../outside".to_string()
        })
    );
}

#[test]
fn a_slug_that_is_not_a_plain_name_never_reaches_the_disk() {
    for slug in [
        "The Ledger",
        "the_ledger",
        "the-ledger.toml",
        "/etc/passwd",
        "",
    ] {
        assert_eq!(
            load_case(&fixtures(), slug),
            Err(AppError::CaseNotFound {
                slug: slug.to_string()
            }),
            "{slug:?} is not a case slug"
        );
    }
}
