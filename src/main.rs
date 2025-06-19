use std::convert::Infallible;

use http_body_util::Full;
use hyper::{body::{self, Bytes}, Request, Response};

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