use crate::cdt::client::CDTClient;

use super::code_preview::create_code_preview;
use super::repl_state::ReplState;
use super::source_code::SourceCode;

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
        Ok((file_name, code_preview_lines)) => {
            let line = std::iter::repeat('—')
                .take(file_name.len())
                .collect::<String>();
            println!("\x1b[90m{}\x1b[0m", file_name);
            println!("{}", line);
            println!("{}", print_code_preview(code_preview_lines));
            println!("{}", line);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    };

    repl_state
}

fn print_code_preview(lines: Vec<(usize, String)>) -> String {
    let lines = lines
        .iter()
        .map(|(i, line)| format!(" {} | {}", i, line))
        .collect::<Vec<String>>();

    let middle_index = (lines.len() - 1) / 2;

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if i == middle_index {
                format!("\x1b[93m{}\x1b[0m", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
