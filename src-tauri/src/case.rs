use std::collections::HashSet;

use crate::{
    error::{AppError, AppResult},
    ids::{FactId, SuspectId},
};

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

/// A fact one suspect is allowed to talk about, borrowed from the case it came from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisibleFact<'a>(&'a Fact);

impl<'a> VisibleFact<'a> {
    pub fn id(&self) -> FactId {
        self.0.id
    }
    pub fn statement(&self) -> &'a str {
        &self.0.statement
    }
}

/// The statements of some visible facts, in order.
pub fn visible_statements<'a>(facts: &[VisibleFact<'a>]) -> Vec<&'a str> {
    facts.iter().map(|f| f.statement()).collect()
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

    /// The suspect with this id, or `None` if the case has no such suspect.
    pub fn suspect(&self, id: SuspectId) -> Option<&Suspect> {
        self.suspects.iter().find(|suspect| suspect.id == id)
    }

    /// Exclusive access to one fact, so callers can reveal it or edit it in place.
    pub fn fact_mut(&mut self, id: FactId) -> Option<&mut Fact> {
        self.facts.iter_mut().find(|fact| fact.id == id)
    }

    /// Every fact this suspect knows and is allowed to see, borrowed from the case.
    /// Same visibility rule as `facts_known_by`: ground-truth-only facts never appear.
    pub fn suspect_facts(&self, suspect: SuspectId) -> impl Iterator<Item = &Fact> {
        self.facts
            .iter()
            .filter(move |fact| !fact.is_ground_truth_only && fact.is_known_by(suspect))
    }

    /// The suspect with this id. `Err` if the case has no such suspect.
    pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> {
        self.suspect(id).ok_or(AppError::SuspectNotFound { id })
    }

    /// Exclusive access to one fact, so the caller can edit it in place.
    /// `Err` if this case has no such fact.
    pub fn require_fact_mut(&mut self, id: FactId) -> AppResult<&mut Fact> {
        self.fact_mut(id).ok_or(AppError::FactNotFound { id })
    }

    /// Lets one suspect in on one fact — from here on it can show up in their answers.
    /// Nothing changes unless both the suspect and the fact exist.
    pub fn reveal(&mut self, fact: FactId, to: SuspectId) -> AppResult<()> {
        self.require_suspect(to)?;
        let fact = self.require_fact_mut(fact)?;
        fact.reveal_to(to);
        Ok(())
    }

    /// Every fact this suspect may talk about, wrapped so a prompt cannot be
    /// handed anything else.
    pub fn visible_to<'a>(&'a self, suspect: SuspectId) -> Vec<VisibleFact<'a>> {
        todo!()
    }
}

/// Whichever of the two facts has the longer statement; `a` on a tie.
pub fn longer_statement<'a>(a: &'a Fact, b: &'a Fact) -> &'a Fact {
    if a.statement.len() < b.statement.len() {
        b
    } else {
        a
    }
}
