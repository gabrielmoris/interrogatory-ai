use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use crate::transcript::Phase;
use std::path::PathBuf;
use std::sync::Mutex;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
    phase: Mutex<Phase>,
}

impl AppState {
    pub fn new(cases_dir: PathBuf) -> Self {
        Self {
            cases_dir,
            phase: Mutex::new(Phase::Briefing),
        }
    }

    /// Calls a suspect in. Only from the briefing.
    pub fn begin(&self, suspect: SuspectId) -> AppResult<()> {
        let mut phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;
        phase.begin(suspect)
    }
}
