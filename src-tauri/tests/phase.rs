//! Stage 10a — where the player is
//!
//! Run with:  cargo test --test phase      (from src-tauri/)
//!
//! A case is played in three parts: the briefing, the interrogation room, and
//! the report. Only one of them has somebody sitting across the table, and
//! only one of them has a conversation so far.
//!
//! So "who is in the room" and "what has been said" are not fields every part
//! carries and most leave empty. They live inside the interrogation, and the
//! only way to reach them is to first ask which part this is.

use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::transcript::{Phase, Speaker, Turn};

fn line(speaker: Speaker, text: &str) -> Turn {
    Turn {
        speaker,
        text: text.to_string(),
    }
}

/// Viktor Lang, three lines into his interrogation.
fn questioning_viktor() -> Phase {
    Phase::Interrogating {
        suspect: SuspectId::new(2),
        turns: vec![
            line(Speaker::Detective, "Where were you on Tuesday night?"),
            line(Speaker::Suspect, "At home. Alone."),
            line(Speaker::Detective, "Who can confirm that?"),
        ],
    }
}

/// Marta Reyes, sat down a moment ago. Nobody has spoken yet.
fn questioning_marta_from_the_start() -> Phase {
    Phase::Interrogating {
        suspect: SuspectId::new(1),
        turns: Vec::new(),
    }
}

#[test]
fn during_the_briefing_nobody_is_in_the_room() {
    assert_eq!(Phase::Briefing.suspect(), None);
}

#[test]
fn during_an_interrogation_the_phase_knows_who_is_in_the_room() {
    assert_eq!(questioning_viktor().suspect(), Some(SuspectId::new(2)));
    assert_eq!(
        questioning_marta_from_the_start().suspect(),
        Some(SuspectId::new(1))
    );
}

#[test]
fn while_writing_the_report_nobody_is_in_the_room() {
    assert_eq!(Phase::Reporting.suspect(), None);
}

#[test]
fn every_line_said_in_the_room_is_counted() {
    // Both sides of the table count: two questions and one answer.
    assert_eq!(questioning_viktor().turn_count(), 3);
}

#[test]
fn an_interrogation_that_just_started_has_no_lines_yet() {
    assert_eq!(questioning_marta_from_the_start().turn_count(), 0);
}

#[test]
fn outside_the_room_there_are_no_lines() {
    assert_eq!(Phase::Briefing.turn_count(), 0);
    assert_eq!(Phase::Reporting.turn_count(), 0);
}
