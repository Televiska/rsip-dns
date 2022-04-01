use rsip::{Port, Transport};
use std::net::{IpAddr, SocketAddr};

/// The (ip, port, transport) tuple resolved that should be used as the next peer target.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Target {
    pub ip_addr: IpAddr,
    pub port: Port,
    pub transport: Transport,
}

impl Target {
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.ip_addr, self.port.into()))
    }
}

impl From<(IpAddr, Port, Transport)> for Target {
    fn from(from: (IpAddr, Port, Transport)) -> Target {
        let (ip_addr, port, transport) = from;

        Target { ip_addr, port, transport }
    }
}
