use std::convert::Infallible;

use serde::Deserialize;
use http_body_util::Full;
use hyper::{
    body::{
        self, Bytes
    },
    Request, Response
};

struct Config {
    address: Address
}

struct Address {
    ip: Vec<u8>,
    port: u16,
}

fn main() {
    println!("Hello, world!");
}

async fn hello(_: Request<body::Incoming>) 
    -> Result<Response<Full<Bytes>>, Infallible> {
    
    Ok(Response::new(
        Full::new(
            Bytes::from_static(b"Hello, world!")
        )
    ))
}
