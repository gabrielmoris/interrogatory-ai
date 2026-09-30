use tauri::async_runtime::spawn_blocking;
use tauri::State;

use crate::case::Case;
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use crate::state::AppState;
use crate::storage::load_case;
use std::path::{Path, PathBuf};

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

pub fn case_intro_from(cases_dir: &Path, slug: &str) -> AppResult<CaseIntro> {
    let case = load_case(cases_dir, slug)?;
    Ok(CaseIntro::from(&case))
}

/// Reads the case on a separate thread, so the window never waits for the disk.
pub async fn case_intro_in_background(cases_dir: PathBuf, slug: String) -> AppResult<CaseIntro> {
    spawn_blocking(move || case_intro_from(&cases_dir, &slug))
        .await
        .map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?
}

#[tauri::command]
pub async fn case_intro(state: State<'_, AppState>, slug: String) -> AppResult<CaseIntro> {
    case_intro_in_background(state.cases_dir.clone(), slug).await
}

/// Calls in the suspect the player picked, once the case is known to have them.
pub fn begin_interrogation_from(state: &AppState, slug: &str, suspect: SuspectId) -> AppResult<()> {
    let case = load_case(&state.cases_dir, slug)?;
    case.require_suspect(suspect)?;
    state.begin(suspect)
}

#[tauri::command]
pub fn begin_interrogation(
    state: State<'_, AppState>,
    slug: String,
    suspect: SuspectId,
) -> AppResult<()> {
    begin_interrogation_from(&state, &slug, suspect)
}

#[tauri::command]
pub async fn ask_suspect(state: State<'_, AppState>, question: String) -> AppResult<String> {
    state.ask(&question).await
}
