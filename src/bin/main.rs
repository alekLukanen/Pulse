use clap::Parser;
use std::error::Error;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use pulse::pulse_db::db::worker::config::{WorkerArgs, WorkerConfig};
use pulse::pulse_db::db::worker::instance::Worker;

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

