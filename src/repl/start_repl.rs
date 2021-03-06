use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::cdt::client::CDTClient;

use super::continue_to_command::continue_to_command;
use super::evaluate_command::{
    evaluate_expression, evaluate_expression_from_command, evalulate_and_stringify_command,
};
use super::handle_pause_of_destry_message::handle_pause_or_destroy_message;
use super::repl_state::{DebuggerState, ReplState, ReplStateCallFrame};
use super::show_source_code_command::{show_source_code_command, show_minified_source_code_command};

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
    client.debugger_pause().unwrap();

    println!("Waiting for the debugger...");
    let mut repl_state = initialize(&mut client);

    if matches!(repl_state.debugger_state, DebuggerState::Exited) {
        println!("Debugger context destroyed...");
        std::process::exit(1);
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
        "s" | "show" => show_source_code_command(client, repl_state),
        "sm" | "show-minified" => show_minified_source_code_command(client, repl_state),
        "c" | "continue" => continue_command(client, repl_state),
        "i" | "step-into" => step_into_command(client, repl_state),
        "o" | "step-out" => step_out_command(client, repl_state),
        cmd if cmd.starts_with("ct ") => continue_to_command(client, line, repl_state),
        cmd if cmd.starts_with("e ") => evaluate_expression_from_command(client, cmd, repl_state),
        cmd if cmd.starts_with("es ") => evalulate_and_stringify_command(client, cmd, repl_state),
        "n" | "next" => next_command(client, repl_state),
        "q" | "quit" => quit_command(),
        "h" | "help" => help_command(client, repl_state),
        _ => evaluate_expression(client, line, repl_state),
    }
}

fn step_into_command(client: &mut CDTClient, _: ReplState) -> ReplState {
    let message = client.debugger_step_into().unwrap();
    handle_pause_or_destroy_message(message)
}

fn step_out_command(client: &mut CDTClient, _: ReplState) -> ReplState {
    let message = client.debugger_step_out().unwrap();
    handle_pause_or_destroy_message(message)
}

fn quit_command() -> ReplState {
    println!("Exiting, see ya!");
    ReplState {
        debugger_state: DebuggerState::Exited,
        call_frames: None,
    }
}

fn continue_command(client: &mut CDTClient, repl_state: ReplState) -> ReplState {
    if !matches!(repl_state.debugger_state, DebuggerState::Paused) {
        println!("Error: debugger is not paused");
        return repl_state;
    }

    client.debugger_resume().unwrap();
    let message = client.runtime_run_if_waiting_for_debugger().unwrap();
    handle_pause_or_destroy_message(message)
}

fn next_command(client: &mut CDTClient, repl_state: ReplState) -> ReplState {
    if !matches!(repl_state.debugger_state, DebuggerState::Paused) {
        println!("Error: debugger is not paused");
        return repl_state;
    }

    client.debugger_step_over().unwrap();
    let message = client.runtime_run_if_waiting_for_debugger().unwrap();
    handle_pause_or_destroy_message(message)
}

fn help_command(_: &mut CDTClient, repl_state: ReplState) -> ReplState {
    let help = "s / show                 show the original source code of the current call frame\n\
                sm / show-minified       show minified source code of the current call frame\n\
                c / continue             resume the execution\n\
                i / step-into            step out of the function\n\
                o / step-out             step into the function\n\
                n / next                 step over in the execution\n\
                ct                       continue to a line (in the minified file)\n\
                q / quit                 quit the debugger\n\
                h / help                 show this help\n\
                es <expresssion>         evalute JS expression and stringify it in the current call frame\n\
                e <expresssion>          evalute JS expression in the current call frame\n\
                <expression>             evalute JS expression in the current call frame";

    println!("{}", help);

    repl_state
}
