use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;

/// Who said one line in the interrogation room.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speaker {
    Detective,
    Suspect,
}

/// One line said in the interrogation room.
#[derive(Debug, Clone, PartialEq)]
pub struct Turn {
    pub speaker: Speaker,
    pub text: String,
}

/// Which part of the case the player is in.
#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    Briefing,
    Interrogating {
        suspect: SuspectId,
        turns: Vec<Turn>,
    },
    Reporting,
}

impl Phase {
    /// The suspect in the room, or `None` outside it.
    pub fn suspect(&self) -> Option<SuspectId> {
        match self {
            Phase::Interrogating { suspect, .. } => Some(*suspect),
            _ => None,
        }
    }

    /// How many lines have been said in the room. `0` outside it.
    pub fn turn_count(&self) -> usize {
        match self {
            Phase::Interrogating { turns, .. } => turns.len(),
            _ => 0,
        }
    }

    /// Where the player is, in words, for an error message.
    pub fn name(&self) -> &str {
        match self {
            Phase::Briefing => "the briefing",
            Phase::Interrogating { .. } => "an interrogation",
            Phase::Reporting => "the report",
        }
    }

    /// Calls a suspect in. Only from the briefing.
    pub fn begin(&mut self, suspect: SuspectId) -> AppResult<()> {
        if !matches!(&self, Phase::Briefing) {
            return Err(AppError::InvalidState {
                action: "begin an interrogation".to_string(),
                state: self.name().to_string(),
            });
        }

        *self = Phase::Interrogating {
            suspect,
            turns: Vec::new(),
        };
        Ok(())
    }

    /// Leaves the room to write the report. Only from an interrogation.
    pub fn finish(&mut self) -> AppResult<()> {
        if !matches!(&self, Phase::Interrogating { .. }) {
            return Err(AppError::InvalidState {
                action: "finish an interrogation".to_string(),
                state: self.name().to_string(),
            });
        }

        *self = Phase::Reporting;

        Ok(())
    }
}
