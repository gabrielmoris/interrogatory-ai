//! Stage 10b — moving between phases
//!
//! Run with:  cargo test --test phase_moves      (from src-tauri/)
//!
//! A case is played in one direction: read the briefing, pick a suspect and go
//! into the room, leave the room and write the report. Those are the only two
//! moves there are.
//!
//! Every other move is the player asking for something that makes no sense —
//! questioning a second suspect from inside the room, walking out of a room
//! they are not in. The phase itself refuses them, and refusing changes nothing.

use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::transcript::{Phase, Speaker, Turn};

/// Viktor Lang, one question in.
fn questioning_viktor() -> Phase {
    Phase::Interrogating {
        suspect: SuspectId::new(2),
        turns: vec![Turn {
            speaker: Speaker::Detective,
            text: "Where were you on Tuesday night?".to_string(),
        }],
    }
}

#[test]
fn every_phase_can_say_where_the_player_is() {
    // The wording ends up in the error message, so the tests pin it.
    assert_eq!(Phase::Briefing.name(), "the briefing");
    assert_eq!(questioning_viktor().name(), "an interrogation");
    assert_eq!(Phase::Reporting.name(), "the report");
}

#[test]
fn the_briefing_leads_into_the_room() {
    let mut phase = Phase::Briefing;

    assert_eq!(phase.begin(SuspectId::new(2)), Ok(()));
    assert_eq!(phase.suspect(), Some(SuspectId::new(2)));
    assert_eq!(phase.turn_count(), 0, "nobody has said anything yet");
}

#[test]
fn the_room_leads_to_the_report() {
    let mut phase = questioning_viktor();

    assert_eq!(phase.finish(), Ok(()));
    assert_eq!(phase, Phase::Reporting);
}

#[test]
fn nobody_can_be_called_in_from_inside_the_room() {
    let mut phase = questioning_viktor();

    assert_eq!(
        phase.begin(SuspectId::new(1)),
        Err(AppError::InvalidState {
            action: "begin an interrogation".to_string(),
            state: "an interrogation".to_string(),
        })
    );
}

#[test]
fn nobody_can_be_called_in_while_the_report_is_being_written() {
    let mut phase = Phase::Reporting;

    assert!(matches!(
        phase.begin(SuspectId::new(1)),
        Err(AppError::InvalidState { .. })
    ));
}

#[test]
fn you_cannot_walk_out_of_a_room_you_are_not_in() {
    assert!(matches!(
        Phase::Briefing.finish(),
        Err(AppError::InvalidState { .. })
    ));
    assert!(matches!(
        Phase::Reporting.finish(),
        Err(AppError::InvalidState { .. })
    ));
}

#[test]
fn a_refused_move_leaves_the_phase_exactly_as_it_was() {
    let mut phase = questioning_viktor();
    let before = phase.clone();

    assert!(phase.begin(SuspectId::new(1)).is_err());

    assert_eq!(
        phase, before,
        "the suspect and the lines said are untouched"
    );
}
