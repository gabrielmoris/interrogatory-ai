use crate::case::{Case, Suspect};
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use serde::Deserialize;

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

        Ok(case)
    }
}

pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    todo!()
}
