## TODO

 - set max timeout for blocking websocket calls
 - continue-to command - continue to a position
 - ns command - combination of next and show

## CDT Protocol notes

Enable debugger

```json
{"id": 1, "method": "Debugger.enable"}
```

Resume debugger

```json
{"id": 1, "method": "Debugger.resume"}
```

Pause debugger

```json
{"id": 1, "method": "Debugger.pause"}
```

Run if waiting for debugger

```json
{"id": 1, "method": "Runtime.runIfWaitingForDebugger"}
```

Get possible breakpoints

```json
{"id": 1, "method": "Debugger.getPossibleBreakpoints", "params": { "start": {"lineNumber": 0, "scriptId": "100"}}}
```

Step over

```json
{"id": 1, "method": "Debugger.stepOver"}
```

Get script source

```json
{"id": 1, "method": "Debugger.getScriptSource", "params": { "scriptId": "138"}}
```

Step into

```json
{"id": 1, "method": "Debugger.stepInto"}
```

Chromium devtools comm

```json
{"id":1,"method":"Runtime.enable","params":{}}
{"id":2,"method":"Debugger.enable","params":{"maxScriptsCacheSize":100000000}}
{"id":3,"method":"Debugger.setPauseOnExceptions","params":{"state":"none"}}
{"id":4,"method":"Debugger.setAsyncCallStackDepth","params":{"maxDepth":32}}
{"id":1,"result":{}}
{"id":5,"method":"Profiler.enable","params":{}}
{"id":6,"method":"Debugger.setBlackboxPatterns","params":{"patterns":[]}}
{"id":7,"method":"Runtime.runIfWaitingForDebugger","params":{}}
```
