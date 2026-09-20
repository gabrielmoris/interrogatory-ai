//! Stage 10c — recording a line
//!
//! Run with:  cargo test --test phase_record      (from src-tauri/)
//!
//! Every line said in the interrogation room is kept: who said it, what they
//! said, in the order they said it. The detective's questions and the
//! suspect's answers go in the same list.
//!
//! Lines exist only inside the room. During the briefing or the report there
//! is nowhere to keep one, so recording is refused — and refusing changes
//! nothing.

use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::transcript::{Phase, Speaker, Turn};

fn line(speaker: Speaker, text: &str) -> Turn {
    Turn {
        speaker,
        text: text.to_string(),
    }
}

/// Viktor Lang, sat down a moment ago. Nobody has spoken yet.
fn questioning_viktor() -> Phase {
    Phase::Interrogating {
        suspect: SuspectId::new(2),
        turns: Vec::new(),
    }
}

#[test]
fn a_line_said_in_the_room_is_kept() {
    let mut phase = questioning_viktor();

    assert_eq!(
        phase.record(Speaker::Detective, "Where were you on Tuesday night?"),
        Ok(())
    );
    assert_eq!(phase.turn_count(), 1);
}

#[test]
fn lines_are_kept_in_order_with_who_said_them() {
    let mut phase = questioning_viktor();

    phase
        .record(Speaker::Detective, "Where were you on Tuesday night?")
        .unwrap();
    phase.record(Speaker::Suspect, "At home. Alone.").unwrap();
    phase
        .record(Speaker::Detective, "Who can confirm that?")
        .unwrap();

    assert_eq!(
        phase,
        Phase::Interrogating {
            suspect: SuspectId::new(2),
            turns: vec![
                line(Speaker::Detective, "Where were you on Tuesday night?"),
                line(Speaker::Suspect, "At home. Alone."),
                line(Speaker::Detective, "Who can confirm that?"),
            ],
        },
        "same suspect, three lines, in the order they were said"
    );
}

#[test]
fn the_first_line_can_follow_straight_after_the_suspect_is_called_in() {
    let mut phase = Phase::Briefing;
    phase.begin(SuspectId::new(1)).unwrap();

    assert_eq!(
        phase.record(Speaker::Detective, "Sit down, Ms Reyes."),
        Ok(())
    );
    assert_eq!(phase.suspect(), Some(SuspectId::new(1)));
    assert_eq!(phase.turn_count(), 1);
}

#[test]
fn nothing_can_be_recorded_during_the_briefing() {
    let mut phase = Phase::Briefing;

    assert_eq!(
        phase.record(Speaker::Detective, "Where were you on Tuesday night?"),
        Err(AppError::InvalidState {
            action: "record a line".to_string(),
            state: "the briefing".to_string(),
        })
    );
    assert_eq!(phase, Phase::Briefing, "a refused line changes nothing");
}

#[test]
fn nothing_can_be_recorded_once_the_report_is_being_written() {
    let mut phase = Phase::Reporting;

    assert!(matches!(
        phase.record(Speaker::Suspect, "One more thing, detective."),
        Err(AppError::InvalidState { .. })
    ));
    assert_eq!(phase, Phase::Reporting, "a refused line changes nothing");
}
