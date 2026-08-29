use crate::case::Case;
use crate::error::{AppError, AppResult};
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
        let case = Case::new(&raw.title, &raw.briefing);
        Ok(case)
    }
}

pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    todo!()
}
