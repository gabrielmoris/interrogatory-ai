//! Stage 3 — `Fact`, `Suspect` and `Case`
//!
//! Run with:  cargo test --test case        (from src-tauri/)
//!
//! Integration test again: it sees only what is `pub`. Two of `Case`'s four
//! fields are deliberately not among them — these tests reach the suspects and
//! the facts through methods, never directly.

use std::collections::HashSet;

use interrogatory_ai_lib::case::{Case, Fact, Suspect};
use interrogatory_ai_lib::ids::{FactId, SuspectId};

fn marta() -> SuspectId {
    SuspectId::new(1)
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

/// Stage 3 asked for this as `Case::facts_known_by` returning owned ids. Stage 4
/// replaced it with `suspect_facts`, a borrowed view, and the decision of
/// 2026-08-25 deleted the older one — one function owns the visibility rule.
/// These tests keep their original assertions, expressed through the survivor.
fn known_ids(case: &Case, suspect: SuspectId) -> Vec<FactId> {
    case.suspect_facts(suspect).map(|f| f.id).collect()
}

#[test]
fn a_suspect_owns_its_name() {
    let s = Suspect::new(marta(), "Marta Reyes");
    assert_eq!(s.id, marta());
    assert_eq!(s.name, "Marta Reyes");
}

#[test]
fn a_statement_is_owned_not_borrowed() {
    // The whole String-vs-&str lesson is in this block. `scratch` dies at the
    // closing brace. If `Fact` merely pointed at it, `f` would be left holding
    // a dangling pointer — and this would not compile.
    let f = {
        let scratch = String::from("The safe was opened at 21:40.");
        Fact::new(FactId::new(1), &scratch)
    };
    assert_eq!(f.statement, "The safe was opened at 21:40.");
}

#[test]
fn a_new_fact_is_known_by_nobody_and_is_not_hidden() {
    let f = Fact::new(FactId::new(1), "The safe was opened at 21:40.");
    assert_eq!(f.id, FactId::new(1));
    assert!(f.known_by.is_empty());
    assert!(!f.is_ground_truth_only);
    assert!(!f.is_known_by(marta()));
}

#[test]
fn revealing_a_fact_mutates_it_and_dedupes() {
    let mut f = Fact::new(FactId::new(1), "The safe was opened at 21:40.");
    f.reveal_to(viktor());
    f.reveal_to(marta());
    f.reveal_to(viktor()); // already there — this is a set, not a list

    let expected: HashSet<SuspectId> = [marta(), viktor()].into_iter().collect();
    assert_eq!(f.known_by, expected);
    assert!(f.is_known_by(marta()));
    assert!(!f.is_known_by(SuspectId::new(99)));
}

#[test]
fn a_case_starts_empty_and_grows() {
    let mut case = Case::new(
        "The Ledger",
        "A bookkeeper is dead and the safe is short 40,000.",
    );
    assert_eq!(case.title, "The Ledger");
    assert_eq!(case.suspect_count(), 0);
    assert_eq!(case.fact_count(), 0);

    case.add_suspect(Suspect::new(marta(), "Marta Reyes"));
    case.add_suspect(Suspect::new(viktor(), "Viktor Lang"));
    case.add_fact(Fact::new(FactId::new(1), "The safe was opened at 21:40."));

    assert_eq!(case.suspect_count(), 2);
    assert_eq!(case.fact_count(), 1);
}

#[test]
fn adding_a_fact_moves_it_into_the_case() {
    let mut case = Case::new("The Ledger", "...");
    let f = Fact::new(FactId::new(1), "The safe was opened at 21:40.");
    case.add_fact(f);

    // Uncomment the next line, run the test, read the error, then comment it
    // back. That error is the point of this test.
    // assert_eq!(f.id, FactId::new(1));

    assert_eq!(case.fact_count(), 1);
}

#[test]
fn a_suspect_sees_only_the_facts_they_know() {
    let mut case = Case::new("The Ledger", "...");
    case.add_suspect(Suspect::new(marta(), "Marta Reyes"));
    case.add_suspect(Suspect::new(viktor(), "Viktor Lang"));

    let mut opened = Fact::new(FactId::new(1), "The safe was opened at 21:40.");
    opened.reveal_to(marta());
    opened.reveal_to(viktor());

    let mut argument = Fact::new(FactId::new(2), "Marta argued with the victim on Tuesday.");
    argument.reveal_to(marta());

    let unlatched = Fact::new(FactId::new(3), "The window was unlatched from the inside.");

    case.add_fact(opened);
    case.add_fact(argument);
    case.add_fact(unlatched);

    // Order follows insertion order, because the facts live in a Vec.
    assert_eq!(
        known_ids(&case, marta()),
        vec![FactId::new(1), FactId::new(2)]
    );
    assert_eq!(known_ids(&case, viktor()), vec![FactId::new(1)]);
    assert!(known_ids(&case, SuspectId::new(99)).is_empty());
}

#[test]
fn ground_truth_only_facts_are_never_visible() {
    // This is the knowledge-gating rule the whole game rests on: a fact marked
    // ground-truth-only never reaches a suspect, even if it lists them.
    let mut case = Case::new("The Ledger", "...");
    case.add_suspect(Suspect::new(marta(), "Marta Reyes"));

    let mut solution = Fact::new(FactId::new(9), "Marta forged the second signature.");
    solution.is_ground_truth_only = true;
    solution.reveal_to(marta());
    case.add_fact(solution);

    assert_eq!(case.fact_count(), 1);
    assert!(known_ids(&case, marta()).is_empty());
}

#[test]
fn a_case_can_be_cloned_and_compared() {
    let mut case = Case::new("The Ledger", "...");
    case.add_suspect(Suspect::new(marta(), "Marta Reyes"));
    case.add_fact(Fact::new(FactId::new(1), "The safe was opened at 21:40."));

    let duplicate = case.clone();
    assert_eq!(duplicate, case);
    assert_eq!(duplicate.fact_count(), 1);

    // `case` is still usable after `.clone()` — but note that you had to ask.
    // Stage 2's ids copied themselves for free; these do not, and the reason is
    // the `String` and the `HashSet` hanging off the heap underneath.
    assert_eq!(case.suspect_count(), 1);
}
