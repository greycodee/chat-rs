pub mod client;
pub mod handler;

use crate::handler::handler::handler_connect;

use client::tcp_client::ChannelData;
use tokio::{
    net::TcpListener,
    sync::broadcast,
};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1212").await.unwrap();
    // 定义广播
    let (tx, _) = broadcast::channel::<ChannelData>(16);
    let mut id: u32 = 0;
    loop {
        let (client_stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handler_connect(client_stream, tx.clone(), id));
        id += 1;
    }
}
