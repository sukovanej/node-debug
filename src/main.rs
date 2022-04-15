mod cdt;
mod http_client;

use cdt::client::CDTClient;
use http_client::get_debuggers;

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
    client.debugger_enable().unwrap();
    println!("received: {:?}", client.read_messages_until_result()?.last());
    client
        .runtime_run_if_waiting_for_debugger(10.to_string())
        .unwrap();
    println!("received: {:?}", client.read_messages_until_result()?.last());

    Ok(())
}

fn main() {
    run().unwrap()
}
