use crate::cdt::client::CDTClient;

use super::handle_pause_of_destry_message::handle_pause_or_destroy_message;
use super::repl_state::ReplState;

pub fn continue_to_command(client: &mut CDTClient, line: &str, repl_state: ReplState) -> ReplState {
    let number_string = &line.chars().skip(3).collect::<String>();
    let number: Option<u32> = number_string.parse().ok();

    if number.is_none() {
        println!("Expected number, got {}", number_string);
        return repl_state;
    }

    let number = number.unwrap();

    let script_id = repl_state
        .get_active_call_frame()
        .map(|call_frame| &call_frame.location.script_id);

    if script_id.is_none() {
        println!("Error: no active frame.");
        return repl_state;
    }

    let script_id = script_id.unwrap();

    let message = client
        .debugger_continue_to_location(script_id.to_owned(), number)
        .unwrap();
    handle_pause_or_destroy_message(message)
}
