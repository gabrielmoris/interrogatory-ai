//! Stage 17b — the player presses Stop
//!
//! Run with:  cargo test --test press_stop      (from src-tauri/)
//!
//! The player can stop the suspect mid-sentence. React calls `stop_suspect`,
//! which calls `AppState::stop`. `stop` puts up the stop sign of the question
//! being answered now: a `CancellationToken`. `ask` has a hand on the same
//! sign. It looks at it each time a message comes out of pipe 2, and once the
//! sign is up, it stops: the reply so far is kept in the room and given back
//! as `Ok`.
//!
//! When `ask` stops, it lets go of pipe 2, so the engine stops too (17a).
//!
//! No app starts here, so there is no React and no player. The helpers below
//! stand in for them: they show the pieces, and press Stop at a set moment.
//! The app is kept in an `Arc` so the test and the player's thread can both
//! use it. Your code does not need one.

use interrogatory_ai_lib::error::AppResult;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::ipc::stop_suspect;
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};
use interrogatory_ai_lib::state::AppState;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::async_runtime::block_on;
use tauri::ipc::{Channel, InvokeResponseBody};
use tauri::State;

/// The app with Viktor in the room, answering through `engine`.
fn viktor_in_the_room(engine: Box<dyn InferenceEngine>) -> Arc<AppState> {
    let app = AppState::new(PathBuf::from("cases"), engine);
    app.begin(SuspectId::new(2)).unwrap();
    Arc::new(app)
}

/// A Tauri channel carries each piece as JSON text: "I " travels as "\"I \"".
fn piece_from(body: InvokeResponseBody) -> String {
    match body {
        InvokeResponseBody::Json(json) => {
            serde_json::from_str(&json).expect("each piece is a JSON string")
        }
        InvokeResponseBody::Raw(_) => panic!("pieces travel as JSON"),
    }
}

/// Stands in for React: every piece `ask` sends comes out of the `Receiver`.
fn react() -> (Channel<String>, Receiver<String>) {
    let (to_test, shown) = mpsc::channel();
    let on_piece = Channel::new(move |body| {
        let _ = to_test.send(piece_from(body));
        Ok(())
    });
    (on_piece, shown)
}

/// Stands in for React and the player: shows each piece, and presses Stop
/// the moment `stop_at` is on the screen.
fn react_presses_stop_at(
    stop_at: &'static str,
    app: Arc<AppState>,
) -> (Channel<String>, Receiver<String>) {
    let (to_test, shown) = mpsc::channel();
    let on_piece = Channel::new(move |body| {
        let piece = piece_from(body);
        if piece == stop_at {
            app.stop().expect("Stop can be pressed");
        }
        let _ = to_test.send(piece);
        Ok(())
    });
    (on_piece, shown)
}

/// Says "I ", then waits until the test tells it to go on, then says
/// "was home.". If no one tells it within 5 seconds, it says nothing more.
struct PausesEngine {
    go_on: Mutex<Receiver<()>>,
}

impl InferenceEngine for PausesEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        if !say("I ") {
            return Ok(());
        }
        let go_on = self.go_on.lock().expect("only this engine uses it");
        if go_on.recv_timeout(Duration::from_secs(5)).is_ok() {
            say("was home.");
        }
        Ok(())
    }
}

/// Talks for a long time: up to 1000 pieces of "and ". It stops early if
/// `say` gives back `false` (17a). When it is done, it tells the test how
/// many pieces it said.
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

/// How many pieces of "and " a reply holds.
fn pieces_in(reply: &AppResult<String>) -> AppResult<usize> {
    reply.clone().map(|text| text.matches("and ").count())
}

#[test]
fn stop_suspect_is_a_command_that_takes_the_state() {
    // This compiles only if `stop_suspect` takes the state and gives back an
    // `AppResult<()>`. No app starts, so its body does not run here.
    fn _shape(state: State<'_, AppState>) -> AppResult<()> {
        stop_suspect(state)
    }
}

