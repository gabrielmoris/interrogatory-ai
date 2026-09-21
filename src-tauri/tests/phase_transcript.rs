//! Stage 10d — what the room will show
//!
//! Run with:  cargo test --test phase_transcript      (from src-tauri/)
//!
//! The interrogation screen shows the conversation so far: every line, in the
//! order it was said, oldest at the top. It also wants the newest line on its
//! own, so it can slide that one in without redrawing the rest.
//!
//! "Show me everything said in this room."  -> `transcript`
//! "What was said last?"                    -> `last_line`
//!
//! Outside the room there is no conversation. During the briefing and during
//! the report both questions still have an answer, and the answer is nothing.
//!
//! Neither question changes anything. Asking twice gives the same answer, and
//! asking does not stop the next line from being recorded.

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

/// Viktor Lang, three lines in.
fn viktor_mid_interview() -> Phase {
    let mut phase = questioning_viktor();
    phase
        .record(Speaker::Detective, "Where were you on Tuesday night?")
        .unwrap();
    phase.record(Speaker::Suspect, "At home. Alone.").unwrap();
    phase
        .record(Speaker::Detective, "Who can confirm that?")
        .unwrap();
    phase
}

#[test]
fn the_room_shows_every_line_in_the_order_it_was_said() {
    let phase = viktor_mid_interview();

    assert_eq!(
        phase.transcript(),
        [
            line(Speaker::Detective, "Where were you on Tuesday night?"),
            line(Speaker::Suspect, "At home. Alone."),
            line(Speaker::Detective, "Who can confirm that?"),
        ],
        "three lines, oldest first"
    );
}

#[test]
fn a_room_where_nobody_has_spoken_yet_shows_nothing() {
    let phase = questioning_viktor();

    assert!(phase.transcript().is_empty());
}

#[test]
fn there_is_no_conversation_during_the_briefing_or_the_report() {
    assert!(Phase::Briefing.transcript().is_empty());
    assert!(Phase::Reporting.transcript().is_empty());

    assert_eq!(Phase::Briefing.last_line(), None);
    assert_eq!(Phase::Reporting.last_line(), None);
}

#[test]
fn the_last_line_is_the_one_that_was_just_said() {
    let phase = viktor_mid_interview();

    assert_eq!(
        phase.last_line(),
        Some(&line(Speaker::Detective, "Who can confirm that?"))
    );
}

#[test]
fn a_room_where_nobody_has_spoken_yet_has_no_last_line() {
    let phase = questioning_viktor();

    assert_eq!(phase.last_line(), None);
}

#[test]
fn asking_what_the_room_shows_does_not_stop_the_next_line_being_recorded() {
    let mut phase = questioning_viktor();

    phase
        .record(Speaker::Detective, "Sit down, Mr Lang.")
        .unwrap();
    assert_eq!(phase.transcript().len(), 1);

    phase
        .record(Speaker::Suspect, "I have nothing to hide.")
        .unwrap();
    assert_eq!(phase.transcript().len(), 2);

    assert_eq!(
        phase.last_line(),
        Some(&line(Speaker::Suspect, "I have nothing to hide."))
    );
}
