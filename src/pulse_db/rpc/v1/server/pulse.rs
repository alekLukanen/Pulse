use crate::pulse_db::rpc::v1::proto::pulse;

#[derive(Debug, Default)]
pub struct PulseConnectService {}

impl PulseConnectService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_server(self) -> pulse::pulse_connect_server::PulseConnectServer<Self> {
        pulse::pulse_connect_server::PulseConnectServer::new(self)
    }
}

#[tonic::async_trait]
impl pulse::pulse_connect_server::PulseConnect for PulseConnectService {
    async fn connect_worker(
        &self,
        _request: tonic::Request<pulse::ConnectWorkerRequest>,
    ) -> std::result::Result<tonic::Response<pulse::ConnectWorkerResponse>, tonic::Status> {
        Ok(tonic::Response::new(pulse::ConnectWorkerResponse {
            id: String::new(),
        }))
    }
}
