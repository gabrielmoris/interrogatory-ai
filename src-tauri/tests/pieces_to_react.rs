//! Stage 16c — React shows the pieces
//!
//! Run with:  cargo test --test pieces_to_react      (from src-tauri/)
//!
//! Since 16b each piece of the reply reaches `ask` by itself. Now `ask` passes
//! each piece on to React the moment it takes it out of pipe 2. React makes a
//! Tauri `Channel` for each question and hands it to `ask_suspect`, which hands
//! it to `ask` as `on_piece`. `ask` sends each piece into it.
//!
//! No app starts here, so there is no React. `react()` below makes a channel
//! like the one React makes, and keeps every piece that arrives, so a test can
//! look at them.
//!
//! Since Stage 17a `say` gives back `true` while someone is listening and
//! `false` once no one is (see `no_one_listens.rs`).

use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::sync::Mutex;
use std::time::Duration;
use tauri::async_runtime::block_on;
use tauri::ipc::{Channel, InvokeResponseBody};

/// Stands in for React: a channel like the one React makes.
/// Every piece `ask` sends into it comes out of the `Receiver`, in order.
fn react() -> (Channel<String>, Receiver<String>) {
    let (to_test, shown) = mpsc::channel();
    let on_piece = Channel::new(move |body| {
        // A Tauri channel carries each piece as JSON text: "I " travels as "\"I \"".
        if let InvokeResponseBody::Json(json) = body {
            let piece: String = serde_json::from_str(&json).expect("each piece is a JSON string");
            let _ = to_test.send(piece);
        }
        Ok(())
    });
    (on_piece, shown)
}

/// A channel to a window that has closed: every send fails.
fn closed_window() -> Channel<String> {
    Channel::new(|_| Err(tauri::Error::WebviewNotFound))
}

/// The app with Viktor in the room, answering through `engine`.
fn viktor_in_the_room(engine: Box<dyn InferenceEngine>) -> AppState {
    let app = AppState::new(PathBuf::from("cases"), engine);
    app.begin(SuspectId::new(2)).unwrap();
    app
}

/// Starts a reply, then fails halfway through it.
struct FailsHalfwayEngine;

impl InferenceEngine for FailsHalfwayEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        say("I was ");
        Err(AppError::Inference {
            message: "out of memory".to_string(),
        })
    }
}

/// Says "I ", then waits until React has it before it says the rest.
/// If React does not get "I " within 5 seconds, it gives up and fails.
struct WaitsForReactEngine {
    react_got: Mutex<Receiver<String>>,
}

impl InferenceEngine for WaitsForReactEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        say("I ");
        let react_got = self.react_got.lock().expect("only this engine uses it");
        match react_got.recv_timeout(Duration::from_secs(5)) {
            Ok(_) => {
                say("was home.");
                Ok(())
            }
            Err(_) => Err(AppError::Inference {
                message: "React did not get the first piece before the reply ended".to_string(),
            }),
        }
    }
}

#[test]
fn each_piece_reaches_react_in_order() {
    let app = viktor_in_the_room(Box::new(MockEngine::new("I was at home all night.")));
    let (on_piece, shown) = react();

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", on_piece)),
        Ok("I was at home all night.".to_string())
    );

    let pieces: Vec<String> = shown.try_iter().collect();
    assert_eq!(pieces, vec!["I ", "was ", "at ", "home ", "all ", "night."]);
    // The whole reply is still kept in the room, as in 16b.
    assert_eq!(app.turn_count(), Ok(2));
}

#[test]
fn react_gets_a_piece_before_the_engine_says_the_next() {
    // React's channel passes each piece straight on to the engine, which is waiting for "I ".
    let (to_engine, react_got) = mpsc::channel();
    let on_piece = Channel::new(move |body| {
        if let InvokeResponseBody::Json(json) = body {
            let _ = to_engine.send(json);
        }
        Ok(())
    });
    let engine = WaitsForReactEngine {
        react_got: Mutex::new(react_got),
    };
    let app = viktor_in_the_room(Box::new(engine));

    // If `ask` kept the pieces until the reply was done, React would get "I "
    // only after the engine had given up waiting for it.
    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", on_piece)),
        Ok("I was home.".to_string())
    );
}

#[test]
fn after_a_failure_react_has_seen_the_pieces_before_it() {
    let app = viktor_in_the_room(Box::new(FailsHalfwayEngine));
    let (on_piece, shown) = react();

    assert_eq!(
        block_on(app.ask("Did you take the ledger?", on_piece)),
        Err(AppError::Inference {
            message: "out of memory".to_string()
        })
    );

    // "I was " had already gone to React. The error tells React to drop it.
    let pieces: Vec<String> = shown.try_iter().collect();
    assert_eq!(pieces, vec!["I was "]);
    // The room keeps only the question, as in 16b.
    assert_eq!(app.turn_count(), Ok(1));
}

#[test]
fn if_react_cannot_take_a_piece_ask_stops_and_keeps_no_reply() {
    let app = viktor_in_the_room(Box::new(MockEngine::new("I was at home all night.")));

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", closed_window())),
        Err(AppError::WindowUnreachable {
            message: "webview not found".to_string()
        })
    );
    // The question was kept. A reply nobody saw is not kept.
    assert_eq!(app.turn_count(), Ok(1));
}
