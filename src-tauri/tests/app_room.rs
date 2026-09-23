//! Stage 10f — two more methods through the lock
//!
//! Run with:  cargo test --test app_room      (from src-tauri/)
//!
//! Stage 10e let a command call a suspect in. The interrogation screen needs
//! two more things from the one `AppState` every command shares: who is in
//! the room, to show their face, and a way to keep each line said there.
//! As in 10e, every test talks to the state through `&` only.

use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::state::AppState;
use interrogatory_ai_lib::transcript::Speaker;
use std::path::PathBuf;

/// The app the moment it starts.
fn fresh_app() -> AppState {
    AppState::new(PathBuf::from("cases"))
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

#[test]
fn nothing_can_be_said_before_a_suspect_is_called_in() {
    let app = fresh_app();

    assert_eq!(
        app.record(Speaker::Detective, "Where were you on Tuesday?"),
        Err(AppError::InvalidState {
            action: "record a line".to_string(),
            state: "the briefing".to_string(),
        })
    );
}

#[test]
fn once_a_suspect_is_in_both_sides_can_speak() {
    let app = fresh_app();
    app.begin(viktor()).unwrap();

    assert_eq!(
        app.record(Speaker::Detective, "Where were you on Tuesday?"),
        Ok(())
    );
    assert_eq!(app.record(Speaker::Suspect, "At the harbour."), Ok(()));
}

#[test]
fn a_new_game_has_nobody_in_the_room() {
    let app = fresh_app();

    assert_eq!(app.suspect(), Ok(None));
}

#[test]
fn the_suspect_called_in_is_the_one_in_the_room() {
    let app = fresh_app();
    app.begin(viktor()).unwrap();

    assert_eq!(app.suspect(), Ok(Some(viktor())));
}
