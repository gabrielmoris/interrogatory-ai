//! Stage 17a — the suspect stops when no one listens
//!
//! Run with:  cargo test --test no_one_listens      (from src-tauri/)
//!
//! Since 16c `ask` can stop before the reply is over: when a piece cannot
//! reach React, `ask` gives back `WindowUnreachable` and lets go of pipe 2.
//! But the engine did not know. It kept calling `say` until its reply was
//! over, talking to no one, and the next question waited behind it.
//!
//! Now `say` answers. It gives back `true` while someone is listening and
//! `false` once no one is. The engine thread gives back whether the piece got
//! into pipe 2, and an engine that hears `false` stops talking.

use interrogatory_ai_lib::engine_thread::{start, Question, Said};
use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use tauri::async_runtime::{block_on, channel};
use tauri::ipc::Channel;

/// A channel to a window that has closed: every send fails.
fn closed_window() -> Channel<String> {
    Channel::new(|_| Err(tauri::Error::WebviewNotFound))
}

/// A channel to no window: every piece `ask` sends into it is dropped.
fn no_window() -> Channel<String> {
    Channel::new(|_| Ok(()))
}

/// Talks for a long time: up to 1000 pieces of "and ". It stops early if
/// `say` gives back `false`. When it is done, it tells the test how many
/// pieces it said.
struct LongWindedEngine {
    tell_test: Sender<usize>,
}

impl InferenceEngine for LongWindedEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        let mut said = 0;
        let mut listening = true;
        while listening && said < 1000 {
            listening = say("and ");
            said += 1;
        }
        let _ = self.tell_test.send(said);
        Ok(())
    }
}

/// A `LongWindedEngine`, and where the test hears how many pieces it said.
fn long_winded() -> (Box<dyn InferenceEngine>, Receiver<usize>) {
    let (tell_test, told) = mpsc::channel();
    (Box::new(LongWindedEngine { tell_test }), told)
}

#[test]
fn the_mock_stops_as_soon_as_say_gives_back_false() {
    let engine = MockEngine::new("I was at home all night.");
    let mut pieces: Vec<String> = Vec::new();

    // This `say` listens to two pieces. Then it gives back `false`: no one is listening.
    let said = engine.reply("Where were you on Tuesday?", &mut |piece| {
        pieces.push(piece.to_string());
        pieces.len() < 2
    });

    assert_eq!(said, Ok(()));
    assert_eq!(pieces, vec!["I ", "was "]);
}

#[test]
fn the_mock_says_its_whole_line_while_someone_listens() {
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
fn the_engine_thread_says_false_once_no_one_reads_pipe_2() {
    let (engine, told) = long_winded();
    let questions = start(engine);
    let (answer_to, mut answer) = channel(1);

    questions
        .send(Question {
            text: "Where were you on Tuesday?".to_string(),
            answer_to,
        })
        .expect("the engine thread is running");

    // Read one piece, then let go of pipe 2 — what `ask` does when it stops early.
    assert_eq!(
        answer.blocking_recv(),
        Some(Said::Piece("and ".to_string()))
    );
    drop(answer);

    let said = told
        .recv_timeout(Duration::from_secs(5))
        .expect("the engine finished its reply");
    // The piece that was read, at most one more waiting in the pipe, and the
    // one that found the pipe closed. Then `say` gave back `false`.
    assert!(
        said <= 3,
        "the engine said {said} pieces; it should stop by the 3rd"
    );
}

#[test]
fn when_ask_stops_early_the_engine_stops_and_the_next_question_is_answered() {
    let (engine, told) = long_winded();
    let app = AppState::new(PathBuf::from("cases"), engine);
    app.begin(SuspectId::new(2)).unwrap();

    // The window has closed: the first piece cannot reach React, so `ask` stops (16c).
    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", closed_window())),
        Err(AppError::WindowUnreachable {
            message: "webview not found".to_string()
        })
    );
    let said = told
        .recv_timeout(Duration::from_secs(5))
        .expect("the engine finished its reply");
    assert!(
        said <= 3,
        "the engine said {said} pieces after `ask` stopped; it should stop by the 3rd"
    );

    // The engine thread is free again: the next question gets its whole answer.
    let reply = block_on(app.ask("Did you take the ledger?", no_window()));
    assert_eq!(told.recv_timeout(Duration::from_secs(5)), Ok(1000));
    assert_eq!(reply, Ok("and ".repeat(1000)));
}
