use std::{sync::Arc, collections::HashMap};
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};
use core::handler;
use core::client;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1212").await.unwrap();
    // 定义广播
    let (tx, _) = broadcast::channel::<client::ChannelData>(16);
    let map = Arc::new(Mutex::new(HashMap::<String, i32>::new()));
    let mut id: u32 = 0;
    loop {
        let (client_stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handler::handler_connect(client_stream, tx.clone(), id,map.clone()));
        id += 1;
    }
}