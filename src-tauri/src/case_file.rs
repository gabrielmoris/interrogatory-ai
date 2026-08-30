use crate::case::{Case, Fact, Suspect};
use crate::error::{AppError, AppResult};
use crate::ids::{FactId, SuspectId};
use serde::Deserialize;

/// Raw case
#[derive(Debug, Deserialize)]
pub struct RawCase {
    pub title: String,
    pub briefing: String,
    #[serde(default)]
    pub facts: Vec<RawFact>,
    pub suspects: Vec<RawSuspect>,
}

#[derive(Debug, Deserialize)]
pub struct RawFact {
    pub id: u32,
    pub statement: String,
    #[serde(default)]
    pub known_by: Vec<u32>,
    #[serde(default)]
    pub is_ground_truth_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct RawSuspect {
    pub id: u32,
    pub name: String,
}

impl TryFrom<RawCase> for Case {
    type Error = AppError;

    fn try_from(raw: RawCase) -> AppResult<Self> {
        let mut case = Case::new(&raw.title, &raw.briefing);
        for raw_suspect in &raw.suspects {
            let suspect_id = SuspectId::new(raw_suspect.id);

            if case.suspect(suspect_id).is_some() {
                return Err(AppError::DuplicateSuspect { id: suspect_id });
            }

            case.add_suspect(Suspect::new(suspect_id, &raw_suspect.name));
        }

        for raw_fact in &raw.facts {
            let fact_id = FactId::new(raw_fact.id);

            if case.fact_mut(fact_id).is_some() {
                return Err(AppError::DuplicateFact { id: fact_id });
            }

            let mut fact = Fact::new(fact_id, &raw_fact.statement);
            fact.is_ground_truth_only = raw_fact.is_ground_truth_only;

            for raw_known_by in &raw_fact.known_by {
                let suspect_id = SuspectId::new(*raw_known_by);
                case.require_suspect(suspect_id)?;
                fact.reveal_to(suspect_id);
            }

            case.add_fact(fact);
        }

        for raw_suspect in &raw.suspects {
            let suspect_id = SuspectId::new(raw_suspect.id);

            if case.suspect_facts(suspect_id).next().is_none() {
                return Err(AppError::SuspectKnowsNothing { id: suspect_id });
            }
        }

        Ok(case)
    }
}

/// Tries to parse, but it it fails the toml error would be mapped to the correct AppError::Parse
pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    let raw: RawCase = toml::from_str(text).map_err(|e| AppError::Parse {
        path: path.to_string(),
        message: e.to_string(),
    })?;

    raw.try_into()
}
