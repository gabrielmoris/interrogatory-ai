use crate::ids::{FactId, SuspectId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
/// Every fallible function in the
/// app fails with one of these, and the React side branches on `kind`.
pub enum AppError {
    #[error("this case has no {id}")]
    SuspectNotFound { id: SuspectId },

    #[error("this case has no {id}")]
    FactNotFound { id: FactId },

    #[error(r#"no case file named "{slug}" was found"#)]
    CaseNotFound { slug: String },

    #[error("could not read {path}: {message}")]
    Io { path: String, message: String },

    #[error("{path} is not a valid case file: {message}")]
    Parse { path: String, message: String },

    #[error("the inference engine failed: {message}")]
    Inference { message: String },

    #[error("cannot {action} while {state}")]
    InvalidState { action: String, state: String },
}

/// A `Result` that fails with `AppError`. Use it instead of spelling out  `Result<T, AppError>` in every signature.
pub type AppResult<T> = Result<T, AppError>;
