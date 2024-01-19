use axum::{
    response::IntoResponse,
    http::{StatusCode, header::{self, HeaderMap}},
    routing::get,
};
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async fn cors_preflight() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    (headers, StatusCode::NO_CONTENT)
}

async fn get_handler() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber)?;

    let (layer, io) = SocketIo::new_layer();

    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        info!("Socket.IO connected: {:?} {:?}", s.ns(), s.id);
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });
    });

    let app = axum::Router::new()
    .route("/", get(|| get_handler()).options(|| cors_preflight()))
    .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
