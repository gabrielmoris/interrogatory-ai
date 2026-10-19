//! Stage 16a — the suspect speaks in pieces
//!
//! Run with:  cargo test --test pieces      (from src-tauri/)
//!
//! A real model does not write its reply all at once. It makes it one small
//! piece at a time: a word, or part of one. So `reply` no longer gives back a
//! `String`. It is handed a function, `say`, and calls it once per piece.
//!
//! `ask` still gives back the whole reply. In 16a the engine thread put the
//! pieces together. Since Stage 16b each piece travels to `ask` on its own,
//! and `ask` puts them together: the tests for that are in `pieces_travel.rs`.
//!
//! Since Stage 17a `say` gives back `true` while someone is listening and
//! `false` once no one is (see `no_one_listens.rs`).

use interrogatory_ai_lib::error::AppResult;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use tauri::async_runtime::block_on;
use tauri::ipc::Channel;

/// A channel to no window: every piece `ask` sends into it is dropped.
/// Since Stage 16c `ask` sends each piece to React (see `pieces_to_react.rs`).
fn no_window() -> Channel<String> {
    Channel::new(|_| Ok(()))
}

/// Says its reply in three pieces that are not whole words, the way a real model can.
struct HesitantEngine;

impl InferenceEngine for HesitantEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        say("I... ");
        say("I was ho");
        say("me.");
        Ok(())
    }
}

#[test]
fn the_mock_says_its_line_one_word_at_a_time() {
    let engine = MockEngine::new("I was at home all night.");
    let mut pieces: Vec<String> = Vec::new();

    let said = engine.reply("Where were you on Tuesday?", &mut |piece| {
        pieces.push(piece.to_string());
        true
    });

    assert_eq!(said, Ok(()));
    assert_eq!(pieces, vec!["I ", "was ", "at ", "home ", "all ", "night."]);
}

#[test]
fn ask_still_gives_back_the_whole_reply() {
    let app = AppState::new(PathBuf::from("cases"), Box::new(HesitantEngine));
    app.begin(SuspectId::new(2)).unwrap();

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", no_window())),
        Ok("I... I was home.".to_string())
    );
    assert_eq!(app.turn_count(), Ok(2));
}
