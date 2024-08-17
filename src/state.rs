use fastwebsockets::Frame;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Pong(Vec<u8>),
    Close(u16, String), // Code, Reason
}
impl Message {
    pub fn as_frame(&self) -> Frame {
        match self {
            Message::Text(text) => Frame::text(text.as_bytes().into()),
            Message::Binary(data) => Frame::binary(data.as_slice().into()),
            Message::Pong(data) => Frame::pong(data.as_slice().into()),
            Message::Close(code, reason) => Frame::close(*code, reason.as_bytes()),
        }
    }
}

pub struct State {
    pub clients: HashMap<SocketAddr, Tx>,
}

pub type Tx = Sender<Message>;
pub type SharedState = Arc<RwLock<State>>;
