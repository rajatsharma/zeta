use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub mod handler;
pub mod listener;
pub mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(address).await?;

    println!("{} Listening", address);

    let state = Arc::new(RwLock::new(state::State {
        clients: HashMap::new(),
    }));
    println!("before listener");
    listener::listener(state.clone()).await?;
    println!("before loop");

    loop {
        println!("Inside loop");
        let (stream, address) = listener.accept().await?;
        let state = state.clone();
        println!("after clone state loop");
        tokio::task::spawn(async move {
            let io = hyper_util::rt::TokioIo::new(stream);
            let connection = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |request| {
                        handler::request_handler(request, address, state.clone())
                    }),
                )
                .with_upgrades();

            if let Err(err) = connection.await {
                eprintln!("Connection Error: {:?}", err);
            }
        });
    }
}
