//! Stage 10e — the app holds the phase
//!
//! Run with:  cargo test --test app_phase      (from src-tauri/)
//!
//! Tauri keeps one `AppState` for as long as the app runs, and hands every
//! command that same one. So once one command has called a suspect into the
//! room, every other command finds the room taken — Stage 10b's rule, now
//! holding across the whole app instead of inside one test.
//!
//! A command never gets the `AppState` to itself. It gets a shared view, `&`,
//! because other commands can be holding the same `AppState` at the same
//! moment, some of them on other threads. Every test here talks to the state
//! the way a command does: through `&` only.

use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use std::thread;

/// The app the moment it starts.
fn fresh_app() -> AppState {
    AppState::new(PathBuf::from("cases"))
}

fn marta() -> SuspectId {
    SuspectId::new(1)
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

/// What `begin` says when someone is already in the room.
fn room_taken() -> AppError {
    AppError::InvalidState {
        action: "begin an interrogation".to_string(),
        state: "an interrogation".to_string(),
    }
}

#[test]
fn a_new_game_lets_a_suspect_be_called_in() {
    let app = fresh_app();

    assert_eq!(app.begin(viktor()), Ok(()));
}

#[test]
fn a_suspect_called_in_by_one_command_holds_for_the_next() {
    let app = fresh_app();
    // Two commands, each holding its own shared view of the one `AppState`.
    let picks_viktor: &AppState = &app;
    let picks_marta: &AppState = &app;

    assert_eq!(picks_viktor.begin(viktor()), Ok(()));
    assert_eq!(
        picks_marta.begin(marta()),
        Err(room_taken()),
        "the second command saw the first one's change"
    );
}

#[test]
fn a_change_made_on_another_thread_is_seen_here() {
    let app = fresh_app();

    let there = thread::scope(|s| s.spawn(|| app.begin(viktor())).join().unwrap());

    assert_eq!(there, Ok(()));
    assert_eq!(app.begin(marta()), Err(room_taken()));
}

#[test]
fn two_begins_at_the_same_moment_exactly_one_gets_in() {
    // The player double-clicks: two `begin` commands run at once, on two
    // threads. One calls its suspect in. The other finds the room already
    // taken. Never both, never neither.
    let app = fresh_app();

    let (for_marta, for_viktor) = thread::scope(|s| {
        let marta_thread = s.spawn(|| app.begin(marta()));
        let viktor_thread = s.spawn(|| app.begin(viktor()));
        (marta_thread.join().unwrap(), viktor_thread.join().unwrap())
    });

    let got_in = [&for_marta, &for_viktor]
        .into_iter()
        .filter(|answer| answer.is_ok())
        .count();
    assert_eq!(got_in, 1, "marta: {for_marta:?}, viktor: {for_viktor:?}");
    assert!(
        for_marta == Err(room_taken()) || for_viktor == Err(room_taken()),
        "the one left out was told the room is taken"
    );
}
