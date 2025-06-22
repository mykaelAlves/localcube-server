use std::net::SocketAddr;

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tokio::task_local;

const CONFIG_FILE_PATH: &str = "config/Config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router = Router::new()
        .route("/", get(handler))
        .route_layer(middleware::from_fn(auth))
        .route("/api/ping", get(ping))
        .route("/api/server/init", post(init_mc_server))
        .route("/api/server/stop", post(close_mc_server))
        ;        

    let config_file = tokio::fs::read_to_string(CONFIG_FILE_PATH).await?;
    let config: Config = toml::from_str(&config_file)?;

    let addr = SocketAddr::from(
        (config.address.ip, config.address.port));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Deserialize)]
struct Config {
    address: Address
}

#[derive(Deserialize)]
struct Address {
    ip: [u8; 4],
    port: u16,
}

#[derive(Clone)]
struct CurrentUser {
    name: String,
}
task_local! {
    pub static USER: CurrentUser;
}

struct UserResponse;

impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        // State is accessed here in the IntoResponse implementation
        let current_user = USER.with(|u| u.clone());
        (StatusCode::OK, current_user.name).into_response()
    }
}

async fn init_mc_server() -> impl IntoResponse {
    // TODO
    Html("<p>Server initialized with Paper 1.16!</p>") 
}

async fn close_mc_server() -> impl IntoResponse {
    // TODO
    Html("<p>Server closed!</p>") 
}

async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if let Some(current_user) = authorize_current_user(auth_header).await {
        // State is setup here in the middleware
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    Some(CurrentUser {
        name: auth_token.to_string(),
    })
}

async fn handler() -> UserResponse {
    UserResponse
}

async fn ping() -> &'static str {
    "pong"
}