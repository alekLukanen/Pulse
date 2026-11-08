use tokio_util::sync::CancellationToken;

use crate::pulse_db::db::worker::config::WorkerConfig;

#[derive(Debug, thiserror::Error)]
pub enum WorkerError {}

pub struct Worker {}

impl Worker {
    pub fn new(config: WorkerConfig) -> Self {
        Self {}
    }

    pub async fn run(&self, ct: CancellationToken) -> Result<(), WorkerError> {
        Ok(())
    }
}
