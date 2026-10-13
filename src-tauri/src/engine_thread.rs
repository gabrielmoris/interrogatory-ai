use crate::error::AppError;
use crate::llm::InferenceEngine;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

/// One message on pipe 2: a piece of the reply, or how the reply ended.
#[derive(Debug, PartialEq)]
pub enum Said {
    /// One piece of the reply, sent the moment the engine says it.
    Piece(String),
    /// The reply is complete. Always the last message.
    Done,
    /// The engine failed. Always the last message.
    Failed(AppError),
}

/// One question for the engine, and where to send what the suspect says.
pub struct Question {
    pub text: String,
    pub answer_to: tauri::async_runtime::Sender<Said>,
}

/// Puts the engine on a thread of its own. Send questions to what comes back.
pub fn start(engine: Box<dyn InferenceEngine>) -> Sender<Question> {
    let (questions, inbox) = channel();
    thread::spawn(move || answer_all(engine, inbox));
    questions
}

/// Runs on the engine's thread. Answers each question until no one can ask any more.
fn answer_all(engine: Box<dyn InferenceEngine>, inbox: Receiver<Question>) {
    for question in inbox {
        let finished = engine.reply(&question.text, &mut |piece| {
            let _ = question
                .answer_to
                .blocking_send(Said::Piece(piece.to_string()));
        });

        let last = match finished {
            Ok(()) => Said::Done,
            Err(error) => Said::Failed(error),
        };

        let _ = question.answer_to.blocking_send(last);
    }
}
