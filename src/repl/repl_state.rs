use crate::cdt::models::DebuggerPausedCallFrame;

#[derive(Clone)]
pub struct ReplState {
    pub call_frames: Option<ReplStateCallFrame>,
    pub debugger_state: DebuggerState,
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
