//! Stage 12 — a question waits until it is awaited
//!
//! Run with:  cargo test --test ask_async      (from src-tauri/)
//!
//! `AppState::ask` is now an `async fn`. Calling it does not ask anything yet.
//! It gives back a future: the question, written down, not asked. The question
//! is asked when someone awaits the future.
//!
//! A test is not async, so it cannot write `.await`. It uses `block_on`
//! instead: "run this future now, and wait here until it is done".

use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::MockEngine;
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use tauri::async_runtime::block_on;

/// The app with Viktor in the room. He always says the same line.
fn viktor_in_the_room() -> AppState {
    let app = AppState::new(
        PathBuf::from("cases"),
        Box::new(MockEngine::new("I was at home all night.")),
    );
    app.begin(SuspectId::new(2)).unwrap();
    app
}

#[test]
fn calling_ask_does_not_ask_yet() {
    let app = viktor_in_the_room();

    let _question = app.ask("Where were you on Tuesday?");

    assert_eq!(app.turn_count(), Ok(0));
}

#[test]
fn awaiting_the_question_asks_it() {
    let app = viktor_in_the_room();

    let question = app.ask("Where were you on Tuesday?");

    assert_eq!(
        block_on(question),
        Ok("I was at home all night.".to_string())
    );
    assert_eq!(app.turn_count(), Ok(2));
}

#[test]
fn a_question_never_awaited_is_never_asked() {
    let app = viktor_in_the_room();

    let forgotten = app.ask("Where were you on Tuesday?");
    drop(forgotten);
    block_on(app.ask("Who can say so?")).unwrap();

    assert_eq!(app.turn_count(), Ok(2));
}
