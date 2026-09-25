//! Stage 10h — React starts an interrogation
//!
//! Run with:  cargo test --test start_interrogation      (from src-tauri/)
//!
//! What the command does is already tested: it hands everything to
//! `begin_interrogation_from`, and `pick_suspect.rs` tests that. This file
//! checks the command exists, with the three inputs React and Tauri give it.
//! No app starts.

use interrogatory_ai_lib::error::AppResult;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::ipc::begin_interrogation;
use interrogatory_ai_lib::state::AppState;
use tauri::State;

#[test]
fn begin_interrogation_takes_the_state_a_slug_and_a_suspect() {
    // This line compiles only if `begin_interrogation` has exactly this shape.
    let _command: fn(State<'_, AppState>, String, SuspectId) -> AppResult<()> = begin_interrogation;
}
