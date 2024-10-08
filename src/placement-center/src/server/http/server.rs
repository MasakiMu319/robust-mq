use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use common_base::config::placement_center::placement_center_conf;
use log::info;
use tokio::{select, sync::broadcast};

use super::{index::index, path_create, path_delete, path_list, path_update, v1_path};

pub const ROUTE_ROOT: &str = "/index";

#[derive(Debug, Clone)]
pub struct HttpServerState {}

impl HttpServerState {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn start_http_server(state: HttpServerState, stop_sx: broadcast::Sender<bool>) {
    let config = placement_center_conf();
    let ip: SocketAddr = match format!("0.0.0.0:{}", config.http_port).parse() {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };

    info!("Broker HTTP Server start. port: {}", config.http_port);

    let app = routes(state);

    let mut stop_rx = stop_sx.subscribe();

    let listener = match tokio::net::TcpListener::bind(ip).await {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e)
        }
    };

    select! {
        val = stop_rx.recv() => {
            match val {
                Ok(flag) => {
                    if flag {
                        info!("HTTP Server stopped successfully");
                    }
                },
                Err(_) => {}
            }
        },

        val = axum::serve(listener, app.clone()) => {
            match val {
                Ok(()) => {},
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }
}

fn routes(state: HttpServerState) -> Router {
    let common = Router::new()
        .route(&v1_path(&path_list(ROUTE_ROOT)), get(index))
        .route(&v1_path(&path_create(ROUTE_ROOT)), post(index))
        .route(&v1_path(&path_update(ROUTE_ROOT)), put(index))
        .route(&v1_path(&path_delete(ROUTE_ROOT)), delete(index));

    let app = Router::new().merge(common);
    return app.with_state(state);
}
