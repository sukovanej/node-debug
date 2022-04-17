use crate::cdt::models::DebuggerPausedResponse;

use super::repl_state::{ReplState, ReplStateCallFrame, DebuggerState};

pub fn handle_pause_or_destroy_message(message: Option<DebuggerPausedResponse>) -> ReplState {
    match message {
        Some(message) => ReplState {
            call_frames: Some(ReplStateCallFrame {
                call_frames: message.params.call_frames.clone(),
                active_id: 0,
            }),
            debugger_state: DebuggerState::Paused,
        },
        None => ReplState {
            call_frames: None,
            debugger_state: DebuggerState::Exited,
        },
    }
}
