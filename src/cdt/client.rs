use std::net::TcpStream;

use serde::Serialize;
use serde_json::{json, to_string_pretty, Error, Value};

use websocket::sync::Client;
use websocket::{ClientBuilder, Message, OwnedMessage};

use crate::cdt::models::Request;

use super::models::{
    DebuggerCallFrameId, DebuggerPausedResponse, Response, ResultScriptSourceResponse,
    RuntimeRemoteObject, RuntimeRemoteObjectId, RuntimeScriptId,
};

fn json_to_message<T: Serialize>(json_value: &T) -> Result<Message<'static>, Error> {
    let deserialized_value = to_string_pretty(json_value)?;
    Ok(Message::text(deserialized_value))
}

pub struct CDTClient {
    client: Client<TcpStream>,
}

pub type CDTClientResult<T> = Result<T, Box<dyn std::error::Error>>;

fn parse_method_message(message: Value) -> CDTClientResult<Response> {
    let method = message
        .get("method")
        .ok_or("message must have a method field")?
        .as_str()
        .ok_or("method field must be string")?;

    Ok(match method {
        "Debugger.scriptParsed" => Response::DebuggerScriptParsed(serde_json::from_value(message)?),
        "Debugger.paused" => Response::DebuggerPaused(serde_json::from_value(message)?),
        "Runtime.executionContextDestroyed" => {
            Response::RuntimeExecutionContextDestroyed(serde_json::from_value(message)?)
        }
        _ => Response::Unknown(message.to_owned()),
    })
}

fn parse_result_message(message: Value) -> CDTClientResult<Response> {
    let result = message
        .get("result")
        .ok_or("message must have a result field")?;

    Ok(if result.get("scriptSource").is_some() {
        Response::ResultScriptSource(serde_json::from_value(message)?)
    } else if result.get("objectId").is_some() {
        Response::ResultRuntimeRemoteObject(serde_json::from_value(message)?)
    } else {
        Response::Result(serde_json::from_value(message)?)
    })
}

fn parse_message(message: &str) -> CDTClientResult<Response> {
    let parsed_message: Value = serde_json::from_str(&message)?;

    if parsed_message.get("method").is_some() {
        parse_method_message(parsed_message)
    } else if parsed_message.get("result").is_some() {
        parse_result_message(parsed_message)
    } else {
        Ok(Response::Unknown(parsed_message))
    }
}

impl CDTClient {
    pub fn new(host: &str, port: &str, id: &str) -> CDTClient {
        let mut client_builder =
            ClientBuilder::new(format!("ws://{}:{}/{}", host, port, id).as_str()).unwrap();
        let client = client_builder.connect_insecure().unwrap();

        CDTClient { client }
    }

    pub fn read_messages_until<F>(&mut self, predicate: F) -> CDTClientResult<Vec<Response>>
    where
        F: Fn(&Response) -> bool,
    {
        let mut messages = Vec::new();

        loop {
            let message = self.client.recv_message()?;
            let message_string = match message {
                OwnedMessage::Text(text) => Ok(text),
                _ => Err(format!("unexpected message: {:?}", message)),
            }?;
            let converted_message = parse_message(&message_string)?;
            messages.push(converted_message);

            if predicate(messages.last().unwrap()) {
                break;
            }
        }

        Ok(messages)
    }

    pub fn read_messages_until_result(&mut self) -> CDTClientResult<Vec<Response>> {
        self.read_messages_until(|message| {
            matches!(
                message,
                Response::Result(_)
                    | Response::ResultScriptSource(_)
                    | Response::ResultRuntimeRemoteObject(_)
            )
        })
    }

    pub fn read_messages_until_paused_or_destroyed(&mut self) -> CDTClientResult<Vec<Response>> {
        self.read_messages_until(|message| {
            matches!(
                message,
                Response::DebuggerPaused(_) | Response::RuntimeExecutionContextDestroyed(_)
            )
        })
    }

    pub fn read_messages_until_script_source(&mut self) -> CDTClientResult<Vec<Response>> {
        self.read_messages_until(|message| matches!(message, Response::ResultScriptSource(_)))
    }

    pub fn runtime_run_if_waiting_for_debugger(
        &mut self,
    ) -> CDTClientResult<Option<DebuggerPausedResponse>> {
        let request = Request::new(1, "Runtime.runIfWaitingForDebugger");
        let message = json_to_message(&request)?;

        self.client.send_message(&message)?;

        let messages = self.read_messages_until_paused_or_destroyed()?;

        let paused_message = match messages.last().unwrap() {
            Response::DebuggerPaused(msg) => Some(msg.clone()),
            Response::RuntimeExecutionContextDestroyed(_msg) => None,
            _ => panic!("debugger_paused expected"),
        };

        Ok(paused_message)
    }

