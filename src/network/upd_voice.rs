use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use log::info;

use crate::{states::server::ServerState, VERSION};

pub struct SrsVoiceServer {
    socket: std::net::UdpSocket,
    connections: Vec<SocketAddr>,
    state: Arc<RwLock<ServerState>>,
}

impl SrsVoiceServer {
    pub fn new(address: &String, port: &u16) -> std::io::Result<Self> {
        let socket = std::net::UdpSocket::bind(format!("{}:{}", address, port))?;
        Ok(Self {
            socket,
            connections: Vec::new(),
            state: Arc::new(RwLock::new(ServerState {
                clients: std::collections::HashMap::new(),
                options: Default::default(),
                version: VERSION.to_owned(),
            })),
        })
    }

    pub fn start(&mut self, state: Arc<RwLock<ServerState>>) -> std::io::Result<()> {
        self.state = state;
        let mut buf = [0; 1024];
        info!("Voice Server started on {}", self.socket.local_addr()?);
        loop {
            let (amt, src) = self.socket.recv_from(&mut buf)?;
            println!("Received {} bytes from {}", amt, src);
            println!("{}", String::from_utf8_lossy(&buf));
        }
    }
}
