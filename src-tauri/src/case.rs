use std::collections::HashSet;

use crate::ids::{FactId, SuspectId};

/// One person the player can interrogate.
#[derive(Debug, Clone, PartialEq)]
pub struct Suspect {
    pub id: SuspectId,
    pub name: String,
}

impl Suspect {
    pub fn new(id: SuspectId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

/// One true statement about the crime, plus who is allowed to know it.
#[derive(Debug, Clone, PartialEq)]
pub struct Fact {
    pub id: FactId,
    pub statement: String,
    pub known_by: HashSet<SuspectId>,
    pub is_ground_truth_only: bool,
}

impl Fact {
    pub fn new(id: FactId, statement: &str) -> Self {
        Self {
            id,
            statement: statement.to_string(),
            known_by: HashSet::new(),
            is_ground_truth_only: false,
        }
    }

    pub fn reveal_to(&mut self, suspect: SuspectId) {
        self.known_by.insert(suspect);
    }

    pub fn is_known_by(&self, suspect: SuspectId) -> bool {
        self.known_by.contains(&suspect)
    }
}

/// One playable case: the briefing, the cast, and the ground truth.
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub title: String,
    pub briefing: String,
    suspects: Vec<Suspect>,
    facts: Vec<Fact>,
}

impl Case {
    pub fn new(title: &str, briefing: &str) -> Self {
        Self {
            title: title.to_string(),
            briefing: briefing.to_string(),
            suspects: Vec::new(),
            facts: Vec::new(),
        }
    }

    pub fn add_suspect(&mut self, suspect: Suspect) {
        self.suspects.push(suspect);
    }

    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }

    pub fn suspect_count(&self) -> usize {
        self.suspects.len()
    }

    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }

    pub fn facts_known_by(&self, suspect: SuspectId) -> Vec<FactId> {
        self.facts
            .iter()
            .filter(|fact| !fact.is_ground_truth_only && fact.is_known_by(suspect))
            .map(|fact| fact.id)
            .collect()
    }

    /// The suspect with this id, or `None` if the case has no such suspect.
    pub fn suspect(&self, id: SuspectId) -> Option<&Suspect> {
        todo!()
    }

    /// Exclusive access to one fact, so callers can reveal it or edit it in place.
    pub fn fact_mut(&mut self, id: FactId) -> Option<&mut Fact> {
        todo!()
    }

    /// Every fact this suspect knows and is allowed to see, borrowed from the case.
    /// Same visibility rule as `facts_known_by`: ground-truth-only facts never appear.
    pub fn suspect_facts(&self, suspect: SuspectId) -> impl Iterator<Item = &Fact> {
        std::iter::empty() // `todo!()` does not compile here — see section 0
    }
}

/// Whichever of the two facts has the longer statement; `a` on a tie.
pub fn longer_statement<'a>(a: &'a Fact, b: &'a Fact) -> &'a Fact {
    todo!()
}
