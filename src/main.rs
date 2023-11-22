pub mod client;
pub mod handler;

use std::{sync::Arc, collections::HashMap};

use crate::handler::handler::handler_connect;

use client::tcp_client::ChannelData;
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1212").await.unwrap();
    // 定义广播
    let (tx, _) = broadcast::channel::<ChannelData>(16);
    let map = Arc::new(Mutex::new(HashMap::<String, i32>::new()));
    // let map_clone = Arc::clone(&map);
    let mut id: u32 = 0;
    loop {
        let (client_stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handler_connect(client_stream, tx.clone(), id,map.clone()));
        id += 1;
    }
}
