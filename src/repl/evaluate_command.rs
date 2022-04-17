use crate::cdt::client::CDTClient;
use crate::cdt::models::{RuntimeRemoteObjectResult, RuntimeRemoteObjectResultValue};

use super::repl_state::ReplState;

pub fn evaluate_expression(
    client: &mut CDTClient,
    expression: &str,
    repl_state: ReplState,
) -> ReplState {
    let call_frame_id = repl_state
        .get_active_call_frame()
        .map(|call_frame| &call_frame.call_frame_id);

    if call_frame_id.is_none() {
        return repl_state;
    }

    let remote_object =
        client.debugger_evaluate_on_call_frame(call_frame_id.unwrap().to_owned(), expression);

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
        match obj.value.unwrap() {
            RuntimeRemoteObjectResultValue::String(str) => {
                format!("\x1b[90m\"\x1b[0m{}\x1b[90m\"\x1b[0m", str)
            }
            RuntimeRemoteObjectResultValue::Number(n) => n.to_string(),
            RuntimeRemoteObjectResultValue::Bool(b) => b.to_string(),
        }
    } else if obj.description.is_some() {
        format!("[\x1b[90mdescription\x1b[0m {}]", obj.description.unwrap())
    } else if obj.class_name.is_some() {
        format!("[\x1b[90mclass\x1b[0m {}]", obj.class_name.unwrap())
    } else {
        "[\x1b[90m<unknown object>\x1b[0m]".to_string()
    }
}
