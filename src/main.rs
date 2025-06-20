use std::{convert::Infallible, net::SocketAddr};
use std::fs;

use hyper::body::Body;
use hyper::server::conn::http1;
use hyper::server::conn::http2;
use hyper::{service, Method, StatusCode};
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use http_body_util::Full;
use hyper::{
    body::{
        self, Bytes
    },
    Request, Response
};
use tokio::net::TcpListener;

const PING_RESPONSE: &[u8] = b"pong";

#[derive(Debug, Deserialize)]
struct Config {
    address: Address
}

#[derive(Debug, Deserialize)]
struct Address {
    ip: [u8; 4],
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config: Config = toml::from_str(
        &fs::read_to_string("config/Config.toml").unwrap()
    ).unwrap();

    println!("{:#?}", config);

    let addr = SocketAddr::from((config.address.ip, config.address.port));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let io = TokioIo::new(socket);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service::service_fn(handler))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }

    Ok(())
}

async fn handler(r: Request<body::Incoming>) 
    -> Result<Response<Full<Bytes>>, Infallible> {
    
    match (r.method(), r.uri().path()) {
        (&Method::GET, "/api/ping") => Ok(Response::new(PING_RESPONSE.into())),
        _ => Ok(
            Response::new(
                Full::new(
                    Bytes::from(
                        "Not found"
                    )
                )
            )
        )
                
    }
}
