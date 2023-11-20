pub mod tcp_client {
    pub struct TcpClient {
        pub host: String,
        pub port: u16,
        pub id: u32,
        pub name: String,
        pub group: String,
    }

    impl TcpClient {
        pub fn new(host: String, port: u16, id: u32, name: String, group: String) -> TcpClient {
            TcpClient {
                host,
                port,
                id,
                name,
                group,
            }
        }
    }
    #[derive(Clone)]
    pub struct ChannelData {
        pub id: u32,
        pub name: String,
        pub group: String,
        pub data: String,
        pub is_notify: bool,
    }
}
