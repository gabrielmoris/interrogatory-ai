//! Stage 1 — Difficulty & Tuning
//!
//! Run with:  cargo test --test difficulty       (from src-tauri/)
//!
//! This is an *integration* test: it lives outside `src/`, so it can only
//! reach items you have marked `pub`. If something here does not resolve,
//! it is either missing or not public.

use interrogatory_ai_lib::difficulty::{Difficulty, Tuning};

#[test]
fn all_lists_every_difficulty_easiest_first() {
    assert_eq!(Difficulty::ALL.len(), 4);
    assert_eq!(Difficulty::ALL[0], Difficulty::Easy);
    assert_eq!(Difficulty::ALL[1], Difficulty::Normal);
    assert_eq!(Difficulty::ALL[2], Difficulty::Hard);
    assert_eq!(Difficulty::ALL[3], Difficulty::Nightmare);
}

#[test]
fn difficulty_is_debug_printable() {
    assert_eq!(format!("{:?}", Difficulty::Hard), "Hard");
}

#[test]
fn easy_is_forgiving() {
    let t: Tuning = Difficulty::Easy.tuning();
    assert_eq!(t.temperature, 0.5);
    assert_eq!(t.facts_volunteered_per_turn, 2);
    assert!(!t.will_lie, "an easy suspect tells the truth, just slowly");
}

#[test]
fn nightmare_is_hostile() {
    let t: Tuning = Difficulty::Nightmare.tuning();
    assert_eq!(t.temperature, 1.1);
    assert_eq!(t.facts_volunteered_per_turn, 0);
    assert!(t.will_lie, "a nightmare suspect lies");
}

#[test]
fn temperature_rises_strictly_with_difficulty() {
    let temps: Vec<f32> = Difficulty::ALL
        .iter()
        .map(|d| d.tuning().temperature)
        .collect();
    for pair in temps.windows(2) {
        assert!(
            pair[1] > pair[0],
            "temperature must increase with difficulty, got {temps:?}"
        );
    }
}

#[test]
fn suspects_volunteer_no_more_as_difficulty_rises() {
    let facts: Vec<u8> = Difficulty::ALL
        .iter()
        .map(|d| d.tuning().facts_volunteered_per_turn)
        .collect();
    for pair in facts.windows(2) {
        assert!(
            pair[1] <= pair[0],
            "harder suspects must not volunteer more, got {facts:?}"
        );
    }
}

#[test]
fn lying_once_started_never_stops() {
    // Whatever values you pick, dishonesty must be monotonic:
    // no difficulty above a lying one may be honest again.
    let mut seen_liar = false;
    for d in Difficulty::ALL {
        let lies = d.tuning().will_lie;
        if seen_liar {
            assert!(
                lies,
                "{d:?} must lie because an easier difficulty already does"
            );
        }
        seen_liar |= lies;
    }
    assert!(seen_liar, "at least one difficulty must lie");
}