    pub fn runtime_enable(&mut self) -> CDTClientResult<Response> {
        let request = Request::new(1, "Runtime.enable");
        let message = json_to_message(&request)?;
        self.client.send_message(&message).unwrap();

        let messages = self.read_messages_until_result()?;
        Ok(messages.last().unwrap().clone())
    }

    pub fn profiler_enable(&mut self) -> CDTClientResult<Response> {
        let request = Request::new(1, "Profiler.enable");
        let message = json_to_message(&request)?;

        self.client.send_message(&message).unwrap();
        let messages = self.read_messages_until_result()?;
        Ok(messages.last().unwrap().clone())
    }

    pub fn debugger_enable(&mut self) -> CDTClientResult<Response> {
        let request = Request::new(1, "Debugger.enable");
        let message = json_to_message(&request)?;

        self.client.send_message(&message).unwrap();
        let messages = self.read_messages_until_result()?;
        Ok(messages.last().unwrap().clone())
    }

    pub fn debugger_resume(&mut self) -> CDTClientResult<()> {
        let request = Request::new(1, "Debugger.resume");
        let message = json_to_message(&request)?;

        self.client.send_message(&message).unwrap();
        Ok(())
    }

    pub fn debugger_pause(&mut self) -> CDTClientResult<()> {
        let request = Request::new(1, "Debugger.resume");
        let message = json_to_message(&request)?;

        self.client.send_message(&message).unwrap();
        Ok(())
    }

    pub fn debugger_get_script_source(
        &mut self,
        script_id: RuntimeScriptId,
    ) -> CDTClientResult<ResultScriptSourceResponse> {
        let request_params = json!({ "scriptId": script_id });
        let request = Request::new_with_params(1, "Debugger.getScriptSource", request_params)?;
        let message = json_to_message(&request)?;

        self.client.send_message(&message).unwrap();

        let messages = self.read_messages_until_script_source().unwrap();
        let last_message = messages
            .last()
            .ok_or("no message received after waiting")?
            .expect_result_script_source()
            .ok_or("result script source expected")?;

        Ok(last_message.clone())
    }

    pub fn debugger_set_pause_on_exception(&mut self) -> CDTClientResult<()> {
        let params = json!({"state": "none"});
        let request = Request::new_with_params(1, "Debugger.setPauseOnExceptions", params)?;
        let message = json_to_message(&request)?;

        self.client.send_message(&message)?;
        Ok(())
    }

    pub fn debugger_evaluate_on_call_frame(
        &mut self,
        call_frame_id: DebuggerCallFrameId,
        expression: &str,
    ) -> CDTClientResult<RuntimeRemoteObject> {
        let params = json!({"callFrameId": call_frame_id, "expression": expression});
        let request = Request::new_with_params(1, "Debugger.evaluateOnCallFrame", params)?;
        let message = json_to_message(&request)?;

        self.client.send_message(&message)?;

        let messages = self.read_messages_until_result()?;
        let remote_object = messages.last().ok_or("no message received after waiting")?;

        let remote_object = match remote_object {
            Response::Result(o) => Ok(o),
            _ => Err("expected runtime remote object"),
        }?;

        Ok(serde_json::from_value(remote_object.result.clone())?)
    }

    pub fn runtime_get_properties(
        &mut self,
        object_id: RuntimeRemoteObjectId,
    ) -> CDTClientResult<()> {
        let params = json!({ "objectId": object_id });
        let request = Request::new_with_params(1, "Runtime.getProperties", params)?;
        let message = json_to_message(&request)?;

        self.client.send_message(&message)?;
        Ok(())
    }

    pub fn debugger_get_possible_breakpoints(
        &mut self,
        script_id: RuntimeScriptId,
    ) -> CDTClientResult<()> {
        let request_params = json!({ "scriptId": script_id });
        let request =
            Request::new_with_params(1, "Debugger.getPossibleBreakpoints", request_params)?;
        let message = json_to_message(&request)?;

        self.client.send_message(&message)?;
        Ok(())
    }

    pub fn debugger_step_over(&mut self) -> CDTClientResult<()> {
        let request = Request::new(1, "Debugger.stepOver");
        let message = json_to_message(&request)?;

        self.client.send_message(&message)?;
        Ok(())
    }
}
