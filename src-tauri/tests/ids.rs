//! Stage 2 — `SuspectId` and `FactId`
//!
//! Run with:  cargo test --test ids        (from src-tauri/)
//!
//! Integration test again: it can only see what is `pub`. Note what it does
//! *not* do — it never touches `.0`. That is deliberate. The inner field must
//! stay private; the only way in is your constructor.

use std::collections::{ HashMap, HashSet };

use interrogatory_ai_lib::ids::{ FactId, SuspectId };

#[test]
fn ids_wrap_a_number_and_give_it_back() {
    let s = SuspectId::new(3);
    let f = FactId::new(7);
    assert_eq!(s.get(), 3);
    assert_eq!(f.get(), 7);
}

#[test]
fn ids_are_debug_printable() {
    assert_eq!(format!("{:?}", SuspectId::new(3)), "SuspectId(3)");
    assert_eq!(format!("{:?}", FactId::new(7)), "FactId(7)");
}

#[test]
fn ids_display_for_humans() {
    // `.to_string()` is not something you write. Find out where it comes from.
    assert_eq!(SuspectId::new(3).to_string(), "suspect #3");
    assert_eq!(FactId::new(7).to_string(), "fact #7");
    assert_eq!(format!("{}", FactId::new(0)), "fact #0");
}

#[test]
fn ids_compare_by_their_number() {
    assert_eq!(SuspectId::new(1), SuspectId::new(1));
    assert_ne!(SuspectId::new(1), SuspectId::new(2));
    assert!(FactId::new(1) < FactId::new(2));
    assert!(SuspectId::new(9) >= SuspectId::new(9));
}

#[test]
fn ids_sort() {
    let mut facts = vec![FactId::new(9), FactId::new(2), FactId::new(5)];
    facts.sort();
    assert_eq!(facts, vec![FactId::new(2), FactId::new(5), FactId::new(9)]);
}

#[test]
fn ids_work_as_map_keys() {
    // This is the shape `Case` will really use: who knows which facts.
    let mut known: HashMap<SuspectId, Vec<FactId>> = HashMap::new();
    known.entry(SuspectId::new(1)).or_default().push(FactId::new(7));
    known.entry(SuspectId::new(1)).or_default().push(FactId::new(8));
    known.entry(SuspectId::new(2)).or_default().push(FactId::new(7));

    assert_eq!(known.len(), 2);
    assert_eq!(known[&SuspectId::new(1)].len(), 2);
    assert_eq!(known[&SuspectId::new(2)], vec![FactId::new(7)]);
}

#[test]
fn ids_work_in_sets_and_dedupe() {
    // And this is the shape scoring will use: facts the player cited.
    let cited: HashSet<FactId> = [FactId::new(4), FactId::new(4), FactId::new(9)]
        .into_iter()
        .collect();
    assert_eq!(cited.len(), 2);
    assert!(cited.contains(&FactId::new(9)));
    assert!(!cited.contains(&FactId::new(5)));
}

#[test]
fn ids_are_copied_not_moved() {
    let a = SuspectId::new(5);
    let b = a; // if this were a move, `a` would be dead on the next line
    assert_eq!(a, b);

    let ids = [FactId::new(1), FactId::new(2)];
    let total: u32 = ids
        .iter()
        .map(|f| f.get())
        .sum();
    assert_eq!(total, 3);
}

#[test]
fn plain_numbers_convert_in() {
    let s = SuspectId::from(3);
    assert_eq!(s, SuspectId::new(3));

    // `.into()` is not something you write either. Same question as `to_string`.
    let f: FactId = (7u32).into();
    assert_eq!(f, FactId::new(7));
}

#[test]
fn the_wrapper_costs_nothing_at_runtime() {
    // The whole argument for the newtype pattern is in these two lines:
    // a real, distinct, un-confusable type that compiles down to a bare u32.
    assert_eq!(std::mem::size_of::<SuspectId>(), std::mem::size_of::<u32>());
    assert_eq!(std::mem::size_of::<FactId>(), std::mem::size_of::<u32>());
}
