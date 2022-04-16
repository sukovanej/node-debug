mod cdt;
mod source_code;

use crate::cdt::client::CDTClient;
use crate::cdt::http_client::get_debuggers;

use crate::source_code::SourceCode;

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
    client.debugger_enable().unwrap();
    client.debugger_set_pause_on_exception().unwrap();
    client.profiler_enable().unwrap();
    let paused_message = client.runtime_run_if_waiting_for_debugger().unwrap();

    let call_frame = &paused_message.params.call_frames[0];
    let top_level_script_id = call_frame.location.script_id.to_owned();

    println!("debugger paused on {:?}", call_frame);

    let source = client
        .debugger_get_script_source(top_level_script_id)
        .unwrap();

    let source_code = SourceCode::from_str(&source.result.script_source);
    let mapping_content = &source_code.source_mapping.unwrap().sources_content[0];

    println!("Paused on {}", mapping_content);

    let expression = "String([provider, 1 + 2])";
    let call_frame_id = &call_frame.call_frame_id;
    let remote_object = client
        .debugger_evaluate_on_call_frame(call_frame_id.to_owned(), expression)
        .unwrap();

    println!("evaluated: {:?}", remote_object);

    Ok(())
}

fn main() {
    run().unwrap()
}
