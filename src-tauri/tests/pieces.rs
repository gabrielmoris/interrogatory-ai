//! Stage 16a — the suspect speaks in pieces
//!
//! Run with:  cargo test --test pieces      (from src-tauri/)
//!
//! A real model does not write its reply all at once. It makes it one small
//! piece at a time: a word, or part of one. So `reply` no longer gives back a
//! `String`. It is handed a function, `say`, and calls it once per piece.
//!
//! For now the engine thread puts the pieces back together and sends the
//! whole reply, so `ask` gives back the same as before. In Stage 16b each
//! piece travels to `ask` on its own.

use interrogatory_ai_lib::engine_thread::{start, Question};
use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use tauri::async_runtime::{block_on, channel};

/// Says its reply in three pieces that are not whole words, the way a real model can.
struct HesitantEngine;

impl InferenceEngine for HesitantEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()> {
        say("I... ");
        say("I was ho");
        say("me.");
        Ok(())
    }
}

/// Starts a reply, then fails halfway through it.
struct FailsHalfwayEngine;

impl InferenceEngine for FailsHalfwayEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()> {
        say("I was ");
        Err(AppError::Inference {
            message: "out of memory".to_string(),
        })
    }
}

#[test]
fn the_mock_says_its_line_one_word_at_a_time() {
    let engine = MockEngine::new("I was at home all night.");
    let mut pieces: Vec<String> = Vec::new();

    let said = engine.reply("Where were you on Tuesday?", &mut |piece| {
        pieces.push(piece.to_string())
    });

    assert_eq!(said, Ok(()));
    assert_eq!(pieces, vec!["I ", "was ", "at ", "home ", "all ", "night."]);
}

#[test]
fn the_engine_thread_puts_the_pieces_back_together() {
    let questions = start(Box::new(HesitantEngine));
    let (answer_to, mut answer) = channel(1);

    questions
        .send(Question {
            text: "Where were you on Tuesday?".to_string(),
            answer_to,
        })
        .expect("the engine thread is running");

    assert_eq!(
        answer.blocking_recv(),
        Some(Ok("I... I was home.".to_string()))
    );
}

#[test]
fn every_answer_starts_from_nothing() {
    let questions = start(Box::new(HesitantEngine));
    let (first_to, mut first) = channel(1);
    let (second_to, mut second) = channel(1);

    questions
        .send(Question {
            text: "Where were you on Tuesday?".to_string(),
            answer_to: first_to,
        })
        .expect("the engine thread is running");
    questions
        .send(Question {
            text: "And on Wednesday?".to_string(),
            answer_to: second_to,
        })
        .expect("the engine thread is running");

    // The second answer holds the second reply only, not both of them.
    assert_eq!(
        first.blocking_recv(),
        Some(Ok("I... I was home.".to_string()))
    );
    assert_eq!(
        second.blocking_recv(),
        Some(Ok("I... I was home.".to_string()))
    );
}

#[test]
fn half_a_reply_is_not_an_answer() {
    let questions = start(Box::new(FailsHalfwayEngine));
    let (answer_to, mut answer) = channel(1);

    questions
        .send(Question {
            text: "Did you take the ledger?".to_string(),
            answer_to,
        })
        .expect("the engine thread is running");

    // The engine said "I was " and then failed. The answer is the failure,
    // not the half sentence.
    assert_eq!(
        answer.blocking_recv(),
        Some(Err(AppError::Inference {
            message: "out of memory".to_string()
        }))
    );
}

#[test]
fn ask_still_gives_back_the_whole_reply() {
    let app = AppState::new(PathBuf::from("cases"), Box::new(HesitantEngine));
    app.begin(SuspectId::new(2)).unwrap();

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?")),
        Ok("I... I was home.".to_string())
    );
    assert_eq!(app.turn_count(), Ok(2));
}
