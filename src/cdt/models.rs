use serde::{Deserialize, Serialize};
use serde_json::{Error, Map, Value};

pub type RuntimeExecutionContextId = i32;
pub type RuntimeScriptId = String;

/// EXAMPLE:
/// {
///     "method":"Debugger.scriptParsed",
///     "params":{
///         "scriptId":"845",
///         "url":"file:///home/suk/kiwi/synthetic-profile-service/node_modules/fast-check/lib/arbitrary/_internals/helpers/EnumerableKeysExtractor.js",
///         "startLine":0,
///         "startColumn":0,
///         "endLine":16,
///         "endColumn":0,
///         "executionContextId":1,
///         "hash":"1a5513906dbddc724ca3d0004c4e5c84212e6299",
///         "executionContextAuxData":{"isDefault":true},
///         "isLiveEdit":false,
///         "sourceMapURL":"",
///         "hasSourceURL":false,
///         "isModule":false,
///         "length":610,
///         "stackTrace":{
///            "callFrames":[
///                {"functionName":"compileFunction","scriptId":"89","url":"node:vm","lineNumber":351,"columnNumber":17}
///            ]
///         },
///         "scriptLanguage":"JavaScript",
///         "embedderName": "file:///home/suk/kiwi/synthetic-profile-service/node_modules/fast-check/lib/arbitrary/_internals/helpers/EnumerableKeysExtractor.js"
///     }
/// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResultResponse {
    result: Value,
}

#[derive(Debug)]
pub enum Response {
    DebuggerScriptParsed(ScriptParsedResponse),
    Result(ResultResponse),
    Unknown(Value),
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
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeRunScriptRequest {
    pub script_id: RuntimeScriptId,
}

impl RuntimeRunScriptRequest {
    pub fn new(script_id: RuntimeScriptId) -> RuntimeRunScriptRequest {
        RuntimeRunScriptRequest { script_id }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: i32,
    pub method: &'static str,
    pub params: Value,
}

impl Request {
    pub fn new(id: i32, method: &'static str) -> Request {
        let params = Value::Object(Map::new());
        Request { id, method, params }
    }

    pub fn new_with_params<T: Serialize>(
        id: i32,
        method: &'static str,
        params: &T,
    ) -> Result<Request, Error> {
        Ok(Request {
            id,
            method,
            params: serde_json::to_value(params)?,
        })
    }
}
