//! Stage 11d — React asks a question
//!
//! Run with:  cargo test --test ask_command      (from src-tauri/)
//!
//! What asking does is already tested: the command hands everything to
//! `AppState::ask`, and `room_exchange.rs` tests that. This file checks the
//! command exists, with the two inputs Tauri and React give it. No app starts.

use interrogatory_ai_lib::error::AppResult;
use interrogatory_ai_lib::ipc::ask_suspect;
use interrogatory_ai_lib::state::AppState;
use tauri::State;

#[test]
fn ask_suspect_takes_the_state_and_a_question() {
    // This line compiles only if `ask_suspect` has exactly this shape.
    let _command: fn(State<'_, AppState>, String) -> AppResult<String> = ask_suspect;
}
