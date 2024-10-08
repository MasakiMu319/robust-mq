use super::server::HttpServerState;
use axum::extract::State;
use common_base::http_response::success_response;

pub async fn index(State(_): State<HttpServerState>) -> String {
    return success_response("{}");
}
