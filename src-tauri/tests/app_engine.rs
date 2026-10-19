//! Stage 11b — the game holds an engine
//!
//! Run with:  cargo test --test app_engine      (from src-tauri/)
//!
//! `AppState` keeps one engine for as long as the app runs. It does not know
//! which kind: a `MockEngine` today, a real language model later, or
//! `EchoEngine` below, which only this test file knows about. The field is a
//! `Box<dyn InferenceEngine>`: "one thing that has `reply`, any type".
//!
//! Since Stage 11c the app only asks inside an interrogation, so the tests
//! call a suspect in first.
//!
//! Since Stage 12 `ask` is async. A test cannot `.await`, so it waits with
//! `block_on` (see `ask_async.rs`).
//!
//! Since Stage 16a an engine says its reply in pieces (see `pieces.rs`).
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

/// A second engine, written here: it repeats the question back.
struct EchoEngine;

impl InferenceEngine for EchoEngine {
    fn reply(&self, prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        say(prompt);
        Ok(())
    }
}

#[test]
fn the_app_asks_the_engine_it_was_given() {
    let app = AppState::new(
        PathBuf::from("cases"),
        Box::new(MockEngine::new("I was at home all night.")),
    );
    app.begin(SuspectId::new(2)).unwrap();

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", no_window())),
        Ok("I was at home all night.".to_string())
    );
}

#[test]
fn any_engine_fits_the_same_field() {
    let app = AppState::new(PathBuf::from("cases"), Box::new(EchoEngine));
    app.begin(SuspectId::new(2)).unwrap();

    assert_eq!(
        block_on(app.ask("Who is Viktor?", no_window())),
        Ok("Who is Viktor?".to_string())
    );
}

#[test]
fn two_kinds_of_engine_can_sit_in_one_list() {
    let engines: Vec<Box<dyn InferenceEngine>> = vec![
        Box::new(MockEngine::new("No comment.")),
        Box::new(EchoEngine),
    ];

    let mut replies: Vec<String> = Vec::new();
    for engine in &engines {
        let mut heard = String::new();
        engine
            .reply("Did you take the ledger?", &mut |piece| {
                heard.push_str(piece);
                true
            })
            .expect("neither engine fails");
        replies.push(heard);
    }

    assert_eq!(replies, vec!["No comment.", "Did you take the ledger?"]);
}
