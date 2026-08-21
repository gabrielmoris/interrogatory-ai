/// The single knob the player picks at case start; each level maps to a Tuning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

/// Settings one difficulty resolves to. Worth noting who consumes it
/// temperature: sampling randomness; higher is more erratic and harder to read..
/// facts_volunteered_per_turn: max facts revealed in a single answer; a ceiling, not a quota.
/// will_lie: whether the suspect may state things that are false, as opposed to merely evading.
#[derive(Debug, Clone, PartialEq)]
pub struct Tuning {
    pub temperature: f32,
    pub facts_volunteered_per_turn: u8,
    pub will_lie: bool,
}

/// Tunes the difficulty levels
impl Difficulty {
    pub const ALL: [Self; 4] = [Self::Easy, Self::Normal, Self::Hard, Self::Nightmare];

    pub fn tuning(self) -> Tuning {
        match self {
            Difficulty::Easy =>
                Tuning {
                    temperature: 0.5,
                    facts_volunteered_per_turn: 2,
                    will_lie: false,
                },
            Difficulty::Normal =>
                Tuning {
                    temperature: 0.7,
                    facts_volunteered_per_turn: 1,
                    will_lie: false,
                },
            Difficulty::Hard =>
                Tuning {
                    temperature: 0.9,
                    facts_volunteered_per_turn: 1,
                    will_lie: true,
                },
            Difficulty::Nightmare =>
                Tuning {
                    temperature: 1.1,
                    facts_volunteered_per_turn: 0,
                    will_lie: true,
                },
        }
    }
}
