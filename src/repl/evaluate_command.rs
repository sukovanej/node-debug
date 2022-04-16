use crate::cdt::client::CDTClient;
use crate::cdt::models::{RuntimeRemoteObjectResult, RuntimeRemoteObjectResultValue};

use super::repl_state::ReplState;

pub fn evaluate_expression(
    client: &mut CDTClient,
    expression: &str,
    repl_state: ReplState,
) -> ReplState {
    if repl_state.call_frames.is_none() {
        return repl_state.clone();
    }

    let call_frames = repl_state.call_frames.as_ref().unwrap();
    let call_frame = &call_frames.call_frames[call_frames.active_id];
    let call_frame_id = &call_frame.call_frame_id;

    let remote_object =
        client.debugger_evaluate_on_call_frame(call_frame_id.to_owned(), expression);

    match remote_object {
        Ok(obj) => {
            println!("{}", runtime_remote_object_to_string(obj.result));
        }
        Err(err) => {
            println!("Error while evaluating: {:?}", err);
        }
    };

    repl_state
}

pub fn evaluate_expression_from_command(
    client: &mut CDTClient,
    line: &str,
    repl_state: ReplState,
) -> ReplState {
    let expression = &line.chars().skip(2).collect::<String>();
    evaluate_expression(client, expression, repl_state)
}

pub fn evalulate_and_stringify_command(
    client: &mut CDTClient,
    line: &str,
    repl_state: ReplState,
) -> ReplState {
    let expression = &line.chars().skip(3).collect::<String>();
    let expression = &format!("JSON.stringify({})", expression);
    evaluate_expression(client, expression, repl_state)
}

fn runtime_remote_object_to_string(obj: RuntimeRemoteObjectResult) -> String {
    if obj.value.is_some() {
        return match obj.value.unwrap() {
            RuntimeRemoteObjectResultValue::String(str) => format!("\"{}\"", str),
            RuntimeRemoteObjectResultValue::Number(n) => n.to_string(),
            RuntimeRemoteObjectResultValue::Bool(b) => b.to_string(),
        };
    } else if obj.description.is_some() {
        return format!("[description {}]", obj.description.unwrap());
    } else if obj.class_name.is_some() {
        return format!("[class {}]", obj.class_name.unwrap());
    }

    return "[<unknown object>]".to_string();
}
