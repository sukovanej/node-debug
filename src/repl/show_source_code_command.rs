use crate::cdt::client::CDTClient;

use super::source_code::SourceCode;
use super::repl_state::ReplState;
use super::code_preview::create_code_preview;

pub fn show_source_code_command(client: &mut CDTClient, repl_state: ReplState) -> ReplState {
    let call_frames = repl_state.call_frames.as_ref().unwrap();
    let call_frame = &call_frames.call_frames[call_frames.active_id];
    let top_level_script_id = call_frame.location.script_id.clone();

    let source = client
        .debugger_get_script_source(top_level_script_id)
        .unwrap();

    let source_code = SourceCode::from_str(&source.result.script_source);
    let maybe_preview = create_code_preview(&source_code, call_frame);

    match maybe_preview {
        Ok((file_name, preview)) => {
            println!("{}", file_name);
            println!("{}", preview);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    };

    repl_state
}
