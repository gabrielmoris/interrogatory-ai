//! Stage 16b — each piece travels to `ask`
//!
//! Run with:  cargo test --test pieces_travel      (from src-tauri/)
//!
//! Since 16a the engine says its reply in pieces. Now the engine thread does
//! not wait for the whole reply: it sends each piece into pipe 2 the moment
//! the engine says it, as `Said::Piece`. When the engine is done, one last
//! message says how it ended: `Said::Done`, or `Said::Failed` with the error.
//!
//! `ask` takes the messages out one by one, in a `loop`, and puts the pieces
//! together. It stops at the last message.

use interrogatory_ai_lib::engine_thread::{start, Question, Said};
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

/// The app with Viktor in the room, answering through `engine`.
fn viktor_in_the_room(engine: Box<dyn InferenceEngine>) -> AppState {
    let app = AppState::new(PathBuf::from("cases"), engine);
    app.begin(SuspectId::new(2)).unwrap();
    app
}

#[test]
fn each_piece_is_sent_on_by_itself_then_done() {
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
        Some(Said::Piece("I... ".to_string()))
    );
    assert_eq!(
        answer.blocking_recv(),
        Some(Said::Piece("I was ho".to_string()))
    );
    assert_eq!(answer.blocking_recv(), Some(Said::Piece("me.".to_string())));
    assert_eq!(answer.blocking_recv(), Some(Said::Done));
    // Nothing after the last message: the engine thread has let go of pipe 2.
    assert_eq!(answer.blocking_recv(), None);
}

#[test]
fn a_failure_is_the_last_message() {
    let questions = start(Box::new(FailsHalfwayEngine));
    let (answer_to, mut answer) = channel(1);

    questions
        .send(Question {
            text: "Did you take the ledger?".to_string(),
            answer_to,
        })
        .expect("the engine thread is running");

    // The piece said before the failure has already gone out.
    assert_eq!(
        answer.blocking_recv(),
        Some(Said::Piece("I was ".to_string()))
    );
    assert_eq!(
        answer.blocking_recv(),
        Some(Said::Failed(AppError::Inference {
            message: "out of memory".to_string()
        }))
    );
    assert_eq!(answer.blocking_recv(), None);
}

#[test]
fn ask_puts_the_pieces_back_together() {
    let app = viktor_in_the_room(Box::new(MockEngine::new("I was at home all night.")));

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?")),
        Ok("I was at home all night.".to_string())
    );
    // The question and the whole reply are kept in the room.
    assert_eq!(app.turn_count(), Ok(2));
}

#[test]
fn ask_gives_back_the_failure_not_half_a_reply() {
    let app = viktor_in_the_room(Box::new(FailsHalfwayEngine));

    assert_eq!(
        block_on(app.ask("Did you take the ledger?")),
        Err(AppError::Inference {
            message: "out of memory".to_string()
        })
    );
    // The question was kept. "I was " was not: half a sentence is not an answer.
    assert_eq!(app.turn_count(), Ok(1));
}
