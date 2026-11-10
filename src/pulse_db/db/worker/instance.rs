use std::net::SocketAddr;

use tokio_util::sync::CancellationToken;
use tonic::transport::Server;
use tracing::info;

use crate::{
    pkg::tokio_helpers,
    pulse_db::{
        db::worker::config::WorkerConfig,
        rpc::v1::{
            proto::pulse::pulse_connect_server::PulseConnectServer,
            server::pulse::PulseConnectService,
        },
    },
};

#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("SocketAddr failed parsing socket address: {0}")]
    SocketAddrError(#[from] std::net::AddrParseError),
    #[error("tonic transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct Worker {
    config: WorkerConfig,
}

impl Worker {
    pub fn new(config: WorkerConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self, ct: CancellationToken) -> Result<(), WorkerError> {
        tokio_helpers::cancel_on_ctrl_c(ct.clone());

        let addr: SocketAddr = self.config.server_address.parse()?;

        let pulse_connect = PulseConnectServer::new(PulseConnectService::new())
            .max_decoding_message_size(5 * 1024 * 1024)
            .max_encoding_message_size(5 * 1024 * 1024);

        info!("rpc server listening on {}", addr);

        Server::builder()
            .add_service(pulse_connect)
            .serve_with_shutdown(addr, ct.cancelled())
            .await?;

        info!("rpc server stopped");

        Ok(())
    }
}
