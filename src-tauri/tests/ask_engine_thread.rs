//! Stage 14b — `ask` goes through the engine thread
//!
//! Run with:  cargo test --test ask_engine_thread      (from src-tauri/)
//!
//! Since 14a the engine can live on a thread of its own. Now the game uses it:
//! `AppState::new` hands the engine to `start`, and `ask` sends each question
//! through the channel, then waits for the answer. Since Stage 15 it awaits the
//! answer channel itself; in 14b it waited on a background thread.
//!
//! The tests below cannot see that waiting. They check what you can see from
//! outside: who answers, and what comes back when the engine fails or crashes.
//!
//! Since Stage 16a an engine says its reply in pieces (see `pieces.rs`).
//!
//! Since Stage 17a `say` gives back `true` while someone is listening and
//! `false` once no one is (see `no_one_listens.rs`).

use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::InferenceEngine;
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use std::thread;
use tauri::async_runtime::block_on;
use tauri::ipc::Channel;

/// A channel to no window: every piece `ask` sends into it is dropped.
/// Since Stage 16c `ask` sends each piece to React (see `pieces_to_react.rs`).
fn no_window() -> Channel<String> {
    Channel::new(|_| Ok(()))
}

/// Answers with the id of the thread it runs on.
struct WhereAmIEngine;

impl InferenceEngine for WhereAmIEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        say(&format!("{:?}", thread::current().id()));
        Ok(())
    }
}

/// Always fails, the way a real model can.
struct BrokenEngine;

impl InferenceEngine for BrokenEngine {
    fn reply(&self, _prompt: &str, _say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        Err(AppError::Inference {
            message: "out of memory".to_string(),
        })
    }
}

/// Crashes, the way a bug in the model code could. `panic!` stops the thread it runs on.
struct CrashingEngine;

impl InferenceEngine for CrashingEngine {
    fn reply(&self, _prompt: &str, _say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
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

    let first = block_on(app.ask("Where were you on Tuesday?", no_window()))
        .expect("this engine never fails");
    let second =
        block_on(app.ask("Who can say so?", no_window())).expect("this engine never fails");
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
        block_on(app.ask("Did you take the ledger?", no_window())),
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
    // so the wait ends with `None` instead of an answer (Stage 15).
    assert_eq!(
        block_on(app.ask("Did you take the ledger?", no_window())),
        Err(AppError::Inference {
            message: "the engine stopped before it answered".to_string()
        })
    );
    assert_eq!(app.turn_count(), Ok(1));
}
