use common_base::config::placement_center::placement_center_conf;
use log::info;
use protocol::kv::kv_service_server::KvServiceServer;
use tokio::{select, sync::broadcast};
use tonic::transport::Server;

use crate::server::grpc::services_kv::GrpcBrokerServices;

pub struct GrpcServer {
    port: usize,
}

impl GrpcServer {
    pub fn new(port: usize) -> Self {
        Self { port }
    }

    pub async fn start(&self, stop_sx: broadcast::Sender<bool>) {
        let addr = format!("0.0.0.0:{}", self.port).parse().unwrap();
        info!("Broker Grpc Server start. port:{}", self.port);
        let service_handler = GrpcBrokerServices::new();
        let mut stop_rx = stop_sx.subscribe();
        select! {
            val = stop_rx.recv() => {
                match val {
                    Ok(flag) => {
                        if flag {
                            info!("HTTP Server Stopped successfully");
                        }
                    }
                    Err(_) => {}
                }
            },
            val = Server::builder().add_service(KvServiceServer::new(service_handler)).serve(addr) => {
                match val {
                    Ok(()) => {},
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
        }
    }
}

pub async fn start_grpc_server(stop_sx: broadcast::Sender<bool>) {
    let config = placement_center_conf();
    let server = GrpcServer::new(config.grpc_port);
    server.start(stop_sx).await;
}
