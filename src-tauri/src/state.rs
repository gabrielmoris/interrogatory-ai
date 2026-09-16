use std::path::PathBuf;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
}

impl AppState {
    pub fn new(cases_dir: PathBuf) -> Self {
        Self { cases_dir }
    }
}
