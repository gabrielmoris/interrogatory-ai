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

use interrogatory_ai_lib::error::AppResult;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;

/// A second engine, written here: it repeats the question back.
struct EchoEngine;

impl InferenceEngine for EchoEngine {
    fn reply(&self, prompt: &str) -> AppResult<String> {
        Ok(prompt.to_string())
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
        app.ask("Where were you on Tuesday?"),
        Ok("I was at home all night.".to_string())
    );
}

#[test]
fn any_engine_fits_the_same_field() {
    let app = AppState::new(PathBuf::from("cases"), Box::new(EchoEngine));
    app.begin(SuspectId::new(2)).unwrap();

    assert_eq!(app.ask("Who is Viktor?"), Ok("Who is Viktor?".to_string()));
}

#[test]
fn two_kinds_of_engine_can_sit_in_one_list() {
    let engines: Vec<Box<dyn InferenceEngine>> = vec![
        Box::new(MockEngine::new("No comment.")),
        Box::new(EchoEngine),
    ];

    let replies: Vec<AppResult<String>> = engines
        .iter()
        .map(|engine| engine.reply("Did you take the ledger?"))
        .collect();

    assert_eq!(
        replies,
        vec![
            Ok("No comment.".to_string()),
            Ok("Did you take the ledger?".to_string()),
        ]
    );
}
