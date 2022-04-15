mod cdt;
mod source_code;
mod http_client;

use cdt::client::CDTClient;
use http_client::get_debuggers;

use crate::{cdt::models::Response, source_code::SourceCode};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let host = "127.0.0.1";
    let port = "9229";

    let debuggers = get_debuggers(host, port)?;
    let id = if debuggers.len() != 1 {
        Err("")
    } else {
        Ok(debuggers[0].id.to_owned())
    }?;

    let mut client = CDTClient::new(host, port, id.as_str());

    client.runtime_enable().unwrap();

    let messages = client.read_messages_until_result()?;
    println!("runtime enabled: {:?}", messages.last());

    client.debugger_enable().unwrap();

    let messages = client.read_messages_until_result()?;
    println!("debugger enabled: {:?}", messages.last());

    client.debugger_set_pause_on_exception().unwrap();

    client.profiler_enable().unwrap();

    let messages = client.read_messages_until_result()?;
    println!("profiler enabled: {:?}", messages.last());

    client.runtime_run_if_waiting_for_debugger().unwrap();

    let messages = client.read_messages_until_result()?;
    println!("runtime_run_if_waiting_for_debugger: {:?}", messages.last());

    let messages = client.read_messages_until_paused()?;
    let paused_message = match messages.last().unwrap() {
        Response::DebuggerPaused(msg) => msg,
        _ => panic!("debugger_paused expected"),
    };
    println!("debugger paused");

    let top_level_script_id = paused_message.params.call_frames[0]
        .location
        .script_id
        .to_owned();

    let source = client
        .debugger_get_script_source(top_level_script_id)
        .unwrap();

    let source_code = SourceCode::from_str(&source.result.script_source);

    println!("{:?}", source_code);

    Ok(())
}

fn main() {
    run().unwrap()
}
