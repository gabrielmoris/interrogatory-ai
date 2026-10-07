use crate::engine_thread::{start, Question};
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use crate::llm::InferenceEngine;
use crate::transcript::{Phase, Speaker};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use tauri::async_runtime::spawn_blocking;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
    phase: Mutex<Phase>,
    /// Where questions for the engine go. The engine itself lives on its own thread.
    questions: Sender<Question>,
}

impl AppState {
    pub fn new(cases_dir: PathBuf, engine: Box<dyn InferenceEngine>) -> Self {
        Self {
            cases_dir,
            phase: Mutex::new(Phase::Briefing),
            questions: start(engine),
        }
    }

    /// Calls a suspect in. Only from the briefing.
    pub fn begin(&self, suspect: SuspectId) -> AppResult<()> {
        let mut phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;
        phase.begin(suspect)
    }

    /// Keeps one line said in the room. Only during an interrogation.
    pub fn record(&self, speaker: Speaker, text: &str) -> AppResult<()> {
        let mut phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;

        phase.record(speaker, text)
    }

    /// The suspect in the room, or `None` outside it.
    pub fn suspect(&self) -> AppResult<Option<SuspectId>> {
        let phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;

        Ok(phase.suspect())
    }

    /// The suspect's reply to what the detective just asked.
    /// The engine answers on its own thread. `ask` waits for that answer on a background thread.
    pub async fn ask(&self, text: &str) -> AppResult<String> {
        self.record(Speaker::Detective, text)?;

        let (answer_to, answer) = channel();
        let question = Question {
            text: text.to_string(),
            answer_to,
        };

        self.questions
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;

        let waiting = spawn_blocking(move || answer.recv());
        let waited = waiting.await.map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?;

        let answered = waited.map_err(|e| AppError::Inference {
            message: e.to_string(),
        })?;
        let reply = answered?;

        self.record(Speaker::Suspect, &reply)?;
        Ok(reply)
    }

    /// How many lines have been said in the room. `0` outside it.
    pub fn turn_count(&self) -> AppResult<usize> {
        let phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;

        Ok(phase.turn_count())
    }
}
