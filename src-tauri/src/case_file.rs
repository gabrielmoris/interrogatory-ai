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
            case.add_suspect(Suspect::new(
                SuspectId::new(raw_suspect.id),
                &raw_suspect.name,
            ));
        }

        for raw_fact in &raw.facts {
            let mut fact = Fact::new(FactId::new(raw_fact.id), &raw_fact.statement);
            fact.is_ground_truth_only = raw_fact.is_ground_truth_only;

            for raw_known_by in &raw_fact.known_by {
                fact.reveal_to(SuspectId::new(*raw_known_by));
            }

            case.add_fact(fact);
        }

        Ok(case)
    }
}

pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    todo!()
}
