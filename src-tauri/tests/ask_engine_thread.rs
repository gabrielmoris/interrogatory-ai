//! Stage 14b — `ask` goes through the engine thread
//!
//! Run with:  cargo test --test ask_engine_thread      (from src-tauri/)
//!
//! Since 14a the engine can live on a thread of its own. Now the game uses it:
//! `AppState::new` hands the engine to `start`, and `ask` sends each question
//! through the channel, then waits for the answer on a background thread.
//!
//! The tests below cannot see that waiting. They check what you can see from
//! outside: who answers, and what comes back when the engine fails or crashes.

use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::InferenceEngine;
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use std::thread;
use tauri::async_runtime::block_on;

/// Answers with the id of the thread it runs on.
struct WhereAmIEngine;

impl InferenceEngine for WhereAmIEngine {
    fn reply(&self, _prompt: &str) -> AppResult<String> {
        Ok(format!("{:?}", thread::current().id()))
    }
}

/// Always fails, the way a real model can.
struct BrokenEngine;

impl InferenceEngine for BrokenEngine {
    fn reply(&self, _prompt: &str) -> AppResult<String> {
        Err(AppError::Inference {
            message: "out of memory".to_string(),
        })
    }
}

/// Crashes, the way a bug in the model code could. `panic!` stops the thread it runs on.
struct CrashingEngine;

impl InferenceEngine for CrashingEngine {
    fn reply(&self, _prompt: &str) -> AppResult<String> {
        panic!("the model crashed");
    }
}

/// The app with Viktor in the room, answering through `engine`.
fn viktor_in_the_room(engine: Box<dyn InferenceEngine>) -> AppState {
    let app = AppState::new(PathBuf::from("cases"), engine);
    app.begin(SuspectId::new(2)).unwrap();
    app
}

#[test]
fn every_answer_comes_from_the_one_engine_thread() {
    let app = viktor_in_the_room(Box::new(WhereAmIEngine));

    let first = block_on(app.ask("Where were you on Tuesday?")).expect("this engine never fails");
    let second = block_on(app.ask("Who can say so?")).expect("this engine never fails");
    let here = format!("{:?}", thread::current().id());

    // Not the test's thread...
    assert_ne!(first, here);
    // ...and the same thread both times: the engine has one home.
    assert_eq!(first, second);
}

#[test]
fn an_engine_failure_comes_back_from_ask() {
    let app = viktor_in_the_room(Box::new(BrokenEngine));

    assert_eq!(
        block_on(app.ask("Did you take the ledger?")),
        Err(AppError::Inference {
            message: "out of memory".to_string()
        })
    );
    // The question was kept; there is no answer to keep.
    assert_eq!(app.turn_count(), Ok(1));
}

#[test]
fn a_crashed_engine_is_an_error_not_a_crash() {
    let app = viktor_in_the_room(Box::new(CrashingEngine));

    // The engine thread dies. Its end of the answer channel goes with it,
    // so the wait ends with an error instead of an answer.
    assert_eq!(
        block_on(app.ask("Did you take the ledger?")),
        Err(AppError::Inference {
            message: "receiving on a closed channel".to_string()
        })
    );
    assert_eq!(app.turn_count(), Ok(1));
}
