//! Stage 5 — `AppError`, `thiserror`, and `Result` at the boundary
//!
//! Run with:  cargo test --test errors      (from src-tauri/)
//!
//! Stage 4's lookups answer "is there one?" with `Option`. That is the right
//! answer for a caller who has a sensible fallback, and the wrong one for a
//! caller who is about to tell the player something went wrong: `None` carries
//! no reason, no id, and nothing a UI can render. This file is about the other
//! return type — `Result<T, AppError>` — and about the one property `AppError`
//! must have that no domain type has needed until now: it has to survive the
//! trip to TypeScript.

use interrogatory_ai_lib::case::{Case, Fact, Suspect};
use interrogatory_ai_lib::error::{AppError, AppResult};
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

// ------------------------------------------------- Option -> Result, and back

#[test]
fn require_suspect_hands_back_the_same_borrow_as_suspect() {
    let case = the_ledger();

    let from_option = case.suspect(viktor()).expect("Viktor is in this case");
    let from_result = case
        .require_suspect(viktor())
        .expect("Viktor is in this case");

    // `require_*` is a thin skin over the Stage 4 lookup. Same bytes, same
    // borrow of the case — the only thing that changed is the failure branch.
    assert!(std::ptr::eq(from_option, from_result));
    assert_eq!(from_result.name, "Viktor Lang");
}

#[test]
fn require_suspect_names_the_id_it_could_not_find() {
    let case = the_ledger();

    // This is the whole point of the stage. `None` says "no". The error says
    // *which* id, so the message the player sees can name it.
    assert_eq!(
        case.require_suspect(SuspectId::new(99)),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(99)
        })
    );
}

#[test]
fn require_fact_mut_still_edits_in_place() {
    let mut case = the_ledger();

    case.require_fact_mut(FactId::new(2))
        .expect("fact #2 is in this case")
        .reveal_to(viktor());

    assert_eq!(case.suspect_facts(viktor()).count(), 2);
}

#[test]
fn require_fact_mut_names_the_id_it_could_not_find() {
    let mut case = the_ledger();

    assert_eq!(
        case.require_fact_mut(FactId::new(404)),
        Err(AppError::FactNotFound {
            id: FactId::new(404)
        })
    );
}

// ------------------------------------------------------------------ `?`

#[test]
fn reveal_lets_one_more_suspect_in() {
    let mut case = the_ledger();
    assert_eq!(case.suspect_facts(viktor()).count(), 1);

    assert_eq!(case.reveal(FactId::new(2), viktor()), Ok(()));

    assert_eq!(case.suspect_facts(viktor()).count(), 2);
}

#[test]
fn reveal_rejects_a_suspect_who_is_not_in_the_case() {
    let mut case = the_ledger();

    assert_eq!(
        case.reveal(FactId::new(1), SuspectId::new(99)),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(99)
        })
    );
}

#[test]
fn reveal_rejects_a_fact_that_is_not_in_the_case() {
    let mut case = the_ledger();

    assert_eq!(
        case.reveal(FactId::new(404), marta()),
        Err(AppError::FactNotFound {
            id: FactId::new(404)
        })
    );
}

#[test]
fn reveal_checks_the_suspect_before_it_checks_the_fact() {
    let mut case = the_ledger();

    // Both ids are wrong. Only the first `?` gets to run, so only the first
    // error is ever built. The order of the two checks is observable
    // behaviour, not an implementation detail — which is why it is pinned.
    assert_eq!(
        case.reveal(FactId::new(404), SuspectId::new(99)),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(99)
        })
    );
}

#[test]
fn a_failed_reveal_changes_nothing() {
    let mut case = the_ledger();
    let before = case.clone();

    assert!(case.reveal(FactId::new(404), marta()).is_err());

    assert_eq!(case, before);
}

#[test]
fn the_question_mark_operator_short_circuits_the_caller_too() {
    // A caller of a fallible function is itself fallible, all the way up to
    // the IPC command. Note the return type: `AppResult<String>`, the alias,
    // not `Result<String, AppError>` written out.
    fn describe(case: &Case, id: SuspectId) -> AppResult<String> {
        let suspect = case.require_suspect(id)?;
        Ok(format!("{} is {}", suspect.id, suspect.name))
    }

    let case = the_ledger();

    assert_eq!(
        describe(&case, marta()),
        Ok("suspect #1 is Marta Reyes".to_string())
    );
    assert_eq!(
        describe(&case, SuspectId::new(99)),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(99)
        })
    );
}

