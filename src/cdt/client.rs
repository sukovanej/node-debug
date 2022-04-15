use std::net::TcpStream;

use serde::Serialize;
use serde_json::{to_string_pretty, Error, Value};

use websocket::sync::Client;
use websocket::{ClientBuilder, Message, OwnedMessage};

use crate::cdt::models::{Request, RuntimeRunScriptRequest};

use super::models::{Response, RuntimeScriptId};

fn json_to_message<T: Serialize>(json_value: &T) -> Result<Message<'static>, Error> {
    let deserialized_value = to_string_pretty(json_value)?;
    Ok(Message::text(deserialized_value))
}

pub struct CDTClient {
    client: Client<TcpStream>,
}

fn parse_method_message(message: &Value) -> Result<Response, Box<dyn std::error::Error>> {
    let method = message.get("method").unwrap().as_str().ok_or("method field must be string")?;

    Ok(
        match method {
            "Debugger.scriptParsed" => {
                Response::DebuggerScriptParsed(serde_json::from_value(message.to_owned())?)
            }
            _ => Response::Unknown(message.to_owned()),
        },
    )
}

fn parse_result_message(message: &Value) -> Result<Response, Box<dyn std::error::Error>> {
    Ok(Response::Result(serde_json::from_value(
        message.to_owned(),
    )?))
}

fn parse_message(message: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let parsed_message: Value = serde_json::from_str(&message)?;

    if parsed_message.get("method").is_some() {
        parse_method_message(&parsed_message)
    } else if parsed_message.get("result").is_some() {
        parse_result_message(&parsed_message)
    } else {
        Ok(Response::Unknown(parsed_message))
    }
}

impl CDTClient {
    pub fn new(host: &str, port: &str, id: &str) -> CDTClient {
        let client = ClientBuilder::new(format!("ws://{}:{}/{}", host, port, id).as_str())
            .unwrap()
            .connect_insecure()
            .unwrap();

        CDTClient { client }
    }

    pub fn read_messages_until_result(
        &mut self,
    ) -> Result<Vec<Response>, Box<dyn std::error::Error>> {
        let mut messages = Vec::new();

        loop {
            let message = self.client.recv_message()?;
            let message_string = match message {
                OwnedMessage::Text(text) => Ok(text),
                _ => Err(format!("unexpected message: {:?}", message)),
            }?;
            let converted_message = parse_message(&message_string)?;
            messages.push(converted_message);

            if let Response::Result(_) = messages.last().unwrap() {
                break;
            }
        }

        Ok(messages)
    }

    pub fn runtime_run_if_waiting_for_debugger(
        &mut self,
        script_id: RuntimeScriptId,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request_params = RuntimeRunScriptRequest::new(script_id);
        let request =
            Request::new_with_params(1, "Runtime.runIfWaitingForDebugger", &request_params)?;
        let message = json_to_message(&request)?;

        self.client.send_message(&message)?;
        Ok(())
    }

    pub fn debugger_enable(&mut self) -> Result<(), Error> {
        let request = Request::new(1, "Debugger.enable");
        let message = json_to_message(&request)?;

        self.client.send_message(&message).unwrap();
        Ok(())
    }

    pub fn debugger_resume(&mut self) -> Result<(), Error> {
        let request = Request::new(1, "Debugger.resume");
        let message = json_to_message(&request)?;

        self.client.send_message(&message).unwrap();
        Ok(())
    }
}

//let debugger_get_possible_breakpoints_json = json!({
//    "id": 1,
//    "method": "Debugger.getPossibleBreakpoints",
//    "params": {
//        "start": 0
//    }
//});
//let debugger_get_possible_breakpoints_message =
//    Message::text(debugger_get_possible_breakpoints_json.to_string());
//
//
//client.send_message(&debugger_get_possible_breakpoints_message).unwrap();
//let resp = client.recv_message().unwrap();
//println!("{:?}", resp);
//
//client.send_message(&resume_message).unwrap();
//let resp = client.recv_message().unwrap();
//println!("{:?}", resp);
