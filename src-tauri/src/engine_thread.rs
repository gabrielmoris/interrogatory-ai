use crate::error::AppResult;
use crate::llm::InferenceEngine;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

/// One question for the engine, and where to send its answer.
pub struct Question {
    pub text: String,
    pub answer_to: tauri::async_runtime::Sender<AppResult<String>>,
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
        let mut heard = String::new();
        let said = engine.reply(&question.text, &mut |piece| heard.push_str(piece));
        let answer = match said {
            Ok(()) => Ok(heard),
            Err(error) => Err(error),
        };
        let _ = question.answer_to.blocking_send(answer);
    }
}
