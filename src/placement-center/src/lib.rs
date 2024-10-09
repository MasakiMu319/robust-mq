use common_base::config::placement_center::placement_center_conf;
use log::info;
use server::{
    grpc::server::start_grpc_server,
    http::server::{start_http_server, HttpServerState},
};
use tokio::{signal, sync::broadcast};

pub mod server;
pub mod storage;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn start_server(stop_sx: broadcast::Sender<bool>) {
    let _config = placement_center_conf();

    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        start_grpc_server(raw_stop_sx).await;
    });

    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        let state = HttpServerState::new();
        start_http_server(state, raw_stop_sx).await;
    });

    awaiting_stop(stop_sx.clone()).await;
}

pub async fn awaiting_stop(stop_end: broadcast::Sender<bool>) {
    signal::ctrl_c().await.expect("failed to listen for event");
    match stop_end.send(true) {
        Ok(_) => {
            info!(
                "{}",
                "When ctrl + c is received, the service starts to stop"
            );
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
