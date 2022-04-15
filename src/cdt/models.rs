use serde::{Deserialize, Serialize};
use serde_json::{Error, Map, Value};

pub type RuntimeExecutionContextId = i32;
pub type RuntimeScriptId = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResultResponse {
    result: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    DebuggerScriptParsed(ScriptParsedResponse),
    DebuggerPaused(DebuggerPausedResponse),
    ResultScriptSource(ResultScriptSourceResponse),
    Result(ResultResponse),
    Unknown(Value),
}

impl Response {
    pub fn expect_result_script_source(&self) -> Option<&ResultScriptSourceResponse> {
        if let Response::ResultScriptSource(res) = self {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScriptParsedResponse {
    pub method: String,
    pub params: DebuggerScriptParsedResponseParams,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerScriptParsedResponseParams {
    pub end_column: i32,
    pub end_line: i32,
    pub execution_context_id: RuntimeExecutionContextId,
    pub hash: String,
    pub script_id: RuntimeScriptId,
    pub start_column: i32,
    pub start_line: i32,
    pub url: String,
    #[serde(rename = "sourceMapURL")]
    pub source_map_url: String,
    #[serde(rename = "hasSourceURL")]
    pub has_source_url: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeRunScriptRequestParams {
    pub script_id: RuntimeScriptId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerGetPossibleBreakpointsParams {
    pub start: DebuggerLocation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerLocation {
    pub script_id: RuntimeScriptId,
    pub line_number: i32,
}

impl DebuggerLocation {
    pub fn new(script_id: RuntimeScriptId, line_number: i32) -> DebuggerLocation {
        DebuggerLocation {
            script_id,
            line_number,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerPausedResponse {
    pub params: DebuggerPausedParams,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerPausedParams {
    pub call_frames: Vec<DebuggerPausedCallFrame>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerPausedCallFrame {
    pub call_frame_id: String,
    pub function_name: String,
    pub function_location: DebuggerLocation,
    pub location: DebuggerLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultScriptSourceResponse {
    pub result: ResultScriptSourceResponseResult
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultScriptSourceResponseResult {
    pub script_source: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: i32,
    pub method: String,
    pub params: Value,
}

impl Request {
    pub fn new(id: i32, method: &str) -> Request {
        let params = Value::Object(Map::new());
        Request { id, method: method.to_owned(), params }
    }

    pub fn new_with_params(id: i32, method: &str, params: Value) -> Result<Request, Error> {
        Ok(Request {
            id,
            method: method.to_owned(),
            params,
        })
    }
}
