pub mod handler{

    use std::sync::Arc;

    use tokio::{sync::{broadcast::{Sender, Receiver}, Mutex}, net::{TcpStream, tcp::{OwnedWriteHalf, OwnedReadHalf}}, io::{AsyncWriteExt, AsyncBufReadExt}};

    use crate::client::tcp_client::{ChannelData, TcpClient};

    pub async fn handler_connect(mut client_stream:TcpStream, tx:Sender<ChannelData>, id:u32){
        let hello_msg = String::from("hello");
        let host = client_stream.peer_addr().unwrap().ip().to_string();
        let port = client_stream.peer_addr().unwrap().port();
        if let Err(e) = client_stream.write_all(hello_msg.as_bytes()).await{
            println!("write error: {}", e);
            return;
        }
    
        let client = Arc::new(
            Mutex::new(
                TcpClient::new(
                    host, 
                    port, 
                    id, 
                    "user".to_string(), 
                    "0".to_string())
            )
        );
    
        let client_clone = client.clone();
        let (client_reader,client_writer) = client_stream.into_split();
        let rx = tx.subscribe();
    
        let mut receive_task = tokio::spawn(receive_from_client(client_reader, tx, client));
        let mut send_task = tokio::spawn(send_to_client(client_writer, rx, client_clone));
    
        if let Err(_e) = tokio::try_join!(&mut receive_task, &mut send_task) {
            eprintln!("read_task/write_task terminated");
            receive_task.abort();
            send_task.abort();        
        }
    }

    async fn receive_from_client(reader: OwnedReadHalf, tx:Sender<ChannelData>, client:Arc<Mutex<TcpClient>>){
        let mut buf_reader = tokio::io::BufReader::new(reader);
        let mut buf = String::new();
        loop {
            match buf_reader.read_line(&mut buf).await{
                Ok(n) if n == 0 => {
                    eprintln!("client disconnected");
                    break;
                },
                Ok(_) => {

                    println!("receive from client: {}", buf);
                    let client = client.lock().await;
                    let channel_data = ChannelData{
                        id: client.id,
                        name: client.name.clone(),
                        group: client.group.clone(),
                        data: buf.drain(..).as_str().to_string(),
                    };
                    if tx.send(channel_data).is_err(){
                        eprintln!("channel closed");
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("read error: {}", e);
                    break;
                }
            }
        }
    }
    
    
    async fn send_to_client(mut write:OwnedWriteHalf, mut rx: Receiver<ChannelData>, client:Arc<Mutex<TcpClient>>){
        loop {
            let channel_data = rx.recv().await.unwrap();
            let client = client.lock().await;
            if client.id == channel_data.id{
                continue;
            }
            if let Err(e) = write.write_all(channel_data.data.as_bytes()).await{
                eprintln!("write error: {}", e);
                break;
            }
        }
    }
    
}