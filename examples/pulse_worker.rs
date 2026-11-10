//! Boots a `pulse_worker` Worker that connects to a running PulseDB worker.
//!
//! Start the database worker first, then run this example:
//!
//! ```sh
//! cargo run --bin main -- --config-file examples/config/db_worker_config.json
//! cargo run --example pulse_worker -- --config-file examples/config/pulse_worker_config.json
//! ```
//!
//! The config's `server_address` is the PulseDB worker's rpc address.
//! Press ctrl-c to shut the worker down.

use std::error::Error;

use clap::Parser;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use pulse::pulse_worker::worker::config::{WorkerArgs, WorkerConfig};
use pulse::pulse_worker::worker::instance::Worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load the configuration from the .json file given by --config-file
    let args = WorkerArgs::parse();
    args.validate()?;

    let config = WorkerConfig::from_file(args.config_file)?;

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_max_level(config.log_level())
        .init();

    info!("--- Starting pulse worker ---");

    // The worker runs until this token is cancelled. `Worker::run` already
    // cancels it on ctrl-c; call `ct.cancel()` to stop it from your own code.
    let ct = CancellationToken::new();

    let worker = Worker::new(config);
    if let Err(err) = worker.run(ct.child_token()).await {
        error!("{:?}", err);
    }

    info!("--- Closing pulse worker ---");

    Ok(())
}
