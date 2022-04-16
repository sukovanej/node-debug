mod cdt;
mod repl;
mod source_code;

use clap::Parser;

use crate::cdt::http_client::get_debuggers;

use crate::repl::start_repl;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value = "9229")]
    port: String,
}

fn main() {
    let args = Args::parse();

    let debuggers = get_debuggers(&args.host, &args.port);

    let debugger_id = match debuggers {
        Ok(debuggers) => debuggers[0].id.to_owned(),
        _ => {
            println!(
                "Error while fetch {}:{}, is it running?",
                &args.host, &args.port
            );
            std::process::exit(1);
        }
    };

    start_repl(&args.host, &args.port, &debugger_id);
}
