pub mod handler{

    use std::{sync::Arc, collections::HashMap};

    use tokio::{sync::{broadcast::{Sender, Receiver}, Mutex}, net::{TcpStream, tcp::{OwnedWriteHalf, OwnedReadHalf}}, io::{AsyncWriteExt, AsyncBufReadExt}};

    use crate::client::tcp_client::{ChannelData, TcpClient};

    pub async fn handler_connect(mut client_stream:TcpStream, tx:Sender<ChannelData>, id:u32,name_map:Arc<Mutex<HashMap<String,i32>>>){
        let mut chat_rs_str = String::from(r"
        ______     __  __     ______     ______   ______     ______    
       /\  ___\   /\ \_\ \   /\  __ \   /\__  _\ /\  == \   /\  ___\   
       \ \ \____  \ \  __ \  \ \  __ \  \/_/\ \/ \ \  __<   \ \___  \  
        \ \_____\  \ \_\ \_\  \ \_\ \_\    \ \_\  \ \_\ \_\  \/\_____\ 
         \/_____/   \/_/\/_/   \/_/\/_/     \/_/   \/_/ /_/   \/_____/ 
                                                                       
       ");
       chat_rs_str += "\n";
       chat_rs_str += "welcome to use chat-rs!\n";
       chat_rs_str += "1. use /nick <name> command to set your nick name.\n";
       chat_rs_str += "2. use /join <group name> command to join a group.\n";

        let host = client_stream.peer_addr().unwrap().ip().to_string();
        let port = client_stream.peer_addr().unwrap().port();
        if let Err(e) = client_stream.write_all(chat_rs_str.as_bytes()).await{
            println!("write error: {}", e);
            return;
        }

        let user_name: String = String::from("user");
        let user_group = String::from("0");
    
        let client = Arc::new(
            Mutex::new(
                TcpClient::new(
                    host.clone(), 
                    port.clone(), 
                    id, 
                    user_name.clone(), 
                    user_group.clone())
            )
        );
        // system notice
        let sys_notice_data = ChannelData{
            id : id,
            name: user_name.clone(),
            group: user_group.clone(),
            data: format!("[Notice] {} joined, EndPoint:{}:{}!",user_name.clone(),host.clone(),port.clone()),
            is_notify:false,
            sys_notice: true,
        };
        if tx.send(sys_notice_data).is_err(){
            eprintln!("system notice filed!");
        } 
        let client_clone = client.clone();
        let (client_reader,client_writer) = client_stream.into_split();
        let rx = tx.subscribe();
        let mut receive_task = tokio::spawn(receive_from_client(client_reader, tx, client,name_map));
        let mut send_task = tokio::spawn(send_to_client(client_writer, rx, client_clone));
    
        if let Err(_e) = tokio::try_join!(&mut receive_task, &mut send_task) {
            eprintln!("read_task/write_task terminated");
            receive_task.abort();
            send_task.abort();        
        }
    }

    async fn receive_from_client(reader: OwnedReadHalf, tx:Sender<ChannelData>, client:Arc<Mutex<TcpClient>>,name_map:Arc<Mutex<HashMap<String,i32>>>){
        let mut buf_reader = tokio::io::BufReader::new(reader);
        let mut buf = String::new();
        loop {
            match buf_reader.read_line(&mut buf).await{
                Ok(n) if n == 0 => {
                    eprintln!("client disconnected");
                    break;
                },
                Ok(_) => {
                    buf.pop();
                    let content = buf.drain(..).as_str().to_string();
                    let mut client = client.lock().await;
                    let mut names = name_map.lock().await;
                    let resp = format!("[{}]{} > {}",client.group,client.name,content);
                    println!("{}",resp);
                    let mut content_vec = content.split_whitespace();
                    let mut channel_data = ChannelData{
                        id: client.id,
                        name: client.name.clone(),
                        group: client.group.clone(),
                        data: resp,
                        is_notify:false,
                        sys_notice: false,
                    };
                    match content_vec.next(){
                        Some("/join")=>{
                            if client.name == "user" {
                                channel_data.is_notify = true;
                                channel_data.data = format!("[Notice] Please use /nick [yourName] to set your nickName first!")
                            }else{
                                client.group=content_vec.next().unwrap().to_string();
                                channel_data.is_notify = true;
                                channel_data.sys_notice = true;
                                channel_data.group = client.group.clone();
                                channel_data.data = format!("[Notice] {} join group {}!",client.name.clone(),client.group);
                            }
                            
                        },
                        Some("/nick")=>{
                            let nick_name = content_vec.next().unwrap().to_string();
                            if nick_name == client.name{
                                continue;
                            }
                            channel_data.is_notify = true;
                            if names.contains_key(&nick_name){
                                channel_data.data = format!("[Error] {} - Nickname is already taken!",nick_name);
                            }else{
                                names.remove(&client.name);
                                client.name = nick_name.clone();
                                channel_data.data = format!("[Notice] You nick is {}!",client.name);
                                names.insert(nick_name.clone(), 1);
                            }
                            
                        }
                        _=>{ 
                            if client.name == "user" {
                                channel_data.is_notify = true;
                                channel_data.data = format!("[Error] Please use /nick [yourName] to set your nickName first!")
                            }
                        }
                    }
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
            let mut channel_data = rx.recv().await.unwrap();
            let client = client.lock().await;
            if channel_data.sys_notice && channel_data.is_notify{
                if client.group == channel_data.group{
                    channel_data.data.push('\n');
                    if let Err(e) = write.write_all(channel_data.data.as_bytes()).await{
                        eprintln!("write error: {}", e);
                        break;
                    }
                }
                continue;
            }
            if channel_data.sys_notice && client.id == channel_data.id{
                continue;
            }
            if !channel_data.is_notify && client.id == channel_data.id{
                continue;
            }
            if channel_data.is_notify && client.id != channel_data.id{
                continue;
            }
            if client.group == channel_data.group{
                channel_data.data.push('\n');
                if let Err(e) = write.write_all(channel_data.data.as_bytes()).await{
                    eprintln!("write error: {}", e);
                    break;
                }
            }
            
        }
    }
    
}