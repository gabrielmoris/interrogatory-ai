use crate::case::Case;
use crate::error::{AppError, AppResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawCase {
    pub title: String,
    #[serde(default)]
    pub facts: Vec<RawFact>,
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

impl TryFrom<RawCase> for Case {
    type Error = AppError;

    fn try_from(raw: RawCase) -> AppResult<Self> {
        todo!()
    }
}

pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    todo!()
}
