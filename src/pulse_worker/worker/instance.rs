use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::pkg::tokio_helpers;
use crate::pulse_db::rpc::v1::proto::pulse::pulse_connect_client::PulseConnectClient;
use crate::pulse_db::rpc::v1::proto::pulse::ConnectWorkerRequest;
use crate::pulse_worker::worker::config::WorkerConfig;

#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("tonic transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
    #[error("rpc request failed: {0}")]
    StatusError(#[from] tonic::Status),
}

pub struct Worker {
    config: WorkerConfig,

    id: uuid::Uuid,
}

impl Worker {
    pub fn new(config: WorkerConfig) -> Self {
        Self {
            config,
            id: uuid::Uuid::new_v4(),
        }
    }

    pub fn id_name(&self) -> String {
        format!("{}-{}", self.config.name, self.id.to_string())
    }

    pub async fn run(&self, ct: CancellationToken) -> Result<(), WorkerError> {
        tokio_helpers::cancel_on_ctrl_c(ct.clone());

        info!("connecting to {}", self.config.server_address);

        // server_address must include the scheme, e.g. "http://127.0.0.1:7001"
        let mut pulse_client = PulseConnectClient::connect(self.config.server_address.clone())
            .await?
            .max_decoding_message_size(5 * 1024 * 1024)
            .max_encoding_message_size(5 * 1024 * 1024);

        let connect_resp = pulse_client
            .connect_worker(tonic::Request::new(ConnectWorkerRequest {
                name: self.id_name(),
            }))
            .await?
            .into_inner();

        info!(
            "connected to {} with id: {}",
            self.config.server_address, connect_resp.id
        );

        ct.cancelled().await;

        Ok(())
    }
}
