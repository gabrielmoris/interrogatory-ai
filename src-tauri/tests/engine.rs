//! Stage 11a — the suspect gets a voice
//!
//! Run with:  cargo test --test engine      (from src-tauri/)
//!
//! Later a real language model will answer the detective's questions. The game
//! must not care which model it is. So the game only asks for "something that
//! can reply" — the trait `InferenceEngine` — and `MockEngine` is the first
//! thing that can. It always gives the same line, so tests stay fast and exact.

use interrogatory_ai_lib::error::AppResult;
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};

/// Asks a question of any engine. This function does not know it is a mock.
fn ask(engine: &impl InferenceEngine, question: &str) -> AppResult<String> {
    engine.reply(question)
}

#[test]
fn the_mock_answers_with_its_line() {
    let engine = MockEngine::new("I was at home all night.");

    assert_eq!(
        engine.reply("Where were you on Tuesday?"),
        Ok("I was at home all night.".to_string())
    );
}

#[test]
fn the_mock_gives_the_same_line_whatever_you_ask() {
    let engine = MockEngine::new("I want my lawyer.");

    assert_eq!(
        engine.reply("Who is Viktor?"),
        engine.reply("Where is the ledger?")
    );
}

#[test]
fn the_game_can_ask_any_engine() {
    let engine = MockEngine::new("I never touched the ledger.");

    assert_eq!(
        ask(&engine, "Did you take the ledger?"),
        Ok("I never touched the ledger.".to_string())
    );
}
