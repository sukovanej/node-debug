use crate::cdt::client::CDTClient;

use super::code_preview::{create_code_preview, create_preview};
use super::repl_state::ReplState;
use super::source_code::SourceCode;

pub fn show_source_code_command(client: &mut CDTClient, repl_state: ReplState) -> ReplState {
    let call_frame = repl_state.get_active_call_frame();

    if call_frame.is_none() {
        println!("Error: no active call frame");
        return repl_state;
    }

    let call_frame = call_frame.unwrap();

    let top_level_script_id = call_frame.location.script_id.clone();
    let source = client
        .debugger_get_script_source(top_level_script_id)
        .unwrap();

    let source_code = SourceCode::from_str(&source.result.script_source);
    let maybe_preview = create_code_preview(&source_code, call_frame);

    match maybe_preview {
        Ok((file_name, code_preview_lines)) => {
            print_code_preview(&file_name, &code_preview_lines);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    };

    repl_state
}

pub fn show_minified_source_code_command(
    client: &mut CDTClient,
    repl_state: ReplState,
) -> ReplState {
    let call_frame = repl_state.get_active_call_frame();

    if call_frame.is_none() {
        println!("Error: no active call frame");
        return repl_state;
    }

    let call_frame = call_frame.unwrap();

    let top_level_script_id = call_frame.location.script_id.clone();
    let source = client
        .debugger_get_script_source(top_level_script_id)
        .unwrap();
    let source_code = SourceCode::from_str(&source.result.script_source);
    let line = call_frame.location.line_number;
    let code_preview_lines = create_preview(source_code.code.lines(), line as usize);
    let file_name = "minified source code";

    print_code_preview(&file_name, &code_preview_lines);

    repl_state
}

fn print_code_preview(file_name: &str, code_preview_lines: &[(usize, String)]) {
    let max_line_length = code_preview_lines
        .iter()
        .map(|(_, line)| line.len())
        .max()
        .unwrap();

    let line_delimiter = std::iter::repeat('—')
        .take(max_line_length + 7)
        .collect::<String>();

    println!("\x1b[90m{}\x1b[0m", file_name);
    println!("{}", line_delimiter);
    println!("{}", get_prettified_code_preview(code_preview_lines));
    println!("{}", line_delimiter);
}

fn get_prettified_code_preview(lines: &[(usize, String)]) -> String {
    let lines = lines
        .iter()
        .map(|(i, line)| format!(" {:03} | {}", i, line))
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
