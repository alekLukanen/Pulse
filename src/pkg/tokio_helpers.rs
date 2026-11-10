use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub fn cancel_on_ctrl_c(ct: CancellationToken) {
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => info!("received ctrl-c, shutting down"),
            Err(err) => error!("failed to listen for ctrl-c: {:?}", err),
        }
        ct.cancel();
    });
}
