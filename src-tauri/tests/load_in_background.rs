//! Stage 13 — slow work goes to its own thread
//!
//! Run with:  cargo test --test load_in_background      (from src-tauri/)
//!
//! Reading a case file from disk makes the program wait. `case_intro_in_background`
//! does that reading on a separate thread, so the window never waits for it.
//! It gives back the same result as `case_intro_from`, just from somewhere else.

use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::ipc::{case_intro, case_intro_from, case_intro_in_background, CaseIntro};
use interrogatory_ai_lib::state::AppState;
use serde_json::json;
use std::path::{Path, PathBuf};
use tauri::async_runtime::block_on;
use tauri::State;

/// `src-tauri/tests/cases` — the two real case files, wherever this repo sits.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("cases")
}

#[test]
fn the_intro_comes_back_the_same_from_the_other_thread() {
    let from_here = case_intro_from(&fixtures(), "the-ledger");

    let from_there = block_on(case_intro_in_background(
        fixtures(),
        "the-ledger".to_string(),
    ));

    assert_eq!(from_there, from_here);
}

#[test]
fn a_missing_case_is_still_case_not_found() {
    // The case not being there is not the thread failing. The error from
    // `load_case` comes back exactly as it was.
    assert_eq!(
        block_on(case_intro_in_background(
            fixtures(),
            "the-missing-hour".to_string()
        )),
        Err(AppError::CaseNotFound {
            slug: "the-missing-hour".to_string()
        })
    );
}

#[test]
fn a_failed_thread_crosses_as_background_failed() {
    let failure = AppError::BackgroundFailed {
        message: "the work stopped".to_string(),
    };

    assert_eq!(
        serde_json::to_value(failure).expect("AppError turns into JSON"),
        json!({ "kind": "backgroundFailed", "message": "the work stopped" })
    );
}

#[test]
fn case_intro_is_async_and_takes_the_state_and_a_slug() {
    // This compiles only if `case_intro` takes these two inputs and,
    // once awaited, gives back an `AppResult<CaseIntro>`.
    async fn _shape(state: State<'_, AppState>, slug: String) -> AppResult<CaseIntro> {
        case_intro(state, slug).await
    }
}
