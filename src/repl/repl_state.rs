use crate::cdt::models::DebuggerPausedCallFrame;

#[derive(Clone)]
pub struct ReplState {
    pub call_frames: Option<ReplStateCallFrame>,
    pub debugger_state: DebuggerState,
}

impl ReplState {
    pub fn get_active_call_frame(&self) -> Option<&DebuggerPausedCallFrame> {
        self.call_frames
            .as_ref()
            .and_then(|call_frames| Some(&call_frames.call_frames[call_frames.active_id]))
    }
}

#[derive(Clone)]
pub enum DebuggerState {
    Paused,
    Exited,
}

#[derive(Clone)]
pub struct ReplStateCallFrame {
    pub call_frames: Vec<DebuggerPausedCallFrame>,
    pub active_id: usize,
}
