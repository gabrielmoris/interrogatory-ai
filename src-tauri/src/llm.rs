use crate::error::AppResult;

/// Anything that can answer the detective. The game only talks to this.
pub trait InferenceEngine: Send + Sync {
    /// Answers the detective one piece at a time: calls `say` once per piece.
    fn reply(&self, prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()>;
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
    /// Says its line one word at a time.
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()> {
        for word in self.line.split_inclusive(' ') {
            say(word);
        }
        Ok(())
    }
}
