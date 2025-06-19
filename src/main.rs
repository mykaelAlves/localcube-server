use std::convert::Infallible;
use std::fs;

use serde::Deserialize;
use http_body_util::Full;
use hyper::{
    body::{
        self, Bytes
    },
    Request, Response
};

#[derive(Debug, Deserialize)]
struct Config {
    address: Address
}

#[derive(Debug, Deserialize)]
struct Address {
    ip: Vec<u8>,
    port: u16,
}

fn main() {
    let config: Config = toml::from_str(
        &fs::read_to_string("config/Config.toml").unwrap()
    ).unwrap();

    println!("{:#?}", config);
}

async fn hello(_: Request<body::Incoming>) 
    -> Result<Response<Full<Bytes>>, Infallible> {
    
    Ok(Response::new(
        Full::new(
            Bytes::from_static(b"Hello, world!")
        )
    ))
}
