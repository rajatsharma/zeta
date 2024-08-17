use crate::state::SharedState;
use fastwebsockets::upgrade::{upgrade, UpgradeFut};
use fastwebsockets::OpCode;
use fastwebsockets::WebSocketError;
use hyper::body::Incoming;
use hyper::Request;
use hyper::Response;
use std::net::SocketAddr;
use tokio::sync::mpsc;

pub async fn request_handler(
    mut request: Request<Incoming>,
    address: SocketAddr,
    state: SharedState,
) -> Result<Response<String>, WebSocketError> {
    let uri = request.uri().path();

    println!("{}", uri);

    match uri {
        "/ws" => {
            let (fut_response, fut) = upgrade(&mut request)?;
            tokio::spawn(async move {
                handle_ws(fut, address, &state).await.unwrap();

                {
                    let mut state = state.write().await;
                    state.clients.remove(&address);
                }
            });

            let mut response = Response::builder()
                .status(fut_response.status())
                .body("random".to_string())
                .unwrap();
            response.headers_mut().clone_from(fut_response.headers());
            Ok(response)
        }
        _ => {
            let response = Response::builder()
                .status(200)
                .body(
                    r#"
                    Use the JavaScript console to interact using websockets

                    sock  = new WebSocket("ws://127.0.0.1:8080/ws")
                    sock.addEventListener("message", console.log)
                    sock.addEventListener("open", () => sock.send("ping"))
                    "#
                    .to_string(),
                )
                .unwrap();
            Ok(response)
        }
    }
}

async fn handle_ws(
    fut: UpgradeFut,
    address: SocketAddr,
    state: &SharedState,
) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);
    let (tx, mut rx) = mpsc::channel(128);

    {
        let mut state = state.write().await;
        state.clients.insert(address, tx);
    }

    loop {
        tokio::select! {
            frame = ws.read_frame() => {
                let frame = frame?;
                match frame.opcode {
                    OpCode::Close => {
                      // break on close connection
                      break;
                    }
                    OpCode::Text => {
                      // Do something here or not
                    }
                    _ => {
                      // Ignore everything else
                    }
                }
            },
            frame = rx.recv() => {
              if let Some(f) = frame {
                  ws.write_frame(f.as_frame()).await?;
              }
            }
        }
    }

    Ok(())
}
