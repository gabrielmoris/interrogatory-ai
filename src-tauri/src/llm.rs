use crate::error::AppResult;

/// Anything that can answer the detective. The game only talks to this.
pub trait InferenceEngine: Send + Sync {
    /// Answers the detective one piece at a time: calls `say` once per piece.
    /// `say` gives back `true` while someone is listening, and `false` once no one is:
    /// then the engine stops talking.
    /// `Ok(())`: the reply ended, whole or stopped. `Err`: the engine failed.
    fn reply(&self, prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()>;
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
    /// Says its line one word at a time. Stops as soon as no one is listening.
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
        for word in self.line.split_inclusive(' ') {
            std::thread::sleep(std::time::Duration::from_millis(300));

            if !say(word) {
                break;
            }
        }
        Ok(())
    }
}
