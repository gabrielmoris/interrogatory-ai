#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

pub struct Tuning {
    pub temperature: f32,
    pub facts_volunteered_per_turn: u8,
    pub will_lie: bool,
}

impl Difficulty {
    pub const ALL: [Difficulty; 4] = [
        Difficulty::Easy,
        Difficulty::Normal,
        Difficulty::Hard,
        Difficulty::Nightmare,
    ];

    pub fn tuning(self) -> Tuning {
        match self {
            Difficulty::Easy => Tuning {
                temperature: 0.5,
                facts_volunteered_per_turn: 2,
                will_lie: false,
            },
            Difficulty::Normal => Tuning {
                temperature: 0.7,
                facts_volunteered_per_turn: 1,
                will_lie: false,
            },
            Difficulty::Hard => Tuning {
                temperature: 0.9,
                facts_volunteered_per_turn: 1,
                will_lie: true,
            },
            Difficulty::Nightmare => Tuning {
                temperature: 1.1,
                facts_volunteered_per_turn: 0,
                will_lie: true,
            },
        }
    }
}
