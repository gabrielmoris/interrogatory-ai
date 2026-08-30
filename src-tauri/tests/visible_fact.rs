//! Stage 7 — the only fact a prompt is allowed to see
//!
//! Run with:  cargo test --test visible_fact      (from src-tauri/)
//!
//! `Case::suspect_facts` already answers "what may this suspect talk about",
//! and Stage 6c leaned on it. But its answer is a plain `&Fact`, and a plain
//! `&Fact` looks exactly like the solution — nothing stops a future
//! `build_prompt(&Fact)` from being handed fact 9.
//!
//! This file asks for a type that cannot be built by accident: `VisibleFact`,
//! made only inside `case.rs`, only by `Case::visible_to`. Once a function
//! takes `&[VisibleFact]`, the solution cannot reach it — not because a rule
//! says so, but because there is no value of that type to pass.

use interrogatory_ai_lib::case::{visible_statements, Case, Fact, Suspect, VisibleFact};
use interrogatory_ai_lib::case_file::parse_case;
use interrogatory_ai_lib::ids::{FactId, SuspectId};

const THE_LEDGER: &str = include_str!("cases/the-ledger.toml");

/// The Ledger, as a value: Marta (1) knows facts 1 and 2, Viktor (2) knows
/// facts 1 and 3, and fact 9 is the solution — known by Marta, visible to
/// nobody.
fn the_ledger() -> Case {
    parse_case(THE_LEDGER, "cases/the-ledger.toml").expect("the-ledger.toml is a case")
}

#[test]
fn visible_to_lists_what_this_suspect_may_talk_about() {
    let case = the_ledger();

    let seen = case.visible_to(SuspectId::new(1));
    let ids: Vec<FactId> = seen.iter().map(|fact| fact.id()).collect();

    assert_eq!(ids, vec![FactId::new(1), FactId::new(2)]);
}

#[test]
fn two_suspects_see_different_things() {
    let case = the_ledger();

    let marta: Vec<FactId> = case
        .visible_to(SuspectId::new(1))
        .iter()
        .map(|fact| fact.id())
        .collect();
    let viktor: Vec<FactId> = case
        .visible_to(SuspectId::new(2))
        .iter()
        .map(|fact| fact.id())
        .collect();

    assert_eq!(marta, vec![FactId::new(1), FactId::new(2)]);
    assert_eq!(viktor, vec![FactId::new(1), FactId::new(3)]);
}

#[test]
fn the_solution_is_visible_to_nobody() {
    let case = the_ledger();
    let solution = FactId::new(9);

    for suspect in [SuspectId::new(1), SuspectId::new(2)] {
        let seen = case.visible_to(suspect);
        assert!(
            !seen.iter().any(|fact| fact.id() == solution),
            "fact 9 is ground-truth-only and must never be handed out"
        );
    }
}

#[test]
fn a_suspect_who_is_not_in_the_case_sees_nothing() {
    let case = the_ledger();

    assert!(case.visible_to(SuspectId::new(99)).is_empty());
}

#[test]
fn the_statement_is_borrowed_from_the_case_not_copied_out_of_it() {
    let mut case = the_ledger();

    // Where fact 1's text lives inside the case.
    let address = case
        .fact_mut(FactId::new(1))
        .expect("fact 1 is in the case")
        .statement
        .as_ptr();

    let seen = case.visible_to(SuspectId::new(1));

    assert_eq!(
        seen[0].statement().as_ptr(),
        address,
        "visible_to must borrow the case's own text, not clone it"
    );
}

#[test]
fn visible_to_agrees_with_suspect_facts() {
    let case = the_ledger();
    let marta = SuspectId::new(1);

    let through_the_wrapper: Vec<FactId> = case
        .visible_to(marta)
        .iter()
        .map(|fact| fact.id())
        .collect();
    let straight: Vec<FactId> = case.suspect_facts(marta).map(|fact| fact.id).collect();

    assert_eq!(
        through_the_wrapper, straight,
        "same rule, same answer, same order"
    );
}

#[test]
fn a_function_that_takes_visible_facts_can_read_them() {
    let case = the_ledger();
    let seen = case.visible_to(SuspectId::new(2));

    assert_eq!(
        visible_statements(&seen),
        vec![
            "The safe was opened at 21:40.",
            "Viktor's key card opened the service door at 21:38.",
        ]
    );
}

#[test]
fn the_wrapper_costs_nothing_at_runtime() {
    assert_eq!(
        std::mem::size_of::<VisibleFact>(),
        std::mem::size_of::<&Fact>(),
        "VisibleFact is one reference and nothing else"
    );
}

#[test]
fn a_hand_built_case_gates_the_same_way() {
    let mut case = Case::new("Two rooms", "A test case built without a file.");
    case.add_suspect(Suspect::new(SuspectId::new(1), "Ada"));

    let mut ordinary = Fact::new(FactId::new(1), "The door was locked.");
    ordinary.reveal_to(SuspectId::new(1));
    case.add_fact(ordinary);

    let mut solution = Fact::new(FactId::new(2), "Ada took the key.");
    solution.is_ground_truth_only = true;
    solution.reveal_to(SuspectId::new(1));
    case.add_fact(solution);

    let seen = case.visible_to(SuspectId::new(1));

    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].statement(), "The door was locked.");
}