// ------------------------------------------------------------- the error type

#[test]
fn every_variant_reads_like_a_sentence() {
    // `#[error("...")]` is not a comment. thiserror turns each one into the
    // `Display` impl, and `Display` is what `to_string()` and `{}` call.
    assert_eq!(
        AppError::SuspectNotFound {
            id: SuspectId::new(99)
        }
        .to_string(),
        "this case has no suspect #99"
    );
    assert_eq!(
        AppError::FactNotFound {
            id: FactId::new(404)
        }
        .to_string(),
        "this case has no fact #404"
    );
    assert_eq!(
        AppError::CaseNotFound {
            slug: "the-ledger".to_string()
        }
        .to_string(),
        r#"no case file named "the-ledger" was found"#
    );
    assert_eq!(
        AppError::Io {
            path: "cases/the-ledger.toml".to_string(),
            message: "No such file or directory (os error 2)".to_string(),
        }
        .to_string(),
        "could not read cases/the-ledger.toml: No such file or directory (os error 2)"
    );
    assert_eq!(
        AppError::Parse {
            path: "cases/the-ledger.toml".to_string(),
            message: "expected a table at line 4".to_string(),
        }
        .to_string(),
        "cases/the-ledger.toml is not a valid case file: expected a table at line 4"
    );
    assert_eq!(
        AppError::Inference {
            message: "the model returned no tokens".to_string()
        }
        .to_string(),
        "the inference engine failed: the model returned no tokens"
    );
    assert_eq!(
        AppError::InvalidState {
            action: "submit a report".to_string(),
            state: "no case is loaded".to_string(),
        }
        .to_string(),
        "cannot submit a report while no case is loaded"
    );
}

#[test]
fn app_error_is_a_real_std_error() {
    // `#[derive(Error)]` implements `std::error::Error`, which is the trait
    // every error-handling crate in the ecosystem keys off. This function
    // accepts any error at all; if the derive were missing it would not
    // accept ours.
    fn describe(e: &dyn std::error::Error) -> String {
        e.to_string()
    }

    let err = AppError::Inference {
        message: "the model returned no tokens".to_string(),
    };

    assert_eq!(
        describe(&err),
        "the inference engine failed: the model returned no tokens"
    );

    // `source()` is the "caused by" chain. Nothing in `AppError` wraps another
    // error yet, so every variant is the root cause.
    assert!(std::error::Error::source(&err).is_none());
}

#[test]
fn errors_cross_the_ipc_boundary_as_tagged_objects() {
    // What the React side will receive from a rejected `invoke()`. Not a
    // string: an object with a machine-readable `kind` and the data belonging
    // to that kind, so the frontend can branch on it and write its own copy.
    let json = serde_json::to_value(AppError::SuspectNotFound {
        id: SuspectId::new(99),
    })
    .expect("AppError serializes");

    assert_eq!(
        json,
        serde_json::json!({ "kind": "suspectNotFound", "id": 99 })
    );

    // The id is a bare number on the wire. `SuspectId` is a newtype, and serde
    // sees straight through newtypes — the wrapper is a Rust-side guarantee,
    // not a wire format.
    assert_eq!(json["id"], serde_json::json!(99));
}

#[test]
fn a_foreign_diagnostic_is_carried_across_as_text() {
    // Our own failures are structured. Diagnostics from things we did not
    // write — the OS, the TOML parser, llama.cpp — are opaque strings, and
    // pretending otherwise would be inventing structure we do not have.
    let json = serde_json::to_value(AppError::Io {
        path: "cases/the-ledger.toml".to_string(),
        message: "No such file or directory (os error 2)".to_string(),
    })
    .expect("AppError serializes");

    assert_eq!(
        json,
        serde_json::json!({
            "kind": "io",
            "path": "cases/the-ledger.toml",
            "message": "No such file or directory (os error 2)",
        })
    );
}
