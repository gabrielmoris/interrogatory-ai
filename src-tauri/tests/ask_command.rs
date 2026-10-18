//! Stage 11d — React asks a question (async since Stage 12)
//!
//! Run with:  cargo test --test ask_command      (from src-tauri/)
//!
//! What asking does is already tested: the command hands everything to
//! `AppState::ask`, and `room_exchange.rs` tests that. This file checks the
//! command exists, with the inputs Tauri and React give it. No app starts.

use interrogatory_ai_lib::error::AppResult;
use interrogatory_ai_lib::ipc::ask_suspect;
use interrogatory_ai_lib::state::AppState;
use tauri::ipc::Channel;
use tauri::State;

#[test]
fn ask_suspect_is_async_and_takes_the_state_a_question_and_a_channel() {
    // This compiles only if `ask_suspect` takes these three inputs and,
    // once awaited, gives back an `AppResult<String>`.
    // Since Stage 16c the third input is the channel React makes for the pieces.
    async fn _shape(
        state: State<'_, AppState>,
        question: String,
        on_piece: Channel<String>,
    ) -> AppResult<String> {
        ask_suspect(state, question, on_piece).await
    }
}
