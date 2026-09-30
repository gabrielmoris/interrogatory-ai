//! Stage 11c — the room keeps the exchange
//!
//! Run with:  cargo test --test room_exchange      (from src-tauri/)
//!
//! Asking a question is two lines in the room: the detective's question and
//! the suspect's reply. `AppState::ask` now keeps both, and it refuses when
//! nobody is in the room. `AppState::turn_count` says how many lines were kept.
//!
//! Since Stage 12 `ask` is async. A test cannot `.await`, so it waits with
//! `block_on` (see `ask_async.rs`).

use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::MockEngine;
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use tauri::async_runtime::block_on;

/// The app the moment it starts. Its suspect always says the same line.
fn fresh_app() -> AppState {
    AppState::new(
        PathBuf::from("cases"),
        Box::new(MockEngine::new("I was at home all night.")),
    )
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

#[test]
fn asking_before_a_suspect_is_in_is_refused() {
    let app = fresh_app();

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?")),
        Err(AppError::InvalidState {
            action: "record a line".to_string(),
            state: "the briefing".to_string(),
        })
    );
}

#[test]
fn a_refused_question_keeps_nothing() {
    let app = fresh_app();
    let _ = block_on(app.ask("Where were you on Tuesday?"));

    assert_eq!(app.turn_count(), Ok(0));
}

#[test]
fn one_question_keeps_two_lines() {
    let app = fresh_app();
    app.begin(viktor()).unwrap();

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?")),
        Ok("I was at home all night.".to_string())
    );
    assert_eq!(app.turn_count(), Ok(2));
}

#[test]
fn three_questions_keep_six_lines() {
    let app = fresh_app();
    app.begin(viktor()).unwrap();

    block_on(app.ask("Where were you on Tuesday?")).unwrap();
    block_on(app.ask("Who can say so?")).unwrap();
    block_on(app.ask("Why is your car wet?")).unwrap();

    assert_eq!(app.turn_count(), Ok(6));
}
