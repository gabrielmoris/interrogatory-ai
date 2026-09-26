use crate::error::AppResult;

/// Anything that can answer the detective. The game only talks to this.
pub trait InferenceEngine {
    /// The suspect's reply to what the detective just said.
    fn reply(&self, prompt: &str) -> AppResult<String>;
}

/// A fake engine for tests: it always gives the same line.
pub struct MockEngine {
    line: String,
}

impl MockEngine {
    pub fn new(line: &str) -> Self {
        Self {
            line: line.to_string(),
        }
    }
}

impl InferenceEngine for MockEngine {
    fn reply(&self, _prompt: &str) -> AppResult<String> {
        Ok(self.line.clone())
    }
}
