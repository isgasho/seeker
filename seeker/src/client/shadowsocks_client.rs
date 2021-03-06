use crate::client::Client;
use config::Address;
use ssclient::SSClient;
use std::io::Result;
use tun::socket::{TunTcpSocket, TunUdpSocket};

#[async_trait::async_trait]
impl Client for SSClient {
    async fn handle_tcp(&self, socket: TunTcpSocket, addr: Address) -> Result<()> {
        self.handle_tcp_connection(socket, addr).await
    }

    async fn handle_udp(&self, socket: TunUdpSocket, addr: Address) -> Result<()> {
        self.handle_udp_connection(socket, addr).await
    }
}
