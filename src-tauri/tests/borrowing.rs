//! Stage 4 — borrowing, `Option<&T>`, and lifetimes
//!
//! Run with:  cargo test --test borrowing      (from src-tauri/)
//!
//! Stage 3 handed out owned data: `facts_known_by` copied ids out of the case so
//! nothing was left pointing back into it. This file does the opposite. Every
//! interesting assertion below is about a value that *lives inside the case* and
//! is only being looked at from outside.

use interrogatory_ai_lib::case::{longer_statement, Case, Fact, Suspect};
use interrogatory_ai_lib::ids::{FactId, SuspectId};

fn marta() -> SuspectId {
    SuspectId::new(1)
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

/// The Ledger, with three facts:
///   #1 known by both, #2 known by Marta only, #9 the solution (ground-truth-only).
fn the_ledger() -> Case {
    let mut case = Case::new(
        "The Ledger",
        "A bookkeeper is dead and the safe is short 40,000.",
    );
    case.add_suspect(Suspect::new(marta(), "Marta Reyes"));
    case.add_suspect(Suspect::new(viktor(), "Viktor Lang"));

    let mut opened = Fact::new(FactId::new(1), "The safe was opened at 21:40.");
    opened.reveal_to(marta());
    opened.reveal_to(viktor());

    let mut argument = Fact::new(FactId::new(2), "Marta argued with the victim on Tuesday.");
    argument.reveal_to(marta());

    let mut solution = Fact::new(FactId::new(9), "Marta forged the second signature.");
    solution.is_ground_truth_only = true;
    solution.reveal_to(marta());

    case.add_fact(opened);
    case.add_fact(argument);
    case.add_fact(solution);
    case
}

// ---------------------------------------------------------------- Option<&T>

#[test]
fn a_missing_suspect_is_none_not_a_panic() {
    let case = the_ledger();
    assert!(case.suspect(SuspectId::new(99)).is_none());
}

#[test]
fn a_present_suspect_is_some_reference() {
    let case = the_ledger();
    let s = case.suspect(viktor()).expect("Viktor is in this case");
    assert_eq!(s.name, "Viktor Lang");
    assert_eq!(s.id, viktor());
}

#[test]
fn the_lookup_returns_a_borrow_not_a_copy() {
    // Two lookups, two references — pointing at the same bytes inside `case`.
    // If `suspect` returned an owned `Suspect`, these would be two different
    // addresses and this assert would fail.
    let case = the_ledger();
    let first = case.suspect(marta()).expect("Marta is in this case");
    let second = case.suspect(marta()).expect("Marta is in this case");
    assert!(std::ptr::eq(first, second));

    // Both borrows are alive at the same time, and so is a third look at the
    // case itself. Shared borrows do not conflict: many readers is fine.
    assert_eq!(case.suspect_count(), 2);
    assert_eq!(first.name, second.name);
}

// ------------------------------------------------------------------- &mut T

#[test]
fn a_fact_can_be_edited_through_the_case() {
    let mut case = the_ledger();

    // This is the method you wished existed at the end of Stage 3.
    let fact = case
        .fact_mut(FactId::new(2))
        .expect("fact #2 is in this case");
    fact.reveal_to(viktor());

    // The edit landed in the case, not in a copy of it.
    assert_eq!(case.suspect_facts(viktor()).count(), 2);
}

#[test]
fn editing_a_missing_fact_is_none() {
    let mut case = the_ledger();
    assert!(case.fact_mut(FactId::new(404)).is_none());
}

#[test]
fn one_writer_excludes_every_reader() {
    let mut case = the_ledger();

    let fact = case
        .fact_mut(FactId::new(1))
        .expect("fact #1 is in this case");

    // Uncomment the next line, run the test, read the error, comment it back.
    // The exclusive borrow held by `fact` is still alive, because `fact` is
    // used again below. `E0502` is the whole lesson of this test.
    // assert_eq!(case.fact_count(), 3);

    fact.statement.push_str(" Confirmed by the alarm log.");

    // Here the borrow is over — `fact` is never used again — so the case is
    // readable once more. That "a borrow ends at its last use, not at the end
    // of the scope" rule has a name: non-lexical lifetimes.
    assert_eq!(case.fact_count(), 3);
}

// ------------------------------------------------- impl Iterator<Item = &Fact>

#[test]
fn suspect_facts_yields_the_facts_themselves() {
    let case = the_ledger();

    let statements: Vec<&str> = case
        .suspect_facts(marta())
        .map(|f| f.statement.as_str())
        .collect();

    assert_eq!(
        statements,
        vec![
            "The safe was opened at 21:40.",
            "Marta argued with the victim on Tuesday.",
        ]
    );
}

#[test]
fn suspect_facts_still_hides_the_ground_truth() {
    let case = the_ledger();

    let ids: Vec<FactId> = case.suspect_facts(marta()).map(|f| f.id).collect();
    assert_eq!(ids, vec![FactId::new(1), FactId::new(2)]);

    assert_eq!(case.suspect_facts(viktor()).count(), 1);
    assert_eq!(case.suspect_facts(SuspectId::new(99)).count(), 0);

    // Fact #9 is in the case, and reachable by other means. It is only
    // `suspect_facts` that refuses to hand it over.
    assert_eq!(case.fact_count(), 3);
}

#[test]
fn suspect_facts_does_no_work_until_it_is_asked() {
    let case = the_ledger();

    // Building the iterator allocates nothing and visits nothing. It is a
    // description of work, not the result of it.
    let mut facts = case.suspect_facts(marta());

    assert_eq!(facts.next().map(|f| f.id), Some(FactId::new(1)));
    assert_eq!(facts.next().map(|f| f.id), Some(FactId::new(2)));
    assert_eq!(facts.next().map(|f| f.id), None);
}

#[test]
fn the_iterator_borrows_the_case_it_came_from() {
    let case = the_ledger();
    let facts = case.suspect_facts(marta());

    // `case` is still readable while `facts` exists — shared borrows stack.
    assert_eq!(case.suspect_count(), 2);
    assert_eq!(facts.count(), 2);

    // What you cannot do is let the iterator outlive the case:
    //
    //     let facts = {
    //         let case = the_ledger();
    //         case.suspect_facts(marta())
    //     };                                  // <- case dropped here
    //     facts.count();                      // E0505 / E0597
    //
    // Try it in a scratch test if you want to see the error. That is the
    // dangling-pointer bug from Stage 3's `a_statement_is_owned_not_borrowed`,
    // caught at compile time again — this time on a whole iterator.
}

// ------------------------------------------------------- explicit lifetimes

#[test]
fn longer_statement_returns_one_of_its_arguments() {
    let short = Fact::new(FactId::new(1), "The safe was opened at 21:40.");
    let long = Fact::new(
        FactId::new(2),
        "Marta argued with the victim on Tuesday, loudly enough for the floor below to hear it.",
    );

    let winner = longer_statement(&short, &long);
    assert_eq!(winner.id, FactId::new(2));

    // Not a clone, not a new Fact — the very same one that was passed in.
    assert!(std::ptr::eq(winner, &long));
}

#[test]
fn longer_statement_prefers_the_first_on_a_tie() {
    let a = Fact::new(FactId::new(1), "Twelve chars");
    let b = Fact::new(FactId::new(2), "Twelve chars");
    assert!(std::ptr::eq(longer_statement(&a, &b), &a));
}
