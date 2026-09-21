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

    /// The error for a move this phase does not allow.
    fn refusal(&self, action: &str) -> AppError {
        AppError::InvalidState {
            action: action.to_string(),
            state: self.name().to_string(),
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
        if !matches!(self, Phase::Briefing) {
            return Err(self.refusal("begin an interrogation"));
        }

        *self = Phase::Interrogating {
            suspect,
            turns: Vec::new(),
        };
        Ok(())
    }

    /// Leaves the room to write the report. Only from an interrogation.
    pub fn finish(&mut self) -> AppResult<()> {
        if !matches!(self, Phase::Interrogating { .. }) {
            return Err(self.refusal("finish an interrogation"));
        }

        *self = Phase::Reporting;

        Ok(())
    }

    /// Keeps one line said in the room. Only during an interrogation.
    pub fn record(&mut self, speaker: Speaker, text: &str) -> AppResult<()> {
        match self {
            Phase::Interrogating { turns, .. } => {
                turns.push(Turn {
                    speaker,
                    text: text.to_string(),
                });
                Ok(())
            }
            _ => Err(self.refusal("record a line")),
        }
    }

    /// Every line said in this room, oldest first. Empty outside the room.
    pub fn transcript(&self) -> &[Turn] {
        match self {
            Phase::Interrogating { turns, .. } => turns,
            _ => &[],
        }
    }

    /// The line said most recently, or `None` if nothing has been said.
    pub fn last_line(&self) -> Option<&Turn> {
        self.transcript().last()
    }
}
