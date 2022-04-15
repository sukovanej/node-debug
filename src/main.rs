use serde::{Serialize, Deserialize};
use serde_json::json;
use websocket::{ClientBuilder, Message, OwnedMessage};

type RuntimeExecutionContextId = i32;
type RuntimeScriptId = String;

#[derive(Serialize, Deserialize, Debug)]
struct ScriptParsedResponse {
    method: String,
    params: ScriptParsedResponseParams,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScriptParsedResponseParams {
    endColumn: i32,
    endLine: i32,
    executionContextId: RuntimeExecutionContextId,
    hash: String,
    scriptId: RuntimeScriptId,
    startColumn: i32,
    startLine: i32,
    url: String,
}

fn main() {
    let hash = "c922f673-4a6e-4367-ab10-599df661a6a2";
    let host = "127.0.0.1";
    let port = "9229";

    let mut client = ClientBuilder::new(format!("ws://{}:{}/{}", host, port, hash).as_str())
        .unwrap()
        .connect_insecure()
        .unwrap();

    let resume_json = json!({
       "id": 1,
       "method": "Debugger.resume"
    });
    let resume_message = Message::text(resume_json.to_string());

    let debugger_enable_json = json!({
       "id": 1,
       "method": "Debugger.enable"
    });
    let debugger_enable_message = Message::text(debugger_enable_json.to_string());

    let debugger_get_possible_breakpoints_json = json!({
        "id": 1,
        "method": "Debugger.getPossibleBreakpoints",
        "params": {
            "start": 0
        }
    });
    let debugger_get_possible_breakpoints_message =
        Message::text(debugger_get_possible_breakpoints_json.to_string());

    client.send_message(&debugger_enable_message).unwrap();
    let resp = client.recv_message().unwrap();
    let resp_text = match resp {
        OwnedMessage::Text(text) => text,
        _ => panic!("unexpected")
    };
    let json_resp: ScriptParsedResponse = serde_json::from_str(&resp_text).unwrap();
    println!("{:?}", json_resp);

    client.send_message(&debugger_get_possible_breakpoints_message).unwrap();
    let resp = client.recv_message().unwrap();
    println!("{:?}", resp);

    client.send_message(&resume_message).unwrap();
    let resp = client.recv_message().unwrap();
    println!("{:?}", resp);
}
