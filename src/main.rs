use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, get_service, post},
    Json, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tower_http::{cors::Any, cors::CorsLayer, services::ServeDir};

lazy_static! {
    static ref GLOBAL_U64: std::sync::Mutex<u64> = std::sync::Mutex::new(0);
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    //static file mounting
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("client/dist");
    let static_files =
        get_service(ServeDir::new(assets_dir).append_index_html_on_directories(true));

    // // build our application with a single route
    //mount the app routes and middleware
    let app = Router::new()
        .route("/api/ping", get(|| async { "Hello, World!" }))
        .route("/api/count", get(get_count))
        .route("/api/count", post(add_count))
        .nest_service("/", static_files.clone())
        .fallback_service(static_files)
        .layer(cors);

    let app = app.fallback(handle_403);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn get_count() -> Json<Count> {
        let count = GLOBAL_U64.lock().unwrap();

        Json(Count { count: *count })
    }

    async fn add_count() -> Json<Count> {
        // Lock the mutex to acquire the mutable reference
        let mut global_u64 = GLOBAL_U64.lock().unwrap();

        // Add 1 to the value
        *global_u64 += 1;

        Json(Count { count: *global_u64 })
    }

    // 403 handler
    async fn handle_403() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            axum::response::Json(serde_json::json!({
            "success":false,
            "message":String::from("The requested resource does not exist on this server!"),
            })),
        )
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Count {
    pub count: u64,
}
