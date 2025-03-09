use std::net::SocketAddr;

use axum::extract::Request;
use axum::http::{Method, StatusCode};
use axum::middleware;
use axum::routing::post;
use axum::{
    routing::get,
    Router,
    response::Response,
    middleware::Next,
    extract::ConnectInfo,
};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers;
use crate::models::AppState;
use crate::go_api::GoApi;
use crate::db_provider::DbProvider;

// middleware to check that request came from localhost
async fn localhost_middleware(ConnectInfo(addr): ConnectInfo<SocketAddr>, req: Request, next: Next) -> Result<Response, StatusCode>{
    if addr.ip().is_loopback() {
        return Ok(next.run(req).await);
    }
    
    Err(StatusCode::FORBIDDEN)
}

pub async fn create_router() -> Router {
    let db_provider = DbProvider::new().await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let api_key = std::env::var("GO_API_KEY").expect("failed to retrieve api key");
    let api = GoApi::new(&api_key);

    let state = AppState { db_provider, api };

    Router::new()
        .route("/api/stops", get(handlers::get_stops))
        .route("/api/stop/:stop_id", get(handlers::get_stop))
        .route("/api/stop/search/:stop_name", get(handlers::search_for_stop))
        .route("/api/route/stop/:stop_id", get(handlers::get_route_from_stop))
        .route("/api/routes", get(handlers::get_all_routes))
        .route("/api/route/:route_id", get(handlers::get_ordered_stops_for_route))
        .route("/api/route/:route_id/stop/:stop_id", get(handlers::get_stop_times))
        .route("/api/admin/reload_db", post(handlers::reload_db))
        .route_layer(middleware::from_fn(localhost_middleware))
        .with_state(state)
        .layer(cors)
}