#[test]
fn stop_between_questions_does_not_stop_the_next_reply() {
    let app = viktor_in_the_room(Box::new(MockEngine::new("I was at home all night.")));

    // Nothing is being said. Stop does nothing — and it must not stop the next reply.
    assert_eq!(app.stop(), Ok(()));

    let (on_piece, _shown) = react();
    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", on_piece)),
        Ok("I was at home all night.".to_string())
    );
}

#[test]
fn stop_keeps_what_the_suspect_said_so_far() {
    let app = viktor_in_the_room(Box::new(MockEngine::new("I was at home all night.")));
    let (on_piece, shown) = react_presses_stop_at("was ", app.clone());

    assert_eq!(
        block_on(app.ask("Where were you on Tuesday?", on_piece)),
        Ok("I was ".to_string())
    );

    // React saw exactly what `ask` gives back.
    let pieces: Vec<String> = shown.try_iter().collect();
    assert_eq!(pieces, vec!["I ", "was "]);
    // The room keeps the question and the reply so far.
    assert_eq!(app.turn_count(), Ok(2));
}

#[test]
fn a_piece_that_comes_after_stop_is_not_shown() {
    let (tell_engine, go_on) = mpsc::channel();
    let engine = PausesEngine {
        go_on: Mutex::new(go_on),
    };
    let app = viktor_in_the_room(Box::new(engine));
    let (on_piece, shown) = react();

    // The player, on a thread of their own: waits until "I " is on the screen,
    // presses Stop while `ask` is waiting for the next piece, then lets the
    // engine say it.
    let player = {
        let app = app.clone();
        thread::spawn(move || {
            let first = shown.recv_timeout(Duration::from_secs(5));
            assert_eq!(first, Ok("I ".to_string()));
            // Give `ask` time to start waiting for the next message.
            thread::sleep(Duration::from_millis(100));
            app.stop().expect("Stop can be pressed");
            let _ = tell_engine.send(());
            shown
        })
    };

    let reply = block_on(app.ask("Where were you on Tuesday?", on_piece));
    let shown = player.join().expect("the player did their part");

    // "was home." came after Stop. It is not shown, and not kept.
    assert_eq!(reply, Ok("I ".to_string()));
    let after_stop: Vec<String> = shown.try_iter().collect();
    assert_eq!(after_stop, Vec::<String>::new());
    assert_eq!(app.turn_count(), Ok(2));
}

#[test]
fn after_stop_the_engine_stops_and_the_next_question_is_answered_whole() {
    let (tell_test, told) = mpsc::channel();
    let app = viktor_in_the_room(Box::new(LongWindedEngine { tell_test }));

    // The player presses Stop as soon as the first "and " is on the screen.
    let (on_piece, _shown) = react_presses_stop_at("and ", app.clone());
    let reply = block_on(app.ask("Where were you on Tuesday?", on_piece));
    assert_eq!(pieces_in(&reply), Ok(1));

    // `ask` stopped, so it let go of pipe 2, and the engine stopped too (17a).
    // `ask` takes one more message after Stop (the one it finds the sign up on),
    // so the engine says at most one piece more than in 17a: the 1st (shown),
    // the 2nd (taken, not shown), at most one waiting in the pipe, and the one
    // that finds the pipe closed.
    let said = told
        .recv_timeout(Duration::from_secs(5))
        .expect("the engine finished its reply");
    assert!(
        said <= 4,
        "the engine said {said} pieces after Stop; it should stop by the 4th"
    );

    // The next question has a new stop sign: it gets its whole answer.
    let (on_piece, _shown) = react();
    let reply = block_on(app.ask("Did you take the ledger?", on_piece));
    assert_eq!(told.recv_timeout(Duration::from_secs(5)), Ok(1000));
    assert_eq!(pieces_in(&reply), Ok(1000));
}
