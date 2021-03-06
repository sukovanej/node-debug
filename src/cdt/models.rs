use serde::{Deserialize, Serialize};
use serde_json::{Error, Map, Value};

pub type RuntimeExecutionContextId = i32;
pub type RuntimeScriptId = String;
pub type DebuggerCallFrameId = String;
pub type RuntimeRemoteObjectId = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeCallArgument {
    value: Value,
    unserializable_value: String,
    object_id: RuntimeRemoteObjectId
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeRemoteObject {
    pub result: RuntimeRemoteObjectResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeRemoteObjectResult {
    pub r#type: String,
    pub object_id: Option<String>,
    pub value: Option<RuntimeRemoteObjectResultValue>,
    pub description: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged, rename_all = "camelCase")]
pub enum RuntimeRemoteObjectResultValue {
    Bool(bool),
    String(String),
    Number(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultResponse {
    pub id: i32,
    pub result: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeExecutionContextDestroyed {
    method: String,
    params: RuntimeExecutionContextDestroyedParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeExecutionContextDestroyedParams {
    execution_context_id: RuntimeExecutionContextId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    DebuggerScriptParsed(ScriptParsedResponse),
    DebuggerPaused(DebuggerPausedResponse),
    ResultScriptSource(ResultScriptSourceResponse),
    ResultRuntimeRemoteObject(RuntimeRemoteObject),
    RuntimeExecutionContextDestroyed(RuntimeExecutionContextDestroyed),
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

    pub fn expect_runtime_remote_object(&self) -> Option<&RuntimeRemoteObject> {
        if let Response::ResultRuntimeRemoteObject(res) = self {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptParsedResponse {
    pub method: String,
    pub params: DebuggerScriptParsedResponseParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerLocation {
    pub script_id: RuntimeScriptId,
    pub line_number: u32,
    pub column_number: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerPausedResponse {
    pub params: DebuggerPausedParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerPausedParams {
    pub call_frames: Vec<DebuggerPausedCallFrame>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub result: ResultScriptSourceResponseResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultScriptSourceResponseResult {
    pub script_source: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: u64,
    pub method: String,
    pub params: Value,
}

impl Request {
    pub fn new(id: u64, method: &str) -> Request {
        let params = Value::Object(Map::new());
        Request {
            id,
            method: method.to_owned(),
            params,
        }
    }

    pub fn new_with_params(id: u64, method: &str, params: Value) -> Result<Request, Error> {
        Ok(Request {
            id,
            method: method.to_owned(),
            params,
        })
    }
}
