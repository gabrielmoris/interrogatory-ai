use crate::engine_thread::{start, Question, Said};
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use crate::llm::InferenceEngine;
use crate::transcript::{Phase, Speaker};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use tauri::async_runtime::channel;
use tauri::ipc::Channel;
use tokio_util::sync::CancellationToken;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
    phase: Mutex<Phase>,
    /// Where questions for the engine go. The engine itself lives on its own thread.
    questions: Sender<Question>,
    /// The stop sign of the question being answered now. `stop` puts it up.
    stop_sign: Mutex<CancellationToken>,
}

impl AppState {
    pub fn new(cases_dir: PathBuf, engine: Box<dyn InferenceEngine>) -> Self {
        Self {
            cases_dir,
            phase: Mutex::new(Phase::Briefing),
            questions: start(engine),
            stop_sign: Mutex::new(CancellationToken::new()),
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

    /// The player pressed Stop: the reply being said now ends where it is.
    pub fn stop(&self) -> AppResult<()> {
        let stop_sign = self.stop_sign.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;
        stop_sign.cancel();
        Ok(())
    }

    /// A new stop sign for the question that starts now.
    /// `AppState` keeps one hand on it, for `stop`. `ask` gets the other.
    fn new_stop_sign(&self) -> AppResult<CancellationToken> {
        let mut stop_sign = self.stop_sign.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;

        *stop_sign = CancellationToken::new();
        Ok(stop_sign.clone())
    }

    /// The suspect's reply to what the detective just asked.
    /// The engine says it piece by piece on its own thread. `ask` sends each piece to React
    /// through `on_piece` the moment it comes, and puts the pieces together.
    /// If the player presses Stop, the reply ends where it is, and that much is kept.
    pub async fn ask(&self, text: &str, on_piece: Channel<String>) -> AppResult<String> {
        let stop_sign = self.new_stop_sign()?;

        self.record(Speaker::Detective, text)?;

        let (answer_to, mut answer) = channel(1);

        let question = Question {
            text: text.to_string(),
            answer_to,
        };

        self.questions
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;

        let mut reply = String::new();

        loop {
            let said = answer.recv().await.ok_or(AppError::Inference {
                message: "the engine stopped before it answered".to_string(),
            })?;

            if stop_sign.is_cancelled() {
                break;
            }

            match said {
                Said::Piece(piece) => {
                    reply.push_str(&piece);

                    on_piece
                        .send(piece)
                        .map_err(|e| AppError::WindowUnreachable {
                            message: e.to_string(),
                        })?;
                }
                Said::Done => break,
                Said::Failed(error) => return Err(error),
            }
        }

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
