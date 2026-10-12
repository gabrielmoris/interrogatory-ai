//! Stage 14a — the engine gets its own thread
//!
//! Run with:  cargo test --test engine_thread      (from src-tauri/)
//!
//! The real model will be slow, and it must run on one thread of its own.
//! `start` puts the engine on that thread and gives back a `Sender`: the end
//! of a channel you drop questions into. Each `Question` carries its own
//! `Sender` for the answer, so the answer goes back to whoever asked.
//!
//! Since Stage 15 the answer pipe is an async channel (`tauri::async_runtime::channel`).
//! The game awaits it. A test is not async, so it waits with `blocking_recv()`.
//!
//! Since Stage 16a an engine says its reply in pieces (see `pieces.rs`). The
//! engine thread puts them back together, so each answer is still one `String`.

use interrogatory_ai_lib::engine_thread::{start, Question};
use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::llm::{InferenceEngine, MockEngine};
use std::thread;
use tauri::async_runtime::channel;

/// Answers with the question it was asked, so each answer can be told apart.
struct EchoEngine;

impl InferenceEngine for EchoEngine {
    fn reply(&self, prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()> {
        say(&format!("You asked: {prompt}"));
        Ok(())
    }
}

/// Answers with the id of the thread it runs on.
struct WhereAmIEngine;

impl InferenceEngine for WhereAmIEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()> {
        say(&format!("{:?}", thread::current().id()));
        Ok(())
    }
}

/// Always fails, the way a real model can.
struct BrokenEngine;

impl InferenceEngine for BrokenEngine {
    fn reply(&self, _prompt: &str, _say: &mut dyn FnMut(&str)) -> AppResult<()> {
        Err(AppError::Inference {
            message: "out of memory".to_string(),
        })
    }
}

#[test]
fn a_question_sent_in_gets_its_answer_back() {
    let questions = start(Box::new(MockEngine::new("I was at home all night.")));
    let (answer_to, mut answer) = channel(1);

    questions
        .send(Question {
            text: "Where were you on Tuesday?".to_string(),
            answer_to,
        })
        .expect("the engine thread is running");

    assert_eq!(
        answer.blocking_recv(),
        Some(Ok("I was at home all night.".to_string()))
    );
}

#[test]
fn each_answer_goes_back_to_whoever_asked() {
    let questions = start(Box::new(EchoEngine));
    let (first_to, mut first) = channel(1);
    let (second_to, mut second) = channel(1);

    questions
        .send(Question {
            text: "Who is Viktor?".to_string(),
            answer_to: first_to,
        })
        .expect("the engine thread is running");
    questions
        .send(Question {
            text: "Where is the ledger?".to_string(),
            answer_to: second_to,
        })
        .expect("the engine thread is running");

    // Read the second answer first: each one waits in its own channel.
    assert_eq!(
        second.blocking_recv(),
        Some(Ok("You asked: Where is the ledger?".to_string()))
    );
    assert_eq!(
        first.blocking_recv(),
        Some(Ok("You asked: Who is Viktor?".to_string()))
    );
}

#[test]
fn the_engine_answers_from_another_thread() {
    let questions = start(Box::new(WhereAmIEngine));
    let (answer_to, mut answer) = channel(1);

    questions
        .send(Question {
            text: "Where are you?".to_string(),
            answer_to,
        })
        .expect("the engine thread is running");

    let there = answer
        .blocking_recv()
        .expect("an answer came back")
        .expect("this engine never fails");
    let here = format!("{:?}", thread::current().id());
    assert_ne!(there, here);
}

#[test]
fn an_engine_failure_comes_back_as_the_answer() {
    // A failed reply is still an answer. It goes back like any other.
    let questions = start(Box::new(BrokenEngine));
    let (answer_to, mut answer) = channel(1);

    questions
        .send(Question {
            text: "Did you take the ledger?".to_string(),
            answer_to,
        })
        .expect("the engine thread is running");

    assert_eq!(
        answer.blocking_recv(),
        Some(Err(AppError::Inference {
            message: "out of memory".to_string()
        }))
    );
}
