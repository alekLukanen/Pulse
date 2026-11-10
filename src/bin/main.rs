use clap::Parser;
use std::error::Error;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use pulse::pulse_db::db::worker::config::{WorkerArgs, WorkerConfig};
use pulse::pulse_db::db::worker::instance::Worker;

const TITLE: &str = r#"
     ____        _          ____  ____
    |  _ \ _   _| |___  ___|  _ \| __ )
    | |_) | | | | / __|/ _ \ | | |  _ \
    |  __/| |_| | \__ \  __/ |_| | |_) |
    |_|    \__,_|_|___/\___|____/|____/"#;

const HEARTBEAT: &str = r#"
_______________      /\       _______________
               \    /  \     /
                \/\/    \   /
                         \ /
                          V"#;

/// Print the startup banner, colored when stdout is a terminal.
fn print_banner() {
    use std::io::IsTerminal;

    let (cyan, red, _, reset) = if std::io::stdout().is_terminal() {
        ("\x1b[1;36m", "\x1b[1;31m", "\x1b[2m", "\x1b[0m")
    } else {
        ("", "", "", "")
    };

    println!("{cyan}{TITLE}{reset}");
    println!("{red}{HEARTBEAT}{reset}");
    println!("");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load the configuration
    let args = WorkerArgs::parse();
    args.validate()?;

    let worker_config = WorkerConfig::from_file(args.config_file)?;

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_max_level(worker_config.log_level())
        .init();

    print_banner();
    info!("config: {:?}", worker_config);

    // setup the worker
    info!("--- Starting PulseDB ---");

    let ct = CancellationToken::new();
    let worker = Worker::new(worker_config);
    if let Err(err) = worker.run(ct.child_token()).await {
        error!("{:?}", err);
    }

    info!("--- Closing PulseDB ---");

    Ok(())
}
