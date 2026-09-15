use crate::case::Case;
use crate::error::AppResult;
use crate::ids::SuspectId;
use crate::storage::load_case;
use std::path::Path;

/// Where case files live, until Stage 9b lets the app choose the directory.
const CASES_DIR: &str = "cases";

/// One suspect as the briefing screen needs them.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SuspectSummary {
    pub id: SuspectId,
    pub name: String,
}

/// Everything the briefing screen is allowed to know about a case.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CaseIntro {
    pub title: String,
    pub briefing: String,
    pub suspects: Vec<SuspectSummary>,
}

impl From<&Case> for CaseIntro {
    fn from(case: &Case) -> Self {
        Self {
            title: case.title.clone(),
            briefing: case.briefing.clone(),
            suspects: case
                .suspects()
                .map(|suspect| SuspectSummary {
                    id: suspect.id,
                    name: suspect.name.clone(),
                })
                .collect(),
        }
    }
}

#[tauri::command]
pub fn case_intro(slug: String) -> AppResult<CaseIntro> {
    let case = load_case(Path::new(CASES_DIR), &slug)?;
    Ok(CaseIntro::from(&case))
}
