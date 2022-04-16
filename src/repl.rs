use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::cdt::client::CDTClient;
use crate::cdt::models::{
    DebuggerPausedCallFrame, RuntimeRemoteObjectResult, RuntimeRemoteObjectResultValue,
};
use crate::source_code::SourceCode;

#[derive(Clone)]
struct ReplState {
    call_frames: Option<ReplStateCallFrame>,
    debugger_state: DebuggerState,
}

#[derive(Clone)]
enum DebuggerState {
    Paused,
    Exited,
}

#[derive(Clone)]
struct ReplStateCallFrame {
    call_frames: Vec<DebuggerPausedCallFrame>,
    active_id: usize,
}

pub fn start_repl(host: &str, port: &str, id: &str) {
    let mut client = CDTClient::new(host, port, id);
    let history_file_path = ".node-debug.history";

    let mut rl = Editor::<()>::new();

    if rl.load_history(history_file_path).is_err() {
        println!("No previous history.");
    }

    client.runtime_enable().unwrap();
    client.debugger_enable().unwrap();
    client.debugger_set_pause_on_exception().unwrap();
    client.profiler_enable().unwrap();

    println!("waiting for the debugger...");
    let mut repl_state = initialize(&mut client);

    if matches!(repl_state.debugger_state, DebuggerState::Exited) {
        panic!("unexpected state");
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                repl_state = run_command(&mut client, &line, repl_state);

                if matches!(repl_state.debugger_state, DebuggerState::Exited) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(history_file_path).unwrap();
}

fn initialize(client: &mut CDTClient) -> ReplState {
    let message = client.runtime_run_if_waiting_for_debugger().unwrap();

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

fn run_command(client: &mut CDTClient, line: &str, repl_state: ReplState) -> ReplState {
    match line {
        "s" | "show" => show_source_code(client, repl_state),
        "c" | "continue" => continue_command(client, repl_state),
        "h" | "help" => help_command(client, repl_state),
        _ => evaluate_expression(client, line, repl_state),
    }
}

fn continue_command(client: &mut CDTClient, repl_state: ReplState) -> ReplState {
    if !matches!(repl_state.debugger_state, DebuggerState::Paused) {
        println!("Error: debugger is not paused");
        return repl_state;
    }

    client.debugger_resume().unwrap();
    let message = client.runtime_run_if_waiting_for_debugger().unwrap();

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

fn evaluate_expression(client: &mut CDTClient, line: &str, repl_state: ReplState) -> ReplState {
    if repl_state.call_frames.is_none() {
        return repl_state.clone();
    }

    let call_frames = repl_state.call_frames.as_ref().unwrap();
    let call_frame = &call_frames.call_frames[call_frames.active_id];
    let call_frame_id = &call_frame.call_frame_id;

    let remote_object = client.debugger_evaluate_on_call_frame(call_frame_id.to_owned(), line);

    match remote_object {
        Ok(obj) => {
            println!("{}", runtime_remote_object_to_string(obj.result));
        }
        Err(err) => {
            println!("evaluated: {:?}", err);
        }
    };

    repl_state
}

fn runtime_remote_object_to_string(obj: RuntimeRemoteObjectResult) -> String {
    if obj.value.is_some() {
        return match obj.value.unwrap() {
            RuntimeRemoteObjectResultValue::String(str) => format!("\"{}\"", str),
            RuntimeRemoteObjectResultValue::Number(n) => n.to_string(),
        };
    } else if obj.description.is_some() {
        return format!("[description {}]", obj.description.unwrap());
    } else if obj.class_name.is_some() {
        return format!("[class {}]", obj.class_name.unwrap());
    }

    return "[<unknown object>]".to_string();
}

fn show_source_code(client: &mut CDTClient, repl_state: ReplState) -> ReplState {
    let call_frames = repl_state.call_frames.as_ref().unwrap();
    let call_frame = &call_frames.call_frames[call_frames.active_id];
    let top_level_script_id = call_frame.location.script_id.clone();

    let source = client
        .debugger_get_script_source(top_level_script_id)
        .unwrap();

    let source_code = SourceCode::from_str(&source.result.script_source);
    let mapping_content = &source_code.source_mapping.unwrap().sources_content[0];

    println!("{}", mapping_content);

    repl_state
}

fn help_command(_: &mut CDTClient, repl_state: ReplState) -> ReplState {
    let help = "s / show                 show source code of the current call frame\n\
         c / continue             resume the execution\n\
         h / help                 show this help\n\
         [javascript expression]  evalute JS expression in the current call frame";

    println!("{}", help);

    repl_state
}
